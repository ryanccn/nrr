use std::sync::{
    Arc, LazyLock,
    atomic::{AtomicBool, Ordering},
};

static ACCEPT_SIGNALS: LazyLock<Arc<AtomicBool>> =
    LazyLock::new(|| Arc::new(AtomicBool::new(true)));

pub fn install() -> Result<(), std::io::Error> {
    for sig in signal_hook::consts::TERM_SIGNALS {
        signal_hook::flag::register_conditional_shutdown(*sig, 1, Arc::clone(&ACCEPT_SIGNALS))?;
    }

    Ok(())
}

pub fn ignore() {
    ACCEPT_SIGNALS.store(false, Ordering::Relaxed);
}

pub fn restore() {
    ACCEPT_SIGNALS.store(true, Ordering::Relaxed);
}
