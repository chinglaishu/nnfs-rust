use crate::vector_calculate;


#[derive(Debug)]
pub struct Accuracy {
  pub accumulated_sum: f32,
  pub accumulated_count: usize,
}

pub trait AccuracyProcess {
  fn calculate(&mut self, predict_vec: &Vec<usize>, true_vec: &Vec<usize>) -> f32 {0.0}
  fn calculate_accumulated(&self) -> f32 {0.0}
  fn new_pass(&mut self) {}
}

impl AccuracyProcess for Accuracy {
  fn calculate(&mut self, predict_vec: &Vec<usize>, true_vec: &Vec<usize>) -> f32 {
    let correct_count = vector_calculate::count_correct_predict(&predict_vec, &true_vec);
    let accuracy: f32 = correct_count as f32 / predict_vec.len() as f32;
    self.accumulated_sum += correct_count as f32;
    self.accumulated_count += predict_vec.len();
    accuracy
  }
  fn calculate_accumulated(&self) -> f32 {
    self.accumulated_sum / self.accumulated_count as f32
  }
  fn new_pass(&mut self) {
    self.accumulated_sum = 0.0;
    self.accumulated_count = 0;
  }
}
