use std::fmt;
use std::os::macos::fs::MetadataExt;
use std::path::PathBuf;

use users::{get_current_uid, uid_t};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct NotFoundError;

impl fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Notifications DB Not Found")
    }
}

fn what_is_my_uid() -> uid_t {
    get_current_uid()
}


fn do_i_own_the_file(uid: u32) -> bool {
    let cur_uid = what_is_my_uid();
    cur_uid == uid
}


pub fn find_db(path: &str) -> Result<PathBuf, NotFoundError>{
    for entry in WalkDir::new(path) {
        match entry {
            Ok(path) => {
                let path_str = path.clone().path();
                if path.clone().into_path().to_str().unwrap().contains("com.apple.notification") {
                    match path.clone().file_name().to_str().unwrap() {
                        "db" => {
                            println!("Possible: {:?}", path);
                            match do_i_own_the_file(path.clone().metadata().unwrap().st_uid()) {
                                true => {
                                    println!("I Own the file - Valid Notifications DB Found");
                                    let valid_path: PathBuf = path.into_path().canonicalize().unwrap();
                                    return Ok(valid_path)
                                }
                                false => {}
                            }
                        }
                        _ => {}

                    }
                }
            }
            Err(_) => {
                // we will get errors as we will hit paths we don't
                //have permissions to.  This is fine, as they say.
            }
        };
    }
    return Err(NotFoundError)
}