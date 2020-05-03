use super::*;
use std::char::decode_utf16;
use std::fs::{self, File};
use std::io::Write;
use tempdir::TempDir;


#[test]
fn test_parse() {

    let mut tempVersion = String::from("0. Initial version.");
    let mut tempVersionLog = String::from("2. more changes \n 0. Initial version.\n 1. SomeChanges");
    assert_eq!(parse_version(tempVersionLog, tempVersion), Ok(true));
}

#[test]
fn test_run_dependency_check() {

}

#[test]
fn test_clobber() {

    //let root = Path::new("../../libs");
    //assert!(env::set_current_dir(&root).is_ok());

    let s = Status::OK;
    let dir1 = TempDir::new_in("./", "test1");
    let dir2 = TempDir::new_in("./", "test2");
    let dir3 = TempDir::new_in("./", "test3");
    let mut test_dir_list = vec![String::from("test1"), String::from("test2"), String::from("test3")];
    let mut tdl = vec![dir1, dir2, dir3];

    // Check that the temp directory actually exists.
    let mut tmp_path;
    for x in &tdl {
        tmp_path = x.as_ref().unwrap().path().to_owned();
        assert!(tmp_path.exists());
    }

    clobber(&mut test_dir_list);

    //make sure directories no longer exist
    for x in &tdl {
        tmp_path = x.as_ref().unwrap().path().to_owned();
        assert_eq!(tmp_path.exists(), false);
    }
}


#[test]
// run process with defaults
fn end_to_end_test_1() {

    let opt = Opt::from_args();
    let dir1 = TempDir::new_in("./", "test1");
    let dir2 = TempDir::new_in("./", "test2");
    let dir3 = TempDir::new_in("./", "test3");
    let mut test_dir_list = vec![String::from("test1"), String::from("test2"), String::from("test3")];
    run_process(opt.script, test_dir_list); //this currently does what its supposed to...not with temp directories.
    //assert_eq!();

}

#[test]
// run proccess with custom commands.
fn end_to_end_test_2() {
    /*
    let opt = Opt::from_args();
    run_process(opt.script, opt.inputs);
    //assert_eq!();
    */
}

