mod day1;
mod day2;

aoc_main::main! {
    year 2015;
    day1 => part_1, part_2;
    day2: generate => part_1, part_2;
}
