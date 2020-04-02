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
fn parse_version(newest_version: String, local_version: String) -> Result<bool, String> {

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
    if updated_v != curr_v {
        fs::File::create(".local_libs_version");
        fs::write(".local_libs_version", newest_version);
        return Ok(true);
    }
    Ok(false)
}
#[allow(unused_must_use)]
fn run_dependency_check() -> Result<String, String> {

    let root = Path::new("../libs");
    assert!(env::set_current_dir(&root).is_ok());

    let mut clobber_needed: bool = false;
    let mut version_log = fs::File::open("version_log").expect("Error: version_log is missing. :(\n");
    let mut newest_version = String::new();
    version_log.read_to_string(&mut newest_version);
    let nv_copy = newest_version.clone();
    if !(Path::new(".local_libs_version").exists()) {
        clobber_needed = true;
    }

    if !clobber_needed {
    let mut local_file = fs::File::open(".local_libs_version").expect("damn.");
        let mut local_version = String::new();
    local_file.read_to_string(&mut local_version);

    clobber_needed = parse_version(newest_version, local_version).unwrap();
    }

    if clobber_needed {
        match clobber() {
            Ok(_o) => {Ok(String::from("Directories successfully clobbered!\n"))}
            Err(_e) => { Err(String::from("Counld not rebuild libs. Delete your folders for desktop, ios, and android, then manually run libs/build-all.sh \n"))}
        };
    }
    fs::File::create(".local_libs_version");
    fs::write(".local_libs_version", nv_copy);
    Ok(String::from("Success"))
}

//fn clobber(directories: Vec<fs::File>) -> std::io::Result<()> {
fn clobber() -> std::io::Result<()> {

    println!("deleting old directories and rebuilding /libs...\n");

    if Path::new("../libs/desktop").exists() {
        fs::remove_dir_all("../libs/desktop")?;
    }
    if Path::new("../libs/android").exists() {
        fs::remove_dir_all("../libs/android")?;
    }
    if Path::new("../libs/ios").exists() {
        fs::remove_dir_all("../libs/ios")?;
    }
    // Now execute the build-all script in a shell.
    let script = String::from("./build-all.sh");
    let mut cmd = Command::new("bash");
    cmd.arg(script);
    match cmd.output() {
        Ok(_t) => {},
        Err(e) => return Err(e),
    };
    Ok(())

}

fn main() {
    run_dependency_check().unwrap();
    }