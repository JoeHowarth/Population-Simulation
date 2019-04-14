use super::*;
use specs::prelude::*;
use specs_derive;
use crate::misc::time::Date;
use chrono::Datelike;
use crate::prelude::*;


pub struct PopUpdate {
    pub old_date: Date,
}

// Changes Cohort health in response to food scarcity, disease, sanitation etc.
pub struct HealthUpdate;

// The
pub struct FoodConsumptionUpdate;

const DEATH_RATES: [f32; 17] = [
    0.2, 0.045, 0.035, 0.02, 0.007, 0.005, 0.005, 0.005,
    0.01, 0.015, 0.02, 0.03, 0.04, 0.06, 0.1, 0.3, 0.8
];
const BIRTH_RATES: [f32; 17] = [0., 0., 0., 0.01, 0.02, 0.55, 0.45, 0.25, 0.1, 0., 0., 0., 0., 0., 0., 0., 0.];

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

//            if dt < 30 { return; }


            let mut b = true;
            for (p, h) in (&mut pop, &health).join() {
                p.update(&BIRTH_RATES, &DEATH_RATES, dt as u16);

                if b {
                    //dbg!(&p);
                    b = false
                }
            }
        }
    }
}

impl<'a> System<'a> for HealthUpdate {
    type SystemData = (WriteStorage<'a, Health>,
                       WriteStorage<'a, FoodConsumption>,
                       ReadStorage<'a, FoodStock>);

    fn run(&mut self, (health, consump, food): Self::SystemData) {}
}

impl<'a> System<'a> for FoodConsumptionUpdate {
    type SystemData = (WriteStorage<'a, FoodStock>,
                       WriteStorage<'a, FoodConsumption>,
                       ReadStorage<'a, RegionPop>,
                       ReadStorage<'a, Health>,
                       ReadExpect<'a, Date>);

    fn run(&mut self, (mut stock, mut consump, pop, health, date): Self::SystemData) {
        if date.day() != 3 {
            return;
        }

        let months_2_harvest = ((9 - date.month()) % 12) as f32;

        for (stock, mut consump, pop, health) in (&mut stock, &mut consump, &pop, &health).join() {
            let base_grain = grain_for_pop(pop);
            let abundance = stock.bushels / (base_grain * months_2_harvest / 12.);
            let r = if abundance > 1.3 {
                1.2
            } else if abundance > 0.8 {
                abundance.sqrt()
            } else if abundance > 0.6 {
                abundance
            } else {
                0.65
            };

            consump.consump = base_grain * r / 12.;
            consump.ratio_of_norm = r;
        }
    }
}


