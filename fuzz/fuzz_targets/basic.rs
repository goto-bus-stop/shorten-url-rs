#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: (String, u8)| {
    shorten_url::shorten(&data.0, data.1 as usize);
});
