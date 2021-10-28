use crate::{dense_layer::DenseLayer, vector, vector_calculate};


#[derive(Debug)]
pub struct LossCrossEntropy {
  pub accumulated_sum: f32,
  pub accumulated_count: i32,
  pub trainable_layer: Vec<DenseLayer>,
  pub d_input_vec: Vec<Vec<f32>>,
}

pub trait LossProcess {
  fn loss_calculate(&mut self, predict_vec: &Vec<Vec<f32>>, true_vec: &Vec<usize>) -> f32 {0.0}
  fn loss_calculate_accumulated(&mut self) -> f32 {0.0}
  fn new_pass(&mut self) {}
  fn combine_loss_and_softmax_backward(&mut self, d_value_vec: &Vec<Vec<f32>>, true_vec: &Vec<usize>) {}
} 

impl LossProcess for LossCrossEntropy {
  fn loss_calculate(&mut self, predict_vec: &Vec<Vec<f32>>, true_vec: &Vec<usize>) -> f32 {
    let total_data_num = true_vec.len();
    let loss_sum: f32 = get_loss(predict_vec, true_vec);
    let loss_mean: f32 = loss_sum / total_data_num as f32;
    self.accumulated_sum += loss_sum;
    self.accumulated_count += total_data_num as i32;
    loss_mean
  }
  fn loss_calculate_accumulated(&mut self) -> f32 {
    self.accumulated_sum / self.accumulated_count as f32
  }
  fn new_pass(&mut self) {
    self.accumulated_sum = 0.0;
    self.accumulated_count = 0;
  }
  fn combine_loss_and_softmax_backward(&mut self, d_value_vec: &Vec<Vec<f32>>, true_vec: &Vec<usize>) {
    for i in 0..d_value_vec.len() {
      let true_index = true_vec[i];
      for a in 0..d_value_vec[0].len() {
        let value = d_value_vec[i][a];
        let use_value: f32 = if a == true_index {value - 1.0} else {value};
        self.d_input_vec[i][a] = use_value / d_value_vec.len() as f32;
      }
    }  
  }
}

pub fn get_loss(predict_vec: &Vec<Vec<f32>>, true_vec: &Vec<usize>) -> f32 {
  let use_small_num: f32 = 0.1;
  let small_num: f32 = use_small_num.powf(7.0) as f32;
  let true_vec_len = true_vec.len();
  let mut correct_confidence_vec: Vec<f32> = vec![0.0; true_vec.len()];
  for i in 0..true_vec_len {
    let index = true_vec[i] as usize;
    let mut value: f32 = predict_vec[i][index];
    value = vector_calculate::limit_value(value, small_num, 1.0 - small_num);
    value = -1.0 * value.log10();
    correct_confidence_vec[i] = value;
  }
  vector_calculate::limit_value_vec(&mut correct_confidence_vec, small_num, 1.0 - small_num);
  vector::time_value_vec(&mut correct_confidence_vec, -1.0);
  let sum = vector_calculate::get_sum(&correct_confidence_vec);
  sum
}

pub fn create_loss_cross_entropy(input_shape_one: usize, input_shape_two: usize) -> LossCrossEntropy {
  let loss = LossCrossEntropy {
    accumulated_sum: 0.0,
    accumulated_count: 0,
    trainable_layer: vec![],
    d_input_vec: vec![vec![0.0; input_shape_two]; input_shape_one],
  };
  loss
}
