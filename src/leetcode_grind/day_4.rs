use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

use lazy_static::lazy_static;
use rand::Rng;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[allow(dead_code)]
pub fn int_to_roman(mut num: i32) -> String {
    let lookup = vec![
        ("M", 1000),
        ("CM", 900),
        ("D", 500),
        ("CD", 400),
        ("C", 100),
        ("XC", 90),
        ("L", 50),
        ("XL", 40),
        ("X", 10),
        ("IX", 9),
        ("V", 5),
        ("IV", 4),
        ("I", 1),
    ];

    let mut res = String::new();
    for i in &lookup {
        while num >= i.1 {
            res.push_str(i.0);
            num -= i.1;
        }
    }
    res
}

#[allow(dead_code)]
pub fn number_to_words(mut num: i32) -> String {
    const WORDS: [&str; 28] = [
        "Zero",
        "One",
        "Two",
        "Three",
        "Four",
        "Five",
        "Six",
        "Seven",
        "Eight",
        "Nine",
        "Ten",
        "Eleven",
        "Twelve",
        "Thirteen",
        "Fourteen",
        "Fifteen",
        "Sixteen",
        "Seventeen",
        "Eighteen",
        "Nineteen",
        "Twenty",
        "Thirty",
        "Forty",
        "Fifty",
        "Sixty",
        "Seventy",
        "Eighty",
        "Ninety",
    ];
    const GROUPS: [(i32, &str); 3] = [
        (1_000_000_000, " Billion"),
        (1_000_000, " Million"),
        (1_000, " Thousand"),
    ];

    fn do_hundreds(mut num: i32, res: &mut String) {
        if num >= 100 {
            res.push_str(WORDS[(num / 100) as usize]);
            res.push_str(" Hundred");
            num %= 100;
            if num == 0 {
                return;
            }
            res.push(' ');
        }
        if num > 20 {
            res.push_str(WORDS[(20 + (num - 20) / 10) as usize]);
            if num % 10 != 0 {
                res.push(' ');
                res.push_str(WORDS[(num % 10) as usize]);
            }
        } else {
            res.push_str(WORDS[num as usize]);
        }
    }

    let mut res = String::new();
    for group in GROUPS {
        if num >= group.0 {
            do_hundreds(num / group.0, &mut res);
            res.push_str(group.1);
            num %= group.0;
            if num == 0 {
                return res;
            }
            res.push(' ');
        }
    }
    do_hundreds(num, &mut res);
    res
}

#[allow(dead_code)]
pub fn merge_trees(
    root1: Option<Rc<RefCell<TreeNode>>>,
    root2: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if root1.is_none() {
        return root2;
    }
    if root2.is_none() {
        return root1;
    }
    let root1 = root1.unwrap();
    let root2 = root2.unwrap();
    root1.borrow_mut().val += root2.borrow().val;
    let mut root1_ref = root1.borrow_mut();
    let root2_ref = root2.borrow();
    root1_ref.left = merge_trees(root1_ref.left.clone(), root2_ref.left.clone());
    root1_ref.right = merge_trees(root1_ref.right.clone(), root2_ref.right.clone());
    Some(root1.clone())
}

/// ```
/// use leetcode::day_4::letter_combinations;
/// assert_eq!(3*3, letter_combinations("23".to_owned()).len());
/// ```
#[allow(dead_code)]
pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return vec![];
    }

    use std::collections::HashMap;
    let lookup = HashMap::from([
        ('2', "abc".chars().collect::<Vec<_>>()),
        ('3', "def".chars().collect::<Vec<_>>()),
        ('4', "ghi".chars().collect::<Vec<_>>()),
        ('5', "jkl".chars().collect::<Vec<_>>()),
        ('6', "mno".chars().collect::<Vec<_>>()),
        ('7', "pqrs".chars().collect::<Vec<_>>()),
        ('8', "tuv".chars().collect::<Vec<_>>()),
        ('9', "wxyz".chars().collect::<Vec<_>>()),
    ]);

    fn permute(
        digits: &Vec<char>,
        permutation: &mut String,
        pressed: usize,
        res: &mut Vec<String>,
        lookup: &HashMap<char, Vec<char>>,
    ) {
        if permutation.len() == digits.len() {
            res.push(permutation.to_owned());
            return;
        }

        let letters = lookup.get(&digits[pressed]).unwrap();
        for ch in letters {
            permutation.push(*ch);
            permute(digits, permutation, pressed + 1, res, lookup);
            permutation.pop();
        }
    }

    let mut res = vec![];
    permute(
        &digits.chars().collect::<Vec<_>>(),
        &mut String::new(),
        0,
        &mut res,
        &lookup,
    );
    res
}

/// ```
/// use leetcode::day_4::is_valid;
/// assert_eq!(is_valid("{}[]()".to_owned()), true)
/// ```
#[allow(dead_code)]
pub fn is_valid(s: String) -> bool {
    let mut stack = vec![];
    for x in s.chars() {
        if let false = match x {
            x @ ('(' | '{' | '[') => {
                stack.push(x);
                true
            }
            ')' if !stack.is_empty() => '(' == stack.pop().unwrap(),
            ']' if !stack.is_empty() => '[' == stack.pop().unwrap(),
            '}' if !stack.is_empty() => '{' == stack.pop().unwrap(),
            _ => false,
        } {
            return false;
        }
    }
    stack.is_empty()
}

/**
 * Forward declaration of guess API
 * @param  num   your gues
 * @return 	     -1 if num is higher than the picked numbe
 *    		      1 if num is lower than the picked number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

lazy_static! {
    static ref S: i32 = rand::thread_rng().gen_range(1..=10);
}

#[allow(dead_code)]
unsafe fn guess(num: i32) -> i32 {
    match num.cmp(&S) {
        Ordering::Greater => -1,
        Ordering::Equal => 0,
        Ordering::Less => 1,
    }
}

#[allow(dead_code)]
unsafe fn guess_number(n: i32) -> i32 {
    let mut low = 1;
    let mut high = n;

    while low <= high {
        let mid = low + (high - low) / 2;
        match guess(mid) {
            1 => low = mid + 1,
            0 => return mid,
            -1 => high = mid - 1,
            _ => panic!(),
        };
    }
    -1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test33() {
        println!("{:?}", int_to_roman(1994));
    }

    #[test]
    fn test34() {
        println!("{:?}", number_to_words(1994));
    }

    #[test]
    fn test_35() {
        println!(
            "{:?}",
            merge_trees(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: None,
                        right: None,
                    }))),
                }))),
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
            )
        );
    }

    #[test]
    fn test36() {
        println!("{:?}", letter_combinations("4446978".to_owned()));
    }

    #[test]
    fn test37() {
        println!("{:?}", is_valid("([])({[]})".to_owned()));
    }

    #[test]
    fn test38() {
        unsafe {
            println!("hidden num: {:?}", *S);
            println!("guessed: {:?}", guess_number(10));
        }
    }
}
