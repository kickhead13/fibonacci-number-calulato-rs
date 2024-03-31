use crate::uHuge::uHuge;

static SQUARE_ROOT: f64 = 2.2360679797749979;
static INVERSE_SQUARE_ROOT: f64 = 1f64 / SQUARE_ROOT;
const HUGE_SIZE: usize = 1000;

pub fn fib_first_expo(n: u64) -> f64 {

    let fraction: f64 = (1f64 + SQUARE_ROOT) / 2f64;
    let mut expo: f64 = fraction;
    for _ in 0..n {
        expo *= fraction;
    }
    return expo;

}

pub fn fib_second_expo(n: u64) -> f64 {

    let fraction: f64 = (1f64 - SQUARE_ROOT) / 2f64;
    let mut expo: f64 = fraction;
    for _ in 0..n {
        expo *= fraction;
    }
    return expo;

}

pub fn fib_diff(n: u64) -> f64{

    return fib_first_expo(n) - fib_second_expo(n);

}

//#[cached]
pub fn fib(n: u64) -> uHuge<HUGE_SIZE> {

    let result: f64 = INVERSE_SQUARE_ROOT * fib_diff(n);
    return uHuge::<HUGE_SIZE>::from(result.round() as u128);

}


pub fn fib_iter(n: u64) -> uHuge<HUGE_SIZE> {

    if n < 36 {
        return fib(n-1);
    }
    let mut result: uHuge<HUGE_SIZE> = fib(35);
    let mut second: uHuge<HUGE_SIZE> = fib(34);
    #[allow(unused_assignments)]
    let mut third: uHuge<HUGE_SIZE> = fib(34);
    let mut counter: u32 = 36;
    while (counter as u64) < n  {
        third = result.clone();
        result = result + second;
        second = third.clone();
        counter+=1;
    }
    return result;

} 