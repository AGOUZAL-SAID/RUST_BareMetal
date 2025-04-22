use clap::Parser;

// Compute the nth Fibonacci number using iteration and checked addition
fn fibo(n: u32) -> Option<u32> {
    let mut a: [u32; 3] = [0, 1, 0];

    if n == 0 {
        return Some(0);
    }
    if n == 1 {
        return Some(1);
    }

    for _ in 0..=n - 2 {
        a[2] = a[1].checked_add(a[0])?; // Checked addition to avoid overflow
        a[0] = a[1];
        a[1] = a[2];
    }

    Some(a[2])
}

/// Compute Fibonacci sequence values
#[derive(Parser)]
struct Args {
    /// The maximal number to print the fibo value of
    value: Option<u32>,

    /// Print intermediate values
    #[clap(short, long, default_value_t = false)]
    verbose: bool,

    /// The minimum number to compute
    #[clap(short, long)]
    mini: Option<u32>,
}

fn main() {
    let args = Args::parse();

    let min: u32 = args.mini.unwrap_or(0); // Default minimum is 0
    let max: u32 = args.value.unwrap_or(50); // Default maximum is 50
    let all: bool = args.verbose; // Whether to print all values

    for i in min..=max {
        match fibo(i) {
            Some(result) => {
                // Print only the final value unless verbose mode is enabled
                if all || i == max {
                    println!("fibo({i}) = {result}");
                }
            }
            None => {
                // Overflow detected
                println!("fibo({i}) => u32 overflow");
                break;
            }
        }
    }
}
