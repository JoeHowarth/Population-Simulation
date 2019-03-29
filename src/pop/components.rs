use super::*;
use specs_derive;
use specs::prelude::*;
use std::{
    collections::VecDeque,
    ops::{
        Add, Mul, Div,
    },
    f64,
    f32,

};

#[derive(Component, Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
pub struct RegionPop {
    pub cohorts: VecDeque<Cohort>
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
pub struct Cohort {
    pub low_age: f64,
    pub high_age: f64,
    pub pop: usize,
    pub mean_age: f64,
    pub male: f64,
}

const NUM_COHORTS: usize = 17;

impl RegionPop {
    pub fn new(dist: &[f32; NUM_COHORTS], pop: usize) -> RegionPop {
        let cohorts = (0..NUM_COHORTS).map(|i| {
            Cohort::new(i * 5, dist[i as usize], pop)
        }).collect();

        RegionPop {
            cohorts,
        }
    }

    pub fn update(&mut self, brs: &[f32; NUM_COHORTS], drs: &[f32; NUM_COHORTS], dt: u16) {
        let mut newborns = 0;
        for c in &mut self.cohorts {
            let br = lerp_pop_arr(c.mean_age, brs);
            let dr = lerp_pop_arr(c.mean_age, drs);
            let dt_frac = dt as f64 / 365.;

            // births
            let women = c.pop as f64 * (1. - c.male);
            newborns += (women * br * dt_frac) as usize; // yearly birth rate * women * frac of year

            // deaths
            let deaths = c.pop as f64 * dr * dt_frac;
            c.pop -= deaths as usize;

            // update ages
            c.low_age += dt_frac ;
            c.high_age += dt_frac;
            c.mean_age += dt_frac;
        }
        if self.cohorts.front().unwrap().high_age > 5.1 {
            // create new 'youngest' cohort
            self.cohorts.push_front(Cohort::youngest(self.cohorts[0].low_age, newborns));

            // merge oldest cohort into next oldest
            let old = self.cohorts.pop_back().unwrap();
            self.cohorts.back_mut().unwrap().merge(&old);
        } else {
            // add newborns to youngest
            let mut c = self.cohorts.front_mut().unwrap();
            c.mean_age = (newborns as f64 * c.low_age * 0.5 + c.pop as f64 * c.mean_age) / (newborns + c.pop) as f64;
            c.pop += newborns;
            c.low_age = 0.;
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
    fn new(low_age: usize, dist: f32, total_pop: usize) -> Cohort {
        Cohort {
            low_age: low_age as f64,
            high_age: (low_age + 5) as f64,
            pop: (total_pop as f64 * dist as f64) as usize,
            mean_age: low_age as f64 + 2.5,
            male: 0.5,
        }
    }

    fn youngest(high_age: f64, pop: usize) -> Cohort {
        Cohort {
            low_age: 0.,
            high_age,
            pop,
            mean_age: high_age / 2.,
            male: 0.5,
        }
    }

    fn merge(&mut self, other: &Cohort) {
        self.mean_age = (self.mean_age * self.pop as f64 + other.mean_age * other.pop as f64) / (self.pop + other.pop) as f64;
        self.pop += other.pop;
        self.high_age = other.mean_age; // questionable...
        self.male = (self.male * self.pop as f64 + other.male * other.pop as f64) / (self.pop + other.pop) as f64;
    }
}

pub fn lerp_pop_arr(x: f64, arr: &[f32; NUM_COHORTS]) -> f64 {
    let l_i = (x / 5.).floor() as usize;
    let h_i = (x / 5.).ceil() as usize;
    let l: f64 = (arr[l_i].clone()).into();
    let h: f64 = (arr[h_i.min(NUM_COHORTS - 1)].clone()).into();
    let a = x / 5. - l_i as f64;


    l * a + h * (1. - a)
}


