#[allow(dead_code)]
pub static GENERAL: &'static [&str] = &["error", "warning", "info", "fatal", "trace", "debug"];

#[allow(dead_code)]
pub static SYSLOG: &'static [&str] = &[
    "emerg", "alert", "crit", "err", "warning", "notice", "info", "debug",
];

#[allow(dead_code)]
pub static APACHE: &'static [&str] = &[
    "emerg", "alert", "crit", "error", "warn", "notice", "info", "debug", "trace1-8",
];
