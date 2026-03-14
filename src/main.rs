use clap::{Parser, ValueEnum};
use trust_dns_resolver::config::*;
use trust_dns_resolver::Resolver;
use std::path::Path;
use std::io::{self, BufRead};
use std::fs::File;

// Struct to hold the three required arguments: mode (single|multi), input (URL or path to file)
// and suppress (true or false).
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args{
    /// The mode to run the program in.
    #[arg(short, long)]
    mode: Mode,

    /// URL or path to file.
    #[arg(short, long)]
    input: String,

    /// Suppress domains with no record. [default: false]
    #[arg(short, long, default_value_t = false)]
    suppress: bool,
}

// Enum to define the two available modes.
#[derive(ValueEnum, Clone, Debug)]
enum Mode {
    Single,
    Multi,
}

// Main, create resolver for DNS, match statement to determine output based on the provided mode.
// If the mode is single, call the print_mx function. If the mode is multi, call the print_mx
// unction and enumerate through each line in the provided file.
fn main() {
    let args = Args::parse();

    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default())
        .expect("Unable to create resolver...");
  
    match args.mode {
        Mode::Single => {
            print_mx(&resolver, args.input.trim(), args.suppress);
        }

        Mode::Multi => {
            eprintln!("Reading from: {}\n", args.input);
            let lines = read_lines(&args.input)
                .expect("Should have been able to read file...");

            for line in lines.map_while(Result::ok) {
                let domain = line.trim().to_string();
                
                print_mx(&resolver, &domain, args.suppress)
            }
        }
    }
}

// Function to read a file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Function to print MX records. If the suppress argument is not provided, then show the domain and
// "No Record.", if the argument is provided, show no output.
fn print_mx(resolver: &Resolver, domain: &str, suppress: bool) {
    match resolver.mx_lookup(domain) {
        Err(_) => {
            if !suppress {
                println!("{} : No Record.", domain);
            }
        }
        Ok(response) => {
            for record in response.iter() {
                println!("{} {} {}", domain, record.preference(), record.exchange())
            }
        }
    }
}
