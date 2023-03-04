pub fn say_hello(name: &str) -> String {
    format!("Hello {}!", name)
}

pub fn get_fibonacci(number: isize) -> u128 {
    if number == 1 {
        return 1;
    } else if number == 2 {
        return 2;
    }

    let mut sum = 0;
    let mut last = 0;
    let mut curr = 1;
    for _ in 1..number {
        sum = last + curr;
        last = curr;
        curr = sum;
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::util::get_fibonacci;
    #[test]
    fn test_fibonacci() {
        assert_eq!(get_fibonacci(1), 1);
        assert_eq!(get_fibonacci(10), 55);
        assert_eq!(get_fibonacci(20), 6765);
    }
}