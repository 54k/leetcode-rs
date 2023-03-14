// todo https://leetcode.com/problems/strong-password-checker/description/
// todo https://leetcode.com/problems/replace-non-coprime-numbers-in-array/description/

// https://leetcode.com/problems/sum-root-to-leaf-numbers/
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn preorder(root: Option<Rc<RefCell<TreeNode>>>, sum: i32, total_sum: &mut i32) {
        if let Some(r) = root {
            let r = r.borrow();
            if r.left.is_none() && r.right.is_none() {
                *total_sum += 10 * sum + r.val;
            }
            preorder(r.left.clone(), 10 * sum + r.val, total_sum);
            preorder(r.right.clone(), 10 * sum + r.val, total_sum);
        }
    }
    let mut total_sum = 0;
    preorder(root, 0, &mut total_sum);
    total_sum
}

// https://leetcode.com/problems/strong-password-checker-ii/
pub fn strong_password_checker_ii(password: String) -> bool {
    if password.len() < 8 {
        return false;
    }
    const SPECIALS: &str = "!@#$%^&*()-+";
    let (mut has_lowercase, mut has_uppercase, mut has_digit, mut has_special) =
        (false, false, false, false);
    let password = password.chars().collect::<Vec<_>>();
    for i in 0..password.len() {
        if !has_lowercase && password[i].is_lowercase() {
            has_lowercase = true;
        }
        if !has_uppercase && password[i].is_uppercase() {
            has_uppercase = true;
        }
        if !has_digit && password[i].is_ascii_digit() {
            has_digit = true;
        }
        if !has_special && SPECIALS.contains(password[i]) {
            has_special = true;
        }
        if i < password.len() - 1 && password[i] == password[i + 1] {
            return false; // same char in adjacent position
        }
    }
    has_lowercase && has_uppercase && has_digit && has_special
}

// https://leetcode.com/problems/validate-ip-address/description/
pub fn valid_ip_address(query_ip: String) -> String {
    "".to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test346() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));
        println!("{}", sum_numbers(root));
    }

    #[test]
    fn test347() {
        println!(
            "{}",
            strong_password_checker_ii("Me+You--IsMyDream".to_string())
        ); // false
        println!(
            "{}",
            strong_password_checker_ii("IloveLe3tcode!".to_string())
        ); // true
    }
}
