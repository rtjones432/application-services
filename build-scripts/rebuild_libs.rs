/*
* This is a script to clobber the {desktop, ios, android} directories
* and rebuild libs via the build-all.sh script.
*/
use std::fs;
use std::path::Path;
use std::env;
use std::process::Command;
use std::io::prelude::*;

#[allow(unused_must_use)]
fn check_version() -> bool {

    let mut clobber_needed: bool = false;
    let root = Path::new("../libs");

    assert!(env::set_current_dir(&root).is_ok());

    let mut file = fs::File::open(".local_libs_version").expect("Counld not rebuild libs. Delete your folders for desktop, ios, and android, then manually run libs/build-all.sh \n");
    let mut local_version = String::new();
    file.read_to_string(&mut local_version);

    //do I need to do further file parsing?
    let mut file2 = fs::File::open("version_log").expect("Counld not rebuild libs. Delete your folders for desktop, ios, and android, then manually run libs/build-all.sh \n");
    let mut newest_version = String::new();
    file2.read_to_string(&mut newest_version);

    //this reads the most recent version number. (everything before the '.')
    let mut s = "".to_owned();
    for c in newest_version.chars() {
        if c == '.' {
            break;
        } else {
            s = s + &c.to_string();
        }
    }
    let mut t = "".to_owned();
    for c in local_version.chars() {
        if c == '.' {
            break;
        } else {
            t = t + &c.to_string();
        }
    }

    if t != s {
        clobber_needed = true;
        fs::write(".local_libs_version", newest_version);
    }
    clobber_needed
}

fn delete_dirs() -> std::io::Result<()> {
    println!("deleting old directories and rebuilding /libs...\n");
    if Path::new("../libs/desktop").exists() {
        fs::remove_dir_all("desktop")?;
    }

    if Path::new("../libs/android").exists() {
        fs::remove_dir_all("/android")?;
    }
    if Path::new("../libs/ios").exists() {
        fs::remove_dir_all("/ios")?;
    }
    // Now execute the build-all script in a shell.
    let mut cmd = Command::new("bash");
    cmd.arg("./build-all.sh");

    match cmd.output() {
        Ok(_o) => {}
        Err(_e) => {}
    }
    Ok(())
}

#[allow(unused_must_use)]
fn main() {

    let clobber = check_version();
        if clobber { delete_dirs(); }

}