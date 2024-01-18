use std::{env, thread::sleep, time::Duration};

fn main() {
    // 480 for Spacewar
    // 606150 for Moonlighter
    let app_id = env::args()
        .nth(1)
        .expect("didn't give app ID")
        .parse::<u32>()
        .expect("invalid ID");
    let _client = steamworks::Client::init_app(app_id)
        .expect("failed to initialise steamworks");
    println!("We gaming?");
    sleep(Duration::from_secs(60));
}
