// Note that this is *not* a suckless-style config file
// It's just a module dealing with settings logic

use clap::{Arg, App, crate_version};

pub struct Setting {
    pub name: String,
    pub help: String,
    pub short: char,
    pub value_name: String, // displayed in CLI help
}

// types appropriate for where they're used
pub struct LoadedSettings {
    pub win_size: (i32, i32),
    pub snake_wait: u64,
    pub min_fruits: usize,
    pub snake_len: u32,
}

pub fn create() -> Vec<Setting> {
    vec![
        Setting {
            name: "window-size".to_string(),
            help: "Size of the window where X and Y are positive integers.".to_string(),
            short: 'w',
            value_name: "XxY".to_string(),
        },
        Setting {
            name: "speed".to_string(),
            help: "Speed of the snake. Can be a number between <1,4> or a number of milliseconds between game updates (appended with \"ms\").".to_string(),
            short: 's',
            value_name: "s".to_string(),
        },
        Setting {
            name: "fruits".to_string(),
            help: "Number of fruits to exist at one time. Must be a positive integer.".to_string(),
            short: 'f',
            value_name: "fruits".to_string(),
        },
        Setting {
            name: "length".to_string(),
            help: "Initial length of the snake. Must be a positive integer.".to_string(),
            short: 'l',
            value_name: "length".to_string(),
        }
    ]
}

pub fn read_cli_args(settings: &Vec<Setting>) -> LoadedSettings {
    let mut app = App::new("Rusty Snake")
        .version(crate_version!())
        .author("Igor Bugajski <igorbugajski@protonmail.com>")
        .about("A snake game. It can be configured through CLI.");

    for setting in settings.iter() {
        app = app.arg(
            Arg::with_name(&setting.name)
                .help(&setting.help)
                .long(&setting.name)
                .short(&setting.short.to_string())
                .takes_value(true)
                .value_name(&setting.value_name)
        );
    }

    let matches = app.get_matches();

    let win_size_string = matches.value_of("window-size").unwrap_or("40x20");
    let win_size = read_window_size(win_size_string);

    let speed_string = matches.value_of("speed").unwrap_or("2");
    let speed = read_speed(speed_string);

    let fruits_string = matches.value_of("fruits").unwrap_or("1");
    let fruits = read_fruits(fruits_string);

    let length_string = matches.value_of("length").unwrap_or("3");
    let length = read_length(length_string);

    LoadedSettings {
        win_size: win_size,
        snake_wait: speed,
        min_fruits: fruits,
        snake_len: length,
    }
}

fn wrong_arg(error_msg: String) {
    eprintln!("{}", error_msg);
    std::process::exit(1);
}

// read *and* exit if invalid
// not using clap::Arg::validator() because it requires a static lifetime
// and with my Setting struct approach (to reuse stuff in settings menu) it won't work
fn read_window_size(input: &str) -> (i32, i32) {
    let error_pref = "Wrong window size:";

    let nums: Vec<&str> = input.split('x').collect();

    if nums[0] == input { // there was no 'x'
        wrong_arg(format!("{} no 'x' present", error_pref));
    }

    let mut size = [0, 0]; // array so it can be indexed

    for (index, num) in nums.iter().enumerate() {
        let value_name: &str;
        
        if index == 0 {
            value_name = "x";
        } else {
            value_name = "y";
        }

        match num.parse::<i32>() {
            Ok(v) => size[index] = v,
            Err(_) => wrong_arg(format!("{} value {} is not a number or too high", error_pref, value_name)),
        }

        if size[index] <= 0 {
            wrong_arg(format!("{} values must be greater than 0", error_pref));
        }
    }

    (size[0], size[1])
}

fn read_speed(input: &str) -> u64 {
    let error_pref = "Wrong snake speed:";

    let num = input.trim_end_matches("ms");
    if num == input { // no "ms"
        match num {
            "1" => return 150,
            "2" => return 100,
            "3" => return 50,
            "4" => return 30,
            _ => wrong_arg(format!("{} unknown predefined speed. Try 1, 2, 3 or 4.", error_pref)),
        }
    } else {
        match num.parse::<u64>() {
             Ok(v) => return v,
             Err(_) => wrong_arg(format!("{} not a number, too high or negative", error_pref)),
        }
    }
    0
}

fn read_fruits(input: &str) -> usize {
    match input.parse::<usize>() {
        Ok(v) => return v,
        Err(_) => wrong_arg("Wrong fruit number: not a number, negative or too high"),
    }
    0
}

fn read_length(input: &str) -> u32 {
    match input.parse::<u32>() {
        Ok(v) => return v,
        Err(_) => wrong_arg(format!("Wrong snake length: not a number, negative or too high"),
    }
    0
}
