use std::{
    alloc::{GlobalAlloc, Layout, System},
    cell::Cell,
};

thread_local! {
    // A constant initializer and Cell access do not allocate inside the allocator.
    static LARGEST_ALLOCATION: Cell<Option<usize>> = const { Cell::new(None) };
}

struct MeasuringAllocator;

#[global_allocator]
static ALLOCATOR: MeasuringAllocator = MeasuringAllocator;

fn record_allocation(size: usize) {
    // Other test threads and allocations outside a measurement are ignored.
    // try_with also avoids panicking if thread-local state is being torn down.
    let _ = LARGEST_ALLOCATION.try_with(|largest| {
        if let Some(previous) = largest.get() {
            largest.set(Some(previous.max(size)));
        }
    });
}

// SAFETY: Every allocation operation delegates to System with its original
// arguments. Measurement only updates thread-local integers and cannot unwind.
unsafe impl GlobalAlloc for MeasuringAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        record_allocation(layout.size());
        // SAFETY: The caller supplies the layout required by GlobalAlloc.
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        record_allocation(layout.size());
        // SAFETY: The caller supplies the layout required by GlobalAlloc.
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        record_allocation(new_size);
        // SAFETY: The pointer, old layout, and new size retain the caller's contract.
        unsafe { System.realloc(ptr, layout, new_size) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // SAFETY: Allocations come from System, and the original layout is forwarded.
        unsafe { System.dealloc(ptr, layout) }
    }
}

/// Measure the largest allocation requested by this thread during an operation.
pub fn measure_largest_allocation<T>(operation: impl FnOnce() -> T) -> (T, usize) {
    struct ResetMeasurement;

    impl Drop for ResetMeasurement {
        fn drop(&mut self) {
            LARGEST_ALLOCATION.with(|largest| largest.set(None));
        }
    }

    LARGEST_ALLOCATION.with(|largest| {
        assert!(
            largest.get().is_none(),
            "allocation measurements cannot nest"
        );
        largest.set(Some(0));
    });
    let _reset = ResetMeasurement;
    let result = operation();
    let largest = LARGEST_ALLOCATION.with(|largest| largest.get().unwrap());
    (result, largest)
}
