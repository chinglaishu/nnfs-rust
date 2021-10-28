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

fn main() {

  // let mut data_vec_vec_x: Vec<Vec<f32>> = vec![vec![0.9, 0.88, 0.05, 0.02], vec![0.1, 0.05, 0.95, 0.87]];
  // let mut data_vec_y: Vec<usize> = vec![0, 1];

  let mut data_vec_vec_x: Vec<Vec<f32>> = vec![vec![0.0; constant::DATA_SIZE]; constant::TOTAL_SAMPLE_NUM];
  let mut data_vec_y: Vec<usize> = vec![0; constant::TOTAL_SAMPLE_NUM];

  vector::random_x_y(&mut data_vec_vec_x, &mut data_vec_y);

  vector::shuffle_x_y(&mut data_vec_vec_x, &mut data_vec_y);

  println!("{:?}", data_vec_vec_x);
  println!("{:?}", data_vec_y);

  let mut layer_1 = dense_layer::create_dense_layer(0, DATA_SIZE, DENSE_LAYER_1_NEURON_NUM, 0.0, 0.0, 0.0, 0.0);
  let mut activatioin_relu_1 = activation::marco_create_activation_relu(BATCH_SIZE, DENSE_LAYER_1_NEURON_NUM);
  let mut layer_2 = dense_layer::create_dense_layer(1, DENSE_LAYER_1_NEURON_NUM, DENSE_LAYER_2_NEURON_NUM, 0.0, 0.0, 0.0, 0.0);
  let mut activation_relu_2 = activation::marco_create_activation_relu(BATCH_SIZE, DENSE_LAYER_2_NEURON_NUM);
  let mut layer_3 = dense_layer::create_dense_layer(2, DENSE_LAYER_2_NEURON_NUM, DENSE_LAYER_3_NEURON_NUM, 0.0, 0.0, 0.0, 0.0);
  let mut activation_softmax = activation::marco_create_activation_softmax(BATCH_SIZE, DENSE_LAYER_3_NEURON_NUM);
  // activation_softmax.forward(&activatioin_relu_1.output_vec);

  // println!("{:?}", activation_softmax.output_vec);
  
  // let mut predict_vec: Vec<usize> = vec![0; DATA_SIZE];

  // activation_softmax.predict(&mut predict_vec);

  // println!("{:?}", predict_vec);

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

  model.train(&data_vec_vec_x, &data_vec_y, 10, 1);


  let mut test_x: Vec<Vec<f32>> = vec![vec![0.0; constant::DATA_SIZE]; constant::BATCH_SIZE];
  let mut test_y: Vec<usize> = vec![0; constant::BATCH_SIZE];
  vector::random_x_y(&mut test_x, &mut test_y);
  vector::shuffle_x_y(&mut test_x, &mut test_y);

  println!("{:?}", test_x);
  println!("{:?}", test_y);

  model.test(&mut test_x, &test_y);

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

}

fn back_main() {
  let mut data_vec_vec_x: Vec<Vec<f32>> = vec![vec![0.9, 0.88, 0.05, 0.02], vec![0.1, 0.05, 0.95, 0.87]];
  let mut data_vec_y: Vec<usize> = vec![0, 1];

  println!("x: {:?}", data_vec_vec_x);

  let layer_1 = &mut dense_layer::create_dense_layer(0, DATA_SIZE, DENSE_LAYER_1_NEURON_NUM, 0.0, 0.0, 0.0, 0.0);

  match layer_1 {
    ModelLayer::ModelDenseLayer(dense_layer) => {
      dense_layer.forward(&data_vec_vec_x);
      vector::copy_vector_2d_f32(&mut data_vec_vec_x, &dense_layer.output_vec);
      println!("dense layer weight vec: {:?}", dense_layer.weight_vec);
      println!("dense layer output: {:?}", dense_layer.output_vec);
    },
    _ => {}
  }

  // println!("layer 1: {:?}", layer_1);

  let activation_softmax_1 = &mut activation::marco_create_activation_softmax(BATCH_SIZE, DENSE_LAYER_1_NEURON_NUM);

  let mut predict_vec: Vec<usize> = vec![0; BATCH_SIZE];

  match activation_softmax_1 {
    ModelLayer::ModelActivationSoftmax(activation_softmax) => {
      activation_softmax.forward(&data_vec_vec_x);
      vector::copy_vector_2d_f32(&mut data_vec_vec_x, &activation_softmax.output_vec);
      activation_softmax.predict(&mut predict_vec);
      println!("softmax output: {:?}", activation_softmax.output_vec);
    },
    _ => {}
  }

  println!("activation softmax predict: {:?}", predict_vec);

  let loss = &mut loss::create_loss_cross_entropy(BATCH_SIZE, 2);

  let loss_mean = loss.loss_calculate(&data_vec_vec_x, &data_vec_y);
  
  println!("loss mean: {}", loss_mean);




}