#[inline(never)]
pub fn black_box<D>(input: D) -> D {
    unsafe {
        let output = std::ptr::read_volatile(&input);
        std::mem::forget(input);
        output
    }
}

pub fn rdtsc_perf<I, O, S, R>(mut setup: S, mut routine: R, samples: usize) -> u64
where
    S: FnMut() -> I,
    R: FnMut(I) -> O,
{
    use core::sync::atomic::compiler_fence;
    use core::sync::atomic::Ordering::SeqCst;
    let mut min_time: u64 = std::u64::MAX;
    for _ in 0..samples {
        let input = setup();
        compiler_fence(SeqCst);
        #[cfg(target_pointer_width = "32")]
        let time_a = unsafe { core::arch::x86::_rdtsc() };
        #[cfg(target_pointer_width = "64")]
        let time_a = unsafe { core::arch::x86_64::_rdtsc() };
        compiler_fence(SeqCst);
        let output = routine(input);
        compiler_fence(SeqCst);
        #[cfg(target_pointer_width = "32")]
        let time_b = unsafe { core::arch::x86::_rdtsc() };
        #[cfg(target_pointer_width = "64")]
        let time_b = unsafe { core::arch::x86_64::_rdtsc() };
        compiler_fence(SeqCst);
        min_time = min_time.min(time_b - time_a);
        let _ = black_box(output);
    }
    min_time
}