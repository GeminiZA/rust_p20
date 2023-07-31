struct Solution;

use std::collections::LinkedList;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let chars: Vec<char> = s.chars().collect();
        let mut stack: LinkedList<char> = LinkedList::new();
        for c in chars {
            if c == '(' || c == '{' || c == '[' {
                stack.push_front(c);
            } else {
                if let Some(cur) = stack.pop_front() {
                    if c == ')' && cur != '(' {
                        return false;
                    }
                    if c == '}' && cur != '{' {
                        return false;
                    }
                    if c == ']' && cur != '[' {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
        if stack.len() > 0 {
            return false;
        } else {
        return true; 
        }
    }
}

fn main() {
    let s: String = "(]".to_string();
    let result = Solution::is_valid(s);
    println!("{}", result);
}
