/*
About: Gets random code from internet; mainly github and gitlabs, but should wotk with standard HTML
*/


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let link = "https://raw.githubusercontent.com/BenRoe/MMM-SystemStats/master/MMM-SystemStats.js";

    let resp = reqwest::get(link).await?.text().await?;
    println!("{:?}", resp);
    Ok(())
}