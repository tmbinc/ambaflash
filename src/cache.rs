use core::mem::size_of_val;
use core::ptr::addr_of;

mod assembly {
    extern "C" {
        pub fn clean_d_cache(addr: usize, size: usize) -> ();
        pub fn _clean_flush_d_cache() -> ();
    }
}

pub fn clean_d_cache<T>(area: &T) {
    unsafe {
        assembly::clean_d_cache(addr_of!(*area) as usize, size_of_val(area));
    }
}

pub fn clean_d_cache_slice<T>(area: &[T]) {
    unsafe {
        assembly::clean_d_cache(area.as_ptr() as usize, size_of_val(area));
    }
}

pub fn _clean_flush_d_cache() {
    unsafe {
        assembly::_clean_flush_d_cache();
    }
}
