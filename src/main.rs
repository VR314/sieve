mod sieve;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = 0)]
    is_prime: usize,

    #[arg(long, default_value_t = 0)]
    largest_prime: usize,

    #[arg(long, default_value_t = 0)]
    count_primes: u8,

    #[arg(long, default_value_t = 0)]
    list_primes: u8,
}

fn main() {
    let args = Args::parse();

    if args.is_prime > 0 {
        println!(
            "{} is {}!",
            args.is_prime,
            if sieve::is_prime(args.is_prime as usize) {
                "prime"
            } else {
                "not prime"
            }
        );
    } else if args.largest_prime > 0 {
        println!(
            "The largest prime smaller than {} is {}!",
            args.largest_prime,
            sieve::largest_prime(args.largest_prime as usize)
        );
    } else if args.count_primes > 0 {
        println!(
            "The number of primes smaller than {} is {}!",
            args.count_primes,
            sieve::count_primes(args.count_primes as usize)
        );
    } else if args.list_primes > 0 {
        println!(
            "The list of primes less than {} is {:?}",
            args.list_primes,
            sieve::list_of_primes(args.list_primes as usize)
        );
    } else {
        println!("Please specify a command line option");
    }
}
