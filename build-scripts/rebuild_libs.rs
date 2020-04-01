/*
* This is a script to clobber the {desktop, ios, android} directories
* and rebuild libs via the build-all.sh script.
*/
use std::fs;
use std::cmp::max;
use std::path::Path;
use std::env;
use std::process::Command;
use std::io::prelude::*;

#[allow(unused_must_use)]
fn parse_version() -> Result<bool, String> {

    let mut local_file = fs::File::open(".local_libs_version").expect("Error: does not exist\n");
    let mut local_version = String::new();
    local_file.read_to_string(&mut local_version);

    let mut version_log = fs::File::open("version_log").expect("Error: version_log is missing. :(\n");
    let mut newest_version = String::new();
    version_log.read_to_string(&mut newest_version);

    //this reads the most recent version number. (everything before the '.')
    let mut updated_v = -1;
    for line in newest_version.lines() {
        let v: Vec<&str> = line.split('.').collect();
        if v.len() != 2 {
            continue;
        }
        if let Ok(v) = v[0].parse::<i32>() {
            updated_v = max(v, updated_v);
        }
    }
    let mut curr_v = -1;
    for line in local_version.lines() {
        let v: Vec<&str> = line.split('.').collect();
        if v.len() != 2 {
            continue;
        }
        if let Ok(v) = v[0].parse::<i32>() {
            curr_v = max(v, curr_v);
        }
    }
    //assert_eq!(updated_v, curr_v);
    if updated_v != curr_v {
        fs::write(".local_libs_version", newest_version);
        //return Ok(true);
    }
    Ok(true)
}
#[allow(unused_must_use)]
fn run_dependency_check() -> Result<String, String> {
    let mut clobber_needed: bool = false;
//    let mut dirs: Vec<fs::File> = Vec::new();
    let mut dirs: Vec<&str> = Vec::new();
    let root = Path::new("../libs");

    assert!(env::set_current_dir(&root).is_ok());

    clobber_needed = parse_version().unwrap();

    if clobber_needed {
        match clobber(dirs) {
            Ok(_o) => {Ok("".to_string())}
            Err(_e) => { Err(String::from("Generic error message"))}
        };
    }
    Ok(String::from("Success"))
}

//fn clobber(directories: Vec<fs::File>) -> std::io::Result<()> {
fn clobber(directories: Vec<&str>) -> std::io::Result<()> {

    println!("deleting old directories and rebuilding /libs...\n");

    for dir in directories {
        fs::remove_dir_all(dir)?;
    }
    // Now execute the build-all script in a shell.
    let mut cmd = Command::new("bash");
    cmd.arg("./build-all.sh");
    match cmd.output() {
        Ok(T) => {}
        Err(e) => return Err(e),
    }
    Ok(())

}

fn main() {

    run_dependency_check().unwrap();
   /* match run_dependency_check() {
        Ok(_T) => Ok("Directories Clobbered Successfully."),
        Err(_e) => "Counld not rebuild libs. Delete your folders for desktop, ios, and android, then manually run libs/build-all.sh \n"),
    };*/
    }