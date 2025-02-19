fn fibo(n: u32) -> u32 {
    if (n==0) {return 0}
    if (n==1) {return 1}
    
}


fn main() {
    for i in 0..=42{
        println!("fibo({i})={}",fibo(i));
    }
}
