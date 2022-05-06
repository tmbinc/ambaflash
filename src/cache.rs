// No-op implementation because we run with cache disabled.

pub fn clean_d_cache<T>(_area: &T) {}

pub fn clean_d_cache_slice<T>(_area: &[T]) {}

pub fn _clean_flush_d_cache() {}
