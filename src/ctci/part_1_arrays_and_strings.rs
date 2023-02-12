// Реализуйте алгоритм, определяющий, все ли. символы в строке ветречаются только один раз.
// А если при этом запрещено Исnольэование дополнительны · структур данных?
fn task_1_1(str: String) -> bool {
    fn set(str: String) -> bool {
        let mut set = vec![0; 128];
        for ch in str.chars() {
            if set[ch as usize - 'a' as usize] > 0 {
                return false;
            }
            set[ch as usize - 'a' as usize] += 1;
        }
        true
    }
    fn bitmask(str: String) -> bool {
        let mut set = 0;
        for ch in str.chars() {
            if set & (1 << (ch as usize - 'a' as usize)) > 0 {
                return false;
            }
            set |= 1 << (ch as usize - 'a' as usize);
        }
        true
    }
    bitmask(str)
}

// Для двух строк напишите метод определяющий является ли одна строка перестановкой другой
fn task_1_2(a: String, b: String) -> bool {
    use std::collections::*;
    if a.len() != b.len() {
        return false;
    }
    let mut s = HashMap::<char, i32>::new();
    for ch in a.chars() {
        *s.entry(ch).or_insert(0) += 1;
    }
    for ch in b.chars() {
        let entry = s.entry(ch).or_insert(0);
        *entry -= 1;
        if *entry < 0 {
            return false;
        }
    }
    true
}

// напишите метод, заменяющий все пробелы в строке символами '%20'.
// можете считать, что длина строки позволяет сохранить дополнительные символы,
// а фактическая длина строки известна заранее.
fn task_1_3(str: String, n: usize) -> String {
    let spaces = str.chars().take(n).filter(|x| *x == ' ').count();
    let mut size = n + spaces * 2;
    let mut str = str.chars().collect::<Vec<_>>();
    for n in (0..n).rev() {
        if str[n] == ' ' {
            size -= 1;
            str[size] = '0';
            size -= 1;
            str[size] = '2';
            size -= 1;
            str[size] = '%';
        } else {
            size -= 1;
            str[size] = str[n];
        }
    }
    str.into_iter().collect()
}

// напишите функцию, которая проверяет,
// является ли заданная строка перестановкой палиндрома.
fn task_1_4(str: String) -> bool {
    let mut bit_set = 0i128;
    for ch in str.chars() {
        if ch != ' ' {
            bit_set ^= 1
                << (ch as i128
                    - if ch.is_lowercase() {
                        'a' as i128
                    } else {
                        'A' as i128
                    });
        }
    }
    ((bit_set - 1) & bit_set) == 0
}

// существуют три вида модифицирующих операций со строками:
// вставка сивола, удаление символа и замена символа.
// Напишите функцию, которая проверяет, находятся ли две строки
// на расстоянии одной модификации или нуля модификаций
fn task_1_5(mut a: String, mut b: String) -> bool {
    if a.len().abs_diff(b.len()) > 1 {
        return false;
    }
    if a.len() > b.len() {
        std::mem::swap(&mut a, &mut b);
    }
    let a = a.chars().collect::<Vec<_>>();
    let b = b.chars().collect::<Vec<_>>();
    let mut i1 = 0;
    let mut i2 = 0;
    let mut has_diff = false;

    while i1 < a.len() && i2 < b.len() {
        if a[i1] != b[i2] {
            if has_diff {
                return false;
            }
            has_diff = true;
            if a.len() == b.len() {
                i1 += 1; // при замене сместить указатель короткой строки
            }
        } else {
            i1 += 1; // при совпадении сместить указатель короткой строки
        }
        i2 += 1; // всегда смещать указатель длинной строки
    }
    true
}

// Реализуйте метод для выполнения простейшего сжатия строк
// с использованием счетчика повторяющихся символов.
// Например строка aabcccccaaa превращается в a2b1c5a3.
// Если сжатая строка не становится короче исходной,
// то метод возвращает исходную строку.
// Предполагается что строка состоит только из букв верхнего и нижнего регистра (a-Z)
fn task_1_6(str: String) -> String {
    let mut ans = String::new();
    let mut counter = 0;
    let str = str.chars().collect::<Vec<_>>();
    for i in 0..str.len() {
        counter += 1;
        if i + 1 >= str.len() || str[i + 1] != str[i] {
            ans.push(str[i]);
            if counter > 1 {
                ans.push_str(&format!("{}", counter));
            }
            counter = 0;
        }
    }
    if ans.len() < str.len() {
        ans
    } else {
        str.into_iter().collect()
    }
}

// Имеется изображение, представленное матрицей НхН, каждый пиксел представлен 4 байтами
// Напишите метод для поворота изображения на 90 градусов.
// Удастся ли вам выполнить эту операцию "на месте"?
fn test_1_7(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    matrix
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1_1() {
        println!("{}", task_1_1("abcde".to_string())); // true
        println!("{}", task_1_1("abcdeb".to_string())); // false
    }

    #[test]
    fn test_1_2() {
        println!("{}", task_1_2("abcde".to_string(), "edcba".to_string())); // true
        println!("{}", task_1_2("aaa".to_string(), "aa".to_string())); // false
        println!("{}", task_1_2("Dog".to_string(), "dog".to_string())); // false
        println!("{}", task_1_2("dog ".to_string(), "dog".to_string())); // false
    }

    #[test]
    fn test_1_3() {
        println!("{}", task_1_3("Mr John Smith    ".to_string(), 13)); // Mr%20John%20Smith
        println!("{}", task_1_3("Mapahash Putamadre  ".to_string(), 18)); // Mapahash%20Putamadre
    }

    #[test]
    fn test_1_4() {
        println!("{}", task_1_4("Tact Coa".to_string())); // true
        println!("{}", task_1_4("Tact Coakkk".to_string())); // false
        println!("{}", task_1_4("tactcoapapa".to_string())); // true
        println!("{}", task_1_4("racecar".to_string())); // true
    }

    #[test]
    fn test_1_5() {
        println!("{}", task_1_5("apple".to_string(), "aple".to_string())); // true
        println!("{}", task_1_5("aple".to_string(), "apple".to_string())); // true
        println!("{}", task_1_5("pale".to_string(), "bale".to_string())); // true
        println!("{}", task_1_5("pade".to_string(), "bale".to_string())); // false
    }

    #[test]
    fn test_1_6() {
        println!("{}", task_1_6("aabcccccaaa".to_string())); // a2b1c5a3
        println!("{}", task_1_6("cccpo".to_string())); // c3po
        println!("{}", task_1_6("rrdd".to_string())); // rrdd.len() == r2d2.len()
    }

    #[test]
    fn test_1_7() {}
}
