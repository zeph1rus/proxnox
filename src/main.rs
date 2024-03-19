use std::string::ParseError;

use db;
use pn;

fn main() -> Result<(), ParseError> {
    println!("Hello, world!");

    let x = pn::find_db("/private/var").unwrap();
    println!("{:?}", x);
    let lastid = db::get_latest_notification_id(&x).unwrap();
    println!("Lastid: {:?}", lastid.id);

    let last_minus_a_bit = db::LastId {
        id: lastid.id - 50
    };
    let _ = db::get_new_notifications(last_minus_a_bit, &x);
    Ok(())
}
