// フィボナッチ数列の計算
use std::env;

// メインプログラム
fn main() {
    println!("<< Fibonacci number >>");
    let args: Vec<String> = env::args().collect();
    let mut n = 11;
    if args.len() > 1 {
        n = args[1].to_string().parse::<u32>().unwrap();
    }
    for i in 1..n {
        println!("fib({}) = {}", i, fib(i));
    }
}

// n に対するフィボナッチ数を返す関数
fn fib(n:u32) -> u32 {
    if n == 0 || n == 1 {
        n
    } else {
        fib(n-2) + fib(n-1)
    }
}
