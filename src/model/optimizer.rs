use crate::dense_layer::DenseLayer;

#[derive(Debug)]
pub struct OptimizerAdam {
  pub learning_rate: f32,
  pub current_learning_rate: f32,
  pub decay: f32,
  pub iteration_count: usize,
  pub epsilon: f32,
  pub beta_1: f32,
  pub beta_2: f32,
}

pub trait OptimizerProcess {
  fn pre_update(&mut self) {}
  fn update(&mut self, dense_layer: &mut DenseLayer) {}
  fn post_update(&mut self) {}
}

impl OptimizerProcess for OptimizerAdam {
  fn pre_update(&mut self) {
    self.current_learning_rate = self.learning_rate * (1.0 / (1.0 + self.decay * self.iteration_count as f32));
    // println!("current learning rate: {}", self.current_learning_rate);
  }
  fn update(&mut self, dense_layer: &mut DenseLayer) {
    calculate_momentum(self.beta_1, &mut dense_layer.weight_momentum_vec, &dense_layer.d_weight_vec);
    calculate_momentum(self.beta_1, &mut dense_layer.bias_momentum_vec, &dense_layer.d_bias_vec);

    let mut weight_momentum_correct_vec: Vec<Vec<f32>> = vec![vec![0.0; dense_layer.neuron_num]; dense_layer.input_num];
    let mut bias_momentum_correct_vec: Vec<Vec<f32>> = vec![vec![0.0; dense_layer.neuron_num]; 1];

    calculate_correct(self.beta_1, self.iteration_count, &mut weight_momentum_correct_vec, &dense_layer.weight_momentum_vec);
    calculate_correct(self.beta_1, self.iteration_count, &mut bias_momentum_correct_vec, &dense_layer.bias_momentum_vec);

    calculate_cache(self.beta_2, &mut dense_layer.weight_cache_vec, &dense_layer.d_weight_vec);
    calculate_cache(self.beta_2, &mut dense_layer.bias_cache_vec, &dense_layer.d_bias_vec);

    let mut weight_cache_correct_vec: Vec<Vec<f32>> = vec![vec![0.0; dense_layer.neuron_num]; dense_layer.input_num];
    let mut bias_cache_correct_vec: Vec<Vec<f32>> = vec![vec![0.0; dense_layer.neuron_num]; 1];

    calculate_correct(self.beta_2, self.iteration_count, &mut weight_cache_correct_vec, &dense_layer.weight_cache_vec);
    calculate_correct(self.beta_2, self.iteration_count, &mut bias_cache_correct_vec, &dense_layer.bias_cache_vec);

    // println!("weight_momentum_correct_vec: {:?}", weight_momentum_correct_vec);
    // println!("weight_cache_correct_vec: {:?}", weight_cache_correct_vec);
    // println!("bias_momentum_correct_vec: {:?}", bias_momentum_correct_vec);
    // println!("bias_cache_correct_vec: {:?}", bias_cache_correct_vec);
    
    // println!("update weight --------------------------------------------------------");
    update_value(self.current_learning_rate, self.epsilon, &mut dense_layer.weight_vec, &weight_momentum_correct_vec, &weight_cache_correct_vec);
    // println!("update bias --------------------------------------------------------");
    update_value(self.current_learning_rate, self.epsilon, &mut dense_layer.bias_vec, &bias_momentum_correct_vec, &bias_cache_correct_vec);
  }
  fn post_update(&mut self) {
    self.iteration_count += 1;
  }
}

fn calculate_momentum(beta_1: f32, momentum_vec: &mut Vec<Vec<f32>>, d_value_vec: &Vec<Vec<f32>>) {
  for i in 0..momentum_vec.len() {
    for a in 0..momentum_vec[i].len() {
      momentum_vec[i][a] = beta_1 * momentum_vec[i][a] + (1.0 - beta_1) * d_value_vec[i][a];
    }
  }
}

fn calculate_cache(beta_2: f32, cache_vec: &mut Vec<Vec<f32>>, d_value_list: &Vec<Vec<f32>>) {
  for i in 0..cache_vec.len() {
    for a in 0..cache_vec[i].len() {
      let add = beta_2 * cache_vec[i][a] + (1.0 - beta_2) * d_value_list[i][a] * d_value_list[i][a];
      cache_vec[i][a] = add;    
    }
  }
}

fn calculate_correct(beta: f32, iteration_count: usize, new_vec: &mut Vec<Vec<f32>>, momentum_or_cache_vec: &Vec<Vec<f32>>) {
  for i in 0..momentum_or_cache_vec.len() {
    for a in 0..momentum_or_cache_vec[i].len() {
      new_vec[i][a] = momentum_or_cache_vec[i][a] / (1.0 - beta.powf(iteration_count as f32 + 1.0));
    }
  }
}

fn update_value(current_learning_rate: f32, epsilon: f32, value_vec: &mut Vec<Vec<f32>>, correct_momentum_vec: &Vec<Vec<f32>>,
  correct_cache_vec: &Vec<Vec<f32>>) {
  for i in 0..correct_momentum_vec.len() {
    for a in 0..correct_momentum_vec[i].len() {
      // println!("before: {}", value_vec[i][a]);
      let add_value = (-current_learning_rate * correct_momentum_vec[i][a]) /
        (correct_cache_vec[i][a].sqrt() + epsilon);
      value_vec[i][a] += add_value;
      // println!("value: {}, add value: {}", value_vec[i][a], add_value);
    }
  }
}
