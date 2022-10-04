use std::cmp::{max, min};
use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;

// Intel version
const BASE_PATH: &str = "/sys/class/backlight/intel_backlight/";
// AMD version
//const BASE_PATH: &str = "/sys/class/backlight/amdgpu_bl0/";
const HELP_STR: &str = "communism: tool to control intel backlight\n--help to display this text\n--up <x> to increase brightness by x percent\n--down <x> to decrease brightness by x percent\n--get show the current brightness in percentage\n";

enum Action {
    Up(f32),
    Down(f32),
    Get,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let max_br_path = BASE_PATH.to_owned() + "max_brightness";
    let cur_br_path = BASE_PATH.to_owned() + "actual_brightness";
    let write_br_path = BASE_PATH.to_owned() + "brightness";

    let max_br_path = PathBuf::from(&(max_br_path));
    let cur_br_path = Path::new(&(cur_br_path));
    let write_br_path = Path::new(&(write_br_path));

    let num: f32;

    let action: Action;
    match args.len() {
        2 => {
            action = match args[1].as_ref() {
                "--get" => Action::Get,
                _ => {
                    print!("{}", HELP_STR.to_owned());
                    exit(0);
                }
            }
        }
        3 => {
            num = match args[2].parse() {
                Ok(n) => n,
                Err(e) => panic!("{}", e),
            };
            action = match args[1].as_ref() {
                "--up" => Action::Up(num),
                "--down" => Action::Down(num),
                _ => {
                    print!("{}", HELP_STR.to_owned());
                    exit(0);
                }
            };
        }

        _ => {
            print!("{}", HELP_STR.to_owned());
            exit(0);
        }
    }

    let mut max_br_file = match File::open(&max_br_path) {
        Err(why) => panic!("couldn't open max_brightness: {}", why),
        Ok(file) => file,
    };
    let mut max_br_str = String::new();
    match max_br_file.read_to_string(&mut max_br_str) {
        Err(why) => panic!("couldn't read max_brightness: {}", why),
        Ok(_) => (),
    };

    let max_br: f32 = max_br_str.trim().parse().unwrap();

    let mut cur_br_file = match File::open(&cur_br_path) {
        Err(why) => panic!("couldn't open actual_brightness: {}", why),
        Ok(file) => file,
    };
    let mut cur_br_str = String::new();
    match cur_br_file.read_to_string(&mut cur_br_str) {
        Err(why) => panic!("couldn't read actual_brightness: {}", why),
        Ok(_) => (),
    };
    let cur_br: i32 = cur_br_str.trim().parse().unwrap();

    let target_br: i32;

    match action {
        Action::Up(num) => {
            target_br = min(max_br as i32, cur_br + ((max_br * (num / 100.0)) as i32));
            write_to_file(target_br, write_br_path);
        },
        Action::Down(num) => {
            target_br = max(0, cur_br - ((max_br * (num / 100.0)) as i32));
            write_to_file(target_br, write_br_path);
        },
        Action::Get => {
            let percent = (cur_br * 100) / 255;
            println!("{}%", percent);
        }
    }
}

fn write_to_file(target_br: i32, write_br_path: &Path) {
    let mut write_br_file = match File::create(&write_br_path) {
        Err(why) => panic!("couldn't create brightness: {}", why),
        Ok(file) => file,
    };

    match write_br_file.write_all(&target_br.to_string().into_bytes()) {
        Err(why) => panic!("couldn't write to brightness: {}", why),
        Ok(_) => (),
    }
}
