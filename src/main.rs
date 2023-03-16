#![allow(dead_code)]

use itertools::Itertools;

use file::read_lines;
use log::ApacheLogEntry;

mod file;
mod log;

fn main() {
    let log1 = read_lines("./apache_1.log").unwrap();
    let log2 = read_lines("./apache_2.log").unwrap();

    let log1 = log1.filter_map(|l| {
        TryInto::<ApacheLogEntry>::try_into(l.ok()?.as_ref()).ok()
    });

    let log2 = log2.filter_map(|l| {
        TryInto::<ApacheLogEntry>::try_into(l.ok()?.as_ref()).ok()
    });

    let log_final = log1.merge(log2).unique().sorted().collect_vec();

    for l in log_final {
        println!("{l:#?}");
    }
}
