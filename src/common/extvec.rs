use std::{uint};

pub fn zip_default<T: Copy, U: Copy>(v1: &[T], v2: &[U], def: (T, U)) -> ~[(T, U)] {
    let mut result = ~[];
    let (l1, l2) = (v1.len(), v2.len());
    let (d1, d2) = def;
    for uint::range(0, uint::max(l1, l2)) |i| {
        let e1 = if i < l1 { copy v1[i] } else { copy d1 };
        let e2 = if i < l2 { copy v2[i] } else { copy d2 };
        result.push((e1, e2));
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zip_default() {
        assert_eq!(zip_default([1, 2, 3], [4u, 5u, 6u], (0, 0u)),
                   ~[(1, 4u), (2, 5u), (3, 6u)]);
        assert_eq!(zip_default([1, 2, 3], [4u], (0, 0u)),
                   ~[(1, 4u), (2, 0u), (3, 0u)]);
        assert_eq!(zip_default([], [], (0, 0u)),
                   ~[]);
    }
}
