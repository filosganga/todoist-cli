extern crate clap;
extern crate xdg;

use clap::{Arg, App, SubCommand};

fn main() {

	let xdg_dirs = xdg::BaseDirectories::with_prefix("todoist-cli").unwrap();
	let config_path = xdg_dirs.place_config_file("config.ini").expect("cannot create configuration directory");


	let matches = App::new("todoist-cli")
		.version("0.1.0")
		.author("Filippo De Luca <me@filippodelucacom.com>")
		.about("todoist command line application")
		.subcommand(SubCommand::with_name("auth")
            .arg(Arg::with_name("debug")
            .short("d")
            .help("print debug information verbosely")))
		.get_matches();

	if let Some(matches) = matches.subcommand_matches("auth") {
        if matches.is_present("debug") {
            println!("Printing debug info...");
        } else {
            println!("Printing normally...");
        }
    }

}