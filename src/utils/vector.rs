use rand::Rng;
use std::mem;

use crate::utils_function;

pub fn random_vec(data_vec: &mut Vec<f32>, from: f32, to: f32) {
  for data in data_vec {
    *data = utils_function::get_random_f32(from, to);
  }
}

pub fn random_vec_2d(data_vec_vec: &mut Vec<&mut Vec<f32>>, from: f32, to: f32) {
  for data_vec in data_vec_vec {
    random_vec(data_vec, from, to);
  }
}

pub fn random_x_y(data_vec_vec_x: &mut Vec<Vec<f32>>, data_vec_y: &mut Vec<i32>) {
  let mut i = 0;
  for data_vec_x in data_vec_vec_x {
    let ran_num_y: i32 = utils_function::get_random_i32(0, 2);
    data_vec_y[i] = ran_num_y;
    let (from, to) = if ran_num_y == 0 { (0.0, 0.2) } else { (0.8, 1.0) }; 
    random_vec(data_vec_x, from, to);
    i+=1;
  }
}

pub fn print_vec_2d(data_vec_vec_x: &Vec<Vec<f32>>) {
  for (i, data_vec) in data_vec_vec_x.iter().enumerate() {
    println!("{:?}", data_vec);
    println!("{:p}", data_vec);
    println!("{:?}", data_vec[0]);
  }
}

pub fn shuffle_x_y(data_vec_vec_x: &mut Vec<Vec<f32>>, data_vec_y: &mut Vec<i32>) {
  let length = data_vec_vec_x.len() as i32;
  for i in 0..length {
    let index = i as usize;
    let target_index = utils_function::get_random_i32(0, length) as usize;
    data_vec_vec_x.swap(index, target_index);
    data_vec_y.swap(index, target_index);
  }
}

// pub trait vec {
//   fn random_vec(&self, from: f32, to: f32) {
//     for (i, data) in data_vec.iter().enumerate() {
//       let item = data_vec[i];
//       data_vec[i] = utils_function::get_random_f32(from, to);
//     }
//   }
// }