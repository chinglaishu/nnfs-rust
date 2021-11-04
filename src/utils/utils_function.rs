use rand::Rng;
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write, Result};
use std::{process, string};
use serde::{Serialize, Deserialize};
use serde_json;

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

// #[derive(Debug, Serialize, Deserialize)]
// pub struct StoreData {
//   layer_1_weight: Vec<Vec<f32>>,
//   layer_1_bias: Vec<Vec<f32>>,

//   layer_2_weight: Vec<Vec<f32>>,
//   layer_2_bias: Vec<Vec<f32>>,

//   layer_3_weight: Vec<Vec<f32>>,
//   layer_3_bias: Vec<Vec<f32>>,
// }

pub fn write_file<T: Serialize>(store_data: &T, file_name: &str) -> Result<()> {
  let file = File::create(file_name)?;
  let mut writer = BufWriter::new(file);
  serde_json::to_writer(&mut writer, &store_data)?;
  writer.flush()?;
  Ok(())
}

pub fn read_file() {

}
