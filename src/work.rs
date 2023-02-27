use chrono::Local;

use std::io;

#[derive(Default)]
pub struct Workday {
    pub day: String,
    pub date: String,
    pub work_one: Work,
    pub break_one: Break,
    pub work_two: Work,
    pub break_two: Break,
    pub work_three: Work,
}

// FIXME
// #[derive(Default)]
pub enum Work {
    // #[default]
    Start(String),
    End(String),
}

// #[derive(Default)]
pub enum Break {
    // #[default]
    Start(String),
    End(String),
}

// FIXME
// impl Default for Work {
//     fn default() -> Self {
//         let df = "".to_string();
//         Self { Start: df }
//     }
// }

// impl Default for Break {
//     fn default() -> Self {
//         let df = "".to_string();
//         Self { Start: df }
//     }
// }

// impl<'a> Default for &'a Work {
//     fn default() -> &'a Work {
//         static VALUE: Work = Work {
//             start: "".to_string(),
//             end: "".to_string(),
//         };
//         &VALUE
//     }
// }

impl Workday {
    pub fn new() -> io::Result<Workday> {
        let day = get_day()?;
        let date = get_date();
        let start_time = get_time();
        let work_one = Work::Start(start_time);
        let break_one = Break::Start(start_time);
        let work_two = Work::Start(start_time);
        let break_two = Break::Start(start_time);
        let work_three = Work::Start(start_time);

        Ok(Workday {
            day,
            date,
            work_one,
            break_one,
            work_two,
            break_two,
            work_three,
        })
    }
}

pub fn get_day() -> io::Result<String> {
    match Local::now().format("%A").to_string().as_str() {
        "Monday" => Ok("Montag".to_string()),
        "Tuesday" => Ok("Dienstag".to_string()),
        "Wedneyday" => Ok("Mittwoch".to_string()),
        "Thrusday" => Ok("Donnerstag".to_string()),
        "Friday" => Ok("Freitag".to_string()),
        "Saturday" => Ok("Samstag".to_string()),
        "Sunday" => Ok("Sonntag".to_string()),
        _ => return Err(io::Error::from(io::ErrorKind::InvalidData)),
    }
}

pub fn get_date() -> String {
    Local::now().format("%d.%m.%Y").to_string()
}

pub fn get_time() -> String {
    Local::now().format("%R").to_string()
}
