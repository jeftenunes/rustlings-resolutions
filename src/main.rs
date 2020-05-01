//rustlings/clippy

fn main() {
    clippy1();
    clippy2();
}

fn clippy1() {
    let x = 1.2331f64;
    let y = 1.2332f64;
    let error = 0f64; 
    if (y - x).abs() > error  {
        println!("Success!");
    }
}

fn clippy2() {
    let mut res = 42;
    let option = Some(12);
    if let Some(x) = option {
        res += x;
    }

    println!("{}", res);
}