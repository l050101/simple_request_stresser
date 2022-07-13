use std::time::{Instant};
use std::io::stdin;

fn get_url() -> String{
    let mut input = String::new();
    stdin().read_line(&mut input)
    	.ok()
        .expect("Failed to read line");
    input
}
async fn stress(url:&str) -> Result<(), Box<dyn std::error::Error>>{
        
    for x in 1..100{
        let resp = reqwest::get(url)
            .await?
            .text()
            .await?;
        println!("cnt : {} {:#?}",x,resp);
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("send 1000 request!");
    println!("input your url! :");
    let url = get_url();

    let started = Instant::now();
    stress(&url).await?;

    let now = Instant::now();
    println!("total : {:?}",now-started);
    Ok(())
}

// "http://ldj.ddns.net:8000/hello/sibal/19"