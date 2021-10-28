use crate::vector;

pub fn get_sum(vec: &Vec<f32>) -> f32 {
  let mut sum: f32 = 0.0;
  for i in vec {
    sum += *i;
  }
  sum
}

pub fn get_sum_2d(vec: &Vec<Vec<f32>>) -> f32 {
  let mut sum: f32 = 0.0;
  for x in vec {
    sum += get_sum(x);
  }
  sum
}

pub fn transpose_vec(output_vec: &mut Vec<Vec<f32>>, vec: &Vec<Vec<f32>>) {
  for i in 0..output_vec.len() {
    for a in 0..output_vec[i].len() {
      output_vec[i][a] = vec[a][i];
    }
  }
}

pub fn get_single_dot_product(vec_1: &Vec<Vec<f32>>, vec_2: &Vec<Vec<f32>>, x: usize, y: usize) -> f32 {
  let mut sum = 0.0;
  for i in 0..vec_2.len() {
    sum += vec_1[x][i] * vec_2[i][y];
  }
  sum
}

pub fn dot_product(output_vec: &mut Vec<Vec<f32>>, vec_1: &Vec<Vec<f32>>, vec_2: &Vec<Vec<f32>>) {
  for x in 0..output_vec.len() {
    for y in 0..output_vec[x].len() {
      output_vec[x][y] = get_single_dot_product(vec_1, vec_2, x, y);
    }
  }
}

pub fn add_bias_to_matrix(vec: &mut Vec<Vec<f32>>, bias_vec: &Vec<Vec<f32>>) {
  for x in 0..vec.len() {
    for y in 0..vec[x].len() {
      vec[x][y] += bias_vec[0][y];
    }
  }
}

pub fn change_to_minus_max_row(output_vec: &mut Vec<Vec<f32>>, input_vec: &Vec<Vec<f32>>) {
  for i in 0..input_vec.len() {
    let max: f32 = vector::get_max(&input_vec[i]);
    for a in 0..input_vec[i].len() {
      output_vec[i][a] = input_vec[i][a] - max;
    }
  }
}

pub fn change_to_probability(vec: &mut Vec<Vec<f32>>) {
  for x in vec {
    let sum = get_sum(x);
    for i in x {
      *i = if sum == 0.0 {0.0} else {*i / sum};
    }
  }  
}

pub fn get_max_value_index(vec: &Vec<f32>) -> usize {
  let mut record_max: f32 = 0.0;
  let mut record_index: usize = 0;
  for i in 0..vec.len() {
    if vec[i] > record_max {
      record_max = vec[i];
      record_index = i;
    }
  }
  record_index
}

pub fn limit_value_vec(vec: &mut Vec<f32>, min: f32, max: f32) {
  for i in vec {
    *i = limit_value(*i, min, max);
  }
}

pub fn limit_value(value: f32, min: f32, max: f32) -> f32 {
  if value < min {
    return min;
  } else if value > max {
    return max
  }
  value
}

pub fn count_correct_predict(predict_vec: &Vec<usize>, true_vec: &Vec<usize>) -> i32 {
  let mut count: i32 = 0;
  for i in 0..predict_vec.len() {
    if predict_vec[i] == true_vec[i] {
      count += 1;
    }
  }
  count
}

#[test]
fn test_get_single_dot_product() {
  /*

          vec_2 (2, 3)
          [
            [3, 3, 3],
            [3, 3, 3],
          ]
  vec_1 (3, 2)
  [         [
    [2, 2],   [12, 12, 12]
    [2, 2],   [12, 12, 12]
    [2, 2],   [12, 12, 12]
  ]         ]   
  */
  
  let vec_1: Vec<Vec<f32>> = vec![vec![2.0; 2]; 3];
  let vec_2: Vec<Vec<f32>> = vec![vec![3.0; 3]; 2];
  // let correct_dot_product_vec = vec![vec![12; 3]; 3];
  let correct_single_dot_product = 12.0;
  let single_dot_product = get_single_dot_product(&vec_1, &vec_2, 0, 0);
  println!("single dot product: {}", single_dot_product);
  assert_eq!(single_dot_product, correct_single_dot_product);
}

#[test] 
fn get_dot_product() {
  /*

          vec_2 (2, 3)
          [
            [3, 3, 3],
            [3, 3, 3],
          ]
  vec_1 (3, 2)
  [         [
    [2, 2],   [12, 12, 12]
    [2, 2],   [12, 12, 12]
    [2, 2],   [12, 12, 12]
  ]         ]   
  */

  let vec_1: Vec<Vec<f32>> = vec![vec![2.0; 2]; 3];
  let vec_2: Vec<Vec<f32>> = vec![vec![3.0; 3]; 2];
  let correct_dot_product_vec: Vec<Vec<f32>> = vec![vec![12.0; 3]; 3];
  let mut dot_product_vec: Vec<Vec<f32>> = vec![vec![0.0; 3]; 3]; 
  dot_product(&mut dot_product_vec, &vec_1, &vec_2);
  println!("dot product: {:?}", dot_product_vec);
  assert_eq!(dot_product_vec, correct_dot_product_vec);
}

#[test]
fn test_add_bias() {
  /*
  [
    [12, 12, 12]
    [12, 12, 12]
    [12, 12, 12]
  ]   
  bias_vec(1, 3)
  [
    [1, 2, 3]
  ]

  after add bias
  [
    [13, 14, 15],
    [13, 14, 15],
    [13, 14, 15],
  ]         
  */
  let mut vec: Vec<Vec<f32>> = vec![vec![12.0; 3]; 3];
  let bias_vec: Vec<Vec<f32>> = vec![vec![1.0, 2.0, 3.0]; 1];
  let correct_vec: Vec<Vec<f32>> = vec![vec![13.0, 14.0, 15.0]; 3];
  add_bias_to_matrix(&mut vec, &bias_vec);
  println!("result: {:?}", vec);
  assert_eq!(vec, correct_vec);
} 
