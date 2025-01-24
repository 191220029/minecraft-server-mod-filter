use std::{io::Read, thread::sleep, time::Duration};

use fake_user_agent::get_edge_rua;
use log::info;
use rand::Rng;
use reqwest::{blocking::Client, IntoUrl};

pub fn get_body<T>(client: &Client, url: T) -> Result<String, reqwest::Error>
where
    T: IntoUrl + std::fmt::Display,
{
    let mut rng = rand::thread_rng();
    let tick = rng.gen_range(5000..15000);
    // wait for a tick
    sleep(Duration::from_millis(tick));

    info!("requesting {}", &url);
    let agent = get_edge_rua();
    let req = client.get(url).header("User-Agent", agent);
    let resp = req.send()?;
    resp.text()
}

pub fn cli_pause() {
    println!("Press any key to exit...");
    let mut stdin = std::io::stdin();
    let _ = stdin.read(&mut [0]);
}
