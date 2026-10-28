pub fn get_time() -> String {
  let now = chrono::Local::now();
  let time = now.format("%H:%M:%S").to_string();
  return time;
}