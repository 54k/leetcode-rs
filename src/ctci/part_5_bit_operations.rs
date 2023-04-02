use std::cmp::max;
use crate::leetcode_grind::day_110::find_length_of_lcis;
use crate::leetcode_grind::day_16::check_if_pangram;

// даны два 32 разрядных числа N и M и две позиции битов i и j.
// Напишите метод для вставки M и N так, чтобы число М занимало позиции с бита j по бит i.
fn task_5_1(n: i32, m: i32, i: i32, j: i32) -> i32 {
    let all_ones = !0;
    let left = all_ones << (j + 1);
    let right = (1 << i) - 1;
    let mask = left | right;
    let n_cleared = n & mask;
    let m_shifted = m << i;
    n_cleared | m_shifted
}

// Дано вещетвенное число в интервале между 0 и 1 (например 0,72), которое
// передается в формате double. Выведите его двоиченое представление.
// Если для точного двоичного представления числа не хватает 32 разрядов, выведите сообщение об ошибке.
fn task_5_2(mut num: f64) -> String {
    fn print_binary_1(mut num: f64) -> String {
        if num >= 1. || num <= 0. {
            return "error".to_string();
        }
        let mut result = String::new();
        result.push('.');
        while num > 0. {
            let r = num * 2.;
            if r >= 1. {
                result.push('1');
                num = r - 1.;
            } else {
                result.push('0');
                num = r;
            }
        }
        result
    }
    fn print_binary_2(mut num: f64) -> String {
        if num >= 1. || num <= 0. {
            return "error".to_string();
        }
        let mut result = ".".to_string();
        let mut frac = 0.5;
        while num > 0. {
            if result.len() > 32 {
                return "error".to_string();
            }
            if num >= frac {
                result.push('1');
                num -= frac;
            } else {
                result.push('0');
            }
            frac /= 2.0;
        }
        result
    }
    print_binary_1(num)
}

// Имеется число, в котором можно изменить ровно один бит из 0 в 1.
// Напишите код для определения длины самой длинной последовательности единиц,
// которая может быть при этом полученя
fn task_5_3(num: i32) -> i32 {
    fn longest_seq_1(num: i32) -> i32 {
        fn get_alternating_sequence(mut num: i32) -> Vec<i32> {
            let mut seq = vec![];
            let mut counter = 0;
            let mut cur_seq = 0;
            for i in 0..32 {
                if num & 1 != cur_seq {
                    seq.push(counter);
                    counter = 0;
                    cur_seq = num & 1;
                }
                counter += 1;
                num >>= 1;
            }
            seq
        }
        fn find_longest_subseq(seq: Vec<i32>) -> i32 {
            let mut max_seq = 1;
            for i in (0..seq.len()).step_by(2) {
                let zero_seq = seq[i];
                let ones_seq_left = if i > 0 { seq[i - 1] } else { 0 };
                let ones_seq_right = if i < seq.len() - 1 { seq[i + 1] } else { 0 };

                let mut this_seq = 0;
                if zero_seq == 1 {
                    this_seq = ones_seq_left + 1 + ones_seq_right;
                } else if zero_seq > 1 {
                    this_seq = 1 + ones_seq_right.max(ones_seq_left)
                } else if zero_seq == 0 {
                    this_seq = ones_seq_right.max(ones_seq_left)
                }
                max_seq = max_seq.max(this_seq);
            }
            max_seq
        }
        find_longest_subseq(get_alternating_sequence(num))
    }
    fn longest_seq_2(mut num: i32) -> i32 {
        if !num == 0 { return 31; }
        let mut current_len = 0;
        let mut prev_len = 0;
        let mut max_length = 1;
        while num != 0 {
            if num & 1 == 1 {
                current_len += 1;
            } else if num & 1 == 0 {
                prev_len = if num & 2 == 0 { 0 } else { current_len };
                current_len = 0;
            }
            max_length = (prev_len + current_len + 1).max(max_length);
            num >>= 1;
        }
        max_length
    }
    longest_seq_2(num)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_task_5_1() {
        println!("{}", task_5_1(1 << 10, 1 << 4 + 3, 2, 6));
    }

    #[test]
    fn test_task_5_2() {
        println!("{}", task_5_2(0.72));
    }

    #[test]
    fn test_task_5_3() {
        println!("{}", task_5_3(1775));
    }
}