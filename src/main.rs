mod utils;
#[allow(non_snake_case)]
mod uHuge;

fn main() {
    println!("The {}-th fibonacci number is: {}", 35, utils::fib_iter(35));
}
