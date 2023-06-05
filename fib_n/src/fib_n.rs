pub fn fib_n(f: u64) -> u64 {
    let mut n = 1;
    let mut m = 1;

    if f < 3 {
        1
    } else {
        let mut ct: u64 = f - 2;
        let mut fib: u64 = 0;
        while ct > 0 {
            fib = n + m;
            n = m;
            m = fib;
            ct -= 1;
        }
        fib
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib() {
        let result = fib_n(1);
        assert_eq!(result, 1);
        let result = fib_n(2);
        assert_eq!(result, 1);
        let result = fib_n(3);
        assert_eq!(result, 2);
        let result = fib_n(4);
        assert_eq!(result, 3);
        let result = fib_n(5);
        assert_eq!(result, 5);
        let result = fib_n(6);
        assert_eq!(result, 8);
        let result = fib_n(7);
        assert_eq!(result, 13);
        let result = fib_n(8);
        assert_eq!(result, 21);
        let result = fib_n(9);
        assert_eq!(result, 34);
        let result = fib_n(10);
        assert_eq!(result, 55);
        let result = fib_n(20);
        assert_eq!(result, 6765);
        let result = fib_n(30);
        assert_eq!(result, 832040);
        let result = fib_n(40);
        assert_eq!(result, 102334155);
    }
}
