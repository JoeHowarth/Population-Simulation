use super::*;
use specs_derive;
use specs::prelude::*;

#[derive(Component, Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
pub struct RegionPop {
    pub cohorts: Vec<Cohort>
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
pub struct Cohort {
    pub low_age: u8,
    pub high_age: u8,
    pub pop: usize,
    pub mean_age: f32,
}


impl RegionPop {
    pub fn new(dist: &[f32; 14], pop: usize) -> RegionPop {
        let cohorts = (0..14).map(|i| {
            Cohort::new(i * 5, dist[i as usize], pop)
        }).collect();

        RegionPop {
            cohorts,
        }
    }

    pub fn pop(&self) -> usize {
        self.cohorts.iter().fold(0, |acc, c| {
            acc + c.pop
        })
    }

    // pub fn dist(&self) ->  [usize; 14]
    // pub fn pop_in_range(low: u8, hi: u8) -> usize
}

impl Cohort {
    pub fn new(low_age: u8, dist: f32, total_pop: usize) -> Cohort {
        Cohort {
            low_age,
            high_age: low_age + 5,
            pop: (total_pop as f64 * dist as f64) as usize,
            mean_age: low_age as f32 + 2.5,
        }
    }
}


