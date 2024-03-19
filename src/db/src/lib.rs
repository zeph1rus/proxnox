use std::fmt;
use std::io::Cursor;

use plist::Value;
use rusqlite;
use rusqlite::Result;

#[derive(Debug, Clone)]
pub struct LastId {
    pub id: u32,
}

#[derive(Debug, Clone)]
pub struct NoNotificationsError;

impl fmt::Display for NoNotificationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No New Notifications Found")
    }
}

#[derive(Debug, Clone)]
pub struct Notification {
    pub title: String,
    pub body: String,
    pub app: String,
}

#[derive(Debug, Clone)]
pub struct RawNotification {
    pub rec_id: u32,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct GetNotificationsResult {
    pub notifications: Vec<Notification>,
    pub new_last_id: u32,
}

pub fn get_new_notifications(last_id: LastId, sqlite_db: &str) -> Result<GetNotificationsResult> {
    let mut new_notifications: Vec<Notification> = Vec::new();
    let mut new_last_id: u32 = 0;
    let conn = rusqlite::Connection::open(sqlite_db)?;
    let mut stmt = conn.prepare("SELECT rec_id, data from record WHERE rec_id > (?1) ORDER BY rec_id ASC ")?;
    let rows = stmt.query_map(&[&last_id.id], |row| {
        Ok(RawNotification {
            rec_id: row.get(0)?,
            data: row.get(1)?,
        })
    })?;
    for r in rows {
        let raw_unwrapped = r.unwrap();

        println!("Record: {:?}", &raw_unwrapped.rec_id);
        new_last_id = raw_unwrapped.rec_id;

        // transform to a cursor which has read and seek traits.
        // can convert from Vec<u8> -> Bytes Slice -> Cursor safely (I think)
        let cursor_seekable = Cursor::new(raw_unwrapped.data);

        let decoded_dictionary = Value::from_reader(cursor_seekable).unwrap().into_dictionary().unwrap();

        let req_dictionary = decoded_dictionary.get("req").unwrap().as_dictionary().unwrap();

        let default_value = Value::String(std::string::String::from("None"));


        let x = Notification {
            app: decoded_dictionary.get("app").as_ref().unwrap().as_string().unwrap().to_owned(),
            title: req_dictionary.get("titl").unwrap_or_else(|| &default_value).as_string().unwrap_or_else(|| "").to_owned(),
            body: req_dictionary.get("body").unwrap_or_else(|| &default_value).as_string().unwrap_or_else(|| "").to_owned(),
        };
        new_notifications.push(x.clone());
    }
    if new_notifications.len() > 0 {
        return Ok(GetNotificationsResult { notifications: new_notifications, new_last_id });
    }

    Err(rusqlite::Error::QueryReturnedNoRows)
}

pub fn get_latest_notification_id(sqlite_db: &str) -> Result<LastId> {
    let conn = rusqlite::Connection::open(sqlite_db)?;
    let mut stmt = conn.prepare("SELECT rec_id from record ORDER BY rec_id DESC LIMIT 1")?;
    let idi = stmt.query_map([], |row| {
        Ok(LastId {
            id: row.get(0)?
        })
    })?;
    for r in idi {
        return Ok(r?);
    }

    Ok(LastId { id: 0 })
}
