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
        let self_len = self.len();
        let other_len = other.len();

        let mut result = false;
        if self_len == 0 {
            result = true;
        }
        else {
            for start_index in 0..other_len {
                let slice_max = start_index + self_len;
                if slice_max > other_len {
                    break;
                }

                let other_slice = &other[start_index..slice_max];
                if self.iter().eq(other_slice.iter()) {
                    result = true;
                    break;
                }
            }
        }
        result
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
