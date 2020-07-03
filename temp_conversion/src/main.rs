use math::round;

const PRECISION: i8 = 1;

// Temperature Conversion Program
fn main() {
    convert_c_to_f(100.0);
    convert_f_to_c(37.0);
}

fn convert_c_to_f(celcius: f64) -> f64 {
    let farrenheit = celcius*(9.0/5.0)+32.0;
    let rounded_farrenheit = round::ceil(farrenheit, PRECISION);
    println!("{}C == {}F", celcius, rounded_farrenheit);
    return rounded_farrenheit;
}

fn convert_f_to_c(farrenheit: f64) -> f64 {
    let sub: f64 = farrenheit-32.0;
    let celcius;
    match sub == 0.0 {
        true => celcius = 0.0,
        false => celcius = sub * (5.0/9.0)
    };
    let rounded_celcius = round::ceil(celcius, PRECISION);
    println!("{}F == {}C", farrenheit, rounded_celcius);
    return rounded_celcius;
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_c_to_f() {
        let celcius = 37.0;
        let expected = 98.7;
        assert!(convert_c_to_f(celcius) == expected);
    }

    #[test]
    fn test_c_to_f_with_zero_value() {
        let celcius = 0.0;
        let expected = 32.0;
        assert!(convert_c_to_f(celcius) == expected);
    }

    #[test]
    fn test_f_to_c() {
        let farrenheit = 100.0;
        let expected = 37.8;
        assert!(convert_f_to_c(farrenheit) == expected);
    }

    #[test]
    fn test_f_to_c_with_zero_value() {
        let farrenheit = 32.0;
        let expected = 0.0;
        assert!(convert_f_to_c(farrenheit) == expected);
    }
}