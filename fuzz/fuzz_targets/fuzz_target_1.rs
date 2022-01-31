#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
use chrono_tz::America::New_York;
use rizzy::*;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let rizzy = Rizzy::new(New_York, None, true);
        let _ = rizzy.handle_line(s);
    }
});
