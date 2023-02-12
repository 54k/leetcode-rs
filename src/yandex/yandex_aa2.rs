use std::cell::RefCell;
use std::cmp::min;
use std::rc::Rc;

// Подотрезок с суммой X.py
// Дан массив целых чисел, нужно найти непустой подотрезок
// (непрерывную подпоследовательность) с заданной суммой X, либо сказать, что это невозможно.
// Для найденного отрезка (если он существует) следует выдать на выход индексы его концов.
// https://leetcode.com/problems/subarray-sum-equals-k/
pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    fn running(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let mut sum = 0;
        for i in 0..nums.len() {
            sum = 0;
            for j in i..nums.len() {
                sum += nums[j];
                if sum == k {
                    ans += 1;
                }
            }
        }
        ans
    }
    fn optimized(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;
        let mut ans = 0;
        let mut sum = 0;
        let mut map = HashMap::new();
        map.insert(0, 1);
        for i in 0..nums.len() {
            sum += nums[i];
            if map.contains_key(&(sum - k)) {
                ans += *map.get(&(sum - k)).unwrap();
            }
            *map.entry(sum).or_insert(0) += 1;
        }
        ans
    }
    optimized(nums, k)
}

// Минимальное произведение ???
// https://leetcode.com/problems/maximum-product-subarray/
pub fn max_product(nums: Vec<i32>) -> i32 {
    fn brute(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 0..nums.len() {
            for j in i..nums.len() {
                let mut prod = 1;
                for k in i..=j {
                    prod *= nums[k];
                }
                ans = ans.max(prod);
            }
        }
        ans
    }
    fn optimized(nums: Vec<i32>) -> i32 {
        // store the result that is the max we have found so far
        let mut ans = nums[0];
        // imax/imin stores the max/min product of
        // subarray that ends with the current number A[i]
        let mut min = ans;
        let mut max = ans;
        for i in 1..nums.len() {
            // multiplied by a negative makes big number smaller, small number bigger
            // so we redefine the extremums by swapping them
            if nums[i] < 0 {
                std::mem::swap(&mut min, &mut max);
            }
            // max/min product for the current number is either the current number itself
            // or the max/min by the previous number times the current one
            max = nums[i].max(max * nums[i]);
            min = nums[i].min(min * nums[i]);
            ans = ans.max(max);
        }
        ans
    }
    optimized(nums)
}

/*
 * дана последовательность целых чисел, найти минимально возможно возможное произведение пары элементов
 * последовательности
 */
pub fn min_product(nums: Vec<i32>) -> i32 {
    let mut has_negative = false;
    let mut all_negative = true;
    for i in 0..nums.len() {
        if nums[i] > 0 {
            all_negative = false;
        } else {
            has_negative = true;
        }
    }
    if all_negative {
        let max_1 = nums.iter().enumerate().max_by(|x, y| x.1.cmp(y.1)).unwrap();
        let max_2 = nums
            .iter()
            .enumerate()
            .filter(|x| x.0 != max_1.0)
            .max_by(|x, y| x.1.cmp(y.1))
            .unwrap();
        max_1.1 * max_2.1
    } else if has_negative {
        let min_1 = nums.iter().enumerate().min_by(|x, y| x.1.cmp(y.1)).unwrap();
        let max_2 = nums
            .iter()
            .enumerate()
            .filter(|x| x.0 != min_1.0)
            .max_by(|x, y| x.1.cmp(y.1))
            .unwrap();
        min_1.1 * max_2.1
    } else {
        let min_1 = nums.iter().enumerate().min_by(|x, y| x.1.cmp(y.1)).unwrap();
        let min_2 = nums
            .iter()
            .enumerate()
            .filter(|x| x.0 != min_1.0)
            .min_by(|x, y| x.1.cmp(y.1))
            .unwrap();
        min_1.1 * min_2.1
    }
}

// Вертикальная ось симметрии.py
// Дан массив точек с целочисленными координатами (x, y).
// Определить, существует ли вертикальная прямая,
// делящая точки на 2 симметричных относительно этой прямой множества.
// Note: Для удобства точку можно представлять не как массив [x, y], а как объект {x, y}

// https://leetcode.com/discuss/interview-experience/365500/amazon-sde1-santiago-aug-2019-offer

// The thing is to create a map, which stores for each value of Y a sorted array of X values for that Y.
// So for each of this Y entries in the map, we can go checking with two pointers
// (one that starts at the beginning and the other at the end) their middle point.
// We store this point and check if the next calls have the same middle point.
// If every two points have the same middle point, it's symmetric.
fn is_vert_sym(points: Vec<(i32, i32)>) -> bool {
    fn leetcode_approach(points: Vec<(i32, i32)>) -> bool {
        // approach:
        // store X coords in the map, with Ys counts
        // find MAX_X --> MAX_X - point.X will find symmetric X
        // find Y and decrement counter, if counter == 0 remove key, if Y map is empty, remove X
        // check if map is empty - then symmetry found
        use std::collections::*;
        let mut map = HashMap::new();
        let mut max_x = i32::MIN;

        for i in 0..points.len() {
            max_x = max_x.max(points[i].0);
            *map.entry(points[i].0)
                .or_insert_with(HashMap::new)
                .entry(points[i].1)
                .or_insert(0) += 1;
        }

        for i in 0..points.len() {
            let k = max_x - points[i].0; // symmetic X for current point
            if !map.contains_key(&k) || !map.get(&k).unwrap().contains_key(&points[i].1) {
                return false;
            }
            let p = map.get_mut(&k).unwrap();
            let cnt = p.get_mut(&points[i].1).unwrap();
            *cnt -= 1;
            if *cnt == 0 {
                p.remove(&points[i].1);
            }
            if p.is_empty() {
                map.remove(&k);
            }
        }

        map.is_empty()
    }

    fn yandex_approach(points: Vec<(i32, i32)>) -> bool {
        use std::collections::*;
        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        for i in 0..points.len() {
            min_x = min_x.min(points[i].0);
            max_x = max_x.max(points[i].0);
        }

        let mut deltas = HashMap::new();

        for (x, y) in points {
            let left_dist = x - min_x;
            let right_dist = max_x - x;
            // println!("{} {}", left_dist, right_dist);
            match left_dist.cmp(&right_dist) {
                std::cmp::Ordering::Less => {
                    *deltas.entry((left_dist, y)).or_insert(0) += 1;
                }
                std::cmp::Ordering::Greater => {
                    *deltas.entry((right_dist, y)).or_insert(0) -= 1;
                }
                _ => {}
            }
        }
        for d in deltas.values() {
            if *d != 0 {
                return false;
            }
        }
        true
    }

    assert_eq!(
        yandex_approach(points.clone()),
        leetcode_approach(points.clone())
    );
    yandex_approach(points)
}

// Поиск подстроки в строке без учета порядка букв.py
// Дан текст T и строка S. Требуется найти подстроку S' в T такую,
// что она совпадает с S с точностью до перестановки букв.
// https://leetcode.com/problems/permutation-in-string/description/
// https://leetcode.com/problems/permutation-in-string/solutions/127729/short-permutation-in-a-long-string/?orderBy=most_relevant
fn check_inclusion(s1: String, s2: String) -> bool {
    let slen1 = s1.len();
    let slen2 = s2.len();
    if slen1 > slen2 {
        return false;
    }
    let s2 = s2.chars().collect::<Vec<_>>();
    let mut hash1 = vec![0; 26];
    for c in s1.chars() {
        hash1[c as usize - 'a' as usize] += 1;
    }
    for i in 0..=(slen2 - slen1) {
        let mut hash2 = vec![0; 26];
        for j in i..i + slen1 {
            hash2[s2[j] as usize - 'a' as usize] += 1;
        }
        if hash1 == hash2 {
            return true;
        }
    }
    false
}

// Пересечение топов двух выдач.py
// Для двух поисковых выдач, заданных массивами DocId'ов (т.е. просто векторами целых чисел)
// длины N, для всех K от 1 до N нужно посчитать количество общих документов в топах размера K.
// Непоисковая формулировка:
// Для двух массивов целых чисел длины N, для всех K от 1 до N,
// посчитать количество общих чисел на префиксах длины K.
fn find_intersection(a: Vec<i32>, b: Vec<i32>) -> Vec<usize> {
    // Все числа в каждом из массивов различные.
    fn different_nums_in_each_array(a: Vec<i32>, b: Vec<i32>) -> Vec<usize> {
        use std::collections::HashSet;
        let mut s1 = HashSet::new();
        let mut s2 = HashSet::new();
        let mut ans = vec![];
        for i in 0..a.len() {
            s1.insert(a[i]);
            s2.insert(b[i]);
            ans.push(s1.intersection(&s2).count());
        }
        ans
    }
    different_nums_in_each_array(a, b)
}

// Дано бинарное дерево с выделенным корнем, в каждой вершине которого записано по одной букве A-Z.
// Две вершины считаются эквивалентными, если поддеревья этих вершин содержат одинаковое множество (т.е. без учета частот) букв.
// Нужно найти любую пару эквивалентных вершин.
// Можно усложнить, задав:
// Нужно найти две эквивалентные вершины с максимальным суммарным размером поддеревьев.

// Решение за O(NK), где N - размер дерева, K - размер алфавита.
// То есть, обход дерева с построением дескриптора (булева массива размера K)
// поддерева и отображения - дескриптор->два максимальных поддерева с этим дескриптором.
//
// Можно поддерживать мап из декскриптора в максимальное поддерево с таким дескриптором
// и обновлять пару с максимальной суммой каждый раз, когда мы пытаемся обновить в мапе максимум.
// Так писать даже проще.
// Хороший кандидат в качестве дескриптора использует битовую маску встреченных букв и получит сложность O(N).

// https://leetcode.com/problems/find-duplicate-subtrees/
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode<T> {
    pub val: T,
    pub left: Option<Rc<RefCell<TreeNode<T>>>>,
    pub right: Option<Rc<RefCell<TreeNode<T>>>>,
}
type Node<T> = Option<Rc<RefCell<TreeNode<T>>>>;
fn same_subtrees(root: Node<char>) -> (Node<char>, Node<char>) {
    use std::collections::*;
    // Можно усложнить, задав:
    // Нужно найти две эквивалентные вершины с максимальным суммарным размером поддеревьев.
    fn traverse(
        root: Node<char>,
        trees_counts: &mut HashMap<i32, (i32, Node<char>)>,
        ans: &mut (i32, Node<char>, Node<char>), // кол-во вершин + пара эквивалентных вершин
    ) -> (i32, i32) {
        if root.is_none() {
            return (0, 0);
        }
        let r = root.clone().unwrap();
        let r = r.borrow();

        let left_subtree = traverse(r.left.clone(), trees_counts, ans);
        let right_subtree = traverse(r.right.clone(), trees_counts, ans);

        let mut mask = 1 << (r.val as i32 - 'A' as i32);
        mask |= left_subtree.1;
        mask |= right_subtree.1;

        let tree_size = left_subtree.0 + right_subtree.0 + 1;

        let entry = trees_counts
            .entry(mask)
            .or_insert((tree_size, root.clone()));

        if entry.0 < tree_size {
            if ans.0 < tree_size {
                *ans = (tree_size, entry.1.clone(), root.clone());
            }
            *entry = (tree_size, root);
        }

        (tree_size, mask)
    }
    let mut ans = (0, None, None);
    traverse(root, &mut HashMap::new(), &mut ans);
    (ans.1, ans.2)
}

// Найти всплески пользовательской активности.py
pub mod user_activity {
    use std::collections::{HashMap, VecDeque};

    // Есть последовательность событий, каждое событие это пара user_id, time, события отсортированы по времени
    // Нужно уметь отвечать на вопрос, сколько за последние 5 минут было пользователей, которые задали >= 1000 запросов.
    pub struct UserActivity {
        events: VecDeque<(i32, i32)>,
        events_count_by_user: HashMap<i32, i32>,
        time_frame: i32,
    }

    impl UserActivity {
        pub fn new(time_frame: i32) -> Self {
            Self {
                events: VecDeque::new(),
                events_count_by_user: HashMap::new(),
                time_frame,
            }
        }

        pub fn add_event(&mut self, user_id: i32, time_minute: i32) {
            let start_time_frame = time_minute - self.time_frame;

            while let Some((_, time)) = self.events.front() {
                if *time >= start_time_frame {
                    break;
                }
                let (uid, _) = self.events.pop_front().unwrap();
                let events_count = self.events_count_by_user.get_mut(&uid).unwrap();
                *events_count -= 1;
                if *events_count == 0 {
                    self.events_count_by_user.remove(&uid);
                }
            }

            self.events.push_back((user_id, time_minute));
            *self.events_count_by_user.entry(user_id).or_insert(0) += 1;
        }

        pub fn query_active_users(&mut self) -> Vec<i32> {
            let mut ans = vec![];
            for (&uid, &events) in &self.events_count_by_user {
                if events >= 1000 {
                    ans.push(uid);
                }
            }
            ans
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::yandex::yandex_aa2::user_activity::UserActivity;

    #[test]
    fn test_subarray_sum() {
        println!("{:?}", subarray_sum(vec![1, 1, 1], 2)); // 2
        println!("{:?}", subarray_sum(vec![1, 2, 3], 3)); // 2
        println!("{:?}", subarray_sum(vec![-1, -1, 1], 0)); // 1
    }

    #[test]
    fn test_is_vert_sym() {
        println!(
            "{:?}",
            is_vert_sym(vec![(0, 0), (0, 0), (1, 1), (2, 2), (3, 1), (4, 0), (4, 0)]),
        ); // True
        println!(
            "{:?}",
            is_vert_sym(vec![(0, 0), (0, 0), (1, 1), (2, 2), (3, 1), (4, 0)]),
        ); //False
        println!("{:?}", is_vert_sym(vec![])); // True
        println!("{:?}", is_vert_sym(vec![(0, 0)])); // True
        println!("{:?}", is_vert_sym(vec![(0, 0), (10, 0)])); // True
        println!("{:?}", is_vert_sym(vec![(0, 0), (11, 1)])); //False
        println!("{:?}", is_vert_sym(vec![(0, 0), (1, 0), (3, 0)])); //False
    }

    #[test]
    fn test_check_inclusion() {
        println!(
            "{}",
            check_inclusion("ab".to_string(), "eidbaooo".to_string())
        ); // true
        println!(
            "{}",
            check_inclusion("ab".to_string(), "eidboaoo".to_string())
        ); // false
        println!(
            "{}",
            check_inclusion("ab".to_string(), "eidboaoo".to_string())
        ); // false
    }

    #[test]
    fn test_find_intersection() {
        println!(
            "{:?}",
            find_intersection(vec![1, 2, 3, 4, 5], vec![6, 2, 3, 9, 10])
        ); // [0, 1, 2, 2, 2]
    }

    #[test]
    fn test_min_product() {
        println!("{}", min_product(vec![9, 4, 2, 5, 3])); // 6
        println!("{}", min_product(vec![1, 2, 3, 4])); // 2
        println!("{}", min_product(vec![-1, -2, -3, -4])); // 2
        println!("{}", min_product(vec![-1, -2, 3, 4])); // -8
        println!("{}", min_product(vec![0, 0, 0, 0])); // 0
    }

    #[test]
    fn test_find_same_subtrees() {
        println!(
            "{:?}",
            same_subtrees(Some(Rc::new(RefCell::new(TreeNode {
                val: 'A',
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 'B',
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 'Z',
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 'Z',
                            left: Some(Rc::new(RefCell::new(TreeNode {
                                val: 'C',
                                left: None,
                                right: None,
                            }))),
                            right: None,
                        }))),
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 'O',
                        left: None,
                        right: None,
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 'C',
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 'X',
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 'X',
                            left: Some(Rc::new(RefCell::new(TreeNode {
                                val: 'X',
                                left: None,
                                right: None,
                            }))),
                            right: None,
                        }))),
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 'P',
                        left: None,
                        right: None,
                    }))),
                }))),
            }))))
        );
    }

    #[test]
    fn test_find_active_users() {
        let mut user_activity = UserActivity::new(5);
        for i in 0..=60 {
            for _ in 0..1500 {
                user_activity.add_event(i, i);
            }
            println!("{:?}", user_activity.query_active_users());
        }
    }
}
