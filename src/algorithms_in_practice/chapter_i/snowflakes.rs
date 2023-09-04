#[derive(Debug, Clone, PartialEq, Eq)]
struct SnowflakeNode {
    snowflake: Vec<i32>,
    next: Option<Box<SnowflakeNode>>,
}

fn snowflakes(snowflakes: Vec<Vec<i32>>) -> bool {
    fn snowflake_code(snowflake: &Vec<i32>, m: i32) -> i32 {
        let mut code = 0;
        for s in snowflake {
            code = (code + *s) % m;
        }
        code
    }
    let n = snowflakes.len();
    let mut sflakes = vec![None; snowflakes.len()];

    for s in snowflakes {
        let mut sflake = SnowflakeNode {
            snowflake: s,
            next: None,
        };

        let code = snowflake_code(&sflake.snowflake, n as i32) as usize;
        sflake.next = sflakes[code].take();
        sflakes[code] = Some(Box::new(sflake));
    }

    has_identical(&sflakes)
}

fn has_identical(snowflakes: &Vec<Option<Box<SnowflakeNode>>>) -> bool {
    for i in 0..snowflakes.len() {
        let mut sflake = snowflakes[i].as_ref();

        while let Some(s) = sflake {
            let mut sflake2 = s.next.as_ref();
            while let Some(s2) = sflake2.take() {
                if snowflakes_eq(&s.snowflake, &s2.snowflake) {
                    return true;
                }
                sflake2 = s2.next.as_ref();
            }
            sflake = s.next.as_ref();
        }
    }
    false
}

fn snowflakes_eq(s1: &Vec<i32>, s2: &Vec<i32>) -> bool {
    fn identical_right(s1: &Vec<i32>, s2: &Vec<i32>, start: usize) -> bool {
        for offset in 0..s1.len() {
            if s1[offset] != s2[(start + offset) % s1.len()] {
                return false;
            }
        }
        true
    }

    fn identical_left(s1: &Vec<i32>, s2: &Vec<i32>, start: usize) -> bool {
        for offset in 0..s1.len() {
            let s2_idx = (offset as i32 - start as i32 + s1.len() as i32) % s1.len() as i32;
            if s1[offset] != s2[s2_idx as usize] {
                return false;
            }
        }
        true
    }

    for start in 0..s1.len() {
        if identical_right(&s1, &s2, start) {
            return true;
        }
        if identical_left(&s1, &s2, start) {
            return true;
        }
    }

    false
}

#[test]
fn test_snowflakes() {
    let res = snowflakes(vec![
        vec![1, 2, 3, 4, 5, 6],
        vec![4, 5, 6, 1, 2, 3],
        vec![8, 3, 9, 10, 15, 4],
        vec![16, 1, 1, 1, 1, 1],
        vec![100016, 1, 1, 1, 1, 1],
    ]);
    println!("{res}");
}
