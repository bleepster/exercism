#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    match (
        _first_list.len() < _second_list.len() && &_second_list[0.._first_list.len()] == _first_list,
        _second_list.len() < _first_list.len() && &_first_list[0.._second_list.len()] == _second_list,
        _first_list == _second_list,
    ) {
        (true, _, _) => Comparison::Sublist,
        (false, true, _) => Comparison::Superlist,
        (false, false, true) => Comparison::Equal,
        _ => Comparison::Unequal,
    }
}
