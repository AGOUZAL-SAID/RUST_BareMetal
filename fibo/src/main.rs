use clap::Parser;

fn fibo(n: u32) -> Option<u32> {
    let mut a: [u32; 3] = [0, 1, 0];
    if n == 0 {
        return Some(0);
    }
    if n == 1 {
        return Some(1);
    }
    for _ in 0..=n - 2 {
        a[2] = a[1].checked_add(a[0])?;
        a[0] = a[1];
        a[1] = a[2];
    }
    Some(a[2])
}

/// compute fibonacci suite value
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
    let min: u32 = args.mini.unwrap_or(0);
    let max: u32 = args.value.unwrap_or(50); // Par défaut, on affiche jusqu'à 50
    let all: bool = args.verbose;

    for i in min..=max {
        match fibo(i) {
            Some(result) => {
                if all || i == max {
                    println!("fibo({i}) = {result}");
                }
            }
            None => {
                println!("fibo({i}) => u32 overflow");
                break;
            }
        }
    }
}
