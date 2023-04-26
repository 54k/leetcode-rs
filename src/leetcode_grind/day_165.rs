pub fn add_digits(mut num: i32) -> i32 {
    let mut digital_root = 0;
    while num > 0 {
        digital_root += num % 10;
        num /= 10;
        if num == 0 && digital_root > 9 {
            num = digital_root;
            digital_root = 0;
        }
    }
    digital_root
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test449() {
        println!("{}", add_digits(38)); // 2
    }
}