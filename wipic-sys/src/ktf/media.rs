use core::mem::transmute;

use wipi_boot::ktf::WIPIC_MEDIA_INTERFACE;
use wipi_types::wipic::WIPICWord;

use crate::MediaError;

pub fn clip_create(
    clip_type: WIPICWord,
    buf_size: WIPICWord,
    callback: WIPICWord,
) -> Result<WIPICWord, MediaError> {
    let clip_create: extern "C" fn(WIPICWord, WIPICWord, WIPICWord) -> i32 =
        unsafe { transmute((*WIPIC_MEDIA_INTERFACE).clip_create) };
    let result = clip_create(clip_type, buf_size, callback);
    if result < 0 {
        return Err(MediaError::Platform(result));
    }

    Ok(result as WIPICWord)
}

pub fn clip_free(clip: WIPICWord) -> Result<(), MediaError> {
    let clip_free: extern "C" fn(WIPICWord) -> i32 =
        unsafe { transmute((*WIPIC_MEDIA_INTERFACE).clip_free) };
    let result = clip_free(clip);
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
    let clip_put_data: extern "C" fn(WIPICWord, WIPICWord, WIPICWord) -> i32 =
        unsafe { transmute((*WIPIC_MEDIA_INTERFACE).clip_put_data) };
    let result = clip_put_data(clip, buf, buf_size);
    if result < 0 {
        return Err(MediaError::Platform(result));
    }

    Ok(result as usize)
}

pub fn play(clip: WIPICWord, repeat: WIPICWord) -> Result<(), MediaError> {
    let play: extern "C" fn(WIPICWord, WIPICWord) -> i32 =
        unsafe { transmute((*WIPIC_MEDIA_INTERFACE).play) };
    let result = play(clip, repeat);
    if result < 0 {
        return Err(MediaError::Platform(result));
    }

    Ok(())
}

pub fn pause(clip: WIPICWord) -> Result<(), MediaError> {
    let pause: extern "C" fn(WIPICWord) -> i32 =
        unsafe { transmute((*WIPIC_MEDIA_INTERFACE).pause) };
    let result = pause(clip);
    if result < 0 {
        return Err(MediaError::Platform(result));
    }

    Ok(())
}

pub fn resume(clip: WIPICWord) -> Result<(), MediaError> {
    let resume: extern "C" fn(WIPICWord) -> i32 =
        unsafe { transmute((*WIPIC_MEDIA_INTERFACE).resume) };
    let result = resume(clip);
    if result < 0 {
        return Err(MediaError::Platform(result));
    }

    Ok(())
}

pub fn stop(clip: WIPICWord) -> Result<(), MediaError> {
    let stop: extern "C" fn(WIPICWord) -> i32 = unsafe { transmute((*WIPIC_MEDIA_INTERFACE).stop) };
    let result = stop(clip);
    if result < 0 {
        return Err(MediaError::Platform(result));
    }

    Ok(())
}

pub fn clip_get_volume(clip: WIPICWord) -> Result<u8, MediaError> {
    let clip_get_volume: extern "C" fn(WIPICWord) -> i32 =
        unsafe { transmute((*WIPIC_MEDIA_INTERFACE).clip_get_volume) };
    let result = clip_get_volume(clip);
    if result < 0 {
        return Err(MediaError::Platform(result));
    }

    Ok(result as u8)
}

pub fn clip_set_volume(clip: WIPICWord, volume: WIPICWord) -> Result<(), MediaError> {
    let clip_set_volume: extern "C" fn(WIPICWord, WIPICWord) -> i32 =
        unsafe { transmute((*WIPIC_MEDIA_INTERFACE).clip_set_volume) };
    let result = clip_set_volume(clip, volume);
    if result < 0 {
        return Err(MediaError::Platform(result));
    }

    Ok(())
}
