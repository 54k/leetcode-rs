use std::error::Error;

fn problem226() -> Result<(), Box<dyn Error>> {
    use std::io;
    let stdin = io::stdin();
    let mut l = String::new();
    stdin.read_line(&mut l)?;
    let mut a = 0;
    let mut b = 1;
    for _ in 2..=l.parse()? {
        let c = (a + b) % 10;
        a = b;
        b = c;
    }
    println!("{}", b);
    Ok(())
}

fn problem228() -> Result<(), Box<dyn Error>> {
    use std::io;
    fn period(m: u128) -> Vec<u128> {
        fn norm(a: u128, m: u128) -> u128 {
            (a % m + m) % m
        }
        fn add(a: u128, b: u128, m: u128) -> u128 {
            norm(norm(a, m) + norm(b, m), m)
        }
        let mut ans = vec![];
        ans.push(0);
        ans.push(1);
        let mut a = 0u128;
        let mut b = 1u128;
        for i in 2..=m * 6 {
            let c = add(a, b, m);
            ans.push(c);
            if i > 2 && ans[i as usize] == 1 && ans[i as usize - 1] == 0 {
                return ans;
            }
            a = b;
            b = c;
        }
        ans
    }
    let stdin = io::stdin();
    let mut l = String::new();
    stdin.read_line(&mut l)?;
    let l = l
        .split(' ')
        .map(|n| n.parse::<u128>().unwrap())
        .collect::<Vec<_>>();

    let n = l[0];
    let m = l[1];
    let p = period(m);
    println!("{}", p[(n % (p.len() as u128 - 2)) as usize]);
    Ok(())
}

fn problem234() {
    use std::io;
    let stdin = io::stdin();
    let mut l = String::new();
    stdin.read_line(&mut l).unwrap();
    let l = l
        .split(' ')
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let a = l[0];
    let b = l[1];
    fn gcd(a: i64, b: i64) -> i64 {
        if b == 0 {
            return a;
        }
        gcd(b, (a % b + b) % b)
    }
    println!("{}", gcd(a, b));
}

struct MaxQueue(Vec<i32>);
impl MaxQueue {
    fn new() -> Self {
        Self(vec![0])
    }
    fn insert(&mut self, val: i32) {
        self.0.push(val);
        self.sift_up();
    }
    fn extract_max(&mut self) -> i32 {
        let last = self.0.len() - 1;
        let res = self.0[1];
        self.0.swap(1, last);
        self.0.pop();
        self.sift_down();
        res
    }
    fn sift_up(&mut self) {
        let mut k = self.0.len() - 1;
        while k / 2 >= 1 && self.0[k] > self.0[k / 2] {
            self.0.swap(k, k / 2);
            k /= 2;
        }
    }
    fn sift_down(&mut self) {
        let mut k = 1;
        let n = self.0.len() - 1;
        while k * 2 <= n {
            let mut j = k * 2;
            if j < n && self.0[j] < self.0[j + 1] {
                j += 1;
            }
            if self.0[k] >= self.0[j] {
                break;
            }
            self.0.swap(k, j);
            k = j;
        }
    }
}

fn problem_pq_max() {
    use std::io;

    macro_rules! parse_line {
    ($($t: ty),+) => ({
        let mut a_str = String::new();
        io::stdin().read_line(&mut a_str).expect("read error");
        let mut a_iter = a_str.split_whitespace();
        (
            $(
            a_iter.next().unwrap().parse::<$t>().ok().unwrap_or_default(),
            )+
        )
    })
}
    let mut max_queue = MaxQueue::new();
    let (n,) = parse_line!(usize);
    for _ in 0..n {
        let (s, val) = parse_line!(String, i32);
        match s.as_str() {
            "Insert" => max_queue.insert(val),
            "ExtractMax" => println!("{}", max_queue.extract_max()),
            _ => panic!(""),
        }
    }
}

pub(self) mod huffman {
    use std::cmp::*;
    use std::collections::*;

    pub fn encode(input: &str) -> HashMap<char, String> {
        #[derive(PartialEq, Eq)]
        struct CharSetFreq {
            char_set: String,
            freq: i32,
        }
        impl PartialOrd<Self> for CharSetFreq {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.freq.cmp(&other.freq))
            }
        }
        impl Ord for CharSetFreq {
            fn cmp(&self, other: &Self) -> Ordering {
                self.freq.cmp(&other.freq)
            }
        }

        let mut freq = HashMap::new();
        for ch in input.chars() {
            *freq.entry(ch).or_insert(0) += 1;
        }

        let mut freq_set = vec![];
        for (ch, freq) in freq {
            freq_set.push(Reverse(CharSetFreq {
                char_set: ch.to_string(),
                freq,
            }));
        }

        let mut queue = BinaryHeap::from(freq_set);
        let mut paths = HashMap::<char, String>::new();

        if queue.len() == 1 {
            paths
                .entry(
                    queue
                        .peek()
                        .unwrap()
                        .0
                        .char_set
                        .chars()
                        .take(1)
                        .collect::<Vec<_>>()[0],
                )
                .or_insert(String::new())
                .push('0');
        }

        while queue.len() > 1 {
            let left = queue.pop().unwrap().0;
            let right = queue.pop().unwrap().0;

            for (lch, rch) in left.char_set.chars().zip(right.char_set.chars()) {
                paths.entry(lch).or_insert(String::new()).push('0');
                paths.entry(rch).or_insert(String::new()).push('1');
            }

            let c = CharSetFreq {
                char_set: format!("{}{}", left.char_set, right.char_set),
                freq: left.freq + right.freq,
            };
            queue.push(Reverse(c));
        }

        paths
            .into_iter()
            .map(|(k, v)| {
                let v = v.chars().rev().collect::<_>();
                (k, v)
            })
            .collect::<HashMap<_, _>>()
    }

    fn solve_encode() {
        let mut input = String::new();
        let stdin = std::io::stdin();
        stdin.read_line(&mut input).unwrap();
        let input = input.trim();
        let codes = encode(input);

        let mut s = String::new();
        for ch in input.chars() {
            s.push_str(&codes[&ch]);
        }
        println!("{} {}", codes.len(), s.len());
        codes
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .for_each(|x| println!("{}", x));
        println!("{}", s);
    }

    pub fn decode() {}
}

// https://gist.github.com/54k/b474dff6727ad2d3bc73c7c8dd6c30be thanks, past myself
fn huffman_encode() {
    use std::cmp::*;
    use std::collections::*;

    struct TreeNode {
        w: i32,
        val: Option<char>,
        left: Option<Box<TreeNode>>,
        right: Option<Box<TreeNode>>,
    }
    impl Eq for TreeNode {}
    impl PartialEq<Self> for TreeNode {
        fn eq(&self, other: &Self) -> bool {
            self.w == other.w
        }
    }
    impl PartialOrd<Self> for TreeNode {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.w.cmp(&other.w))
        }
    }
    impl Ord for TreeNode {
        fn cmp(&self, other: &Self) -> Ordering {
            self.w.cmp(&other.w)
        }
    }

    let mut input = String::new();
    // std::io::stdin().read_line(&mut input).unwrap();
    input.push_str(
        "adassadasdsadsadsaddasdasdasdasdsdasdsdasdasdsadsdasdasasasssddsaasdsadsdasdasdsd",
    );

    let input = input.trim().chars().collect::<Vec<_>>();

    let mut freq = HashMap::new();
    for &ch in &input {
        *freq.entry(ch).or_insert(0) += 1;
    }

    let mut heap = BinaryHeap::new();
    for (&i, &freq) in &freq {
        heap.push(Reverse(Box::new(TreeNode {
            w: freq,
            val: Some(i),
            left: None,
            right: None,
        })));
    }

    while heap.len() > 1 {
        let i = heap.pop().unwrap().0;
        let j = heap.pop().unwrap().0;
        let k = Box::new(TreeNode {
            w: i.w + j.w,
            val: None,
            left: Some(i),
            right: Some(j),
        });
        heap.push(Reverse(k));
    }

    let root = heap.pop().unwrap().0;
    let mut symbol_codes = BTreeMap::new();

    fn fill_symbol_codes(
        root: Box<TreeNode>,
        symbol_codes: &mut BTreeMap<char, String>,
        code: &mut String,
    ) {
        match root.val {
            Some(v) => {
                if code.is_empty() {
                    symbol_codes.insert(v, "0".to_string());
                } else {
                    symbol_codes.insert(v, code.to_string());
                }
            }
            None => {
                fill_symbol_codes(
                    root.left.unwrap(),
                    symbol_codes,
                    &mut format!("{}{}", code, 0),
                );
                fill_symbol_codes(
                    root.right.unwrap(),
                    symbol_codes,
                    &mut format!("{}{}", code, 1),
                );
            }
        }
    }

    fill_symbol_codes(root, &mut symbol_codes, &mut "".to_string());

    let mut ans = String::new();
    for ch in input {
        ans.push_str(symbol_codes.get(&ch).unwrap())
    }
    println!("{} {}", symbol_codes.len(), ans.len());
    for (k, v) in &symbol_codes {
        println!("{}: {}", k, v);
    }
    println!("{}", ans);
}

fn huffman_decode() {
    use std::collections::*;
    let mut input = String::new();

    let stdin = std::io::stdin();
    stdin.read_line(&mut input).unwrap();

    let x = input
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    input.clear();

    let (n, _len) = (x[0], x[1]);
    let mut codes = HashMap::new();

    for _ in 0..n {
        stdin.read_line(&mut input).unwrap();
        let x = input.split(':').map(|x| x.trim()).collect::<Vec<_>>();
        let (ch, code) = (
            x[0].chars().take(1).collect::<Vec<_>>().pop().unwrap(),
            x[1],
        );
        codes.insert(code.to_string(), ch);
        input.clear();
    }

    stdin.read_line(&mut input).unwrap();
    let input = input.trim().chars().collect::<Vec<_>>();

    let mut cur = String::new();
    for ch in input {
        cur.push(ch);
        if codes.contains_key(&cur) {
            print!("{:?}", codes.get(&cur).unwrap());
            cur.clear();
        }
    }
}

fn knapsack_solve(mut w: i32, mut knapsack: Vec<(i32, i32)>) -> f64 {
    use std::cmp::*;
    knapsack.sort_by(|a, b| {
        if a.1 == 0 {
            return Ordering::Less;
        }
        if b.1 == 0 {
            return Ordering::Greater;
        }
        let ac = a.0 as f64 / a.1 as f64;
        let bc = b.0 as f64 / b.1 as f64;
        ac.partial_cmp(&bc).unwrap()
    });
    let mut ans = 0f64;
    for i in knapsack.into_iter().rev() {
        if w >= i.1 {
            w -= i.1;
            ans += i.0 as f64;
        } else {
            let c = (i.0 as f64 / i.1 as f64) * w as f64;
            w = 0;
            ans += c;
        }
    }
    ans
}

fn knapsack() {
    let mut input = String::new();

    let stdin = std::io::stdin();
    stdin.read_line(&mut input).unwrap();

    let x = input
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    input.clear();

    let (n, w) = (x[0], x[1]);
    let mut knapsack = vec![];
    for _ in 0..n {
        stdin.read_line(&mut input).unwrap();
        let x = input
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let (c, w) = (x[0], x[1]);
        knapsack.push((c, w));
        input.clear();
    }

    println!("{:.3}", knapsack_solve(w, knapsack));
}

fn intersections_solve(mut intervals: Vec<(i32, i32)>) -> Vec<i32> {
    intervals.sort_by(|a, b| a.1.cmp(&b.1));
    let mut points = vec![];
    points.push(intervals[0].1);
    for i in intervals.into_iter().skip(1) {
        let last_point = *points.last().unwrap();
        if last_point >= i.0 && last_point <= i.1 {
            continue;
        }
        points.push(i.1);
    }
    points
}

fn intersections() {
    let mut input = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut input).unwrap();

    let n = input.trim().parse::<i32>().unwrap();
    input.clear();

    let mut intervals = vec![];
    for _ in 0..n {
        stdin.read_line(&mut input).unwrap();
        let x = input
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let (s, e) = (x[0], x[1]);
        intervals.push((s, e));
        input.clear();
    }
    let ans = intersections_solve(intervals);
    println!("{:?}", ans.len());
    println!(
        "{:?}",
        ans.into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

fn solve_terms(mut k: i32) -> Vec<i32> {
    let mut ans = vec![];
    let mut i = 1;
    while k - i > i {
        ans.push(i);
        k -= i;
        i += 1;
    }
    ans.push(k);
    ans
}

fn terms() {
    let mut input = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut input).unwrap();

    let k = input.trim().parse::<i32>().unwrap();
    input.clear();
    let ans = solve_terms(k);
    println!("{}", ans.len());
    println!(
        "{}",
        ans.into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

fn solve_bin_search(arr: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
    queries
        .into_iter()
        .map(|query| -> i32 {
            let mut start = 0;
            let mut end = arr.len() as i32 - 1;

            while start <= end {
                let mid = start + (end - start) / 2;
                if arr[mid as usize] == query {
                    return mid as i32 + 1;
                } else if arr[mid as usize] < query {
                    start = mid + 1;
                } else {
                    end = mid - 1;
                }
            }

            return -1;
        })
        .collect::<Vec<_>>()
}

fn bin_search() {
    let mut input = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut input).unwrap();
    let arr = input
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    input.clear();
    stdin.read_line(&mut input).unwrap();
    let queries = input
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    input.clear();
    println!(
        "{}",
        solve_bin_search(arr, queries)
            .into_iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

fn counting_sort() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.clear();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut vv = vec![0usize; 11];
    line.split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .for_each(|x| vv[x] += 1);
    for i in 1..=10 {
        while vv[i] > 0 {
            print!("{} ", i);
            vv[i] -= 1;
        }
    }
    println!();
}

fn solve_merge_sort(arr: &mut [i32]) {
    fn merge(
        arr: &mut [i32],
        tmp: &mut [i32],
        left: usize,
        mid: usize,
        right: usize,
        count: &mut usize,
    ) {
        println!("sorting {} {} {}", left, mid, right);
        let mut i = left;
        let mut j = mid;

        for k in left..right {
            if j == right || (i < mid && arr[i] <= arr[j]) {
                tmp[k] = arr[i];
                i += 1;
            } else {
                *count += mid - i;
                tmp[k] = arr[j];
                j += 1;
            }
        }
        arr[left..right].copy_from_slice(&tmp[left..right]);
    }

    fn merge_sort_rec(arr: &mut [i32], tmp: &mut [i32], start: i32, end: i32, count: &mut usize) {
        if end <= start + 1 {
            println!("Exit");
            return;
        }
        let mid = (start + end) / 2;

        merge_sort_rec(arr, tmp, start, mid, count);
        merge_sort_rec(arr, tmp, mid, end, count);

        merge(arr, tmp, start as usize, mid as usize, end as usize, count);
    }

    fn merge_sort(arr: &mut [i32]) -> usize {
        let mut tmp = vec![0; arr.len()];
        let mut count = 0;
        merge_sort_rec(arr, &mut tmp, 0, arr.len() as i32, &mut count);
        count
    }
    println!("{}", merge_sort(arr));
}

fn merge_sort() {
    let mut line = String::new();

    let input = std::io::stdin();
    input.read_line(&mut line).unwrap();
    let _ = line.trim().parse::<usize>().unwrap();
    line.clear();
    input.read_line(&mut line).unwrap();
    let mut arr = line
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    solve_merge_sort(&mut arr);
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    fn test226() {
        problem226().unwrap();
    }

    // #[test]
    fn test228() {
        problem228().unwrap();
    }

    #[test]
    fn test229() {
        // problem_pq_max();

        let mut max_queue = MaxQueue::new();
        max_queue.insert(2);
        max_queue.insert(3);
        max_queue.insert(18);
        max_queue.insert(15);
        max_queue.insert(18);
        max_queue.insert(12);
        max_queue.insert(12);
        max_queue.insert(2);
        println!("{}", max_queue.extract_max());
        println!("{}", max_queue.extract_max());
        println!("{}", max_queue.extract_max());
        println!("{}", max_queue.extract_max());
        println!("{}", max_queue.extract_max());
        println!("{}", max_queue.extract_max());
        println!("{}", max_queue.extract_max());
        println!("{}", max_queue.extract_max());
    }

    #[test]
    fn test_huffman() {
        // huffman_encode();
        huffman_decode();
    }

    #[test]
    fn test_huffman_mod() {
        println!("{:?}", huffman::encode("a"));
        println!("{:?}", huffman::encode("adsads"));
    }

    #[test]
    fn test_knapsack() {
        println!(
            "{:.3}",
            knapsack_solve(50, vec![(60, 20), (100, 50), (120, 30)])
        );
    }

    #[test]
    fn test_intersections() {
        println!(
            "{:?}",
            intersections_solve(vec![(4, 7), (1, 3), (2, 5), (5, 6)])
        );
    }

    #[test]
    fn test_terms() {
        println!("{:?}", solve_terms(12));
    }

    #[test]
    fn test_binsearch() {
        println!(
            "{:?}",
            solve_bin_search(vec![1, 5, 8, 12, 13], vec![8, 1, 23, 1, 11])
        );
    }
    #[test]
    fn test_merge_sort() {
        solve_merge_sort(&mut vec![2, 3, 9, 2, 9]);
        solve_merge_sort(&mut vec![5, 4, 3, 2, 1]);
    }
}
