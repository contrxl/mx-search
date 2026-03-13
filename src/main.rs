use trust_dns_resolver::config::*;
use trust_dns_resolver::Resolver;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process;

fn main() {
    // Construct a new Resolver with default configuration options
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();

    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    
    if config.choice == "single" {
         let mx_response = resolver.mx_lookup(config.input.trim());

        match mx_response {
            Err(_) => println!("{} : No Records", config.input),
            Ok(mx_response) => {
                let addresses = mx_response.iter();
                for record in addresses {
                    println!("{} {} {}", config.input, record.preference(), record.exchange());
                }
            }
        }
    }


    if config.choice == "multi" {
        println!("Path: {}", config.input);

        let contents = read_lines(config.input)
            .expect("Should have been able to read the file.");
        for content in contents.map_while(Result::ok) {
            let mx_response = resolver.mx_lookup(content.trim());

            match mx_response {
                Err(_) => println!("{} : No Records", content),
                Ok(mx_response) => {
                    let addresses = mx_response.iter();
                    for record in addresses {
                        println!("{} {} {}", content, record.preference(), record.exchange());
                    }
                }
            }
        }
    }
}

struct Config {
    choice: String,
    input: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }

        let choice = &args[1].clone();
        let input  = &args[2].clone();

       Ok(Config { choice: choice.to_string(), input: input.to_string() })
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
