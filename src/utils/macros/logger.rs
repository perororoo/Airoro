#[macro_export]
macro_rules! airoro_log {
    ($($arg:tt)*) => {
      println!("[{}] {}", get_time!(), format_args!($($arg)*));
    };
}