use rand::RngExt;

use crate::airoro_log;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum GachaResult {
  R,
  SR,
  SSR,
  DEFAULT,
}


pub fn get_rarity(rolled_number: &i8) -> GachaResult {
  airoro_log!("[GACHA::GET_RARITY]: Getting rarity...");
  let result: GachaResult = match rolled_number {
    1..=79 => GachaResult::R,
    80..=97 => GachaResult::SR,
    98..=100 => GachaResult::SSR,
    _ => GachaResult::DEFAULT
  };

  match result {
    GachaResult::R => airoro_log!("[GACHA::GET_RARITY]: Rarity: R"),
    GachaResult::SR => airoro_log!(" [GACHA::GET_RARITY]: Rarity: SR"),
    GachaResult::SSR => airoro_log!("[GACHA::GET_RARITY]: Rarity: SSR"),
    GachaResult::DEFAULT => airoro_log!("[GACHA::GET_RARITY]: The roll went out of range!! D:")
  }

  return result;
}

pub fn roll_chance(user: &str, range_min: i8, range_max: i8) -> i8 {
  let mut rng = rand::rng();
  let rolled_number: i8 = rng.random_range(range_min..=range_max);
  
  airoro_log!("[GACHA::ROLL_CHANCE]: {} rolled a {} !!", user, rolled_number);

  return rolled_number;
}

