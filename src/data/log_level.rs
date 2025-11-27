#[allow(dead_code)]
pub static GENERAL: &[&str] = &["error", "warning", "info", "fatal", "trace", "debug"];

#[allow(dead_code)]
pub static SYSLOG: &[&str] = &[
    "emerg", "alert", "crit", "err", "warning", "notice", "info", "debug",
];

#[allow(dead_code)]
pub static APACHE: &[&str] = &[
    "emerg", "alert", "crit", "error", "warn", "notice", "info", "debug", "trace1-8",
];
