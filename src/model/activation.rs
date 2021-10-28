use crate::model_trait::ModelLayerProcess;
use crate::vector;
use crate::vector_calculate;
use std::f32::consts;
use crate::model::ModelLayer;

#[derive(Debug)]
pub struct ActivationReLu {
  pub input_vec: Vec<Vec<f32>>, // -> output of before layer
  pub output_vec: Vec<Vec<f32>>,
  pub d_input_vec: Vec<Vec<f32>>,
}

#[derive(Debug)]
pub struct ActivationSoftmax {
  pub input_vec: Vec<Vec<f32>>,
  pub output_vec: Vec<Vec<f32>>,
  pub d_input_vec: Vec<Vec<f32>>,
}

pub trait Process {
  fn predict(&self, predict_vec: &mut Vec<usize>) {}
}

impl ModelLayerProcess for ActivationReLu {
  fn forward(&mut self, new_input_vec: &Vec<Vec<f32>>) {
    vector::copy_vector_2d_f32(&mut self.input_vec, &new_input_vec);
    vector::limit_value_vec_2d(&mut self.output_vec, &self.input_vec);
  }
  fn backward(&mut self, d_value_vec: &Vec<Vec<f32>>) {
      for i in 0..d_value_vec.len() {
        for a in 0..d_value_vec[i].len() {
          let value = d_value_vec[i][a];
          self.d_input_vec[i][a] = if value < 0.0 {0.0} else {value};
        }
      }
  }
}

impl ModelLayerProcess for ActivationSoftmax {
  fn forward(&mut self, new_input_vec: &Vec<Vec<f32>>) {
    vector_calculate::change_to_minus_max_row(&mut self.output_vec, &new_input_vec);
    // println!("output after a {:?} ", self.output_vec);
    for x in &mut self.output_vec {
      vector::pow_value_vec(x, consts::E);
    }
    // println!("output after x {:?} ", self.output_vec);
    vector_calculate::change_to_probability(&mut self.output_vec);
  }
}

impl Process for ActivationSoftmax {
  fn predict(&self, predict_vec: &mut Vec<usize>) {
    for i in 0..self.output_vec.len() {
      let max_value_index = vector_calculate::get_max_value_index(&self.output_vec[i]);
      predict_vec[i] = max_value_index;
    }
  }
}

macro_rules! marco_create_activation {
  ($func:ident, $type:ident, $model_layer_type:expr) => {
    pub fn $func(input_shape_one: usize, input_shape_two: usize) -> ModelLayer {
      let activation = $type {
        input_vec: vec![vec![0.0; input_shape_two]; input_shape_one], 
        output_vec: vec![vec![0.0; input_shape_two]; input_shape_one], 
        d_input_vec: vec![vec![0.0; input_shape_two]; input_shape_one],
      };
      $model_layer_type(activation)
    }
  };
}

marco_create_activation!(marco_create_activation_relu, ActivationReLu, ModelLayer::ModelActivationReLu);
marco_create_activation!(marco_create_activation_softmax, ActivationSoftmax, ModelLayer::ModelActivationSoftmax);
