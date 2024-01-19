use std::env;

use anyhow::Result;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(a) => match a.as_str() {
            "xkcd" => xkcd(args)?,
            _ => help()
        },
        None => eprintln!("please specify api source or help")
    };

    Ok(())
}

fn help() {
    println!("xkcd [num]: get latest xkcd or number if supplied.")
}

fn xkcd(args: Vec<String>) -> Result<()> {
    use apis::Xkcd;

    let body = match args.get(2) {
        Some(n) => Xkcd::get(n.parse()?)?,
        None => Xkcd::get_latest()?,
    };

    println!("xkcd {} - Published {}-{}-{}\n\n{}\n\n\n{}\n\n\nalt: {}\n\n\nimage url: {}", body.num, body.year, body.month, body.day, body.safe_title, body.transcript, body.alt, body.img);

    Ok(())
}
