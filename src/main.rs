use trust_dns_resolver::config::*;
use trust_dns_resolver::Resolver;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // Construct a new Resolver with default configuration options
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();

    let args: Vec<String> = env::args().collect();
    
    let (choice, input) = parser(&args);
    
    if choice == "single" {
        let mx_response = resolver.mx_lookup(input.trim());

        match mx_response {
            Err(_) => println!("{} : No Records", input),
            Ok(mx_response) => {
                let addresses = mx_response.iter();
                for record in addresses {
                    println!("{} {} {}", input, record.preference(), record.exchange());
                }
            }
        }
    }

    if choice == "multi" {
        println!("Path: {input}");

        let contents = read_lines(input)
            .expect("Should have been able to read the file.");
        for content in contents.map_while(Result::ok) {
            let mx_response = resolver.mx_lookup(content.trim());

            match mx_response {
                Err(_) => println!("{} : No Records", input),
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

fn parser(args: &[String]) -> (&str, &str){
        let choice = &args[1];
        let input  = &args[2];

        (choice, input)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
