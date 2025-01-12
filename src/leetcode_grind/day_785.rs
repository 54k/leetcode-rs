// https://leetcode.com/problems/check-if-a-parentheses-string-can-be-valid/description/?envType=daily-question&envId=2025-01-12
pub fn can_be_valid(s: String, locked: String) -> bool {
    let s = s.chars().collect::<Vec<_>>();
    let locked = locked.chars().collect::<Vec<_>>();

    if s.len() % 2 == 1 {
        return false;
    }

    let mut open = 0;
    let mut unlocked = 0;

    for i in 0..s.len() {
        if locked[i] == '0' {
            unlocked += 1;
        } else if s[i] == '(' {
            open += 1;
        } else if s[i] == ')' {
            if open > 0 {
                open -= 1;
            } else if unlocked > 0 {
                unlocked -= 1;
            } else {
                return false;
            }
        }
    }

    let mut balance = 0;
    for i in (0..s.len()).rev() {
        if locked[i] == '0' {
            balance -= 1;
            unlocked -= 1;
        } else if s[i] == '(' {
            balance += 1;
            open -= 1;
        } else if s[i] == ')' {
            balance -= 1;
        }

        if balance > 0 {
            return false;
        }

        if unlocked == 0 && open == 0 {
            break;
        }
    }

    if open > 0 {
        return false;
    }
    true
}
