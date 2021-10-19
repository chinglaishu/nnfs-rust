use constant::DATA_NUM;


#[path = "utils/link_list.rs"] mod link_list;
#[path = "utils/test_data.rs"] mod test_data;
#[path = "constant/constant.rs"] mod constant;
#[path = "utils/vector.rs"] mod vector;
#[path = "utils/utils_function.rs"] mod utils_function;

// fn get_data_arr_arr_x<'a>(data_arr_arr_x: &mut Vec<&mut Vec<f32>>) {
//   let start_num: f32 = 0.0;
//   for i in 0..constant::DATA_NUM {
//     data_arr_arr_x[0] = &mut data_arr_arr_x[0];
//   }
// }

fn main() {

  let mut data_vec_vec_x: Vec<Vec<f32>> = vec![vec![0.0; constant::DATA_SIZE]; constant::DATA_NUM];
  let mut data_vec_y: Vec<i32> = vec![0; constant::DATA_NUM];

  vector::random_x_y(&mut data_vec_vec_x, &mut data_vec_y);

  println!("{:?}", data_vec_vec_x);
  println!("{:?}", data_vec_y);
  
  vector::shuffle_x_y(&mut data_vec_vec_x, &mut data_vec_y);

  println!("{:?}", data_vec_vec_x);
  println!("{:?}", data_vec_y);
  

  // println!("{}", constant::DATA_SIZE);
  // println!("Hello, world!");
  // let mut data_arr: [[f32; constant::DATA_SIZE]; constant::X_NUM] = [[0.0; constant::DATA_SIZE]; constant::X_NUM];
  // test_data::random_array_2d(&data_arr);
  // println!("{:?}", data_arr);

  // println!("{}", constant::DATA_SIZE);
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
  // let mut data_arr_y = vec![0; constant::DATA_NUM];
  // test_data::random_x_y(&mut data_arr_arr_x, &mut data_arr_y);
  // println!("{:?}", data_arr_arr_x);

}
