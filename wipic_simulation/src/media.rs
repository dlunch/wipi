use std::{
    collections::HashMap,
    panic::{AssertUnwindSafe, catch_unwind},
    sync::{
        Mutex,
        mpsc::{self, Receiver, Sender},
    },
    thread,
    time::{Duration, Instant},
};

use midir::{MidiOutput, MidiOutputConnection};
use once_cell::sync::Lazy;
use rodio::{OutputStreamBuilder, Sink, buffer::SamplesBuffer, conversions::SampleTypeConverter};
use smaf_player::{SmafEvent, parse_smaf};
use wipi_types::wipic::WIPICWord;

const ERR_INVALID: i32 = -9;
const ERR_INVALID_HANDLE: i32 = -25;
const CONTROL_TICK: Duration = Duration::from_millis(5);
const DEFAULT_VOLUME: u8 = 100;

enum PlaybackCommand {
    Stop,
    Pause,
    Resume,
    SetVolume(u8),
}

struct ClipState {
    data: Vec<u8>,
    volume: u8,
    playbacks: Vec<Sender<PlaybackCommand>>,
}

struct MediaState {
    next_clip_id: WIPICWord,
    clips: HashMap<WIPICWord, ClipState>,
}

impl MediaState {
    fn new() -> Self {
        Self {
            next_clip_id: 1,
            clips: HashMap::new(),
        }
    }
}

static MEDIA_STATE: Lazy<Mutex<MediaState>> = Lazy::new(|| Mutex::new(MediaState::new()));

pub fn clip_create(_clip_type: WIPICWord, _buf_size: WIPICWord, _callback: WIPICWord) -> i32 {
    let mut media = MEDIA_STATE.lock().unwrap();
    let clip_id = media.next_clip_id;
    media.next_clip_id = media.next_clip_id.saturating_add(1);

    media.clips.insert(
        clip_id,
        ClipState {
            data: Vec::new(),
            volume: DEFAULT_VOLUME,
            playbacks: Vec::new(),
        },
    );

    clip_id as i32
}

pub fn clip_free(clip: WIPICWord) -> i32 {
    let clip_state = MEDIA_STATE.lock().unwrap().clips.remove(&clip);
    let Some(clip_state) = clip_state else {
        return ERR_INVALID_HANDLE;
    };

    stop_playbacks(clip_state.playbacks);

    0
}

/// # Safety
/// `buf` must point to a buffer of `buf_size` bytes.
pub unsafe fn clip_put_data(clip: WIPICWord, buf: *const u8, buf_size: usize) -> i32 {
    let data = if buf_size == 0 {
        Vec::new()
    } else {
        if buf.is_null() {
            return ERR_INVALID;
        }
        unsafe { std::slice::from_raw_parts(buf, buf_size) }.to_vec()
    };

    let parse_result = catch_unwind(AssertUnwindSafe(|| parse_smaf(&data)));
    let Ok(events) = parse_result else {
        return ERR_INVALID;
    };
    if events.is_empty() {
        return ERR_INVALID;
    }

    let mut media = MEDIA_STATE.lock().unwrap();
    let Some(clip_state) = media.clips.get_mut(&clip) else {
        return ERR_INVALID_HANDLE;
    };

    clip_state.data = data;

    buf_size as i32
}

pub fn play(clip: WIPICWord, repeat: bool) -> i32 {
    let (volume, tx) = {
        let mut media = MEDIA_STATE.lock().unwrap();
        let Some(clip_state) = media.clips.get_mut(&clip) else {
            return ERR_INVALID_HANDLE;
        };

        if clip_state.data.is_empty() {
            return ERR_INVALID;
        }

        let (tx, rx) = mpsc::channel();
        let data = clip_state.data.clone();
        let volume = clip_state.volume;
        clip_state.playbacks.push(tx.clone());

        thread::spawn(move || playback_worker(rx, data, repeat, volume));

        (clip_state.volume, tx)
    };

    // Keep the sender in the clip state and set the initial volume on worker start.
    let _ = tx.send(PlaybackCommand::SetVolume(volume));

    0
}

pub fn stop(clip: WIPICWord) -> i32 {
    control_playbacks(clip, PlaybackCommand::Stop)
}

pub fn pause(clip: WIPICWord) -> i32 {
    control_playbacks(clip, PlaybackCommand::Pause)
}

pub fn resume(clip: WIPICWord) -> i32 {
    control_playbacks(clip, PlaybackCommand::Resume)
}

pub fn clip_get_volume(clip: WIPICWord) -> i32 {
    let media = MEDIA_STATE.lock().unwrap();
    let Some(clip_state) = media.clips.get(&clip) else {
        return ERR_INVALID_HANDLE;
    };

    clip_state.volume as i32
}

pub fn clip_set_volume(clip: WIPICWord, volume: u8) -> i32 {
    let mut media = MEDIA_STATE.lock().unwrap();
    let Some(clip_state) = media.clips.get_mut(&clip) else {
        return ERR_INVALID_HANDLE;
    };

    let level = volume.min(100);
    clip_state.volume = level;
    clip_state
        .playbacks
        .retain(|playback| playback.send(PlaybackCommand::SetVolume(level)).is_ok());

    0
}

fn control_playbacks(clip: WIPICWord, command: PlaybackCommand) -> i32 {
    let mut media = MEDIA_STATE.lock().unwrap();
    let Some(clip_state) = media.clips.get_mut(&clip) else {
        return ERR_INVALID_HANDLE;
    };

    clip_state
        .playbacks
        .retain(|playback| playback.send(command_clone(&command)).is_ok());

    0
}

fn command_clone(command: &PlaybackCommand) -> PlaybackCommand {
    match command {
        PlaybackCommand::Stop => PlaybackCommand::Stop,
        PlaybackCommand::Pause => PlaybackCommand::Pause,
        PlaybackCommand::Resume => PlaybackCommand::Resume,
        PlaybackCommand::SetVolume(volume) => PlaybackCommand::SetVolume(*volume),
    }
}

fn stop_playbacks(playbacks: Vec<Sender<PlaybackCommand>>) {
    for playback in playbacks {
        let _ = playback.send(PlaybackCommand::Stop);
    }
}

fn playback_worker(rx: Receiver<PlaybackCommand>, data: Vec<u8>, repeat: bool, initial_volume: u8) {
    let mut volume = initial_volume.min(100);

    let output_stream = OutputStreamBuilder::open_default_stream().ok();
    let sink = output_stream
        .as_ref()
        .map(|stream| Sink::connect_new(stream.mixer()));
    if let Some(sink) = sink.as_ref() {
        sink.set_volume(volume as f32 / 100.0);
    }

    let mut midi_out = open_midi_connection();
    let mut active_notes: Vec<(u8, u8)> = Vec::new();

    let events = parse_smaf(&data);
    if events.is_empty() {
        return;
    }

    'playback: loop {
        let mut paused = false;
        let mut pause_started: Option<Instant> = None;
        let mut paused_total = Duration::from_millis(0);
        let start = Instant::now();

        for (time, event) in &events {
            let event_time = Duration::from_millis(*time as u64);
            loop {
                if process_commands(
                    &rx,
                    &sink,
                    &mut volume,
                    &mut paused,
                    &mut pause_started,
                    &mut paused_total,
                ) {
                    break 'playback;
                }

                if paused {
                    thread::sleep(CONTROL_TICK);
                    continue;
                }

                let elapsed = Instant::now().saturating_duration_since(start);
                let elapsed = elapsed.saturating_sub(paused_total);
                if elapsed >= event_time {
                    break;
                }

                let remaining = event_time.saturating_sub(elapsed);
                thread::sleep(remaining.min(CONTROL_TICK));
            }

            handle_event(
                event,
                volume,
                sink.as_ref(),
                midi_out.as_mut(),
                &mut active_notes,
            );
        }

        if !repeat {
            break;
        }
    }

    for (channel, note) in &active_notes {
        let _ = send_midi_message(midi_out.as_mut(), [0x80 | (*channel & 0x0f), *note, 0]);
    }
}

fn process_commands(
    rx: &Receiver<PlaybackCommand>,
    sink: &Option<Sink>,
    volume: &mut u8,
    paused: &mut bool,
    pause_started: &mut Option<Instant>,
    paused_total: &mut Duration,
) -> bool {
    loop {
        let command = match rx.try_recv() {
            Ok(command) => command,
            Err(mpsc::TryRecvError::Empty) => return false,
            Err(mpsc::TryRecvError::Disconnected) => return true,
        };

        match command {
            PlaybackCommand::Stop => return true,
            PlaybackCommand::Pause => {
                if !*paused {
                    *paused = true;
                    *pause_started = Some(Instant::now());
                    if let Some(sink) = sink.as_ref() {
                        sink.pause();
                    }
                }
            }
            PlaybackCommand::Resume => {
                if *paused {
                    *paused = false;
                    if let Some(started) = pause_started.take() {
                        *paused_total += Instant::now().saturating_duration_since(started);
                    }
                    if let Some(sink) = sink.as_ref() {
                        sink.play();
                    }
                }
            }
            PlaybackCommand::SetVolume(level) => {
                *volume = level.min(100);
                if let Some(sink) = sink.as_ref() {
                    sink.set_volume(*volume as f32 / 100.0);
                }
            }
        }
    }
}

fn handle_event(
    event: &SmafEvent,
    volume: u8,
    sink: Option<&Sink>,
    midi_out: Option<&mut MidiOutputConnection>,
    active_notes: &mut Vec<(u8, u8)>,
) {
    match event {
        SmafEvent::Wave {
            channel,
            sampling_rate,
            data,
        } => {
            let Some(sink) = sink else {
                return;
            };

            let buffer = SamplesBuffer::new(
                *channel as u16,
                *sampling_rate,
                SampleTypeConverter::new(data.iter().copied()).collect::<Vec<_>>(),
            );
            sink.append(buffer);
            sink.set_volume(volume as f32 / 100.0);
        }
        SmafEvent::MidiNoteOn {
            channel,
            note,
            velocity,
        } => {
            let scaled_velocity = ((*velocity as u16 * volume as u16) / 100).min(127) as u8;
            let _ = send_midi_message(midi_out, [0x90 | (*channel & 0x0f), *note, scaled_velocity]);
            active_notes.push((*channel, *note));
        }
        SmafEvent::MidiNoteOff {
            channel,
            note,
            velocity,
        } => {
            let _ = send_midi_message(midi_out, [0x80 | (*channel & 0x0f), *note, *velocity]);
            active_notes.retain(|(active_channel, active_note)| {
                !(*active_channel == *channel && *active_note == *note)
            });
        }
        SmafEvent::MidiProgramChange { channel, program } => {
            let _ = send_midi_message(midi_out, [0xc0 | (*channel & 0x0f), *program]);
        }
        SmafEvent::MidiControlChange {
            channel,
            control,
            value,
        } => {
            let _ = send_midi_message(midi_out, [0xb0 | (*channel & 0x0f), *control, *value]);
        }
        SmafEvent::End => {}
    }
}

fn open_midi_connection() -> Option<MidiOutputConnection> {
    let midi_out = MidiOutput::new("wipic_simulation").ok()?;
    let midi_ports = midi_out.ports();
    let port = midi_ports.last()?;

    midi_out.connect(port, "wipic_simulation").ok()
}

fn send_midi_message(
    midi_out: Option<&mut MidiOutputConnection>,
    message: impl AsRef<[u8]>,
) -> Result<(), midir::SendError> {
    let Some(midi_out) = midi_out else {
        return Ok(());
    };

    midi_out.send(message.as_ref())
}
