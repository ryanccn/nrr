use std::sync::LazyLock;

#[must_use]
#[inline]
pub fn itoa(input: impl itoa::Integer) -> String {
    itoa::Buffer::new().format(input).to_owned()
}

pub static NRR_LEVEL: LazyLock<usize> = LazyLock::new(|| {
    std::env::var("__NRR_LEVEL")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(1)
});
