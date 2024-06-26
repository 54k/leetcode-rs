// Требуется выдать запрошенную сумму купюрами в рублях начиная от более крупных к более мелким.
// Купюры существуют 50 руб, 100 руб, 500 руб, 1000 руб, 5000 руб.
// При этом число купюр ограничено, на вход даётся объект с количеством купюр.
// Этот объект нужно менять, чтобы количество купюр всегда было в актуальном состоянии.
// Если выдать заданную сумму нельзя — вывести сообщение об ошибке.
// Не давать решать через покупюрный набор (можно приводить пример когда запрашивают миллион,
// а в банкомате есть только полтинники).

// l = {
// 1000: 6,
// 500: 5,
// 100: 5,
// 50: 4
// }
//
// pprint(atm(1250, l))  # '1x1000 2x100 1x50'
// pprint(atm(1000000, l))  # 'Error: Not enough money'
// pprint(atm(2400, l))  # '2x1000 3x100 2x50'
// pprint(atm(6512, l))  # 'Error: Incorrect value'
// pprint(atm(50, l))  # '1x50'
// pprint(atm(50, l))  # 'Error: Not enough money'
// pprint(atm(5500, l))  # '3x1000 5x500'

use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap, HashSet};

// как решать:
// взять лимиты и отсортировать купюры
// идти от самой старшей купюры к младшей и вычитать из суммы, уменьшая счетчик купюр
// если не удалось набрать сумму вернуть ошибку
// вывести путь - завести словарь аналогичный лимитам и прибавлять кол-во выданных купюр
#[allow(dead_code)]
fn atm(mut sum: i32, limits: &mut HashMap<i32, i32>) -> String {
    const ERR_NO_MONEY: &str = "Error: Not enough money";
    const ERR_INVALID_AMOUNT: &str = "Error: Incorrect value";
    let mut sorted_nominals = limits.keys().copied().collect::<Vec<_>>();
    sorted_nominals.sort();

    let total_money: i32 = limits.iter().map(|(k, v)| k * v).sum();

    if sum % sorted_nominals[0] > 0 {
        return ERR_INVALID_AMOUNT.to_string();
    }
    if total_money < sum {
        return ERR_NO_MONEY.to_string();
    }

    let mut out = BTreeMap::new();
    for i in (0..sorted_nominals.len()).rev() {
        let nominal = sorted_nominals[i];
        let mut desired_cnt = sum / nominal;

        if desired_cnt == 0 {
            continue;
        }

        let remaining_cnt = *limits.get(&nominal).unwrap();

        if desired_cnt >= remaining_cnt {
            desired_cnt = remaining_cnt;
        }

        sum -= nominal * desired_cnt;
        *limits.get_mut(&nominal).unwrap() -= desired_cnt;
        out.insert(nominal, desired_cnt);
    }

    if sum > 0 {
        return ERR_NO_MONEY.to_string();
    }

    let mut ans = String::new();
    for (nominal, cnt) in out.iter().rev() {
        ans.push_str(&format!("{}x{} ", cnt, nominal));
    }
    ans
}

// У нас есть набор билетов вида:
//
// [
//     { 'from': 'London', 'to': 'Moscow' },
//     { 'from': 'NY', 'to': 'London' },
//     { 'from': 'Moscow', 'to': 'SPb' },
//     ...
// ]
//
// Из этих билетов можно построить единственный, неразрывный маршрут.
// Петель и повторов в маршруте нет.
//
// Нужно написать программу, которая возвращает билеты
// в порядке следования по маршруту.

// Как решать:
// граф представлен списком ребер без циклов и повторов,
// как вариант решения это топологическая сортировка такого графа
// создать список смежности, запустить дфс и вывести путь - довольно сложно
// можно так же найти стартовую точку и начиная с нее построить путь до конца
#[allow(dead_code)]
fn tickets(edges: Vec<(String, String)>) -> Vec<(String, String)> {
    #[allow(dead_code)]
    fn topological_sort(edges: Vec<(String, String)>) -> Vec<(String, String)> {
        fn dfs(
            v: &str,
            adj: &HashMap<String, Vec<String>>,
            ans: &mut Vec<(String, String)>,
            visited: &mut HashMap<String, bool>,
        ) {
            if *visited.get(v).unwrap() {
                return;
            }
            *visited.get_mut(v).unwrap() = true;
            if adj.contains_key(v) {
                for u in adj.get(v).unwrap() {
                    dfs(u, adj, ans, visited);
                    ans.push((v.to_string(), u.to_string()));
                }
            }
        }

        let mut adj = HashMap::new();
        let mut visited = HashMap::new();

        for (from, to) in edges {
            visited.insert(from.clone(), false);
            visited.insert(to.clone(), false);
            adj.entry(from).or_insert(vec![]).push(to);
        }

        let mut path = vec![];
        let v = visited.clone();
        let keys = v.keys();
        for v in keys {
            dfs(v, &adj, &mut path, &mut visited);
        }
        path.reverse();
        path
    }

    #[allow(dead_code)]
    fn select_start_walk_edges(edges: Vec<(String, String)>) -> Vec<(String, String)> {
        let mut adj = HashMap::new();
        let mut in_degree = HashMap::new();
        for (from, to) in edges {
            *in_degree.entry(to.clone()).or_insert(0) += 1;
            in_degree.entry(from.clone()).or_insert(0);
            adj.insert(from, to);
        }
        let mut start = String::new();
        for (k, v) in in_degree.iter() {
            if *v == 0 {
                start = k.clone();
            }
        }
        let mut path = vec![];
        while adj.contains_key(&start) {
            let next = adj.get(&start).unwrap();
            path.push((start.clone(), next.clone()));
            start = next.clone();
        }
        path
    }

    // трики яндексовский метод
    #[allow(dead_code)]
    fn left_look_right_look(edges: Vec<(String, String)>) -> Vec<(String, String)> {
        let mut from_dict = HashMap::new();
        let mut to_dict = HashMap::new();

        let mut left_route = vec![];
        let mut right_route = vec![];

        for i in 0..edges.len() {
            from_dict.insert(edges[i].0.clone(), edges[i].clone());
            to_dict.insert(edges[i].1.clone(), edges[i].clone());
        }

        let mut ticket = Some(edges[0].clone());

        while let Some(t) = ticket {
            right_route.push(t.clone());
            ticket = from_dict.remove(&t.1);
        }

        ticket = to_dict.remove(&edges[0].0);

        while let Some(t) = ticket {
            left_route.push(t.clone());
            ticket = to_dict.remove(&t.0);
        }
        left_route.reverse();

        left_route.extend(right_route);
        left_route.into_iter().collect()
    }

    topological_sort(edges)
}

// Группировка анаграмм.py
// https://leetcode.com/problems/group-anagrams/description/

// Дан массив строк, нужно сгруппировать в нем анаграммы.
// Слово X является анаграммой слова Y, если оно может быть
// получено из другого перестановкой букв.

// pprint(
// group_anagrams(
// ['сон', 'нос', 'сорт', 'трос', 'торт', 'рост']
// )
// )

// Как решать:
// анаграмму удобно представить словарем символов
// завести словарь с ключом словаря анаграммы и списком строк
// вывести значения
fn group_anagrams(words: Vec<String>) -> Vec<Vec<String>> {
    fn group_anagrams1(words: Vec<String>) -> Vec<Vec<String>> {
        let mut anagrams = HashMap::new();
        for word in words {
            let mut anagram_key = BTreeMap::new(); // руст такой руст, hashmap ведет себя как линкед мап
            for ch in word.chars() {
                *anagram_key.entry(ch).or_insert(0) += 1;
            }
            anagrams
                .entry(format!("{:?}", anagram_key))
                .or_insert(vec![])
                .push(word);
        }
        anagrams.values().cloned().collect()
    }
    fn group_anagrams2(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::*;
        let mut ans = HashMap::new();
        for s in strs {
            let mut hist = vec![0; 26];
            for c in s.chars() {
                hist[c as usize - 'a' as usize] += 1;
            }
            ans.entry(hist).or_insert(vec![]).push(s);
        }
        ans.into_iter().fold(vec![], |mut acc, (_, v)| {
            acc.push(v);
            acc
        })
    }
    group_anagrams2(words)
}

// Даны два массива a и b . Нужно найти минимум abs(a[i]-b[j]) :
// Нужно решение без дополнительной памяти.
// Решается сортировкой каждого массива в отдельности и параллельным проходом по массивам.
// Хорошо, если кандидат задаст вопрос про пустые массивы. Можно попросить в этом случае возвращать INT_MAX.
// Очень хорошо, если кандидат спросит про случай minDistance({INT_MAX}, {-1}) (например) и поймет,
// что ответ на задачу на самом деле не помещается в int.
// Как решать:
// уже расписано в описании задачи
// по движению указателей почему так - мы стараемся минимизировать расстояние между двумя элементами массива,
// поэтому каждый раз двигаем указатель массива с меньшим элементов к большему, тк это уменьшит расстояние
#[allow(dead_code)]
fn min_distance(mut a: Vec<i32>, mut b: Vec<i32>) -> i64 {
    a.sort();
    b.sort();
    let mut ans = i64::MAX;
    let mut i = 0;
    let mut j = 0;
    while i < a.len() && j < b.len() {
        ans = (a[i] as i64 - b[j] as i64).abs().min(ans);
        if a[i] < a[j] {
            i += 1;
        } else {
            j += 1;
        }
    }
    ans
}

// Сжатие строки (RLE) [не более 30 мин].py
// Задача 1.5 из первого раздела CTCI (Карьера программиста)
// Дана строка, состоящая из букв A-Z:
// "AAAABBBCCXYZDDDDEEEFFFAAAAAABBBBBBBBBBBBBBBBBBBBBBBBBBBB"
// Нужно написать функцию RLE, которая на выходе даст строку вида:
// "A4B3C2XYZD4E3F3A6B28"
// И сгенерирует любую ошибку, если на вход пришла невалидная строка.
//
// Пояснения:
// 1. Если символ встречается 1 раз, он остается без изменений
// 2. Если символ повторяется более 1 раза, к нему добавляется количество повторений

// Как решать:
// задача очень похожа на задачу слияния интервалов
// https://emre.me/coding-patterns/merge-intervals/
// собственно используется такой же подход и паттерн
#[allow(dead_code)]
fn rle(s: String) -> String {
    let s = s.chars().collect::<Vec<_>>();
    let mut ans = String::new();
    let mut cur = s[0];
    let mut cnt = 1;

    for ch in s.into_iter().skip(1) {
        if !ch.is_alphabetic() {
            panic!("invalid character");
        }
        if ch != cur {
            if cnt > 1 {
                ans.push_str(&format!("{}{}", cur, cnt));
            } else {
                ans.push(cur);
            }
            cnt = 1;
            cur = ch;
        } else {
            cnt += 1;
        }
    }
    if cnt > 1 {
        ans.push_str(&format!("{}{}", cur, cnt));
    } else {
        ans.push(cur);
    }
    ans
}

// Удалить повторяющиеся символы и получить лексикографически наименьшую строку.py
// Дана строка. Нужно удалить из нее повторяющиеся символы таким образом
// чтобы результирующая строка была лексикографически наименьшей
// как решать:
// задача похожа на 8.2.2.
// Ближайшие меньшие элементы из книги Лааконсена Олимпиадное программирование

// https://leetcode.com/problems/remove-duplicate-letters/description/

// Как решать:
// Поддерживая словарь количества букв,
// заводим стэк который трэкает последний добавленный в него элемент
// множество selected нужно для того чтобы понимать включен символ в результат (есть ли уже на стэке или нет)
// Если мы встречаем символ на стэке который больше следующего и мы можем его выкинуть ( то есть count в словаре > 0 ),
// то значение из стэка можно удалить пока не встретим элемент строго меньше текущего
// это похоже на задачу о нахождении ближайшего минимума для элементов массива
// тк каждый элемент в стэке строго меньше следующего - получается строка которая лексикографически меньше
#[allow(dead_code)]
fn remove_duplicate_letters(s: String) -> String {
    let mut count_map = HashMap::new();
    for c in s.chars() {
        *count_map.entry(c).or_insert(0) += 1;
    }
    let mut stack = vec![];
    let mut selected = HashSet::new();

    for c in s.chars() {
        *count_map.get_mut(&c).unwrap() -= 1;
        if !selected.contains(&c) {
            while !stack.is_empty()
                && count_map[&stack[stack.len() - 1]] > 0
                && stack[stack.len() - 1] as i32 > c as i32
            {
                selected.remove(&stack.pop().unwrap());
            }

            stack.push(c);
            selected.insert(c);
        }
    }

    stack.into_iter().collect()
}

// Даны три массива a , b , c и число x .
// Посчитать число троек  (i, j, k) , таких что
// a[i] + b[j] + c[k] == x
// Массивы можно модифицировать inplace.
// a = [1, 2, 3, 4, 5, 6, 7]
// b = [2, 3, 5, 7, 11]
// c = [1, 1, 2, 3, 5, 7]
// a_len = len(a)
// b_len = len(b)
// c_len = len(c)
//
// x = 15
// sorted arrays
// mem const, complexibility = n^2
// Find s - c[k] in top-to-tail merged a and b
// находим s - c[k] в отсортированных массивах

// как решать:
// примерно как 3sum c литкода только тут 3 массива и там 1
// задача сводится к задаче 2sum с отсортированными массивами где ты должен набрать необходимую сумму,
// используя два указателя, один стоит на минимальном элементе первого массива (в начале)
// второй на самом большем второго массива (в конце)
fn find_triplets_sum(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>, x: i32) -> i32 {
    let mut ans = 0;
    let a_len = a.len();
    let b_len = b.len();
    let c_len = c.len();

    for k in 0..c_len {
        let ck = c[k];
        let mut i = 0;
        let mut j = b_len as i32 - 1;

        while i < a_len && j >= 0 {
            let ai = a[i as usize];
            let bj = b[j as usize];

            let s = ai + bj + ck;

            match s.cmp(&x) {
                Ordering::Equal => {
                    // сумма набрана увеличиваем счетчик ответа
                    ans += 1;
                    println!("{}-{}-{} {}+{}+{}={} ans: {}", i, j, k, ai, bj, ck, x, ans);
                    i += 1;
                    j -= 1;
                }
                Ordering::Less => {
                    // не добрали сумму, пытаемся увеличить
                    i += 1;
                }
                Ordering::Greater => {
                    // перебор, пытаемся уменьшить сумму
                    j -= 1;
                }
            }
        }
    }
    ans
}

// https://leetcode.com/problems/intersection-of-two-arrays/description/
// Given two integer arrays nums1 and nums2, return an array of their intersection.
// Each element in the result must be unique and you may return the result in any order.
pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut res = vec![];
    let mut nums1 = nums1.into_iter().collect::<HashSet<_>>();
    for n in nums2 {
        if nums1.contains(&n) {
            res.push(n);
            nums1.remove(&n);
        }
    }
    res
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

// https://leetcode.com/problems/merge-two-sorted-lists/description/
pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    fn rec(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if list1.is_none() {
            list2
        } else if list2.is_none() {
            list1
        } else {
            let mut l2 = list2.as_mut()?;
            let mut l1 = list1.as_mut()?;
            if l1.val < l2.val {
                l1.next = rec(l1.next.take(), list2);
                list1
            } else {
                l2.next = rec(list1, l2.next.take());
                list2
            }
        }
    }
    rec(list1, list2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_atm() {
        // pprint(atm(1250, l))  # '1x1000 2x100 1x50'
        // pprint(atm(1000000, l))  # 'Error: Not enough money'
        // pprint(atm(2400, l))  # '2x1000 3x100 2x50'
        // pprint(atm(6512, l))  # 'Error: Incorrect value'
        // pprint(atm(50, l))  # '1x50'
        // pprint(atm(50, l))  # 'Error: Not enough money'
        // pprint(atm(5500, l))  # '3x1000 5x500'

        let mut limits = [(1000, 6), (500, 5), (100, 5), (50, 4)]
            .into_iter()
            .collect::<HashMap<i32, i32>>();

        println!("{}", atm(1250, &mut limits));
        println!("{}", atm(1000000, &mut limits));
        println!("{}", atm(2400, &mut limits));
        println!("{}", atm(6512, &mut limits));
        println!("{}", atm(50, &mut limits));
        println!("{}", atm(50, &mut limits));
        println!("{}", atm(5500, &mut limits));
    }

    #[test]
    fn test_tickets() {
        // [
        //     { 'from': 'London', 'to': 'Moscow' },
        //     { 'from': 'NY', 'to': 'London' },
        //     { 'from': 'Moscow', 'to': 'SPb' },
        //     { 'from': 'Los Angeles', 'to': 'NY' },
        //     ...
        // ]

        // Los Angeles -> NY -> London -> Moscow -> Spb
        println!(
            "{:?}",
            tickets(vec![
                ("London".to_string(), "Moscow".to_string()),
                ("NY".to_string(), "London".to_string()),
                ("Los Angeles".to_string(), "NY".to_string()),
                ("Moscow".to_string(), "SPb".to_string()),
            ])
        );
    }

    #[test]
    fn test_anagrams() {
        // pprint(
        // group_anagrams(
        // ['сон', 'нос', 'сорт', 'трос', 'торт', 'рост']
        // )
        // )
        println!(
            "{:?}",
            group_anagrams(vec![
                "сон".to_string(),
                "нос".to_string(),
                "сорт".to_string(),
                "трос".to_string(),
                "торт".to_string(),
                "рост".to_string()
            ])
        );
    }

    #[test]
    fn test_min_distance() {
        // pprint(
        //     min_distance(
        //         [1, 4, 8, 12, 18, 328, 99482], [35, -27, 325, 9482]
        //     )
        // )
        println!("{}", min_distance(vec![], vec![])); // i64::MAX
        println!("{}", min_distance(vec![i32::MIN], vec![-1])); // i32::MAX
        println!(
            "{}",
            min_distance(vec![1, 4, 8, 12, 18, 328, 99482], vec![35, -27, 325, 9482])
        ); // i32::MAX
    }

    #[test]
    fn test_rle() {
        // print(
        //     rle("AAAABBBCCXYZDDDDEEEFFFAAAAAABBBBBBBBBBBBBBBBBBBBBBBBBBBB")
        // )
        println!(
            "{:?}",
            rle("AAAABBBCCXYZDDDDEEEFFFAAAAAABBBBBBBBBBBBBBBBBBBBBBBBBBBB".to_string())
        ); // A4B3C2XYZD4E3F3A6B28
        println!("{:?}", rle("RRDD".to_string()));
        println!("{:?}", rle("CCCPO".to_string()));
    }

    #[test]
    fn test_remove_duplicate_letters() {
        println!(
            "{}",
            remove_duplicate_letters(
                "PPZZPPAAAABBBCCXYZDDDDEEEFFFAAAAAABBBBBBBBBBBBBBBBBBBBBBBBBBBB".to_string()
            )
        ); // PABCXYZDEF
    }

    #[test]
    fn test_find_triplets_sum() {
        // a = [1, 2, 3, 4, 5, 6, 7]
        // b = [2, 3, 5, 7, 11]
        // c = [1, 1, 2, 3, 5, 7]
        // a_len = len(a)
        // b_len = len(b)
        // c_len = len(c)
        //
        // x = 15
        println!(
            "{}",
            find_triplets_sum(
                vec![1, 2, 3, 4, 5, 6, 7],
                vec![2, 3, 5, 7, 11],
                vec![1, 1, 2, 3, 5, 7],
                15
            )
        ); // 16
    }

    #[test]
    fn test_intersect_list() {
        println!("{:?}", intersection(vec![1, 2, 2, 1], vec![2, 2]));
        println!("{:?}", intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]));
    }
}
