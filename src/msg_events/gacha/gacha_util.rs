use rand::RngExt;
use crate::time::get_time;

#[derive(PartialEq)]
pub enum GachaResult {
  R,
  SR,
  SSR,
  DEFAULT,
}


pub fn get_rarity(rolled_number: &i8) -> GachaResult {
  println!("[{}] [GACHA::GET_RARITY]: Getting rarity...", get_time());
  let result: GachaResult = match rolled_number {
    1..=79 => GachaResult::R,
    80..=97 => GachaResult::SR,
    98..=100 => GachaResult::SSR,
    _ => GachaResult::DEFAULT
  };

  match result {
    GachaResult::R => println!("[{}] [GACHA::GET_RARITY]: Rarity: R", get_time()),
    GachaResult::SR => println!("[{}]  [GACHA::GET_RARITY]: Rarity: SR", get_time()),
    GachaResult::SSR => println!("[{}] [GACHA::GET_RARITY]: Rarity: SSR", get_time()),
    GachaResult::DEFAULT => println!("[{}] [GACHA::GET_RARITY]: The roll went out of range!! D:", get_time())
  }

  return result;
}

pub fn roll_chance(user: &str, range_min: i8, range_max: i8) -> i8 {
  let mut rng = rand::rng();
  let rolled_number: i8 = rng.random_range(range_min..=range_max);
  
  println!("[{}] [GACHA::ROLL_CHANCE]: {} rolled a {} !!", get_time(), user, rolled_number);

  return rolled_number;
}

