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

        let pos = other.iter().position(|&v| v == self[0]);
        if pos == None {
            return false;
        }

        let start = pos.unwrap();
        let end = self.len() + start;
        let slice = &other[start..end];

        slice.iter().eq(self.iter())
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
