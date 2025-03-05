fn fibo(n: u32) -> u32 {
    let mut a: [u32; 3] = [0, 1, 0];
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    for _ in 0..=n - 2 {
        a[2] = a[1].checked_add(a[0]).unwrap();
        a[0] = a[1];
        a[1] = a[2];
    }
    a[2]
}

fn main() {
    for i in 0..=50 {
        println!("fibo({i})={}", fibo(i));
    }
}
