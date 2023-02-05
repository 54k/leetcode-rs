// https://leetcode.com/problems/maximize-distance-to-closest-person/
// Места в кинотеатре расположены в один ряд.
// Только что пришедший зритель выбирает место, чтобы сидеть максимально далеко
// от остальных зрителей в ряду. То есть расстояние от того места,
// куда сядет зритель до ближайшего к нему зрителя должно быть максимально.
//
// Гарантируется, что в ряду всегда есть свободные места и уже сидит хотя бы один зритель.
// Напишите функцию, которая по заданному ряду мест (массиву из нулей и единиц)
// вернёт расстояние от выбранного пришедшим зрителем места до другого ближайшего зрителя.

// Let's just call this what it is, this has nothing to do with picking seats,
// this is the algorithm for picking a urinal in a public bathroom :D

// https://leetcode.com/problems/maximize-distance-to-closest-person/
pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
    let mut i = 0i32;
    let mut j = seats.len() as i32 - 1;
    while i < seats.len() as i32 && seats[i as usize] == 0 {
        i += 1;
    }
    while j >= 0 && seats[j as usize] == 0 {
        j -= 1;
    }
    let mut ans = 0;
    ans = ans.max(i).max(seats.len() as i32 - 1 - j);
    let mut cur = 0;
    for k in i..=j {
        if seats[k as usize] == 0 {
            cur += 1;
        } else {
            ans = ans.max((cur + 1) / 2);
            cur = 0;
        }
    }
    ans
}

// Дана строка из десятичных цифр (длинное число, младшие разряды расположены по младшему индексу).
// Написать код, который умножит это число на число 1 <= n <= 9.
// Ограничения по памяти: O(1) дополнительной памяти,
// т.е. надо использовать исходную строку (считаем, что возможное увеличение длины
// на 1 разряд не приведёт к реаллокации).
pub fn multiply_string(str: String, n: i32) -> String {
    let mut str = str.chars().collect::<Vec<_>>();
    str.reverse();
    let mut carry = 0;
    for i in 0..str.len() {
        let c = str[i].to_string().parse::<i32>().unwrap() * n + carry;
        str[i] = (c % 10).to_string().chars().take(1).max().unwrap();
        carry = c / 10;
    }
    if carry > 0 {
        str.push(carry.to_string().chars().take(1).max().unwrap());
    }
    str.into_iter().rev().collect()
}

// Дан массив из нулей и единиц. Нужно определить, какой максимальный по длине
// подинтервал единиц можно получить, удалив ровно один элемент массива.

// https://leetcode.com/problems/longest-subarray-of-1s-after-deleting-one-element/
// Maintain a sliding window where there is at most one zero on it.
pub fn longest_subarray(nums: Vec<i32>) -> i32 {
    let mut start = 0;
    let mut zeroes = 0;
    let mut ans = 0;

    for end in 0..nums.len() {
        if nums[end] == 0 {
            zeroes += 1;
        }
        while zeroes > 1 {
            if nums[start] == 0 {
                zeroes -= 1;
            }
            start += 1;
        }
        ans = ans.max(end - start);
    }
    ans as i32
}

// Считаем глубину каждой вершины, потом идем двумя указателями вверх от вершин, пока они не встретятся.
// O(h) по времени, где h - высота дерева. Обычно люди сначала предлагают этот вариант, его и стоит писать.
//
// Также можно обсудить усложненный вариант, с требованием O(d) по времени, где d - расстояние между вершинами.
// Здесь нужно бежать вверх от каждой из двух вершин поочередно на расстояния, растущие по степеням двойки.
// Как только получится прийти в какого-то общего предка, решение сводится к предыдущему.

// https://www.baeldung.com/cs/tree-lowest-common-ancestor

// The idea of this approach is to have two pointers, one on each node.
// We need to move these pointers towards the root in a way that allows them to meet at the LCA.
//
// The first thing we can notice is that these pointers should be in the same depth.
// In addition to that, the deeper pointer can never be the LCA.
// Therefore, our first step should be to keep moving the deeper pointer to its parent until the two pointers are on the same depth.
//
// Now, we have two pointers that are on the same depth.
// We can keep moving both pointers towards the root one step at a time until they meet at one node.
// Since this node is the deepest node that our pointers can meet at, therefore,
// it’s the deepest common ancestor of our starting nodes, which is the LCA.

// 3.2. Preprocessing
//
// In order to implement this solution, we will need to calculate the depth and the parent of each node.
// This can be done with a simple DFS traversal starting from the root.

// 3.3. Finding the LCA
//
// After calling the previous function starting from the root, we will have the depth and parent of each node.
// We can now apply the approach that we discussed earlier.
pub fn lowest_common_ancestor(edges: Vec<(usize, usize)>, x: usize, y: usize, n: usize) -> usize {
    let mut adj = vec![vec![]; n + 1]; // 1-индексация в дереве
    for (from, to) in edges {
        adj[from].push(to);
        adj[to].push(from);
    }
    // Finding the depth and parent of each node
    let mut visited = vec![false; n + 1];
    let mut depth = vec![0; n + 1];
    let mut parent = vec![0; n + 1];

    fn dfs(
        v: usize,
        adj: &Vec<Vec<usize>>,
        depth: &mut Vec<i32>,
        parent: &mut Vec<usize>,
        visited: &mut Vec<bool>,
    ) {
        visited[v] = true;
        for &u in &adj[v] {
            if !visited[u] {
                parent[u] = v;
                depth[u] = depth[v] + 1;
                dfs(u, adj, depth, parent, visited);
            }
        }
    }
    // Firstly, we set the current node as visited.
    // Secondly, we set the parent and depth of the child node before calling the function recursively for it.
    // The complexity of the preprocessing step in this approach is O(n), where n is the number of nodes.
    // The complexity is considered efficient because we only need to apply it once.
    dfs(1, &adj, &mut depth, &mut parent, &mut visited);

    // First of all, we keep moving the deeper node to its parent until both nodes have the same depth.
    let mut v = x;
    let mut u = y;
    while depth[v] != depth[u] {
        if depth[v] > depth[u] {
            v = parent[v];
        } else {
            u = parent[u];
        }
    }
    // After that, we move both nodes to their parents until they meet.
    while v != u {
        v = parent[v];
        u = parent[u];
    }

    // Although the approach is considered fairly simple, however,
    // finding the LCA of a pair of nodes also has a complexity of O(h), where h is the height of the tree.
    // This complexity can go as far as becoming O(n) in case of an almost linear tree
    // (consider two long chains connected at the root).
    v
}

// Дан отсортированный массив целых чисел a , целое число K и индекс элемента index.
// Необходимо вернуть в любом порядке K чисел из массива,
// которые являются ближайшими по значению к элементу a[index] .

// https://leetcode.com/problems/find-k-closest-elements/solutions/106419/o-log-n-java-1-line-o-log-n-k-ruby/ StefanPochmann is a god
// https://leetcode.com/problems/find-k-closest-elements/
pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
    fn pochmann_solution(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut lo = 0;
        let mut hi = arr.len() - k as usize;
        while lo < hi {
            let mid = (lo + hi) / 2;
            if x - arr[mid] > arr[mid + k as usize] - x {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        arr[lo..lo + k as usize].to_vec()
    }
    pochmann_solution(arr, k, x)
}

// Без условия :( Странное решение от яндекса не делает трим одного пробела в начале,
// да и вообще воняет O(n^2) говной c этим охуительным удалением буквы из середины массива
// https://leetcode.com/discuss/interview-question/algorithms/124659/google-phone-screen-given-a-string-with-spaces-normalize-it

// from pprint import pprint
//
// def norm(st):
//     i = len(st) - 1
//     while i > 0:
//       if st[i] == ' ' and st[i - 1] == ' ':
//       st.pop(i)
//       i -= 1
//     pprint(st)
//
//
// ex = [' ', ' ', 's', 'o', ' ', 'm', 'e', ' ',  ' ',  ' ', ' ', ' ', 's', 't', 'r', 'i', 'n', 'g', ' ',  ' ',  ' ', ' ']
//
// norm(ex)
pub fn normalize_spaces(str: String) -> String {
    let mut ans = String::new();
    let str = str.chars().collect::<Vec<_>>();
    let mut first_char_found = false;
    for i in 0..str.len() - 1 {
        if str[i] != ' ' {
            first_char_found = true
        }
        if first_char_found {
            if str[i] == ' ' && str[i + 1] == ' ' {
                // NOP
            } else {
                ans.push(str[i]);
            }
        }
    }
    ans
}

// Тоже без условия :(
// что то в духе есть строка в которой не только буквы,
// все не буквы надо пропустить и понять является ли оставшееся палиндромомом

// А мудила в НИИ - инвалид умa! -> true

// class Solution:
// def isPalindrome(self, s: str) -> bool:
// left = 0
// right = len(s) - 1
//
// while left < right:  # Строгое не равно работает как с четной так и с нечетной строкой
// left_char = s[left]
// right_char = s[right]
//
// if not left_char.isalnum():  # .isalpha() для только букв
// left += 1  # Увеличиваем индекс пока не наткнемся на валидный символ
// continue
//
// if not right_char.isalnum():
// right -= 1
// continue
// # Когда оба символа валидные, чекаем на раввенство
// if left_char.lower() != right_char.lower():
// return False
//
// left += 1
// right -= 1
//
// return True
//
//
// # Чуть более пижонски
//
// def isPalindrome(self, s):
// l = 0
// r = len(s)-1
// while l < r:
// while l < r and not s[l].isalnum():
// l += 1
// while l < r and not s[r].isalnum():
// r -= 1
// if s[l].lower() != s[r].lower():
// return False
// l += 1
// r -= 1
// return True
pub fn palindrome(s: String) -> bool {
    let s = s.chars().collect::<Vec<_>>();
    let mut l = 0;
    let mut r = s.len() - 1;
    while l < r {
        while l < r && !s[l].is_alphabetic() {
            l += 1;
        }
        while l < r && !s[r].is_alphabetic() {
            r -= 1;
        }
        if s[l].to_lowercase().last().unwrap() != s[r].to_lowercase().last().unwrap() {
            return false;
        }
        l += 1;
        r -= 1;
    }
    true
}

// Даны даты заезда и отъезда каждого гостя.
// Необходимо написать функцию, которая считает максимальное число посетителей, которые одновременно проживали в гостинице.
//
// Ограничение по сложности — строго меньше O(N^2)

// Лааконсен олимпиадное программирование, алгоритм заметающей прямой
// Для решения задачи будем создавать для каждого клиента два события: приход и уход.
// Затем отсортируем события по времени и обойдем их.
// Что- бы найти максимальное число клиентов, заведем счетчик и будем увели- чивать его значение,
// когда клиент приходит, и уменьшать, когда уходит.
// Наибольшее значение, принимаемое счетчиком, и будет искомым ответом.
// Найденный алгоритм работает за время O(n log n), потому что сортировка событий занимает время O(n log n),
// а заметание – время O(n).

// https://www.geeksforgeeks.org/find-the-point-where-maximum-intervals-overlap/
pub fn max_guests(mut guests: Vec<(i32, i32)>) -> i32 {
    guests = guests
        .into_iter()
        .flat_map(|(i, o)| vec![(i, 1), (o, -1)])
        .collect();
    // сортируем по времени если время одинаковое сортируем по типу эвента - выезжающие после вьезжающих,
    // но это надо уточнить на собесе как они хотят
    guests.sort_by(|(t, c), (t2, c2)| if t == t2 { c2.cmp(c) } else { t.cmp(t2) });
    let mut ans = 0;
    let mut cur = 0;
    for (_, c) in guests {
        cur += c;
        ans = ans.max(cur);
    }
    ans
}

// Пофильтровать один список по другому сортированному списку
// pprint(s_f2([1, 3, 5, 7, 8, 9], [1, 2, 6, 8, 9])) --> [3, 5, 7]
pub fn filter_by_another(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    use std::cmp::Ordering;
    let mut ans = vec![];
    let mut i = 0;
    let mut j = 0;

    while i < a.len() {
        match a[i].cmp(&b[j]) {
            Ordering::Equal => {
                i += 1;
                j += 1;
            }
            Ordering::Less => {
                ans.push(a[i]);
                i += 1;
            }
            Ordering::Greater => {
                j += 1;
            }
        }
    }
    ans
}

// Нужно реализовать функцию OneEditApart , проверяющую, можно ли одну строку получить из другой не более,
// чем за одно исправление (удаление, добавление, изменение символа):

// https://wentao-shao.gitbook.io/leetcode/string/161.one-edit-distance
// https://leetcode.com/problems/one-edit-distance/ задача для премиум аккаунта, увы
// а вообще литкод охуел - 300 баксов в год, чтобы пить урину каждый день

// Так же это задача 1.5 из книги Cracking the code interview (Карьера программиста)

// Given two strings s and t, determine if they are both one edit distance apart.
//
// Note:
// There are 3 possiblities to satisify one edit distance apart:
//
// Insert a character into s to get t
// Delete a character from s to get t
// Replace a character of s to get t
// Example 1:
//
// Input: s = "ab", t = "acb"
// Output: true
// Explanation: We can insert 'c' into s to get t.
// Example 2:
//
// Input: s = "cab", t = "ad"
// Output: false
// Explanation: We cannot get t from s by only one step.
// Example 3:
//
// Input: s = "1203", t = "1213"
// Output: true
// Explanation: We can replace '0' with '1' to get t.
pub fn is_one_edit_distance(mut s: String, mut t: String) -> bool {
    if s.len() > t.len() {
        std::mem::swap(&mut s, &mut t);
    }
    let len_s = s.len();
    let len_t = t.len();
    if (len_t - len_s) > 1 {
        // случай где нужно заведомо больше операций
        return false;
    }
    let s = s.chars().collect::<Vec<_>>();
    let t = t.chars().collect::<Vec<_>>();
    let mut i = 0;
    let mut j = 0;

    if len_s == len_t {
        // нужна замена символа - смотрим сколько символов отличаются
        while i < s.len() {
            let mut diff = 0;
            if s[i] != t[j] {
                diff += 1;
                if diff > 1 {
                    return false;
                }
            }
            i += 1;
        }
        true
    } else {
        // нужна вставка символа (или удаление этот кейс покрывает оба случая) проверяем,
        // что меньшая строка полностью содержится в текущей
        while i < len_s && j < len_t {
            if s[i] == t[j] {
                i += 1;
                j += 1;
            } else {
                j += 1;
            }
        }
        i == len_s
    }
}

// Реализовать функцию fuzzysearch как в редакторе sublime text 3 .
// Для незнакомых с редактором и/или термином fuzzysearch (нечёткий поиск),
// можно упомянуть более формальное название — approximate string matching
// (нахождение приблизительного соответствия строк). По факту требуется проверить,
// является ли первая строка подпоследовательностью второй.
//
// Хочется, чтобы кандидат задавал вопросы про тесткейсы:
// 1) пустая строка?, 2) регистр?, 3) 'wl' (подстрока, не начинающаяся на ту же букву, что строка)

// https://leetcode.com/problems/is-subsequence/
pub fn is_subsequence(s: String, t: String) -> bool {
    // такое решение не валится на пустых строках
    // закрывает все три вопроса, кроме регистра
    fn approach1(s: String, t: String) -> bool {
        let s = s.chars().collect::<Vec<_>>();
        let t = t.chars().collect::<Vec<_>>();
        let mut i = 0;
        let mut j = 0;
        while i < s.len() && j < t.len() {
            if s[i] != t[j] {
                j += 1;
            } else {
                i += 1;
                j += 1;
            }
        }
        i == s.len()
    }
    // решение яндекса - валится если s -> пустая строка
    fn approach2(s: String, t: String) -> bool {
        let s = s.chars().collect::<Vec<_>>();
        let t = t.chars().collect::<Vec<_>>();
        if s.is_empty() {
            return true; // костылик, яндекс не благодари
        }
        let mut i = 0;
        for c in t {
            if c == s[i] {
                i += 1;
            }
            if i == s.len() {
                return true;
            }
        }
        false
    }

    approach2(s, t)
}

// Дан вектор, надо удалить из него нули, сохранив порядок остальных элементов.
// Интересует как с использованием стандартных средств, так и без них.
// https://leetcode.com/problems/move-zeroes/
pub fn move_zeroes(nums: &mut Vec<i32>) {
    fn no_std(nums: &mut Vec<i32>) {
        let mut slow = 0;
        let mut fast = 0;
        while fast < nums.len() {
            if nums[fast] != 0 {
                nums.swap(slow, fast);
                slow += 1;
                fast += 1;
            } else {
                fast += 1;
            }
        }
    }

    fn with_std(nums: &mut Vec<i32>) {
        use std::cmp::Ordering;
        nums.sort_by(|a, b| {
            if *a == 0 {
                Ordering::Greater
            } else if *b == 0 {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });
    }
    no_std(nums)
}

// Inplace заменяет в массиве символы пробела на последовательность символов '%', '2', '0'.

// Задача номер 3 (строки и массивы) из книги Cracking the coding interview (Карьера программиста на русском)
// На вход подается строка и число original length

// Общий подход к задачам обработки строк - изменять строку, начиная с конца и двигаясь от конца строки к началу.
// Это полезно, потому что в конце строки есть дополнительное пространство,
// в котором можно изменять символы, не думая о возможной потере информации.

// Воспользуемся этим методом в данной задаче.
// Алгоритм использует двупроходное сканирование.
// При первом проходе мы подсчитываем, сколько пробелов находится в строке.
// Умножая полученное значение на 3, мы получим количество дополнительных символов в результирующей строке.
// При втором проходе, который осуществляется в обратном направлении, выполняется фактическая модификация строки.
// Обнаруживая пробел, мы заменяем его на %20. Если символ не является пробелом, то он просто копируется.

// Ввод: "Mr John Smith    ", 13

// https://leetcode.com/discuss/interview-question/124608/amazon-phone-screen-urlify-a-given-string-replace-spaces-with-20
fn urlify(mut s: String, len: usize) -> String {
    let mut s = s.chars().collect::<Vec<_>>();
    let mut spaces_count = 0;
    for i in 0..len {
        if s[i] == ' ' {
            spaces_count += 1;
        }
    }
    let mut new_length = len + spaces_count * 2; // 2 потому что нужно 1 пробел и %2 символ для замены
    for i in (0..len).rev() {
        // пойнтер на старый конец строки
        if s[i] == ' ' {
            s[new_length - 1] = '0';
            s[new_length - 2] = '2';
            s[new_length - 3] = '%';
            new_length -= 3;
        } else {
            s[new_length - 1] = s[i];
            new_length -= 1;
        }
    }
    s.into_iter().collect()
}

// Дан список интов, повторяющихся элементов в списке нет.
// Нужно преобразовать это множество в строку, сворачивая соседние по числовому ряду числа в диапазоны.

// сворачивая соседние по числовому ряду числа в диапазоны -> очень умная формулировка, если проще,
// то нужно все соседние числа (восходящая последовательность, e.g. 1,2,3 свернуть в интервал 1,3
// 1,3,4,5 будет свернут в два интервала [1],[3,5]

// Пустая последовательность сворачивается в пустой массив

// https://leetcode.com/problems/summary-ranges/
pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    if nums.is_empty() {
        return vec![];
    }
    let mut ans = vec![];
    let mut interval = (nums[0], nums[0]);
    for i in 1..nums.len() {
        if nums[i] - nums[i - 1] == 1 {
            interval.1 = nums[i];
        } else {
            ans.push(interval);
            interval = (nums[i], nums[i]);
        }
    }
    ans.push(interval);
    ans.into_iter()
        .map(|interval| {
            if interval.0 == interval.1 {
                format!("{}", interval.0)
            } else {
                format!("{}->{}", interval.0, interval.1)
            }
        })
        .collect()
}

// Сложение ступенчатых графиков.py
//
// Есть два ступенчатых графика некоторых наблюдаемых величин,
// заданных отсортированными списками координат начала ступенек (<время измерения>, <значение>).
// Подразумевается, что величина сохраняет своё значение между измерениями.
// Все времена измерений в обоих графиках различны.
//
// Очень хуевоe условие/обьяснение
//
// pprint(
// charts_sum(
// [[1, 2], [4, 1]],
// [[2, 4], [3, 6], [5, 7]]
// )
// ) --> [[1, 2], [2, 6], [3, 8], [4, 7], [5, 8]]
fn charts_sum(mut chart1: Vec<(i32, i32)>, mut chart2: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let len1 = chart1.len();
    let len2 = chart2.len();
    let mut ans = vec![];

    let mut i = 0;
    let mut j = 0;

    let mut last_1 = 0;
    let mut last_2 = 0;

    loop {
        if i < len1 && j < len2 && chart1[i].0 == chart2[j].0 {
            ans.push((chart1[i].0, chart1[i].0 + chart2[j].0));
            last_1 = chart1[i].0;
            last_2 = chart2[j].0;
            i += 1;
            j += 1;
        } else if j >= len2 || (i < len1 && chart1[i].0 < chart2[j].0) {
            ans.push((chart1[i].0, chart1[i].1 + last_2));
            last_1 = chart1[i].1;
            i += 1;
        } else {
            ans.push((chart2[j].0, chart2[j].1 + last_1));
            last_2 = chart2[j].1;
            j += 1;
        }
        if i == len1 && j == len2 {
            break;
        }
    }

    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_dist_to_closest() {
        println!("{}", max_dist_to_closest(vec![0, 1])); // 1
        println!("{}", max_dist_to_closest(vec![0, 0, 1])); // 2

        println!(
            "{}",
            max_dist_to_closest(vec![0, 1, 0, 0, 0, 1, 1, 0, 1, 1])
        ); // 2
        println!("{}", max_dist_to_closest(vec![1, 0, 0, 0, 1, 0, 1])); // 2
        println!("{}", max_dist_to_closest(vec![1, 0, 0, 1])); // 1

        println!("{}", max_dist_to_closest(vec![1, 0, 0, 0])); // 3
        println!("{}", max_dist_to_closest(vec![1, 0])); // 1
    }

    #[test]
    fn test_multiply_string() {
        println!("{}", multiply_string("20".to_string(), 5)); // 100
        println!("{}", multiply_string("95".to_string(), 5)); // 475
    }

    #[test]
    fn test_longest_subarray() {
        println!("{}", longest_subarray(vec![1, 1, 0, 1])); // 3
        println!("{}", longest_subarray(vec![0, 1, 1, 1, 0, 1, 1, 0, 1])); // 5
        println!("{}", longest_subarray(vec![1, 1, 1])); // 2 You must delete one element.
    }

    #[test]
    fn test_lowest_common_ancestor() {
        println!(
            "{}",
            lowest_common_ancestor(
                vec![
                    (1, 2),
                    (1, 3),
                    (2, 7),
                    (2, 8),
                    (3, 4),
                    (3, 5),
                    (3, 6),
                    (7, 9),
                    (7, 10),
                    (8, 11),
                ],
                7,
                11,
                11,
            )
        ); // 2
    }

    #[test]
    fn test_find_closest_elements() {
        println!("{:?}", find_closest_elements(vec![1, 2, 3, 4, 5], 4, 3)); // [1,2,3,4]
        println!("{:?}", find_closest_elements(vec![1, 2, 3, 4, 5], 4, -1)); // [1,2,3,4]
    }

    #[test]
    fn test_normalize_spaces() {
        println!(
            "{}",
            normalize_spaces("    so  me    string     ".to_string())
        ); // 'so me string'
    }

    #[test]
    fn test_palindromes() {
        println!("{}", palindrome("Не дебил - и беден?".to_string())); // true
        println!("{}", palindrome("Ум за рамки - к маразму!".to_string())); // true
        println!("{}", palindrome("Мы доломали! Сила молодыМ!".to_string())); // true
        println!(
            "{}",
            palindrome("А мудила в НИИ - инвалид умА!".to_string())
        ); // true
        println!("{}", palindrome("Улыбок тебе, дед Макар!".to_string())); // false
    }

    #[test]
    fn test_max_guests() {
        println!("{}", max_guests(vec![])); // 0
        println!("{}", max_guests(vec![(1, 2)])); // 1
        println!("{}", max_guests(vec![(1, 2), (2, 3)])); // 2
        println!("{}", max_guests(vec![(1, 5), (0, 1), (4, 5)])); // 2
    }

    #[test]
    fn test_filter_by_another() {
        println!(
            "{:?}",
            filter_by_another(vec![1, 3, 5, 7, 8, 9], vec![1, 2, 6, 8, 9])
        ); // [3, 5, 7]
    }

    #[test]
    fn test_is_one_edit_distance() {
        println!(
            "{:?}",
            is_one_edit_distance("ab".to_string(), "acb".to_string())
        ); // true
        println!(
            "{:?}",
            is_one_edit_distance("cab".to_string(), "ad".to_string())
        ); // false
        println!(
            "{:?}",
            is_one_edit_distance("1203".to_string(), "1213".to_string())
        ); // true
        println!(
            "{:?}",
            is_one_edit_distance("12145".to_string(), "1213".to_string())
        ); // false
    }

    #[test]
    fn test_is_subsequence() {
        println!(
            "{}",
            is_subsequence("abc".to_string(), "ahbgdc".to_string())
        ); // true
        println!(
            "{}",
            is_subsequence("axc".to_string(), "ahbgdc".to_string())
        ); // false
        println!("{}", is_subsequence("".to_string(), "ahbgdc".to_string())); // true
    }

    #[test]
    fn test_move_zeroes() {
        let mut vec1 = vec![0, 1, 0, 3, 12];
        move_zeroes(&mut vec1);
        println!("{:?}", vec1);
        let mut vec2 = vec![0];
        move_zeroes(&mut vec2);
        println!("{:?}", vec2);
        let mut vec3 = vec![3, 1, 2, 0, 0];
        move_zeroes(&mut vec3);
        println!("{:?}", vec3);
    }

    #[test]
    fn test_urlify() {
        println!("{}", urlify("Mr John Smith    ".to_string(), 13));
        println!("{}", urlify(" my  ur l           ".to_string(), 9));
    }

    #[test]
    fn test_summary_ranges() {
        println!("{:?}", summary_ranges(vec![0, 1, 2, 4, 5, 7])); // ["0->2","4->5","7"]
        println!("{:?}", summary_ranges(vec![0, 2, 3, 4, 6, 8, 9])); // ["0","2->4","6","8->9"]
        println!("{:?}", summary_ranges(vec![])); // []
    }

    #[test]
    fn test_charts_sum() {
        println!(
            "{:?}",
            charts_sum(vec![(1, 2), (4, 1)], vec![(2, 4), (3, 6), (5, 7)])
        ); // [(1, 2), (2, 6), (3, 8), (4, 7), (5, 8)]
    }
}
