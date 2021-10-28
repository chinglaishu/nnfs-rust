use rand::Rng;

use crate::utils_function;

// println!("{}", constant::BATCH_SIZE);
// println!("Hello, world!");
// let mut data_arr: [[f32; constant::BATCH_SIZE]; constant::X_NUM] = [[0.0; constant::BATCH_SIZE]; constant::X_NUM];
// test_data::random_array_2d(&data_arr);
// println!("{:?}", data_arr);

// println!("{}", constant::BATCH_SIZE);
// println!("Hello, world!");
// let start_num: f32 = 0.0;
// let mut data_arr_arr_x = Vec::new();
// let mut arr_1 = vec![0.0; 5];
// let mut arr_2 = vec![1.0; 5];
// let mut arr_3 = vec![2.0; 5];
// println!("{:p}", &arr_1);
// println!("{:p}", &arr_2);
// data_arr_arr_x.push(&arr_1);
// data_arr_arr_x.push(&arr_2);
// data_arr_arr_x.push(&arr_3);
// println!("{:?}", data_arr_arr_x);
// test_data::print_vec_2d(data_arr_arr_x);
// get_data_arr_arr_x(&mut data_arr_arr_x);
// let mut data_arr_y = vec![0; constant::DATA_SIZE];
// test_data::random_x_y(&mut data_arr_arr_x, &mut data_arr_y);
// println!("{:?}", data_arr_arr_x);

pub fn get_random_true_false() -> bool {
  let ran_num: i32 = utils_function::get_random_i32(0, 1);
  ran_num != 0
}

pub fn random_x_y(data_arr_arr_x: &mut Vec<Vec<f32>>, data_arr_y: &mut Vec<i32>) {
  for (i, data_arr_x) in data_arr_arr_x.iter().enumerate() {
    let ran_num_y: i32 = utils_function::get_random_i32(0, 1);
    data_arr_y[i] = ran_num_y;
    let (from, to) = if ran_num_y == 0 { (0.0, 0.2) } else { (0.8, 1.0) }; 
    random_array(data_arr_x, from, to)
  }
}

pub fn random_array_2d(data_arr_arr: &mut Vec<&mut Vec<f32>>, from: f32, to: f32) {
  for data_arr in data_arr_arr {
    random_array(data_arr, from, to);
  }
}

pub fn random_array(data_arr: &Vec<f32>, from: f32, to: f32) {
  for (i, data) in data_arr.iter().enumerate() {
    // data_arr[i] = get_random_f32(from, to);
  }

  // for data in data_arr {
  //   data = get_random_f32(from, to);
  // }
}

pub fn print_vec_2d(data_arr_arr_x: &Vec<Vec<f32>>) {
  for (i, data_arr) in data_arr_arr_x.iter().enumerate() {
    println!("{:?}", data_arr);
    println!("{:p}", data_arr);
    println!("{:?}", data_arr[0]);

  }
}

// pub fn random_array_2d<Matrix: AsRef<[Row]>, Row: AsRef<[f32]>>(data_arr: Matrix) {
//   let mut rng = rand::thread_rng();
//   println!("{:?}", data_arr);
  
//   // for i in features.as_ref() {
//   //   for a in i.as_ref() {
//   //     let ran_num = rng.gen_range(0.8..1.0);
//   //     *a = ran_num;
//   //   }
//   // }
// }

// pub fn generate_array_2d(data_arr: &[[f32]]) {
//   println!("{}", data_size);
//   let mut rng = rand::thread_rng();
//   let mut data_arr: [[f32; data_size]; x_num] = [[0.0; data_size]; x_num];
//   for i in 0..x_num {
//     let mut arr: [f32; data_size] = [0.0; data_size];
//     for a in 0..data_size {
//       let ran_num = rng.gen_range(0.8..1.0);
//       arr[a] = ran_num;
//     }
//     data_arr[i] = arr;
//   }
//   data_arr
// }

// pub fn generate_array() -> [f32; datasize] {
  
// }
