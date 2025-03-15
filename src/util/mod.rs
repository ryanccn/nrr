use std::sync::LazyLock;

pub mod exit_code;
pub mod signals;

pub use exit_code::ExitCode;

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
