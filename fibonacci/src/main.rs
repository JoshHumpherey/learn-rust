fn main() {
    let res = gen_fib(100);
    println!("{}", res);
}

fn gen_fib(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => gen_fib(n-1) + gen_fib(n-2)
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
