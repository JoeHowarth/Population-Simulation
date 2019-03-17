use specs_derive;
use specs::prelude::*;
use chrono::prelude::{NaiveDate};

pub type Date = NaiveDate;


pub fn init_date(world: &mut World)  {
    world.add_resource(Date::from_num_days_from_ce(0));
}

#[derive(Default)]
pub struct UpdateDate;

impl<'a> System<'a> for UpdateDate {
    type SystemData = (WriteExpect<'a, Date>);

    fn run(&mut self, mut date: Self::SystemData) {
        *date = date.succ();

        info!("Date: {}", date.format("%b %e %-Y"));
    }

}

