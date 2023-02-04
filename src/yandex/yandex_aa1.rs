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
fn product(str: String, n: i32) -> String {
    todo!();
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
    fn test_longest_subarray() {
        println!("{}", longest_subarray(vec![1, 1, 0, 1])); // 3
        println!("{}", longest_subarray(vec![0, 1, 1, 1, 0, 1, 1, 0, 1])); // 5
        println!("{}", longest_subarray(vec![1, 1, 1])); // 2 You must delete one element.
    }
}
