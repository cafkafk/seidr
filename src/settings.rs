use std::sync::atomic::AtomicBool;

pub static QUIET: AtomicBool = AtomicBool::new(false);

pub static EMOJIS: AtomicBool = AtomicBool::new(false);
