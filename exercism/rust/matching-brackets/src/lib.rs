use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

pub fn brackets_are_balanced(string: &str) -> bool {
    let open = HashSet::from(['[', '{', '(']);

    let close = HashMap::from([
        (']', '['),
        ('}', '{'), 
        (')', '(')
    ]);

    let mut stack = VecDeque::new();
    for c in string.chars() {
        if open.contains(&c) {
            stack.push_back(*open.get(&c).unwrap());
        }
        if close.contains_key(&c) {
            let last_bracket = stack.pop_back();
            let open_bracket = *close.get(&c).unwrap();
            if last_bracket.is_none() || last_bracket.unwrap() != open_bracket {
                return false;
            }
        }
    }
    
    stack.is_empty()
}
