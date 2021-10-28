use rand::Rng;
use std::mem;

use crate::{constant::{DATA_SIZE, OUTPUT_CATEGORY}, utils_function};

macro_rules! marco_check_vector_have_same_shape_2d {
  ($func:ident, $type:ident) => {
    pub fn $func(vec_1: &Vec<Vec<$type>>, vec_2: &Vec<Vec<$type>>) -> bool {
      let x = vec_2.len();
      let y = if x == 0 {0} else {vec_2[0].len()};

      let vec_x = vec_1.len();
      let vec_y = if vec_x == 0 {0} else {vec_1[0].len()};

      x == vec_x && y == vec_y
    }
  };
}

marco_check_vector_have_same_shape_2d!(check_vector_have_same_shape_2d_f32, f32);
marco_check_vector_have_same_shape_2d!(check_vector_have_same_shape_2d_i32, i32);

// macro_rules! marco_ {
//   () => {
    
//   };
// }

macro_rules! marco_copy_vector_2d {
  ($func:ident, $type:ident, $default:expr, $check_same_shape_function:expr) => {
    pub fn $func(vec: &mut Vec<Vec<$type>>, copy_vec: &Vec<Vec<$type>>) {
      let x = copy_vec.len();
      let y = if x == 0 {0} else {copy_vec[0].len()};
      if !$check_same_shape_function(vec, copy_vec) {
        vec.clear();
        vec.resize(x, vec![$default; y]);
      }
      for i in 0..x {
        for a in 0..y {
          vec[i][a] = copy_vec[i][a];
        }
      }
    }
  };
}

marco_copy_vector_2d!(copy_vector_2d_f32, f32, 0.0, check_vector_have_same_shape_2d_f32);
marco_copy_vector_2d!(copy_vector_2d_i32, i32, 0, check_vector_have_same_shape_2d_i32);

pub fn fill_vec_2d_f32(vec: &mut Vec<Vec<f32>>, value: f32) {
  for i in 0..vec.len() {
    for a in 0..vec[i].len() {
      vec[i][a] = value;
    }
  }
}

pub fn get_shape_of_vec_2d(vec: &Vec<Vec<f32>>) -> (usize, usize) {
  let shape_one = vec.len();
  let shape_two = if shape_one == 0 {0} else {vec[0].len() };
  (shape_one, shape_two)
}

pub fn pow_value_vec(vec: &mut Vec<f32>, value: f32) {
  for i in vec {
    *i = value.powf(*i);
  }
}

pub fn time_value_vec(vec: &mut Vec<f32>, value: f32) {
  for i in vec {
    *i *= value;
  }
  // vec.iter().map(|&x| x * value).collect::<Vec<f32>>();
  // vec
}

pub fn random_vec(data_vec: &mut Vec<f32>, from: f32, to: f32) {
  for data in data_vec {
    *data = utils_function::get_random_f32(from, to);
  }
}

pub fn random_vec_2d(data_vec_vec: &mut Vec<Vec<f32>>, from: f32, to: f32) {
  for data_vec in data_vec_vec {
    random_vec(data_vec, from, to);
  }
}

pub fn random_weight_vec(vec: &mut Vec<Vec<f32>>) {
  for i in 0..vec.len() {
    for a in 0..vec[i].len() {
      vec[i][a] = utils_function::get_random_f32(0.0, 10.0) * 0.1;
    }
  }
}

// pub fn random_x_y(data_vec_vec_x: &mut Vec<Vec<f32>>, data_vec_y: &mut Vec<usize>) {
//   let mut i = 0;
//   for data_vec_x in data_vec_vec_x {
//     let ran_num_y: usize = utils_function::get_random_i32(0, 2) as usize;
//     data_vec_y[i] = ran_num_y;
//     let (from, to) = if ran_num_y == 0 { (0.0, 0.2) } else { (0.8, 1.0) }; 
//     random_vec(data_vec_x, from, to);
//     i+=1;
//   }
// }

pub fn random_x_y(data_vec_vec_x: &mut Vec<Vec<f32>>, data_vec_y: &mut Vec<usize>) {
  for i in 0..data_vec_vec_x.len() {
    let ran_num_y: usize = utils_function::get_random_i32(0, OUTPUT_CATEGORY as i32) as usize;
    data_vec_y[i] = ran_num_y;
    let base_num = DATA_SIZE / OUTPUT_CATEGORY;
    let (from, to) = (base_num * ran_num_y, base_num * (ran_num_y + 1));
    let index_vec: Vec<usize> = (from..to).collect();
    for a in 0..data_vec_vec_x[i].len() {
      let (random_from, random_to) = if index_vec.contains(&a) {(0.9, 1.0)} else {(0.0, 0.1)};
      data_vec_vec_x[i][a] = utils_function::get_random_f32(random_from, random_to);
    }
  }
}

pub fn print_vec_2d(data_vec_vec_x: &Vec<Vec<f32>>) {
  for (i, data_vec) in data_vec_vec_x.iter().enumerate() {
    println!("{:?}", data_vec);
    println!("{:p}", data_vec);
    println!("{:?}", data_vec[0]);
  }
}

pub fn shuffle_x_y(data_vec_vec_x: &mut Vec<Vec<f32>>, data_vec_y: &mut Vec<usize>) {
  let length = data_vec_vec_x.len();
  for i in 0..length {
    let index = i as usize;
    let target_index = utils_function::get_random_i32(0, length as i32) as usize;
    data_vec_vec_x.swap(index, target_index);
    data_vec_y.swap(index, target_index);
  }
}

pub fn limit_value_vec_2d(output_vec: &mut Vec<Vec<f32>>, input_vec: &Vec<Vec<f32>>) {
  for i in 0..input_vec.len() {
    for a in 0..input_vec[0].len() {
      output_vec[i][a] = if input_vec[i][a] < 0.0 {0.0} else {input_vec[i][a]};
    }
  }
}

pub fn get_max(vec: &Vec<f32>) -> f32 {
  let mut max: f32 = if vec.len() == 0 {0.0} else {vec[0].clone() };
  for value in vec {
    if *value > max {max = value.clone(); }
  }
  max
}

pub fn get_min(vec: &Vec<f32>) -> f32 {
  let mut min: f32 = if vec.len() == 0 {0.0} else {vec[0].clone() };
  for value in vec {
    if *value < min {min = value.clone(); }
  }
  min
}

#[test]
fn test_get_max() {
  let vec: Vec<f32> = vec![0.0, 2.5, 5.2, 0.1];
  let correct_max: f32 = 5.2;
  let max = get_max(&vec);
  assert_eq!(max, correct_max);
}

// pub trait vec {
//   fn random_vec(&self, from: f32, to: f32) {
//     for (i, data) in data_vec.iter().enumerate() {
//       let item = data_vec[i];
//       data_vec[i] = utils_function::get_random_f32(from, to);
//     }
//   }
// }