// https://leetcode.com/problems/find-and-replace-in-string/
pub fn find_replace_string(
    s: String,
    mut indices: Vec<i32>,
    mut sources: Vec<String>,
    mut targets: Vec<String>,
) -> String {
    let mut changes = vec![]; // idx, from, to
    while let Some(i) = indices.pop() {
        changes.push((i as usize, sources.pop().unwrap(), targets.pop().unwrap()));
    }
    changes.sort_by(|x, y| x.0.cmp(&y.0));

    let mut replaced = String::new();
    let original = s.chars().collect::<Vec<_>>();

    let mut i = 0;
    let mut j = 0;

    while i < original.len() && j < changes.len() {
        let change = &changes[j];
        if i == change.0 {
            let from = change.1.chars().collect::<Vec<_>>();
            if original[i..i + from.len()] == from[..from.len()] {
                replaced.push_str(&change.2);
                i += from.len();
            } else {
                replaced.push(original[i]);
                i += 1;
            }
        } else {
            replaced.push(original[i]);
            i += 1;
        }
        if i > change.0 {
            j += 1;
        }
    }

    if i < original.len() {
        replaced.extend(&original[i..]);
    }

    replaced
}

// todo https://leetcode.com/problems/find-all-k-distant-indices-in-an-array/description/
// todo https://leetcode.com/problems/split-with-minimum-sum/description/
// todo https://leetcode.com/problems/task-scheduler/description/
// todo https://leetcode.com/problems/strong-password-checker/description/
// todo https://leetcode.com/problems/replace-non-coprime-numbers-in-array/description/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test337() {
        println!(
            "{}",
            find_replace_string(
                "abcd".to_string(),
                vec![0, 2],
                vec!["ab".to_string(), "cd".to_string()],
                vec!["eee".to_string(), "ffff".to_string()],
            )
        ); // eeecd

        println!(
            "{}",
            find_replace_string(
                "abcd".to_string(),
                vec![0, 2],
                vec!["ab".to_string(), "ec".to_string()],
                vec!["eee".to_string(), "ffff".to_string()],
            )
        ); // eeecd

        println!(
            "{}",
            find_replace_string(
                "vmokgggqzp".to_string(),
                vec![3, 5, 1],
                vec!["kg".to_string(), "ggq".to_string(), "mo".to_string()],
                vec!["s".to_string(), "so".to_string(), "bfr".to_string()],
            )
        ); // vbfrssozp

        println!(
            "{}",
            find_replace_string(
                "mhnbzxkwzxtaanmhtoirxheyanoplbvjrovzudznmetkkxrdmr".to_string(),
                vec![46, 29, 2, 44, 31, 26, 42, 9, 38, 23, 36, 12, 16, 7, 33, 18],
                vec![
                    "rym", "kv", "nbzxu", "vx", "js", "tp", "tc", "jta", "zqm", "ya", "uz", "avm",
                    "tz", "wn", "yv", "ird"
                ]
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>(),
                vec![
                    "gfhc", "uq", "dntkw", "wql", "s", "dmp", "jqi", "fp", "hs", "aqz", "ix",
                    "jag", "n", "l", "y", "zww"
                ]
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>(),
            )
        ); // "mhnbzxkwzxtaanmhtoirxheaqznoplbvjrovzudznmetkkxrdmr"
    }
}
