pub fn restore_ip_addresses(mut s: String) -> Vec<String> {
    fn backtrack(s: &mut String, i: usize, dots: usize, res: &mut Vec<String>) {
        fn is_valid(s: &mut str) -> bool {
            let v = s.split('.').filter(|&c| !c.is_empty()).collect::<Vec<_>>();
            v.len() == 4
                && v.into_iter().all(|x| {
                    if x.len() > 1 && x.chars().take(1).collect::<Vec<_>>()[0] == '0' {
                        return false;
                    }
                    x.parse::<u32>().unwrap() <= 255
                })
        }

        if dots == 3 && is_valid(s) {
            res.push(s.to_string());
            return;
        }

        if dots >= 3 {
            return;
        }

        for k in i..s.len() {
            let mut c = s.clone();
            c.insert(k, '.');
            backtrack(&mut c, k + 2, dots + 1, res);
        }
    }

    if s.len() < 4 || s.len() > 12 {
        return vec![];
    }
    let mut res = vec![];
    backtrack(&mut s, 1, 0, &mut res);
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test170() {
        println!("{:?}", restore_ip_addresses("25525511135".to_string()));
        println!("{:?}", restore_ip_addresses("0000".to_string()));
        println!("{:?}", restore_ip_addresses("101023".to_string()));
    }
}
