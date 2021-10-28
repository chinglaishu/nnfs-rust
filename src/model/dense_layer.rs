use crate::{constant::{BATCH_SIZE, DATA_SIZE}, model::ModelLayer, model_trait::{ModelLayerProcess}, vector, vector_calculate::{self, transpose_vec}};

#[derive(Debug)]
pub struct DenseLayer {
  pub id: i32,
  pub input_num: usize,
  pub neuron_num: usize,
  pub weight_regularizer_l1: f32,
  pub weight_regularizer_l2: f32,
  pub bias_regularizer_l1: f32,
  pub bias_regularizer_l2: f32,
  pub weight_vec: Vec<Vec<f32>>, 
  pub bias_vec: Vec<Vec<f32>>, 
  pub input_vec: Vec<Vec<f32>>, 
  pub output_vec: Vec<Vec<f32>>, 
  pub d_input_vec: Vec<Vec<f32>>,
  pub d_weight_vec: Vec<Vec<f32>>,
  pub d_bias_vec: Vec<Vec<f32>>,
  pub weight_momentum_vec: Vec<Vec<f32>>,
  pub bias_momentum_vec: Vec<Vec<f32>>,
  pub weight_cache_vec: Vec<Vec<f32>>,
  pub bias_cache_vec: Vec<Vec<f32>>,
}

impl ModelLayerProcess for DenseLayer {
  fn forward(&mut self, new_input_vec: &Vec<Vec<f32>>) {
    vector::copy_vector_2d_f32(&mut self.input_vec, &new_input_vec);
    vector_calculate::dot_product(&mut self.output_vec, &self.input_vec, &self.weight_vec);
    vector_calculate::add_bias_to_matrix(&mut self.output_vec, &self.bias_vec);
  }
  fn backward(&mut self, d_value_vec: &Vec<Vec<f32>>) {
    self.get_d_weight_vec(d_value_vec);
    self.get_d_bias_vec(d_value_vec);
    regularization(&mut self.d_weight_vec, &self.weight_vec, self.weight_regularizer_l1, self.weight_regularizer_l2);
    regularization(&mut self.d_bias_vec, &self.bias_vec, self.bias_regularizer_l1, self.bias_regularizer_l2);
    self.get_d_input_vec(d_value_vec);
  }
}

pub trait DenseLayerProcess {
  fn get_d_weight_vec(&mut self, d_value_vec: &Vec<Vec<f32>>) {}
  fn get_d_bias_vec(&mut self, d_value_vec: &Vec<Vec<f32>>) {}
  fn get_d_input_vec(&mut self, d_value_vec: &Vec<Vec<f32>>) {}
  fn print_data(&self) {}
}

impl DenseLayerProcess for DenseLayer {
  fn get_d_weight_vec(&mut self, d_value_vec: &Vec<Vec<f32>>) {
    let mut input_transpose_vec = vec![vec![0.0; BATCH_SIZE]; self.input_num];
    vector_calculate::transpose_vec(&mut input_transpose_vec, &self.input_vec);
    vector_calculate::dot_product(&mut self.d_weight_vec, &input_transpose_vec, &d_value_vec);
  }
  fn get_d_bias_vec(&mut self, d_value_vec: &Vec<Vec<f32>>) {
    vector::fill_vec_2d_f32(&mut self.d_bias_vec, 0.0);
    for i in 0..self.d_bias_vec.len() {
      for a in 0..self.d_bias_vec[i].len() {
        self.d_bias_vec[0][a] += d_value_vec[i][a];
      }
    }
  }
  fn get_d_input_vec(&mut self, d_value_vec: &Vec<Vec<f32>>) {
    let mut weight_transpose_vec = vec![vec![0.0; self.input_num]; self.neuron_num];
    transpose_vec(&mut weight_transpose_vec, &self.weight_vec);
    vector_calculate::dot_product(&mut self.d_input_vec, d_value_vec, &weight_transpose_vec);
  }
  fn print_data(&self) {
    println!("weight vec: {:?}", self.weight_vec);
    println!("bias vec: {:?}", self.bias_vec);
  }
}

fn regularization(d_value_vec: &mut Vec<Vec<f32>>, value_vec: &Vec<Vec<f32>>, l1: f32, l2: f32) {
  for i in 0..d_value_vec.len() {
    for a in 0..d_value_vec[i].len() {
      let time_num: f32 = if value_vec[i][a] < 0.0 {-1.0} else {1.0};
      d_value_vec[i][a] += l1 * time_num + 2.0 * l2 * value_vec[i][a];
    }
  }
}

pub fn create_dense_layer(id: i32, input_num: usize, neuron_num: usize, weight_regularizer_l1: f32,
  weight_regularizer_l2: f32, bias_regularizer_l1: f32, bias_regularizer_l2: f32) -> ModelLayer {
  let mut dense_layer = DenseLayer {
    id,
    input_num,
    neuron_num,
    weight_regularizer_l1,
    weight_regularizer_l2,
    bias_regularizer_l1,
    bias_regularizer_l2,
    weight_vec: vec![vec![0.0; neuron_num]; input_num],
    bias_vec: vec![vec![0.0; neuron_num]; 1],
    input_vec: vec![vec![0.0; input_num]; BATCH_SIZE],
    output_vec: vec![vec![0.0; neuron_num]; BATCH_SIZE],

    d_input_vec: vec![vec![0.0; input_num]; BATCH_SIZE],
    d_weight_vec: vec![vec![0.0; neuron_num]; input_num],
    d_bias_vec: vec![vec![0.0; neuron_num]; 1],
    weight_momentum_vec: vec![vec![0.0; neuron_num]; input_num],
    bias_momentum_vec: vec![vec![0.0; neuron_num]; 1],
    weight_cache_vec: vec![vec![0.0; neuron_num]; input_num],
    bias_cache_vec: vec![vec![0.0; neuron_num]; 1],
  };
  vector::random_weight_vec(&mut dense_layer.weight_vec);
  vector::fill_vec_2d_f32(&mut dense_layer.bias_vec, 0.0);
  ModelLayer::ModelDenseLayer(dense_layer)
}
