use alloc::boxed::Box;
use core::time::Duration;

use wipi_types::wipic::WIPICWord;

// Timeout is split into two u32 fields instead of a single u64 to keep struct
// alignment at 4 bytes. The WIPI allocator (MC_knlAlloc) only guarantees 4-byte
// alignment, but Rust requires 8-byte alignment for u64.
struct TimerContext {
    callback: Box<dyn FnMut()>,
    periodic: bool,
    timeout_low: u32,
    timeout_high: u32,
    raw_timer: WIPICWord,
}

pub struct Timer {
    ctx: *mut TimerContext,
}

extern "C" fn timer_trampoline(_timer_ptr: *mut u8, param: *mut u8) {
    let ctx = unsafe { &mut *(param as *mut TimerContext) };
    (ctx.callback)();

    if ctx.periodic {
        let timer_ptr = &mut ctx.raw_timer as *mut WIPICWord as *mut u8;
        wipic_sys::kernel::set_timer(timer_ptr, ctx.timeout_low, ctx.timeout_high, param);
    }
}

impl Timer {
    fn new_inner(callback: impl FnMut() + 'static, interval: Duration, periodic: bool) -> Self {
        let interval_ms = interval.as_millis() as u64;
        let timeout_low = interval_ms as u32;
        let timeout_high = (interval_ms >> 32) as u32;
        let ctx = Box::into_raw(Box::new(TimerContext {
            callback: Box::new(callback),
            periodic,
            timeout_low,
            timeout_high,
            raw_timer: 0,
        }));

        unsafe {
            let timer_ptr = &mut (*ctx).raw_timer as *mut WIPICWord as *mut u8;
            wipic_sys::kernel::def_timer(timer_ptr, timer_trampoline);
            wipic_sys::kernel::set_timer(timer_ptr, timeout_low, timeout_high, ctx as *mut u8);
        }

        Self { ctx }
    }

    pub fn once(interval: Duration, callback: impl FnMut() + 'static) -> Self {
        Self::new_inner(callback, interval, false)
    }

    pub fn periodic(interval: Duration, callback: impl FnMut() + 'static) -> Self {
        Self::new_inner(callback, interval, true)
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        unsafe {
            let timer_ptr = &mut (*self.ctx).raw_timer as *mut WIPICWord as *mut u8;
            wipic_sys::kernel::unset_timer(timer_ptr);
            drop(Box::from_raw(self.ctx));
        }
    }
}
