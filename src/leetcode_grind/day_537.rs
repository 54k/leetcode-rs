pub fn compare_version(version1: String, version2: String) -> i32 {
    let s1 = version1.split(".").collect::<Vec<_>>();
    let s2 = version2.split(".").collect::<Vec<_>>();

    let n1 = s1.len();
    let n2 = s2.len();

    for i in 0..n1.max(n2) {
        let i1 = if i < n1 {
            s1[i].parse::<i32>().unwrap()
        } else {
            0
        };
        let i2 = if i < n2 {
            s2[i].parse::<i32>().unwrap()
        } else {
            0
        };

        if i1 != i2 {
            return if i1 > i2 { 1 } else { -1 };
        }
    }
    0
}
