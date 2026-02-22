use wipi_types::wipic::WIPICWord;

use crate::MediaError;

pub fn clip_create(
    clip_type: WIPICWord,
    buf_size: WIPICWord,
    callback: WIPICWord,
) -> Result<WIPICWord, MediaError> {
    let result = wipic_simulation::media::clip_create(clip_type, buf_size, callback);
    if result < 0 {
        return Err(MediaError::Platform(result));
    }

    Ok(result as WIPICWord)
}

pub fn clip_free(clip: WIPICWord) -> Result<(), MediaError> {
    let result = wipic_simulation::media::clip_free(clip);
    if result < 0 {
        return Err(MediaError::Platform(result));
    }

    Ok(())
}

pub fn clip_put_data(
    clip: WIPICWord,
    buf: WIPICWord,
    buf_size: WIPICWord,
) -> Result<usize, MediaError> {
    let result =
        unsafe { wipic_simulation::media::clip_put_data(clip, buf as *const u8, buf_size) };
    if result < 0 {
        return Err(MediaError::Platform(result));
    }

    Ok(result as usize)
}

pub fn play(clip: WIPICWord, repeat: WIPICWord) -> Result<(), MediaError> {
    let result = wipic_simulation::media::play(clip, repeat != 0);
    if result < 0 {
        return Err(MediaError::Platform(result));
    }

    Ok(())
}

pub fn pause(clip: WIPICWord) -> Result<(), MediaError> {
    let result = wipic_simulation::media::pause(clip);
    if result < 0 {
        return Err(MediaError::Platform(result));
    }

    Ok(())
}

pub fn resume(clip: WIPICWord) -> Result<(), MediaError> {
    let result = wipic_simulation::media::resume(clip);
    if result < 0 {
        return Err(MediaError::Platform(result));
    }

    Ok(())
}

pub fn stop(clip: WIPICWord) -> Result<(), MediaError> {
    let result = wipic_simulation::media::stop(clip);
    if result < 0 {
        return Err(MediaError::Platform(result));
    }

    Ok(())
}

pub fn clip_get_volume(clip: WIPICWord) -> Result<u8, MediaError> {
    let result = wipic_simulation::media::clip_get_volume(clip);
    if result < 0 {
        return Err(MediaError::Platform(result));
    }

    Ok(result as u8)
}

pub fn clip_set_volume(clip: WIPICWord, volume: WIPICWord) -> Result<(), MediaError> {
    let result = wipic_simulation::media::clip_set_volume(clip, volume as u8);
    if result < 0 {
        return Err(MediaError::Platform(result));
    }

    Ok(())
}
