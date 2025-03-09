/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#[macro_use]
extern crate clap;
extern crate scan;

use std::process::exit;
use std::path::Path;
use clap::{Arg, Command};

fn main() {
    let matches = Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg_required_else_help(true)
        .arg(
            Arg::new("ip")
                .help("IP of scanner")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::new("destination file")
                .help("Destination file")
                .index(2)
                .required(true),
        )
        .arg(
            Arg::new("dpi")
                .short('d')
                .long("dpi")
                .help("Scan resolution")
                .default_value("75")
                .value_parser(clap::value_parser!(i16)),
        )
        .arg(
            Arg::new("force")
                .short('f')
                .long("force")
                .help("Force scan and override destination file"),
        )
        .get_matches();

    let ip = matches.get_one::<String>("ip").unwrap();
    let scanner_base_path = format!("http://{}:80/eSCL", ip);
    let scan_resolution = matches.get_one::<i16>("dpi").unwrap();
    let destination_file = matches.get_one::<String>("destination file").unwrap();
    let dest_path = Path::new(destination_file);

    let dest_ext = dest_path.extension().and_then(std::ffi::OsStr::to_str).unwrap().to_lowercase();
    let allowed_exts = ["pdf", "jpg", "jpeg"];

    if !allowed_exts.contains(&dest_ext.as_str()) {
        eprintln!("Allowed output extensions are: {:?}", allowed_exts);
        exit(1);
    }

    if !matches.contains_id("force") && dest_path.exists() {
        eprintln!("Output file exists! Exiting...");
        exit(1);
    }


    scan::scan(&scanner_base_path, *scan_resolution, dest_path);

    println!("Done!");
}
