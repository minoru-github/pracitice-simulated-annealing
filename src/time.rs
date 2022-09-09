pub fn update() -> f64 {
    static mut STARTING_TIME_MS: Option<f64> = None;
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    let time_ms = t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9;
    unsafe {
        let now = match STARTING_TIME_MS {
            Some(starting_time_ms) => time_ms - starting_time_ms,
            None => {
                STARTING_TIME_MS = Some(time_ms);
                0.0 as f64
            }
        };
        now
    }
}
