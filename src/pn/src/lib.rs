use std::fmt;
use std::os::macos::fs::MetadataExt;

use reqwest;
use serde::{Deserialize, Serialize};
use users::get_current_uid;
use walkdir::WalkDir;

#[derive(Debug, Serialize, Deserialize)]
struct DiscordPost {
    content: String,
}


#[derive(Debug, Clone)]
pub struct NotFoundError;

impl fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Notifications DB Not Found")
    }
}


fn do_i_own_the_file(uid: u32) -> bool {
    let cur_uid = get_current_uid();
    cur_uid == uid
}


pub fn try_send_notification(text: &str, webhook: &str) {
    println!("Sending Notification: \n{}\n\n", text);
    let post_data = DiscordPost { content: text.parse().unwrap() };
    let client = reqwest::blocking::Client::new();
    let _ = client.post(webhook).json(&post_data).send();
}

pub fn find_db(path: &str) -> Result<String, NotFoundError> {
    println!("Searching for notifications database");
    for entry in WalkDir::new(path) {
        match entry {
            Ok(path) => {
                let path_str = String::from(path.clone().into_path().to_str().unwrap());
                let path_fn = String::from(path.clone().file_name().to_string_lossy());

                if path_str.contains("com.apple.notification") {
                    match path_fn.as_str() {
                        "db" => {
                            println!("Possible Notifications DB Found: {:?}", path);
                            match do_i_own_the_file(path.clone().metadata().unwrap().st_uid()) {
                                true => {
                                    println!("Found Notifications DB (file is owned by current user)");
                                    return Ok(path_str);
                                }
                                false => {
                                    println!("Found a db but I don't own it, ignoring");
                                }
                            }
                        }
                        _ => {
                            // file is not named "db" so we're ignoring it
                        }
                    }
                }
            }
            Err(_) => {
                // we will get errors as we will hit paths we don't
                //have permissions to.  This is fine, as they say.
            }
        };
    }
    return Err(NotFoundError);
}