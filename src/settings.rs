// Note that this is *not* a suckless-style config file
// It's just a module dealing with settings logic

use clap::{Arg, App, crate_version};

pub struct Setting {
    pub name: String,
    pub help: String,
    pub short: char,
    pub value_name: String, // displayed in CLI help
}

pub struct LoadedSettings {
    pub win_size: (i32, i32),
}

pub fn create() -> Vec<Setting> {
    vec![
        Setting {
            name: "window-size".to_string(),
            help: "size of the window where X and Y are positive integers".to_string(),
            short: 'w',
            value_name: "XxY".to_string(),
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

    LoadedSettings {
        win_size: win_size,
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
