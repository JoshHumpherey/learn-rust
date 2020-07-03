fn main() {
    let res = gen_fib(10);
    println!("{}", res);
}

fn gen_fib(n: i32) -> i32 {
    if n <= 2 {
        return 1;
    }
    else {
        let mut prev = 1;
        let mut curr = 1;
        let mut sum = 0;
        for i in 2..n {
            sum = curr + prev;
            prev = curr;
            curr = sum;
        }
        return sum;
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_gen_fib_1() {
        let n = 1;
        let expected = 1;
        assert!(gen_fib(n) == expected);
    }

    #[test]
    fn test_gen_fib_2() {
        let n = 2;
        let expected = 1;
        assert!(gen_fib(n) == expected);
    }

    #[test]
    fn test_gen_fib_3() {
        let n = 3;
        let expected = 2;
        assert!(gen_fib(n) == expected);
    }

    #[test]
    fn test_gen_fib_10() {
        let n = 12;
        let expected = 144;
        assert!(gen_fib(n) == expected);
    }
}
