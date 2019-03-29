use specs_derive;
use specs::prelude::*;
use chrono::prelude::NaiveDate;
use crate::networking::*;
use chrono::Datelike;

pub type Date = NaiveDate;

const DATE_FORMAT: &str = "%b %e %-Y";

pub fn init_date(world: &mut World) {
    world.add_resource(Date::from_num_days_from_ce(0));
}

#[derive(Default)]
pub struct UpdateDate;

impl<'a> System<'a> for UpdateDate {
    type SystemData = (WriteExpect<'a, Date>);

    fn run(&mut self, mut date: Self::SystemData) {
        *date = date.succ();

        if date.day() == 15 {
            info!("Date: {}", date.format(DATE_FORMAT));
        }
    }
}

pub struct DateSender {
    pub out: ClientSender
}

impl<'a> System<'a> for DateSender {
    type SystemData = ReadExpect<'a, Date>;

    fn run(&mut self, date: Self::SystemData) {
        let data = JsonDate {
            str: &format!("{}",date.format(DATE_FORMAT)),
            m: date.month(),
            d: date.day(),
            y: date.year(),
        };

        self.out.sub_push(SubPush {
            section: Sections::Date,
            component: "Date",
            data,
            keys: None,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct JsonDate<'a> {
    m: u32,
    d: u32,
    y: i32,
    str: &'a str,
}

