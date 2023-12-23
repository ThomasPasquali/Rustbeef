use std::ops::RangeInclusive;
use rand::{Rng, rngs::ThreadRng};

use super::Choice;


const DEFAULT_MIN_PROB: f32 = 0.01;

pub struct ProbabilisticChoice {
  rng: ThreadRng,
  intervals: Vec<RangeInclusive<usize>>
}
impl ProbabilisticChoice {
  pub fn new (weights: Vec<f32>) -> Result<ProbabilisticChoice, String> {
    let u_weights: Vec<usize> = ProbabilisticChoice::normalize_to_sum_100(weights);
    let tot: usize = u_weights.iter().sum();
    if tot != 100 {
      return Err(format!("Total ({}) is not 100", tot).to_string());
    }
    let mut intervals: Vec<RangeInclusive<usize>> = vec![];
    let mut i: usize = 0;
    for w in u_weights {
      let j = i + w;
      intervals.push(i..=j);
      i = j;
    }

    Ok(ProbabilisticChoice { intervals, rng: rand::thread_rng() })
  }

  fn normalize_to_sum_100(weights: Vec<f32>) -> Vec<usize> {
    let sum: f32 = weights.iter().sum();
    let mut normalized_values: Vec<usize> = weights
        .iter()
        .map(|value| ((value / sum) * 100.0) as usize)
        .collect();

    let current_sum: usize = normalized_values.iter().sum();
    let diff = 100 - current_sum;

    if diff != 0 {
      let max = normalized_values.iter().max().unwrap();
      let max_index = normalized_values.iter().position(|v| v == max).unwrap();
      normalized_values[max_index] += diff;
    }

    println!("Prob weights:\n{:?}\n{:?}", &weights, &normalized_values);
    normalized_values
  }


  pub fn make (&mut self) -> usize {
    let x: usize = self.rng.gen_range(1..=100);
    for (i, r) in self.intervals.iter().enumerate() {
      if r.contains(&x) {
        return i;
      }
    }
    0
  }

  pub fn inverse_weighted_choice (costs: &Vec<Choice>) -> Result<usize, String> {
    let tot: usize = costs.iter().sum();
    let inverse_proportions: Vec<f32> = costs.iter()
      .map(|cost| (tot as f32) / (*cost as f32))
      .collect();

    let proportions_tot: f32 = inverse_proportions.iter().sum();
    let normalized_proportions: Vec<f32> = inverse_proportions.iter()
      .map(|cost| (*cost as f32) / (proportions_tot as f32))
      .collect();

    let mut choice = ProbabilisticChoice::new(normalized_proportions)?;
    // let mut choice = ProbabilisticChoice::new(normalized_proportions.iter().map(|x| if let Some(value) = x {*value} else {DEFAULT_MIN_PROB}).collect())?;
    Ok(choice.make())
  }

}

#[cfg(test)]
mod prob_choice_tests {
  use super::ProbabilisticChoice;
  #[test]
  fn test () {
    let p = vec![0.15, 0.2, 0.65];
    let choice = ProbabilisticChoice::new(p.clone());
    match choice {
        Ok(mut c) => {
          println!("{:?}", &p);
          for _ in 0..50 {
            println!("Choice: {}", c.make());
          }
        },
        Err(e) => {
          println!("{e}")
        }
    }
  }
}