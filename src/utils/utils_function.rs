use rand::Rng;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Result, Write};
use std::path::Path;
use std::{process, string};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoreData {
  pub data: HashMap<String, Vec<Vec<f32>>>
}

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

pub fn write_file<T: Serialize>(store_data: &T, file_name: &str) -> Result<()> {
  // can not create folder
  let file = File::create(file_name)?;
  let mut writer = BufWriter::new(file);
  serde_json::to_writer(&mut writer, &store_data)?;
  writer.flush()?;
  Ok(())
}

pub fn read_file<P: AsRef<Path>>(path: P) -> Result<StoreData> {
  // Open the file in read-only mode with buffer.
  let file = File::open(path)?;
  let reader = BufReader::new(file);

  // Read the JSON contents of the file as an instance of `User`.
  let u = serde_json::from_reader(reader)?;

  // Return the `User`.
  Ok(u)
}

// pub fn read_file(filename: impl AsRef<Path>) -> Result<String> {
//   let use_str: Result<String> = BufReader::new(File::open(filename)?).lines().collect();
//   match use_str {
//     Ok(res) => {
//       let result = serde_json::from_reader(use_str)?;
//       Ok(result)
//     },
//     Err(err) => {err}
//   }
// }
