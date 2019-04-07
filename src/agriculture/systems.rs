use super::*;
use crate::prelude::*;
use chrono::Datelike;


pub const BUSHELS_PER_HECTARE_2_TO_1: f32 = 13.;
pub const HECTARES_TO_KM: u16 = 1000;
pub const SEED_RATIO: f32 = 1.343;
pub const BUSHELS_MALE: f32 = 24.;
pub const BUSHELS_FEMALE: f32 = 20.;
pub const BUSHELS_CHILD: f32 = 14.;


pub struct GrainYield;

impl<'a> System<'a> for GrainYield {
    type SystemData = (ReadStorage<'a, FarmData>,
                       ReadStorage<'a, RegBaseFarmData>,
                       ReadStorage<'a, RegionPop>,
                       WriteStorage<'a, FoodStock>,
                       ReadExpect<'a, Date>);

    fn run(&mut self, (farm_data, base, pop, mut food, date): Self::SystemData) {
        if date.day() != 1 || date.month() != 8 {
            return;
        }

        for (farm_data, base, pop, mut food) in (&farm_data, &base, &pop, &mut food).join() {
            // add some density dependence (ie. more people should marginally increase food output)

            // can only harvest so much
            let max_harvested_km = ((pop.pop_in_range(14, 60) * 1) as f64 / 1000.) as f32;
            let auc = if farm_data.auc > max_harvested_km {
                max_harvested_km + 0.5*(farm_data.auc - max_harvested_km)
            } else {
                farm_data.auc
            };
            let byield = base_yield(2, auc, base.fertility);

            food.bushels += byield;
        }
    }
}

/// Yield in bushels for given seed ratio {2,3,4}:1, area km2, fertility mean = 1.
pub fn base_yield(seed_ratio: u8, area: f32, fertility: f32) -> f32 {
    let bph = BUSHELS_PER_HECTARE_2_TO_1 * (seed_ratio - 1) as f32 * SEED_RATIO * fertility;
    let bpkm = bph * 1000.;
    bpkm * area
}

// Normal grain consumption for cohort
pub fn grain_for_cohort(c: &Cohort) -> f32 {
    let &Cohort { pop, mean_age, male, .. } = c;
    let bm = if mean_age < 14. {
        lerp(BUSHELS_CHILD, BUSHELS_MALE, mean_age as f32 / 15.)
    } else {
        BUSHELS_MALE
    } as f64;

    let bf = if mean_age < 14. {
        lerp(BUSHELS_CHILD, BUSHELS_FEMALE, mean_age as f32 / 15.)
    } else {
        BUSHELS_FEMALE
    } as f64;

    (bm * pop as f64 * male + bf as f64 * pop as f64 * (1. - male)) as f32
}

pub fn grain_for_pop(p: &RegionPop) -> f32 {
    p.cohorts.iter().fold(0., |grain, c| grain_for_cohort(c) + grain)
}

