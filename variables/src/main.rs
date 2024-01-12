use std::io;

fn main() {
    convert_f_c();
    fib()
}

fn int(n: String) -> i64 {
    return n.trim().parse().expect("input number")
}

fn fib() {
    let mut n = String::new();

    println!("Input your nth for fib");
    io::stdin()
        .read_line(&mut n)
        .expect("Fail read line");
    
    let n: i64 = int(n);

    let f = fibonacci(n);
    println!("{n}th fib is {f}");
}

fn fibonacci(n: i64) -> i64 {
    if n == 0 {
        return 0
    } else if n == 1{
        return 1
    } else {
        return fibonacci(n-1) + fibonacci(n-2)
    }
}

fn convert_f_c() {
    let tem_c: f32 = 37.0;
    let tem_f: f32 = 98.6;
    let c = convert(tem_f, true);
    let f = convert(tem_c, false);
    println!("C{tem_c} is {f} in F");
    println!("F{tem_f} is {c} in C");
}

fn convert(tem: f32, to_c: bool) -> f32 {
    let x = if to_c {
        5.0 * (tem - 32.0) / 9.0
    } else {
        9.0 * tem / 5.0 + 32.0
    };
    return x
}