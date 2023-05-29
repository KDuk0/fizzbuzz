fn main() {
    print_fizzbuzz(50);
}

fn is_divisible(n: u8, divisor: u8) -> bool {
    if divisor == 0 {
        return false;
    }

    n % divisor == 0
}

fn fizzbuzz(n: u8) -> String {
    let fizz = if is_divisible(n, 3) { "fizz" } else { "" };
    let buzz = if is_divisible(n, 5) { "buzz" } else { "" };

    if fizz.is_empty() && buzz.is_empty() {
        return format!("{n}");
    }

    format!("{}{}", fizz, buzz)
}

fn print_fizzbuzz(n:u8) {
    for i in 1..=n {
        println!("{}", fizzbuzz(i));
    }
}