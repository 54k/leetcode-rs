fn task_1_1() {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let res = task_1_1_solver(buf);
    if let Err(e) = res {
        println!("{}", e);
    } else {
        println!("{}", res.unwrap());
    }
}

fn task_1_1_solver(str: String) -> Result<String, i32> {
    let mut stack = vec![];
    let str = str.chars().collect::<Vec<_>>();
    for (i, &ch) in str.iter().enumerate() {
        match ch {
            '(' => {
                stack.push((')', i + 1));
            }
            '{' => {
                stack.push(('}', i + 1));
            }
            '[' => {
                stack.push((']', i + 1));
            }
            ')' | ']' | '}' => {
                if let Some((c, _)) = stack.pop() {
                    if c != ch {
                        return Err(i as i32 + 1);
                    }
                } else {
                    return Err(i as i32 + 1);
                }
            }
            _ => {}
        }
    }
    if stack.is_empty() {
        Ok("Success".to_string())
    } else {
        Err(stack.last().unwrap().1 as i32)
    }
}

fn task_1_2() {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    println!("{}", task_1_2_solver(buf.trim().to_string()));
}

fn task_1_2_solver(buf: String) -> i32 {
    let parents: Vec<i32> = buf
        .split(' ')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    fn calc_height(root: usize, adj: &Vec<Vec<usize>>) -> i32 {
        let mut height = 1;
        for &c in adj[root].iter() {
            height = height.max(1 + calc_height(c, adj));
        }
        height
    }
    let mut root = None;
    let mut adj = vec![vec![]; parents.len()];
    for (i, &p) in parents.iter().enumerate() {
        if p > -1 {
            adj[p as usize].push(i);
        } else {
            root = Some(i);
        }
    }
    calc_height(root.unwrap(), &adj)
}

fn task_1_3() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let v = buf
        .trim()
        .split(' ')
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let buf_capacity = v[0];
    let packets_len = v[1];
    buf.clear();
    let mut arrivals = vec![];
    let mut durations = vec![];
    for _ in 0..packets_len {
        std::io::stdin().read_line(&mut buf).unwrap();
        let v = buf
            .trim()
            .split(' ')
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        buf.clear();
        arrivals.push(v[0]);
        durations.push(v[1]);
    }
    let result = task_1_3_solver(buf_capacity as usize, arrivals, durations);
    if !result.is_empty() {
        for x in result {
            println!("{}", x);
        }
    }
}

fn task_1_3_solver(size: usize, arrival: Vec<i32>, duration: Vec<i32>) -> Vec<i32> {
    use std::collections::VecDeque;
    let mut queue = VecDeque::new();
    let mut res = vec![];
    let mut packets = vec![]; // (arrival, duration)
    for i in 0..duration.len() {
        packets.push((arrival[i], duration[i]));
    }
    for p in packets {
        if queue.len() == size {
            if queue.front().copied().unwrap() <= p.0 {
                queue.pop_front();
            } else {
                res.push(-1);
                continue;
            }
        }

        let release_time = queue.back().copied().unwrap_or(0);
        let start_time = release_time.max(p.0);
        res.push(start_time);
        queue.push_back(start_time + p.1);
    }
    res
}

fn task_1_4() {
    struct Node {
        val: i32,
        max: i32,
        next: Option<Box<Node>>,
    }
    struct MaxStack {
        top: Option<Box<Node>>,
    }
    impl MaxStack {
        fn new() -> Self {
            Self { top: None }
        }
        fn push(&mut self, val: i32) {
            let mut node = Node {
                val,
                max: val,
                next: None,
            };
            if let Some(t) = self.top.take() {
                node.max = node.max.max(t.max);
                node.next = Some(t);
            }
            self.top = Some(Box::new(node));
        }
        fn pop(&mut self) -> i32 {
            self.top
                .take()
                .map(|x| {
                    self.top = x.next;
                    x.val
                })
                .unwrap_or(0)
        }
        fn max(&self) -> i32 {
            self.top.as_ref().map(|x| x.max).unwrap_or(0)
        }
    }
    let mut stack = MaxStack::new();
    let mut ans = vec![];
    let stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<i32>().unwrap();
    buf.clear();
    for _ in 0..n {
        stdin.read_line(&mut buf).unwrap();
        let args = buf.trim().split(' ').collect::<Vec<_>>();
        match args[0] {
            "max" => {
                ans.push(stack.max());
            }
            "push" => {
                stack.push(args[1].parse::<i32>().unwrap());
            }
            "pop" => {
                stack.pop();
            }
            _ => panic!("Unknown arg"),
        };
        buf.clear();
    }

    ans.into_iter().for_each(|x| {
        println!("{}", x);
    });
}

fn task_1_5() {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let arr = buf
        .trim()
        .split(' ')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let m = buf.trim().parse::<usize>().unwrap();
    buf.clear();
    println!(
        "{}",
        task_1_5_solver(arr, m)
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

fn task_1_5_solver(arr: Vec<i32>, m: usize) -> Vec<i32> {
    use std::collections::VecDeque;
    let mut ans = vec![];
    let mut queue: VecDeque<(i32, usize)> = VecDeque::new();
    for i in 0..arr.len() {
        while !queue.is_empty() && queue.back().unwrap().0 < arr[i] {
            queue.pop_back();
        }
        queue.push_back((arr[i], i));
        if i == m - 1 {
            ans.push(queue.front().unwrap().0);
        }
        if i >= m {
            if queue.front().unwrap().1 == (i - m) {
                queue.pop_front();
            }
            ans.push(queue.front().unwrap().0);
        }
    }
    ans
}

fn task_2_1() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf)?;
    let n = buf.trim().parse::<i32>()?;
    buf.clear();
    stdin.read_line(&mut buf)?;
    let mut arr = buf
        .trim()
        .split(' ')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let swaps = task_2_1_solver(&mut arr);
    println!("{}", swaps.len());
    for swap in swaps {
        println!("{} {}", swap.0, swap.1);
    }
    Ok(())
}

fn task_2_1_solver(arr: &mut Vec<i32>) -> Vec<(i32, i32)> {
    fn sift_down(arr: &mut Vec<i32>, mut k: usize) -> Vec<(i32, i32)> {
        let mut swaps = vec![];
        let n = arr.len() - 1;
        while 2 * k < n {
            let mut j = 2 * k + 1;
            if j < n && arr[j + 1] <= arr[j] {
                j += 1;
            }
            if arr[k] <= arr[j] {
                break;
            }
            swaps.push((k as i32, j as i32));
            arr.swap(k, j);
            k = j;
        }
        swaps
    }
    let mut ans = vec![];
    for i in (0..=(arr.len() - 1) / 2).rev() {
        ans.extend(sift_down(arr, i));
    }
    ans
}

fn task_2_2() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf)?;
    let v = buf
        .trim()
        .split(' ')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    buf.clear();
    let (n, m) = (v[0], v[1]);
    stdin.read_line(&mut buf)?;
    let v = buf
        .trim()
        .split(' ')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let res = task_2_2_solver(n, v);
    for r in res {
        println!("{} {}", r.0, r.1);
    }
    Ok(())
}

fn task_2_2_solver(n: i32, tasks: Vec<i64>) -> Vec<(i32, i64)> {
    struct Heap(Vec<(i64, i32)>);
    impl Heap {
        fn new(n: i32) -> Self {
            let mut v = vec![(-1, -1)];
            for i in 0..n {
                v.push((0i64, i));
            }
            Self(v)
        }
        fn push(&mut self, val: (i64, i32)) {
            let arr = &mut self.0;
            arr.push(val);
            let n = arr.len() - 1;
            self.sift_up(n);
        }
        fn pop(&mut self) -> (i64, i32) {
            let arr = &mut self.0;
            let res = arr[1];
            let n = arr.len() - 1;
            arr[1] = arr[n];
            arr.pop();
            self.sift_down(1);
            res
        }
        fn sift_up(&mut self, mut i: usize) {
            let arr = &mut self.0;
            while i > 1 && arr[i] < arr[i / 2] {
                arr.swap(i / 2, i);
                i /= 2;
            }
        }
        fn sift_down(&mut self, mut i: usize) {
            let arr = &mut self.0;
            let n = arr.len() - 1;
            while 2 * i < n {
                let mut j = 2 * i;
                if j < n && arr[j + 1] <= arr[j] {
                    j += 1;
                }
                if arr[i] <= arr[j] {
                    break;
                }
                arr.swap(i, j);
                i = j;
            }
        }
    }
    let mut ans = vec![];
    let mut heap = Heap::new(n);
    for t in tasks {
        let (time, processor) = heap.pop();
        ans.push((processor, time)); // num of processor, task time
        heap.push((time + t, processor));
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1_1() {
        println!("{:?}", task_1_1_solver("foo(bar[i);".to_string())); // 10
        println!("{:?}", task_1_1_solver("foo(bar[i]);".to_string())); // Success
        println!("{:?}", task_1_1_solver("()[]}".to_string())); // 5
        println!("{:?}", task_1_1_solver("()[]}a".to_string())); // 5
        println!("{:?}", task_1_1_solver("()[](a".to_string())); // 5
    }

    #[test]
    fn test_1_2() {
        println!("{}", task_1_2_solver("4 -1 4 1 1".to_string())); // 3
        println!("{}", task_1_2_solver("-1 0 4 0 3".to_string())); // 4
        println!("{}", task_1_2_solver("9 7 5 5 2 9 9 9 2 -1".to_string())); // 4
    }

    #[test]
    fn test_1_3() {
        println!("{:?}", task_1_3_solver(1, vec![0, 0], vec![1, 1])); // [0, -1]
        println!("{:?}", task_1_3_solver(1, vec![0, 1], vec![1, 1])); // [0, 1]
        println!("{:?}", task_1_3_solver(1, vec![0, 1], vec![1, 1])); // [0, 1]

        println!(
            "{:?}",
            task_1_3_solver(
                1,
                vec![
                    16, 29, 44, 58, 72, 88, 95, 108, 123, 139, 152, 157, 169, 183, 192, 202, 213,
                    229, 232, 236, 239, 247, 251, 267, 275
                ],
                vec![0, 3, 6, 0, 2, 8, 7, 6, 9, 6, 6, 3, 3, 1, 0, 8, 8, 3, 3, 3, 4, 8, 2, 7, 7]
            )
        ); // 16 29 44 58 72 88 -1 108 123 139 152 -1 169 183 192 202 213 229 232 236 239 247 -1 267 275

        println!(
            "{:?}",
            task_1_3_solver(
                11,
                vec![
                    6, 15, 24, 25, 33, 47, 58, 65, 70, 79, 93, 103, 110, 123, 138, 144, 159, 167,
                    179, 182, 186, 198, 208, 222, 235
                ],
                vec![
                    23, 44, 28, 15, 7, 41, 25, 5, 14, 8, 43, 11, 25, 27, 40, 19, 2, 23, 43, 31, 7,
                    16, 41, 23, 26
                ]
            )
        ); // 6 29 73 101 116 123 164 189 194 208 216 259 270 295 322 362 -1 381 -1 -1 -1 404 420 461 484
    }

    #[test]
    fn test_1_5() {
        println!("{:?}", task_1_5_solver(vec![2, 7, 3, 1, 5, 2, 6, 2], 4)); // 7, 7, 5, 6, 6
        println!("{:?}", task_1_5_solver(vec![2, 1, 5], 1)); // 2, 1, 5
        println!("{:?}", task_1_5_solver(vec![2, 3, 9], 3)); // 9
        println!("{:?}", task_1_5_solver(vec![4, 3, 2, 1], 1)); // 4, 3, 2, 1
    }

    #[test]
    fn test_2_1() {
        let mut arr = vec![5, 4, 3, 2, 1]; // 1-index based
        println!("{:?}", task_2_1_solver(&mut arr));
        println!("{:?}", arr);
    }

    #[test]
    fn test_2_2() {
        println!("{:?}", task_2_2_solver(2, vec![1, 2, 3, 4, 5]));
        println!("{:?}", task_2_2_solver(4, vec![1; 20]));
    }
}
