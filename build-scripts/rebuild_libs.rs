/*
* This is a script to clobber the {desktop, ios, android} directories
* and rebuild libs via the build-all.sh script..
*/
use std::fs;
use std::path::Path;
use std::env;
use std::process::Command;
use std::io::prelude::*;

fn check_version() -> std::io::Result<()> {

    let root = Path::new("../libs");
   //println!("moving to {} directory\n", root.display());
    assert!(env::set_current_dir(&root).is_ok());
    //println!("changed working directory to {}.\n", root.display());

    let mut local_version = fs::File::open(".current_libs_version")?;
    let mut contents = String::new();
    local_version.read_to_string(&mut contents)?;
    //println!("{}\n", contents);

    fs::write(".current_libs_version", "XXX")?;
    Ok(())
}

fn delete_dirs() -> std::io::Result<()> {

    // Deletes the following directories if they exists, then rebuilds libs
    if contents.to_uppercase() != "UPDATED: FALSE" {

        //the .exists function is currently not working?
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
        println!("here/n");
        cmd.arg("./build-all.sh");

        match cmd.output() {
            Ok(_o) => {}
            Err(_e) => {
                println!("Counld not rebuild libs. You will need to manually run libs/build-all.sh \n");
            }
        }
    }
}
fn main() {
    check_version();
    delete_dirs();

}