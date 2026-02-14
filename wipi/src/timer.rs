use alloc::boxed::Box;
use core::time::Duration;

use wipi_types::wipic::WIPICWord;

struct TimerContext {
    callback: Box<dyn FnMut()>,
    periodic: bool,
    interval_ms: u64,
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
        let low = ctx.interval_ms as u32;
        let high = (ctx.interval_ms >> 32) as u32;

        wipic_sys::kernel::set_timer(timer_ptr, low, high, param);
    }
}

impl Timer {
    fn new_inner(callback: impl FnMut() + 'static, interval: Duration, periodic: bool) -> Self {
        let interval_ms = interval.as_millis() as u64;
        let ctx = Box::into_raw(Box::new(TimerContext {
            callback: Box::new(callback),
            periodic,
            interval_ms,
            raw_timer: 0,
        }));

        unsafe {
            let timer_ptr = &mut (*ctx).raw_timer as *mut WIPICWord as *mut u8;
            let low = interval_ms as u32;
            let high = (interval_ms >> 32) as u32;

            wipic_sys::kernel::def_timer(timer_ptr, timer_trampoline);
            wipic_sys::kernel::set_timer(timer_ptr, low, high, ctx as *mut u8);
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
