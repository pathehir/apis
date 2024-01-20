use std::env;

use anyhow::Result;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(a) => match a.as_str() {
            "xkcd" => xkcd(args)?,
            "bb" => bb(args)?,
            _ => help()
        },
        None => eprintln!("please specify api source or help")
    };

    Ok(())
}

fn help() {
    println!("xkcd [num]: get latest xkcd or number if supplied.\n
        bb [num]: get one (or multiple) breaking bad quotes.")
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

fn bb(args: Vec<String>) -> Result<()> {
    use apis::breakingbad::Quote;

    let quotes = Quote::get(args.get(2).map(|n| n.parse().unwrap()))?;

    for quote in quotes {
        println!("\"{}\" - {}", quote.quote(), quote.author());
    }

    Ok(())
}
