#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: (String, u8)| {
    let (s, len) = data;
    if let Ok(url) = url::Url::parse(&s) {
        shorten_url::shorten(url.as_str(), len as usize);
    }
});
