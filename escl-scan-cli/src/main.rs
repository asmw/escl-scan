#[macro_use]
extern crate clap;
extern crate scan;

use clap::{App, AppSettings, Arg};
use std::process::exit;
use std::path::Path;

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::with_name("ip")
                .help("IP of scanner")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("output file")
                .help("Destination file")
                .index(2)
                .required(true),
        )
        .arg(
            Arg::with_name("dpi")
                .short("d")
                .long("dpi")
                .help("Scan resolution")
                .default_value("75"),
        )
        .arg(
            Arg::with_name("force")
                .short("f")
                .long("force")
                .help("Force scan and override output file"),
        )
        .get_matches();

    let ip = matches.value_of("ip").unwrap();
    let scanner_base_path = format!("http://{}:80/eSCL", ip);
    let scan_resolution: i16 = matches.value_of("dpi").unwrap().parse().unwrap();
    let destination_file = matches.value_of("output file").unwrap();

    if !matches.is_present("force") && Path::new(destination_file).exists() {
        eprintln!("Output file exists! Exiting...");
        exit(1);
    }

    scan::scan(&scanner_base_path, scan_resolution, destination_file);

    println!("Done!");
}
