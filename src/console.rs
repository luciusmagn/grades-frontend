#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (log_1(&format_args!($($t)*).to_string().into()))
}
