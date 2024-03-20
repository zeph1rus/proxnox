use std::thread;
use std::time::Duration;

use clap::Parser;

use db;
use db::LastId;
use pn;

const IGNORE_APPS: [&'static str; 2] = ["discord", "Discord"];

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    webhook: String,
}

fn allow_notification(app_name: &str) -> bool {
    IGNORE_APPS.into_iter().all(|i| { !app_name.contains(i) })
}


fn print_sep() {
    println!("{}", "_".repeat(40))
}

fn main() {
    println!("ProxNox - MacOS => Discord Notifications Forwarder");

    let args = Args::parse();

    println!("Sending to Webhook: {}", args.webhook);

    let x = pn::find_db("/private/var").unwrap();

    let lastid = db::get_latest_notification_id(&x).unwrap();
    println!("Initial Notification ID: {:?}", lastid.id);
    print_sep();
    let mut current_id: u32 = lastid.id;

    loop {
        let new_notifications = db::get_new_notifications(LastId { id: current_id }, &x);
        match new_notifications {
            Ok(notifications) => {
                for n in notifications.notifications {
                    match allow_notification(&n.app) {
                        true => {
                            pn::try_send_notification(&n.notification_string(), &args.webhook);
                            print_sep();
                        }
                        false => {
                            println!("Notification ignored");
                            print_sep();
                        }
                    }
                }
                current_id = notifications.new_last_id;
            }

            _ => {}
        }

        thread::sleep(Duration::from_secs(10));
    }
}
