use std::fs::File;
use std::env;

use constant::DATA_SIZE;

use crate::{accuracy::Accuracy, activation::Process, constant::{BATCH_SIZE, DENSE_LAYER_1_NEURON_NUM, DENSE_LAYER_2_NEURON_NUM, DENSE_LAYER_3_NEURON_NUM}, dense_layer::DenseLayer, loss::{LossCrossEntropy, LossProcess}, model::{Model, ModelLayer, ModelProcess}, model_trait::ModelLayerProcess, optimizer::OptimizerAdam};

#[path = "utils/link_list.rs"] mod link_list;
#[path = "utils/test_data.rs"] mod test_data;
#[path = "constant/constant.rs"] mod constant;
#[path = "utils/vector.rs"] mod vector;
#[path = "utils/utils_function.rs"] mod utils_function;
#[path = "model/dense_layer.rs"] mod dense_layer;
#[path = "model/activation.rs"] mod activation;
#[path = "utils/vector_calculate.rs"] mod vector_calculate;
#[path = "model/loss.rs"] mod loss;
#[path = "model/optimizer.rs"] mod optimizer;
#[path = "model/model.rs"] mod model;
#[path = "model/accuracy.rs"] mod accuracy;
#[path = "utils/model_trait.rs"] mod model_trait;

use std::io::Write;

fn main() {

  let args: Vec<String> = env::args().collect();
  let is_save = args.contains(&String::from("save"));
  let is_load = args.contains(&String::from("load"));
  let is_train = args.contains(&String::from("train"));

  let mut data_vec_vec_x: Vec<Vec<f32>> = vec![vec![0.0; constant::DATA_SIZE]; constant::TOTAL_SAMPLE_NUM];
  let mut data_vec_y: Vec<usize> = vec![0; constant::TOTAL_SAMPLE_NUM];

  vector::random_x_y(&mut data_vec_vec_x, &mut data_vec_y);
  vector::shuffle_x_y(&mut data_vec_vec_x, &mut data_vec_y);

  // println!("{:?}", data_vec_vec_x);
  // println!("{:?}", data_vec_y);

  let mut layer_1 = dense_layer::create_dense_layer(0, DATA_SIZE, DENSE_LAYER_1_NEURON_NUM, 0.0, 0.0, 0.0, 0.0);
  let mut activatioin_relu_1 = activation::marco_create_activation_relu(BATCH_SIZE, DENSE_LAYER_1_NEURON_NUM);
  let mut layer_2 = dense_layer::create_dense_layer(1, DENSE_LAYER_1_NEURON_NUM, DENSE_LAYER_2_NEURON_NUM, 0.0, 0.0, 0.0, 0.0);
  let mut activation_relu_2 = activation::marco_create_activation_relu(BATCH_SIZE, DENSE_LAYER_2_NEURON_NUM);
  let mut layer_3 = dense_layer::create_dense_layer(2, DENSE_LAYER_2_NEURON_NUM, DENSE_LAYER_3_NEURON_NUM, 0.0, 0.0, 0.0, 0.0);
  let mut activation_softmax = activation::marco_create_activation_softmax(BATCH_SIZE, DENSE_LAYER_3_NEURON_NUM);

  let mut loss = loss::create_loss_cross_entropy(BATCH_SIZE, DENSE_LAYER_3_NEURON_NUM);
  let mut optimizer = OptimizerAdam {
    learning_rate: 0.005,
    current_learning_rate: 0.005,
    decay: 0.001,
    iteration_count: 0,
    epsilon: 0.0000001,
    beta_1: 0.9,
    beta_2: 0.999,
  };
  let mut accuracy = Accuracy {
    accumulated_sum: 0.0,
    accumulated_count: 0,
  };

  let mut model = Model {
    layer_vec: vec![layer_1, activatioin_relu_1, layer_2, activation_relu_2, layer_3, activation_softmax],
    loss,
    accuracy,
    optimizer,
    predict_vec: vec![0; BATCH_SIZE],
  };

  if is_load {
    model.load_dense_layer();
  }

  if is_train {
    model.train(&data_vec_vec_x, &data_vec_y, 8, 1);
  }

  if is_save {
    model.save_dense_layer();
  }

  let mut test_x: Vec<Vec<f32>> = vec![vec![0.0; constant::DATA_SIZE]; constant::BATCH_SIZE];
  let mut test_y: Vec<usize> = vec![0; constant::BATCH_SIZE];
  vector::random_x_y(&mut test_x, &mut test_y);
  vector::shuffle_x_y(&mut test_x, &mut test_y);

  // println!("{:?}", test_x);
  // println!("{:?}", test_y);

  model.test(&mut test_x, &test_y);
}
