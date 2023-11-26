#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let first_len = _first_list.len();
    let second_len = _second_list.len();
    if first_len == second_len {
        match is_first_sublist_of_second(_first_list, _second_list) {
            true => return Comparison::Equal,
            _ => return Comparison::Unequal,
        }
    }

    if first_len < second_len {
        match is_first_sublist_of_second(_first_list, _second_list) {
           true => return Comparison::Sublist,
           _ => return Comparison::Unequal, 
        }
    }

    match is_first_sublist_of_second(_second_list, _first_list) {
       true => return Comparison::Superlist,
       _ => return Comparison::Unequal, 
    }
}

pub fn is_first_sublist_of_second<T: PartialEq>(first: &[T], second: &[T]) -> bool {
    if first.len() == 0 {
        return true;
    }

    let mut i = 0;
    while i < second.len() {
        if second[i] == first[0] {
            if second.len() - i < first.len() {
                return false;
            }
            if &second[i + 1..i + first.len()] == &first[1..] {
                return true;
            }
        }
        i += 1;
    }

    false
}
