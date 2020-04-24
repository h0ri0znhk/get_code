/*
About: Gets random code from internet; mainly github and gitlabs, but should work with standard HTML
*/

use std::{thread, time};
use std::io::{self, Write};
use rand::distributions::{Distribution, Uniform};

const PAUSE: [u16; 5] = [15, 25, 35, 35, 100];

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let link = "https://raw.githubusercontent.com/BenRoe/MMM-SystemStats/master/MMM-SystemStats.js";

    let resp = reqwest::get(link).await?.text().await?;

    process_file(resp);

    // println!("{:?}", resp);
    Ok(())
}

fn process_file(input: String) {

    let contents = input;

    let mut rng = rand::thread_rng();
    let die = Uniform::from(0..5);

    for var in contents.chars() {
        let throw = die.sample(&mut rng);

        let millis = time::Duration::from_millis(PAUSE[throw].into());

        thread::sleep(millis);
        print!("{}", var);
        io::stdout().flush().unwrap();
    }
}