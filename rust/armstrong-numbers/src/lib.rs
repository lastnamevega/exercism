fn digits(num: u32) -> Vec<u32> {
    let mut digits: Vec<u32> = vec![];
    let mut temp = num;

    while temp > 0 {
        digits.push(temp % 10);
        temp /= 10;
    }

    digits
}

fn armstrong_sum(num: u32) -> u32 {
    let digits = digits(num);
    let exponent = digits.len() as u32;

    digits
        .iter()
        .fold(0, |sum, digit| sum + digit.pow(exponent))
}

pub fn is_armstrong_number(num: u32) -> bool {
    num == armstrong_sum(num)
}
