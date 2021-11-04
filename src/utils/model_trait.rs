
use crate::model;

pub trait ModelLayerProcess {
  fn forward(&mut self, new_input_vec: &Vec<Vec<f32>>) {}
  fn backward(&mut self, d_value_vec: &Vec<Vec<f32>>) {}
  fn get_weight_bias_vec(&mut self) {}
}
