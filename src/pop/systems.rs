use super::*;
use specs::prelude::*;
use specs_derive;
use crate::misc::time::Date;
use chrono::Datelike;
use crate::prelude::*;
use crate::pop::NUM_COHORTS;

const DEATH_RATES: [f32; 17] = [
    0.2, 0.045, 0.035, 0.02, 0.007, 0.005, 0.005, 0.005,
    0.01, 0.015, 0.02, 0.03, 0.04, 0.06, 0.1, 0.3, 0.8
];
const BIRTH_RATES: [f32; 17] = [0., 0., 0., 0.01, 0.02, 0.55, 0.45, 0.25, 0.1, 0., 0., 0., 0., 0., 0., 0., 0.];
const HARVEST_MONTH: u32 = 9;
const HEALTH_CHANGE_RATE: f32 = 0.10;

pub struct PopUpdate {
    pub old_date: Date,
}

// Changes Cohort health in response to food scarcity, disease, sanitation etc.
pub struct HealthUpdate;

// Recalculate Monthly Food Consumption
pub struct FoodConsumptionSystem;

impl<'a> System<'a> for PopUpdate {
    type SystemData = (ReadStorage<'a, Health>,
                       WriteStorage<'a, RegionPop>,
                       ReadExpect<'a, Date>);

    fn run(&mut self, (health, mut pop, date): Self::SystemData) {
        let day = date.day();
        let month = date.month();
        if day == 15 && (month == 1 || month == 3 || month == 6 || month == 9) {
            let dt = (*date - self.old_date).num_days();
            self.old_date = date.clone();


            for (p, h) in (&mut pop, &health).join() {
                let brs: ArrayVec<[f32; NUM_COHORTS]> = BIRTH_RATES.iter().map(|&br| br * h.health).collect();
                let drs: ArrayVec<[f32; NUM_COHORTS]> = DEATH_RATES.iter().map(|&dr| dr * h.health).collect();
                p.update(brs.as_slice(), &drs, dt as u16);
            }
        }
    }
}




impl<'a> System<'a> for HealthUpdate {
    type SystemData = (WriteStorage<'a, Health>,
                       ReadStorage<'a, FoodConsumption>,
                       ReadStorage<'a, FoodStock>);

    fn run(&mut self, (mut health, consump, food): Self::SystemData) {
        for (health, consump, food) in (&mut health, &consump, &food).join() {
            let r = consump.ratio_of_norm;

            // later add more interesting interactions with other systems
            let target_health = r;


            // half-life for health change is .7/rate, so .10 rate
            //    ==> will get to half the difference in 7 months (periods)
            health.health += HEALTH_CHANGE_RATE * (target_health - health.health);
        }
    }
}

impl<'a> System<'a> for FoodConsumptionSystem {
    type SystemData = (WriteStorage<'a, FoodStock>,
                       WriteStorage<'a, FoodConsumption>,
                       ReadStorage<'a, RegionPop>,
                       ReadExpect<'a, Date>);

    fn run(&mut self, (mut stock, mut consump, pop, date): Self::SystemData) {
        if date.day() != 3 {
            return;
        }

        let months_2_harvest = ((HARVEST_MONTH - date.month()) % 12) as f32;

        for (mut stock, mut consump, pop) in (&mut stock, &mut consump, &pop).join() {
            let base_grain = grain_for_pop(pop);
            let abundance = stock.bushels / (base_grain * months_2_harvest / 12.);
            let r = if abundance > 1.4 {
                1.2
            } else if abundance > 0.8 {
                abundance.sqrt()
            } else if abundance > 0.6 {
                abundance
            } else {
                0.6
            };

            consump.consump = base_grain * r / 12.;
            consump.ratio_of_norm = r;

            stock.bushels = (stock.bushels - consump.consump).max(0.);
        }
    }
}

pub fn logistic(r: f32, x: f32, k: f32) -> f32 {
    r * x * (k - x) / k
}


