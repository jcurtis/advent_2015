mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

aoc_main::main! {
    year 2015;
    day1 => part_1, part_2;
    day2: generate => part_1, part_2;
    day3: generate => part_1, part_2;
    day4 => part_1, part_2;
    day5 => part_1;
}
