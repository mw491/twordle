use chrono::{Duration, Local, NaiveDate};
use rand::{thread_rng, Rng};

fn get_index() -> usize {
    // let mut gamestart_date = Local.with_ymd_and_hms(2023, 1, 1, 0, 0, 0);
    let mut gamestart_date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap_or_default();
    let current_date = Local::now().date_naive();
    let mut index: usize = 0;

    while gamestart_date < current_date {
        index = index + 1;
        gamestart_date = gamestart_date + Duration::days(1);
    }

    index
}

pub fn gen() -> String {
    let word_list: Vec<&str> = include_str!("twordle_list.txt").lines().collect();
    let index = get_index();
    word_list.get(index % word_list.len()).unwrap().to_string()
}

pub fn unlimited() -> String {
    let word_list: Vec<&str> = include_str!("twordle_list.txt").lines().collect();
    word_list[thread_rng().gen_range(0..word_list.len())].to_string()
}
