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

fn main() {
    for i in 0..=50 {
        match fibo(i) {
            Some(result) => println!("fibo({i}) = {result}"),
            None => {
                println!("fibo({i}) => u32 overflow");
                break;
            }
        }
    }
}
