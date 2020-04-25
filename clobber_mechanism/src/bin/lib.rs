/*
* This is a script to clobber the {desktop, ios, android} directories
* and rebuild libs via the build-all.sh script.
*/
use std::fs;
use std::cmp::max;
use std::path::{Path, PathBuf};
use std::env;
use std::process::Command;
use std::io::prelude::*;
use structopt::StructOpt;

#[allow(non_snake_case)]
pub enum Status {
    OK,
    FAIL,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "clobber", about = "eeeeeeeeeeeeeeeeeeeeeee")]
struct Opt {
    #[structopt(short = "s",
    long  = "script",
    help  = "script to rebuild the library",
    default_value = "build-all.sh")]
    script: String,

    #[structopt(short = "d",
    long  = "directories",
    default_value = "desktop, ios, android",
    use_delimiter=true,
    parse(from_os_str))]
    inputs: Vec<PathBuf>
}


// function reads the most recent version number. (everything before the '.')
fn parse(file: String) -> i32 {

    let mut most_recent_version = -1;
    for line in file.lines() {
        let v: Vec<&str> = line.split('.').collect();
        if v.len() != 2 {
            continue;
        }
        if let Ok(v) = v[0].parse::<i32>() {
            most_recent_version = max(v, most_recent_version);
        }
    }
    most_recent_version
}
#[allow(unused_must_use)]
fn parse_version(newest_version: String, local_version: String) -> Result<bool, String> {

    if parse(newest_version.clone()) != parse(local_version) {
        return Ok(true)
    }
    Ok(false)
}

#[allow(unused_must_use)]
fn run_dependency_check(vl: String, loc_version: String) -> Result<bool, String> {

    let root = Path::new("../../libs");
    assert!(env::set_current_dir(&root).is_ok());
    //  If there is no local version log, clobber is needed.
    if !(Path::new(&loc_version).exists()) {
        fs::File::create(".local_libs_version");
        return Ok(true);
    }

    let mut version_log = fs::File::open(vl).expect("Error: version_log is missing. :(\n");
    let mut newest_version = String::new();
    version_log.read_to_string(&mut newest_version);

    let mut local_file = fs::File::open(loc_version).expect("damn.");
    let mut local_version = String::new();
    local_file.read_to_string(&mut local_version);

    return parse_version(newest_version, local_version);
    //fs::write("../libs/.local_libs_version",  )
}


fn clobber(directories: &mut Vec<String>) -> Status {

    println!("deleting old directories and rebuilding /libs...\n");

    for x in directories {
        println!("{}", x);
        let curr_dir = Path::new(&x);
        if curr_dir.exists() {
            match fs::remove_dir_all(curr_dir) {
                Ok(_t) => {},
                Err(_e) => return Status::FAIL,
            }
        }
    }
    Status::OK
}


fn rebuild(build_script: String) -> Status {

    // Now execute the build-all script in a shell.
    let mut cmd = Command::new("bash");
    cmd.arg(build_script);
    match cmd.output() {
        Ok(_t) => {},
        Err(_e) => return Status::FAIL,
    };
    Status::OK
}

fn run_process(script: String, mut dirs_to_clobber: Vec<String>) {

    let version_log = String::from("version_log");
    let local_version = String::from(".local_libs_version");

    let clobber_needed: bool;
    clobber_needed = run_dependency_check(version_log.clone(), local_version.clone()).unwrap();

    println!("clobbering: {:?}", dirs_to_clobber);
    if clobber_needed {
        match clobber(&mut dirs_to_clobber) {
            Status::OK => println!("Directories successfully clobbered!\n"),
            Status::FAIL => println!("Counld not rebuild libs. Delete your folders for desktop, ios, and android, then manually run libs/build-all.sh \n"),
        }
        rebuild(script);
    }
    //update the local version log.
    match fs::copy(&version_log, &local_version) {
        Ok(_) => {},
        Err(_) => {},
    }
}

#[allow(dead_code)]
fn main() {
    let opt = Opt::from_args();
    println!("{:#?}", opt);
    let script = String::from("build-all.sh");
    let mut dirs_to_clobber: Vec<String> = Vec::new(); // = "".to_string(); // = vec![String::from("desktop"), String::from("ios"), String::from("android")];
    for x in opt.inputs {
        dirs_to_clobber.push(opt.inputs[x].as_path().display().to_string());
        //dirs_to_clobber.push(String::from(opt.inputs[x]));
    }
    run_process(opt.script, dirs_to_clobber);

}

#[cfg(test)]
mod tests;
