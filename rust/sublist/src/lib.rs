use std::cmp::Ordering;

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

impl<T: PartialEq> Compare for &[T] {
    fn is_sublist(&self, other: &Self) -> bool {
        other.windows(self.len()).any(|other| &other == self)
    }
}

pub fn sublist<T: PartialEq>(first: &[T], second: &[T]) -> Comparison {
    match first.len().cmp(&second.len()) {
        Ordering::Equal => {
            if first == second {
                return Comparison::Equal;
            }
        }
        Ordering::Less => {
            if first.is_empty() || first.is_sublist(&second) {
                return Comparison::Sublist;
            }
        }
        Ordering::Greater => {
            if second.is_empty() || second.is_sublist(&first) {
                return Comparison::Superlist;
            }
        }
    }
    return Comparison::Unequal;
}
