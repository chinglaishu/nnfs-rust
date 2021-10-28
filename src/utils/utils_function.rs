use rand::Rng;

macro_rules! marco_get_random {
  ($func:ident, $type:ident) => {
    pub fn $func(from: $type, to: $type) -> $type {
      let mut rng = rand::thread_rng();
      rng.gen_range(from..to)
    }
  };
}

marco_get_random!(get_random_f32, f32);
marco_get_random!(get_random_i32, i32);

pub fn get_random_true_false() -> bool {
  let ran_num: i32 = get_random_i32(0, 1);
  ran_num != 0
}


