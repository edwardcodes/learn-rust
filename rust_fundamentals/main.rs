// find prime number for the given range
fn main() {
    const N: i32 = 101;

    for i in 1..N + 1 {
        if is_prime(i) {
            println!("{}", i);
        }
    }
}

fn is_prime(number: i32) -> bool {
    if (number == 1) || (number == 0) {
        return false;
    }
    for i in 2..number {
        if number % i == 0 {
            return false;
        }
    }
    return true;
}
