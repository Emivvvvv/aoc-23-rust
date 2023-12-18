mod file;
mod clap;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
// mod day11;
// mod day12;
// mod day13;
// mod day14;
// mod day15;
// mod day16;
// mod day17;
// mod day18;
// mod day19;
// mod day20;
// mod day21;
// mod day22;
// mod day23;
// mod day24;
// mod day25;

const COMPLETED_DAYS: i32 = 10;


fn main() {
    let cli = clap::cli();
    let matches = cli.get_matches();

    if matches.get_flag("run-all") {
        for day in 1..=COMPLETED_DAYS {
            print_answers_of_day(day)
        }
    } else {
        print_answers_of_day(matches.get_one::<String>("day-number").unwrap().parse::<i32>().unwrap())
    }
}

fn print_answers_of_day(day: i32) {
    println!("========================================");
    println!("Day {day} answers:");

    let answers = match day {
        1 => day1::answers(file::input("1")),
        2 => day2::answers(file::input("2")),
        3 => day3::answers(file::input("3")),
        4 => day4::answers(file::input("4")),
        5 => day5::answers(file::input("5")),
        6 => day6::answers(file::input("6")),
        7 => day7::answers(file::input("7")),
        8 => day8::answers(file::input("8")),
        9 => day9::answers(file::input("9")),
        10 => day10::answers(file::input("10")),
        // 11 => day11::answers(file::input("11")),
        // 12 => day12::answers(file::input("12")),
        // 13 => day13::answers(file::input("13")),
        // 14 => day14::answers(file::input("14")),
        // 15 => day15::answers(file::input("15")),
        // 16 => day16::answers(file::input("16")),
        // 17 => day17::answers(file::input("17")),
        // 18 => day18::answers(file::input("18")),
        // 19 => day19::answers(file::input("19")),
        // 20 => day20::answers(file::input("20")),
        // 21 => day21::answers(file::input("21")),
        // 22 => day22::answers(file::input("22")),
        // 23 => day23::answers(file::input("23")),
        // 24 => day24::answers(file::input("24")),
        // 25 => day25::answers(file::input("25")),
        _ => panic!("The solution is not available!")
    };

    for i in 0..answers.len() {
        println!("Part {}: {}", i + 1, answers[i]);
    }
    println!("========================================");
}
