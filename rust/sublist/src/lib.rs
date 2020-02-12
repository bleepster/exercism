#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

trait Compare {
    fn is_sublist(&self, other: &Self) -> bool;
}

impl<T: PartialEq + Copy> Compare for &[T] {
    fn is_sublist(&self, other: &Self) -> bool {
        if self.len() == 0 {
            return true;
        }

        if self.len() > other.len() {
            return false;
        }

        for other_index in 0..other.len() {
            let mut is_list = true;
            let mut i = 0;
            for self_index in 0..self.len() {
                if other_index + i >= other.len() || self[self_index] != other[other_index + i] {
                    is_list = false;
                    break;
                }
                i = i + 1;
            }

            if is_list {
                return true;
            }
        }

        false
    }
}

pub fn sublist<T: PartialEq + Copy>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    match (
        _first_list == _second_list,
        _first_list.is_sublist(&_second_list),
        _second_list.is_sublist(&_first_list),
    ) {
        (true, _, _) => Comparison::Equal,
        (false, true, false) => Comparison::Sublist,
        (false, false, true) => Comparison::Superlist,
        _ => Comparison::Unequal,
    }
}
