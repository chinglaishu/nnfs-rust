use crate::{accuracy::{Accuracy, AccuracyProcess}, activation::{ActivationReLu, ActivationSoftmax, Process}, constant::BATCH_SIZE, dense_layer::{DenseLayer, DenseLayerProcess}, loss::{LossCrossEntropy, LossProcess}, optimizer::{OptimizerAdam, OptimizerProcess}, vector};
use crate::model_trait::ModelLayerProcess;

#[derive(Debug)]
pub enum ModelLayer {
  ModelDenseLayer(DenseLayer),
  ModelActivationReLu(ActivationReLu),
  ModelActivationSoftmax(ActivationSoftmax),
}

#[derive(Debug)]
pub struct Model {
  pub layer_vec: Vec<ModelLayer>,
  pub loss: LossCrossEntropy,
  pub optimizer: OptimizerAdam,
  pub accuracy: Accuracy,
  pub predict_vec: Vec<usize>,
}

pub trait ModelProcess {
  fn train(&mut self, input_vec_x: &Vec<Vec<f32>>, input_vec_y: &Vec<usize>, epochs: usize, print_interval: usize) {}
  fn test(&mut self, test_x: &mut Vec<Vec<f32>>, test_y: &Vec<usize>) {}
  fn model_forward(&mut self, use_input_vec_x: &mut Vec<Vec<f32>>) {}
  fn model_backward(&mut self, output_vec_x: &Vec<Vec<f32>>, true_vec_y: &Vec<usize>) {}
  fn model_update_trainable_layer(&mut self) {}
}

impl ModelProcess for Model {
  fn train(&mut self, input_vec_x: &Vec<Vec<f32>>, input_vec_y: &Vec<usize>, epochs: usize, print_interval: usize) {
    for i in 0..epochs {
      self.loss.new_pass();
      self.accuracy.new_pass();
      let input_num = input_vec_x.len();
      let last_index = input_num;
      let batch_round = (input_num as f32 / BATCH_SIZE as f32).floor() as usize;
      for a in 0..batch_round {
        let start_index = a * BATCH_SIZE;
        let use_end_index = (a + 1) * BATCH_SIZE;
        let mut end_index = if use_end_index > last_index {last_index} else {use_end_index};
        let sample_num = end_index - start_index;
        let mut use_input_vec_x = input_vec_x[start_index..end_index].to_vec();
        let use_input_vec_y = input_vec_y[start_index..end_index].to_vec();
        self.model_forward(&mut use_input_vec_x);
        let loss_mean = self.loss.loss_calculate(&use_input_vec_x, &use_input_vec_y);
        let accuracy = self.accuracy.calculate(&self.predict_vec, &use_input_vec_y);
        println!("loss: {}, accuracy: {}", loss_mean, accuracy);
        // println!("predict: {:?}", self.predict_vec);
        // println!("true: {:?}", use_input_vec_y);
        // println!("accuracy: {}", accuracy);
        self.model_backward(&use_input_vec_x, &use_input_vec_y);
        self.optimizer.pre_update();
        self.model_update_trainable_layer();
        self.optimizer.post_update();
      }
      let accumulated_loss = self.loss.loss_calculate_accumulated();
      let accuracy = self.accuracy.calculate_accumulated();
      if i % print_interval == 0 {
        println!("Epoch: {}, loss: {}, accuracy: {}, learning rate: {}",
          i, accumulated_loss, accuracy, self.optimizer.current_learning_rate);
      }
    }
  }
  fn test(&mut self, test_x: &mut Vec<Vec<f32>>, test_y: &Vec<usize>) {
    self.loss.new_pass();
    self.accuracy.new_pass();
    let mut use_x = test_x.to_vec();
    self.model_forward(&mut use_x);
    let loss_mean = self.loss.loss_calculate(&use_x, &test_y);
    let accuracy = self.accuracy.calculate(&self.predict_vec, &test_y);
    println!("[Test] loss: {}, accuracy: {}", loss_mean, accuracy);
  }
  fn model_forward(&mut self, use_input_vec_x: &mut Vec<Vec<f32>>) {
    for i in 0..self.layer_vec.len() {
      let layer = &mut self.layer_vec[i];
      match layer {
        ModelLayer::ModelDenseLayer(dense_layer) => {
          dense_layer.forward(use_input_vec_x);
          vector::copy_vector_2d_f32(use_input_vec_x, &dense_layer.output_vec);
        },
        ModelLayer::ModelActivationReLu(activation_relu) => {
          activation_relu.forward(use_input_vec_x);
          vector::copy_vector_2d_f32(use_input_vec_x, &activation_relu.output_vec);
        },
        ModelLayer::ModelActivationSoftmax(activation_softmax) => {
          activation_softmax.forward(use_input_vec_x);
          vector::copy_vector_2d_f32(use_input_vec_x, &activation_softmax.output_vec);
          activation_softmax.predict(&mut self.predict_vec);
        },
      }
    }
  }
  fn model_backward(&mut self, output_vec_x: &Vec<Vec<f32>>, true_vec_y: &Vec<usize>) {
    self.loss.combine_loss_and_softmax_backward(output_vec_x, true_vec_y);
    let mut d_value_vec = &mut self.loss.d_input_vec;
    for i in (0..self.layer_vec.len()).rev() {
      let layer = &mut self.layer_vec[i];
      match layer {
        ModelLayer::ModelDenseLayer(dense_layer) => {
          dense_layer.backward(&d_value_vec);
          vector::copy_vector_2d_f32(&mut d_value_vec, &dense_layer.d_input_vec);
        },
        ModelLayer::ModelActivationReLu(activation_relu) => {
          activation_relu.backward(&d_value_vec);
          vector::copy_vector_2d_f32(&mut d_value_vec, &activation_relu.d_input_vec);
        },
        _ => {
        }
      }
    }
  }
  fn model_update_trainable_layer(&mut self) {
    for i in 0..self.layer_vec.len() {
      let layer = &mut self.layer_vec[i];
      match layer {
        ModelLayer::ModelDenseLayer(dense_layer) => {
          self.optimizer.update(dense_layer);
        },
        _ => {}
      }
    }
  }
}

// #[test]
// fn test_model_train() {
  
// }
