mod utils;
#[allow(non_snake_case)]
mod uHuge;

fn main() {
    println!("The {}-th fibonacci number is: {}", 20000, utils::fib_iter(20000));
}
