
struct DenseLayer {
  id: i32,
  input_num: i32,
  neuron_num: i32,
  weight_regularizer_L1: f32,
  weight_regularizer_L2: f32,
  bias_regularizer_L1: f32,
  bias_regularizer_L2: f32,
  weight_vec: Vec<Vec<f32>>, 
  bias_vec: Vec<Vec<f32>>, 
  input_vec: Vec<Vec<f32>>, 
  output_vec: Vec<Vec<f32>>, 
  dInput_vec: Vec<Vec<f32>>,
  dWeight_vec: Vec<Vec<f32>>,
  dBias_vec: Vec<Vec<f32>>,
  weight_momentum_vec: Vec<Vec<f32>>,
  bias_momentum_vec: Vec<Vec<f32>>,
  weight_cache_vec: Vec<Vec<f32>>,
  bias_cache_vec: Vec<Vec<f32>>,
}


