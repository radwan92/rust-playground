pub fn now() -> std::time::Duration {
    #[cfg(target_family = "wasm")]
    unsafe {
        std::time::Duration::from_millis(crate::emscripten::emscripten_get_now() as u64)
    }

    #[cfg(not(target_family = "wasm"))]
    {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
    }
}
