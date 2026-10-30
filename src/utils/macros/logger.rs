#[macro_export]
macro_rules! airoro_log {
    ($($arg:tt)*) => {
      println!("[{}] {}", $crate::get_time!(), format_args!($($arg)*))
    };
}