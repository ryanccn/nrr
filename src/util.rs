use std::sync::OnceLock;

#[must_use]
#[inline]
pub fn itoa(input: impl itoa::Integer) -> String {
    itoa::Buffer::new().format(input).to_owned()
}

pub fn get_level() -> &'static usize {
    static ONCE_LOCK: OnceLock<usize> = OnceLock::new();
    ONCE_LOCK.get_or_init(|| {
        std::env::var("__NRR_LEVEL")
            .ok()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(1)
    })
}
