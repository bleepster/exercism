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

        let mut i_self = self.iter();
        let mut i_other = other.iter();

        let mut v_self = i_self.next();
        let mut v_sliced = i_other.next();

        loop {
            if v_sliced == None {
                break;
            }

            if v_self != None && (v_self.unwrap() == v_sliced.unwrap()) {
                v_self = i_self.next();
            }

            v_sliced = i_other.next();
        }

        if v_self == None {
            return true;
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
