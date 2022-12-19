use std::{env, thread, time::Duration};

fn health_check(interval: u64, url: &String) {
    let client = match reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(1))
        .build()
    {
        Ok(result) => result,
        Err(err) => {
            print!("Error: {}", err);
            return;
        }
    };

    loop {
        match client.get(url).send() {
            Ok(response) => {
                print!("Checking '{}'. Result: ", url);

                if response.status() != 200 {
                    println!("ERR({})", response.status().as_u16());
                } else {
                    println!("OK(200)");
                }
            }

            Err(_) => {
                println!("URL parsing error");
                return;
            }
        }

        thread::sleep(Duration::from_secs(interval));
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 2 {
        println!("Invalid amount of arguments.");
        return;
    }

    let url = &args[1];
    match args[0].parse::<u64>() {
        Ok(interval) => health_check(interval, url),
        Err(_) => {
            println!("Incorrect value of interval");
        }
    };
}
