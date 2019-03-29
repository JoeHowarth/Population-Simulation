use super::*;
use specs::prelude::*;
use specs_derive;
use crate::misc::time::Date;
use chrono::Datelike;


pub struct PopUpdate {
    pub old_date: Date,
}

const DEATH_RATES: [f32; 17] = [
    0.2, 0.042, 0.035, 0.02, 0.007, 0.005, 0.005, 0.005,
    0.01, 0.015, 0.02, 0.03, 0.04, 0.06, 0.1, 0.3, 0.8
];
const BIRTH_RATES: [f32; 17] = [0., 0., 0., 0.01, 0.02, 0.55, 0.45, 0.25, 0.1, 0., 0., 0., 0., 0., 0., 0., 0.];

impl<'a> System<'a> for PopUpdate {
    type SystemData = (WriteStorage<'a, RegionPop>,
                       ReadExpect<'a, Date>);

    fn run(&mut self, (mut pop, date): Self::SystemData) {
        let day = date.day();
        let month = date.month();
        if day == 15 && (month == 1 || month == 3 || month == 6 || month == 9) {
            let dt = (*date - self.old_date).num_days();
            self.old_date = date.clone();

//            if dt < 30 { return; }


            let mut b = true;
            for p in (&mut pop).join() {
                p.update(&BIRTH_RATES, &DEATH_RATES, dt as u16);

                if b {
                    dbg!(&p);
                    b = false
                }
            }

        }


    }
}


/*
impl Default for PopUpdate {
    fn default() -> PopUpdate {
        PopUpdate {
            old_date: None
        }
    }
}*/
