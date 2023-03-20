use crate::grind_169::week_4::kth_smallest;
use std::iter::Map;

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

fn task_2_3() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf)?;
    let n_m = buf
        .trim()
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    buf.clear();
    stdin.read_line(&mut buf)?;
    let tables = buf
        .trim()
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    buf.clear();
    let mut queries = vec![];
    for _ in 0..n_m[1] {
        stdin.read_line(&mut buf)?;
        let q = buf
            .trim()
            .split(' ')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        queries.push((q[0], q[1]));
        buf.clear()
    }
    stdin.read_line(&mut buf)?;
    task_2_3_solver(tables, queries).into_iter().for_each(|x| {
        println!("{}", x);
    });
    Ok(())
}

fn task_2_3_solver(tables: Vec<usize>, queries: Vec<(usize, usize)>) -> Vec<usize> {
    struct UF {
        parents: Vec<usize>,
        rank: Vec<usize>,
        max_rank: usize,
    }
    impl UF {
        fn new() -> Self {
            Self {
                parents: vec![],
                rank: vec![],
                max_rank: 0,
            }
        }
        fn make_set(&mut self, i: usize, s: usize) {
            self.parents.push(i);
            self.rank.push(s);
            self.max_rank = self.max_rank.max(s);
        }
        fn find(&mut self, i: usize) -> usize {
            if i != self.parents[i] {
                self.parents[i] = self.find(self.parents[i]);
            }
            self.parents[i]
        }
        fn union(&mut self, i: usize, j: usize) -> usize {
            let mut p_i = self.find(i);
            let mut p_j = self.find(j);
            if p_i == p_j {
                return self.max_rank;
            }
            if self.rank[p_i] < self.rank[p_j] {
                std::mem::swap(&mut p_i, &mut p_j);
            }
            self.rank[p_i] += self.rank[p_j];
            self.max_rank = self.max_rank.max(self.rank[p_i]);
            self.parents[p_j] = p_i;
            self.max_rank
        }
    }
    let mut uf = UF::new();
    for (i, s) in tables.into_iter().enumerate() {
        uf.make_set(i, s);
    }
    let mut ans = vec![];
    for (a, b) in queries {
        ans.push(uf.union(a - 1, b - 1));
    }
    ans
}

fn task_2_4() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf)?;
    let n_e_d = buf
        .trim()
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    buf.clear();
    let mut joins = vec![];
    for _ in 0..n_e_d[1] {
        stdin.read_line(&mut buf)?;
        let pair = buf
            .trim()
            .split(' ')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        joins.push((pair[0], pair[1]));
        buf.clear()
    }
    let mut disjoints = vec![];
    for _ in 0..n_e_d[2] {
        stdin.read_line(&mut buf)?;
        let pair = buf
            .trim()
            .split(' ')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        disjoints.push((pair[0], pair[1]));
        buf.clear()
    }
    stdin.read_line(&mut buf)?;
    println!("{}", (task_2_4_solver(n_e_d[0], joins, disjoints) as i32));
    Ok(())
}

fn task_2_4_solver(n: usize, joins: Vec<(usize, usize)>, disjoints: Vec<(usize, usize)>) -> bool {
    // Решение:
    // - строим parent для переменных на самих себя
    //     - определим две функции
    //     - корень(x) -- возвращает окончательный парент (корень) для x
    //     - свернуть(x, p) -- заменяет всю цепочку от x до корня x на значение p
    //     - для всех e -- корнем j сворачиваем ветки i и j
    //     - для всех d -- ищем равенство корней; выходим с 0, если нашли
    //     - пишем 1, если до этого не вышли с нулем
    struct UF(Vec<usize>);
    impl UF {
        fn new(n: usize) -> Self {
            let mut links = vec![];
            for i in 0..n {
                links.push(i);
            }
            Self(links)
        }
        fn find(&mut self, i: usize) -> usize {
            if i != self.0[i] {
                self.0[i] = self.find(self.0[i]);
            }
            self.0[i]
        }
        fn same(&mut self, i: usize, j: usize) -> bool {
            self.find(i) == self.find(j)
        }
        fn union(&mut self, i: usize, j: usize) {
            let i = self.find(i);
            let j = self.find(j);
            if i == j {
                return;
            }
            self.0[i] = j
        }
    }
    let mut uf = UF::new(n);
    for (i, j) in joins {
        uf.union(i - 1, j - 1);
    }
    for (i, j) in disjoints {
        if uf.same(i - 1, j - 1) {
            return false;
        }
    }
    true
}

fn task_3_1() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf)?;
    let queries_len = buf.trim().parse::<usize>().unwrap();
    buf.clear();
    let mut queries = vec![];
    for _ in 0..queries_len {
        stdin.read_line(&mut buf)?;
        queries.push(buf.trim().split(' ').map(|x| x.to_string()).collect());
        buf.clear();
    }
    for x in task_3_1_solver(queries) {
        println!("{}", x);
    }
    Ok(())
}

fn task_3_1_solver(queries: Vec<Vec<String>>) -> Vec<String> {
    #[derive(Debug, Clone)]
    struct Entry(usize, String);
    struct PhoneBook(Vec<Option<Entry>>);
    impl PhoneBook {
        fn new(size: usize) -> Self {
            Self(vec![None; size])
        }
        fn add(&mut self, num: usize, name: String) {
            let idx = self.find_idx(num);
            let _ = self.0[idx].insert(Entry(num, name));
        }
        fn del(&mut self, num: usize) {
            let idx = self.find_idx(num);
            self.0[idx].take();
        }
        fn find(&self, num: usize) -> Option<&String> {
            let idx = self.find_idx(num);
            self.0[idx].as_ref().map(|x| &x.1)
        }
        fn find_idx(&self, num: usize) -> usize {
            let mut idx = self.get_idx(num);
            while self.0[idx].is_some() && self.0[idx].as_ref().unwrap().0 != num {
                idx = (idx + 1) % self.0.len();
            }
            idx
        }
        fn get_idx(&self, num: usize) -> usize {
            num % self.0.len()
        }
    }
    let mut query_answer = vec![];
    let mut phone_book = PhoneBook::new(10_000_000);
    for query in queries {
        match query[0].as_str() {
            "add" => {
                phone_book.add(query[1].parse::<usize>().unwrap(), query[2].to_string());
            }
            "del" => {
                phone_book.del(query[1].parse::<usize>().unwrap());
            }
            "find" => {
                if let Some(t) = phone_book.find(query[1].parse::<usize>().unwrap()) {
                    query_answer.push(t.clone());
                } else {
                    query_answer.push("not found".to_string());
                }
            }
            _ => panic!("Unknown command"),
        }
    }
    query_answer
}

fn task_4_1() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf)?;
    let m = buf.trim().parse::<usize>().unwrap();
    buf.clear();
    stdin.read_line(&mut buf)?;
    let queries_len = buf.trim().parse::<usize>().unwrap();
    buf.clear();
    let mut queries = vec![];
    for _ in 0..queries_len {
        stdin.read_line(&mut buf)?;
        queries.push(buf.trim().split(' ').map(|x| x.to_string()).collect());
        buf.clear();
    }
    for x in task_4_1_solver(m, queries) {
        println!("{}", x);
    }
    Ok(())
}

fn task_4_1_solver(m: usize, queries: Vec<Vec<String>>) -> Vec<String> {
    use std::ptr::null_mut;
    #[derive(Debug, Clone)]
    struct DLLNode {
        val: String,
        prev: *mut DLLNode,
        next: *mut DLLNode,
    }
    impl DLLNode {
        fn evict(&mut self) {
            unsafe {
                let prev = self.prev;
                let next = self.next;
                (*prev).next = next;
                (*next).prev = prev;
            }
        }
    }
    #[derive(Debug, Clone)]
    struct DLL {
        head: *mut DLLNode,
        tail: *mut DLLNode,
    }
    impl DLL {
        fn new() -> Self {
            unsafe {
                let dummy_head = Box::into_raw(Box::new(DLLNode {
                    val: "head".to_string(),
                    prev: null_mut(),
                    next: null_mut(),
                }));
                let dummy_tail = Box::into_raw(Box::new(DLLNode {
                    val: "tail".to_string(),
                    prev: null_mut(),
                    next: null_mut(),
                }));
                (*dummy_head).prev = dummy_head;
                (*dummy_head).next = dummy_tail;
                (*dummy_tail).prev = dummy_head;
                (*dummy_tail).next = dummy_tail;
                Self {
                    head: dummy_head,
                    tail: dummy_tail,
                }
            }
        }
        fn find(&self, key: &String) -> *mut DLLNode {
            unsafe {
                let mut node = self.head;
                while &(*node).val != key && node != self.tail {
                    node = (*node).next
                }
                node
            }
        }
        fn push_front(&self, key: String) {
            unsafe {
                let next = (*self.head).next;
                let new_node = Box::into_raw(Box::new(DLLNode {
                    val: key,
                    prev: self.head,
                    next,
                }));
                (*self.head).next = new_node;
                (*next).prev = new_node;
            }
        }
    }
    struct Mapping {
        buckets: Vec<DLL>,
    }
    impl Mapping {
        fn new(m: usize) -> Self {
            Self {
                buckets: vec![DLL::new(); m],
            }
        }
        fn add(&mut self, key: String) {
            if self.find(&key) == *"yes" {
                return;
            }
            self.buckets[self.hash(&key)].push_front(key);
        }
        fn del(&mut self, key: String) {
            unsafe {
                let found = self.buckets[self.hash(&key)].find(&key);
                if (*found).val == key {
                    (*found).evict()
                }
            }
        }
        fn check(&mut self, i: usize) -> String {
            unsafe {
                let mut result = vec![];
                let mut next = (*self.buckets[i].head).next;
                while next != self.buckets[i].tail {
                    result.push((*next).val.as_str());
                    next = (*next).next;
                }
                result.join(" ")
            }
        }
        fn find(&self, key: &String) -> String {
            unsafe {
                let node = self.buckets[self.hash(key)].find(key);
                if &(*node).val == key {
                    "yes".to_string()
                } else {
                    "no".to_string()
                }
            }
        }
        fn hash(&self, key: &str) -> usize {
            const X: usize = 263;
            const MOD: usize = 1000000007;
            let mut hash = 0;
            let mut factor = 1;
            let key = key.chars().map(|x| x as usize).collect::<Vec<usize>>();

            for i in 0..key.len() {
                hash = (hash % MOD + ((key[i] * factor) % MOD)) % MOD;
                factor = (factor * X) % MOD;
            }
            hash % self.buckets.len()
        }
    }

    let mut query_ans = vec![];
    let mut mapping = Mapping::new(m);
    for query in queries {
        match query[0].as_str() {
            "add" => {
                mapping.add(query[1].to_string());
            }
            "del" => {
                mapping.del(query[1].to_string());
            }
            "check" => query_ans.push(mapping.check(query[1].parse::<usize>().unwrap())),
            "find" => query_ans.push(mapping.find(&query[1].to_string())),
            _ => panic!(),
        }
    }
    query_ans
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

    #[test]
    fn test_2_3() {
        println!(
            "{:?}",
            task_2_3_solver(
                vec![1, 1, 1, 1, 1],
                vec![(3, 5), (2, 4), (1, 4), (5, 4), (5, 3)],
            )
        );
    }

    #[test]
    fn test_2_4() {
        println!(
            "{}",
            task_2_4_solver(
                6,
                vec![(2, 3), (1, 5), (2, 5), (3, 4), (4, 2)],
                vec![(6, 1), (4, 6), (4, 5)],
            )
        );
    }

    #[test]
    fn test_3_1() {
        println!(
            "{:?}",
            task_3_1_solver(vec![
                vec!["add".to_string(), "911".to_string(), "police".to_string()],
                vec!["add".to_string(), "76213".to_string(), "Mom".to_string()],
                vec!["add".to_string(), "17239".to_string(), "Bob".to_string()],
                vec!["find".to_string(), "76213".to_string()],
                vec!["find".to_string(), "910".to_string()],
                vec!["find".to_string(), "911".to_string()],
                vec!["del".to_string(), "910".to_string()],
                vec!["del".to_string(), "911".to_string()],
                vec!["find".to_string(), "911".to_string()],
                vec!["find".to_string(), "76213".to_string()],
                vec!["add".to_string(), "76213".to_string(), "daddy".to_string()],
                vec!["find".to_string(), "76213".to_string()],
            ])
        );

        println!(
            "{:?}",
            task_3_1_solver(vec![
                vec!["add".to_string(), "5".to_string(), "police".to_string()],
                vec!["add".to_string(), "11".to_string(), "Mom".to_string()],
                vec!["add".to_string(), "17".to_string(), "Bob".to_string()],
                vec!["find".to_string(), "17".to_string()],
                vec!["find".to_string(), "11".to_string()],
                vec!["find".to_string(), "5".to_string()],
            ])
        );
    }

    #[test]
    fn test_4_1() {
        // println!(
        //     "{:?}",
        //     task_4_1_solver(
        //         25,
        //         vec![
        //             vec!["add".to_string(), "qaxndhusptgrewo".to_string()],
        //             vec!["check".to_string(), "7".to_string()],
        //         ]
        //     )
        // );
        //
        // println!(
        //     "{:?}",
        //     task_4_1_solver(
        //         250,
        //         vec![
        //             vec!["add".to_string(), "uiljkwhypgmfdst".to_string()],
        //             vec!["check".to_string(), "72".to_string()],
        //         ]
        //     )
        // );
        //
        // println!(
        //     "{:?}",
        //     task_4_1_solver(
        //         2500,
        //         vec![
        //             vec!["add".to_string(), "pweiknqgcxazjyh".to_string()],
        //             vec!["check".to_string(), "263".to_string()],
        //         ]
        //     )
        // );
        //
        // println!(
        //     "{:?}",
        //     task_4_1_solver(
        //         25000,
        //         vec![
        //             vec!["add".to_string(), "ilvpygszwdeurjn".to_string()],
        //             vec!["check".to_string(), "9134".to_string()],
        //         ]
        //     )
        // );
        //
        // println!(
        //     "{:?}",
        //     task_4_1_solver(
        //         250000,
        //         vec![
        //             vec!["add".to_string(), "xnzrvwcutfgbqje".to_string()],
        //             vec!["check".to_string(), "170995".to_string()],
        //         ]
        //     )
        // );
        //
        // println!(
        //     "{:?}",
        //     task_4_1_solver(
        //         25000000,
        //         vec![
        //             vec!["add".to_string(), "zhjutqslrpyfcoa".to_string()],
        //             vec!["check".to_string(), "9581368".to_string()],
        //         ]
        //     )
        // );
        //
        // println!(
        //     "{:?}",
        //     task_4_1_solver(
        //         250000000,
        //         vec![
        //             vec!["add".to_string(), "kxlcfgpezjmuynv".to_string()],
        //             vec!["check".to_string(), "127141737".to_string()],
        //         ]
        //     )
        // );
        //
        // println!(
        //     "{:?}",
        //     task_4_1_solver(1, vec![vec!["check".to_string(), "0".to_string()],])
        // );
        //
        // println!(
        //     "{:?}",
        //     task_4_1_solver(
        //         5,
        //         vec![
        //             vec!["add".to_string(), "world".to_string()],
        //             vec!["add".to_string(), "HellO".to_string()],
        //             vec!["check".to_string(), "4".to_string()],
        //             vec!["find".to_string(), "World".to_string()],
        //             vec!["find".to_string(), "world".to_string()],
        //             vec!["del".to_string(), "world".to_string()],
        //             vec!["check".to_string(), "4".to_string()],
        //             vec!["del".to_string(), "HellO".to_string()],
        //             vec!["add".to_string(), "luck".to_string()],
        //             vec!["add".to_string(), "GooD".to_string()],
        //             vec!["check".to_string(), "2".to_string()],
        //             vec!["del".to_string(), "good".to_string()],
        //         ]
        //     )
        // );
        //
        // println!(
        //     "{:?}",
        //     task_4_1_solver(
        //         4,
        //         vec![
        //             vec!["add".to_string(), "test".to_string()],
        //             vec!["add".to_string(), "test".to_string()],
        //             vec!["find".to_string(), "test".to_string()],
        //             vec!["del".to_string(), "test".to_string()],
        //             vec!["find".to_string(), "test".to_string()],
        //             vec!["find".to_string(), "Test".to_string()],
        //             vec!["add".to_string(), "Test".to_string()],
        //             vec!["find".to_string(), "Test".to_string()],
        //         ]
        //     )
        // );

        println!(
            "{:?}",
            task_4_1_solver(
                4,
                vec![
                    vec!["check".to_string(), "0".to_string()],
                    vec!["find".to_string(), "help".to_string()],
                    vec!["add".to_string(), "help".to_string()],
                    vec!["add".to_string(), "del".to_string()],
                    vec!["add".to_string(), "add".to_string()],
                    vec!["find".to_string(), "add".to_string()],
                    vec!["find".to_string(), "del".to_string()],
                    vec!["del".to_string(), "del".to_string()],
                ]
            )
        );
    }
}
