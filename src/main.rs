fn fact(n: i32) -> i32 {
    if n <= 0 {
        return 1;
    }
    return n * fact(n - 1);
}

fn fib(n:i32) -> i32{
    if n <= 1 {
        return 1;
    }
    return fib(n-1) + fib(n-2);
}

fn main() {
    println!("Hello, world!");
    println!("{}", fact(10));
    println!("{}", fib(45));
}