pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
    let mut c = (0..26).collect();

    fn find(c: &mut Vec<usize>, a: usize) -> usize {
        if c[a] == a {
            return a;
        }
        c[a] = find(c, c[a]);
        c[a]
    }

    fn union(c: &mut Vec<usize>, a: usize, b: usize) {
        let a = find(c, a);
        let b = find(c, b);
        if a == b {
            return;
        }
        if a < b {
            c[b] = a;
        } else {
            c[a] = b;
        }
    }

    s1.chars().zip(s2.chars()).for_each(|(x, y)| {
        union(&mut c, x as usize - 'a' as usize, y as usize - 'a' as usize);
    });

    base_str
        .chars()
        .map(|ch| find(&mut c, ch as usize - 'a' as usize) + 'a' as usize)
        .map(|u| char::from_u32(u as u32).unwrap())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test159() {
        println!(
            "{}",
            smallest_equivalent_string(
                "parker".to_string(),
                "morris".to_string(),
                "parser".to_string(),
            )
        ); // makkek
    }
}
