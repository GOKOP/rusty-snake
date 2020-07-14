// Note that this is *not* a suckless-style config file
// It's just a module dealing with settings logic
// default values are however defined here

use clap::{crate_version, App, Arg};

// milliseconds of delay for predefined speeds
const SPEED1: u64 = 150;
const SPEED2: u64 = 100;
const SPEED3: u64 = 50;
const SPEED4: u64 = 30;

// default config in strings because it's used while unwrapping user cli input
const DEF_WINDOW: &str = "40x20";
const DEF_SPEED: &str = "2";
const DEF_FRUITS: &str = "1";
const DEF_LENGTH: &str = "3";

// holds info about settings; made to allow reuse of the information in GUI settings (todo)
pub struct Setting {
    pub name: String,
    pub help: String,
    pub short: char,        // for CLI
    pub value_name: String, // displayed in CLI help; set to "" if not supposed to take value
}

// types appropriate for where they're used
pub struct LoadedSettings {
    pub win_size: (i32, i32),
    // milliseconds of delay between game updates (aka speed but lower numbers are faster)
    pub snake_wait: u64,
    pub min_fruits: usize,
    pub snake_len: u32,
    pub use_color: bool,
    pub benchmark: bool,
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
        },
        Setting {
            name: "no-color".to_string(),
            help: "Don't use colors.".to_string(),
            short: 'c',
            value_name: "".to_string(),
        },
        Setting {
            name: "benchmark".to_string(),
            help: "Display measured vs expected update time while playing".to_string(),
            short: 'b',
            value_name: "".to_string(),
        }
    ]
}

pub fn read_cli_args(settings: &Vec<Setting>) -> LoadedSettings {
    // construct clap app with all the defined settings
    let mut app = App::new("Rusty Snake")
        .version(crate_version!())
        .author("Igor Bugajski <igorbugajski@protonmail.com>")
        .about("A snake game. It can be configured through CLI.");

    for setting in settings.iter() {
        let mut takes_value = true;

        if setting.value_name == "" {
            takes_value = false;
        }

        app = app.arg(
            Arg::with_name(&setting.name)
                .help(&setting.help)
                .long(&setting.name)
                .short(&setting.short.to_string())
                .value_name(&setting.value_name)
                .takes_value(takes_value),
        );
    }

    let matches = app.get_matches();

    let win_size_string = matches.value_of("window-size").unwrap_or(DEF_WINDOW);
    let speed_string = matches.value_of("speed").unwrap_or(DEF_SPEED);
    let fruits_string = matches.value_of("fruits").unwrap_or(DEF_FRUITS);
    let length_string = matches.value_of("length").unwrap_or(DEF_LENGTH);

    LoadedSettings {
        win_size: read_window_size(win_size_string),
        snake_wait: read_speed(speed_string),
        min_fruits: read_fruits(fruits_string),
        snake_len: read_length(length_string),
        use_color: !matches.is_present("no-color"),
        benchmark: matches.is_present("benchmark"),
    }
}

/*
 all the input interpreting functions handle data validation themselves
 this is because clap::Arg::validator() requires a function pointer
 and it wants it with 'static lifetime
 that doesn't play very well with my settings-data-in-struct approach
 or maybe it does but I'm just a n00b, dunno
 if someone who knows is reading this then please let me know
*/

// communicate error and quit; cleaner than panic!
fn wrong_arg(error_msg: String) {
    eprintln!("{}", error_msg);
    std::process::exit(1);
}

fn read_window_size(input: &str) -> (i32, i32) {
    let error_pref = "Wrong window size:";

    let nums: Vec<&str> = input.split('x').collect();

    if nums[0] == input {
        // there was no 'x'
        wrong_arg(format!("{} no 'x' present", error_pref));
    }

    let mut size = [0, 0]; // array so it can be indexed

    for (index, num) in nums.iter().enumerate() {
        let value_name: &str;

        // this is for the error string
        if index == 0 {
            value_name = "x";
        } else {
            value_name = "y";
        }

        match num.parse::<i32>() {
            Ok(v) => size[index] = v,
            Err(_) => wrong_arg(format!(
                "{} value {} is not a number or too high",
                error_pref, value_name
            )),
        }

        if size[index] <= 0 {
            wrong_arg(format!("{} values must be greater than 0", error_pref));
        }
    }

    (size[0], size[1])
}

fn read_speed(input: &str) -> u64 {
    let error_pref = "Wrong snake speed:";

    // remove possible trailing "ms" to see if it was there
    let num = input.trim_end_matches("ms");

    // it wasn't, so the value is matched against predefined speeds
    if num == input {
        match num {
            "1" => return SPEED1,
            "2" => return SPEED2,
            "3" => return SPEED3,
            "4" => return SPEED4,
            _ => wrong_arg(format!(
                "{} unknown predefined speed. Try 1, 2, 3 or 4.",
                error_pref
            )),
        }
    // it was, so the preceeding number is used directly
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
        Err(_) => wrong_arg("Wrong fruit number: not a number, negative or too high".to_string()),
    }
    0
}

fn read_length(input: &str) -> u32 {
    match input.parse::<u32>() {
        Ok(v) => return v,
        Err(_) => wrong_arg("Wrong snake length: not a number, negative or too high".to_string()),
    }
    0
}
