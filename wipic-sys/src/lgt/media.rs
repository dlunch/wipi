use core::mem::transmute;

use wipi_boot::lgt::get_external_method;
use wipi_types::{
    lgt::wipic::{ImportModule, WIPICMethod},
    wipic::WIPICWord,
};

use crate::MediaError;

pub fn clip_create(
    clip_type: WIPICWord,
    buf_size: WIPICWord,
    callback: WIPICWord,
) -> Result<WIPICWord, MediaError> {
    let clip_create: extern "C" fn(WIPICWord, WIPICWord, WIPICWord) -> i32 = unsafe {
        transmute(get_external_method(
            ImportModule::WIPIC,
            WIPICMethod::ClipCreate as _,
        ))
    };
    let result = clip_create(clip_type, buf_size, callback);
    if result < 0 {
        return Err(MediaError::Platform(result));
    }

    Ok(result as WIPICWord)
}

pub fn clip_free(clip: WIPICWord) -> Result<(), MediaError> {
    let clip_free: extern "C" fn(WIPICWord) -> i32 = unsafe {
        transmute(get_external_method(
            ImportModule::WIPIC,
            WIPICMethod::ClipFree as _,
        ))
    };
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
    let clip_put_data: extern "C" fn(WIPICWord, WIPICWord, WIPICWord) -> i32 = unsafe {
        transmute(get_external_method(
            ImportModule::WIPIC,
            WIPICMethod::ClipPutData as _,
        ))
    };
    let result = clip_put_data(clip, buf, buf_size);
    if result < 0 {
        return Err(MediaError::Platform(result));
    }

    Ok(result as usize)
}

pub fn play(clip: WIPICWord, repeat: WIPICWord) -> Result<(), MediaError> {
    let play: extern "C" fn(WIPICWord, WIPICWord) -> i32 = unsafe {
        transmute(get_external_method(
            ImportModule::WIPIC,
            WIPICMethod::Play as _,
        ))
    };
    let result = play(clip, repeat);
    if result < 0 {
        return Err(MediaError::Platform(result));
    }

    Ok(())
}

pub fn pause(_clip: WIPICWord) -> Result<(), MediaError> {
    Err(MediaError::Unsupported)
}

pub fn resume(_clip: WIPICWord) -> Result<(), MediaError> {
    Err(MediaError::Unsupported)
}

pub fn stop(clip: WIPICWord) -> Result<(), MediaError> {
    let stop: extern "C" fn(WIPICWord) -> i32 = unsafe {
        transmute(get_external_method(
            ImportModule::WIPIC,
            WIPICMethod::Stop as _,
        ))
    };
    let result = stop(clip);
    if result < 0 {
        return Err(MediaError::Platform(result));
    }

    Ok(())
}

pub fn clip_get_volume(clip: WIPICWord) -> Result<u8, MediaError> {
    let clip_get_volume: extern "C" fn(WIPICWord) -> i32 = unsafe {
        transmute(get_external_method(
            ImportModule::WIPIC,
            WIPICMethod::ClipGetVolume as _,
        ))
    };
    let result = clip_get_volume(clip);
    if result < 0 {
        return Err(MediaError::Platform(result));
    }

    Ok(result as u8)
}

pub fn clip_set_volume(clip: WIPICWord, volume: WIPICWord) -> Result<(), MediaError> {
    let clip_set_volume: extern "C" fn(WIPICWord, WIPICWord) -> i32 = unsafe {
        transmute(get_external_method(
            ImportModule::WIPIC,
            WIPICMethod::ClipSetVolume as _,
        ))
    };
    let result = clip_set_volume(clip, volume);
    if result < 0 {
        return Err(MediaError::Platform(result));
    }

    Ok(())
}
