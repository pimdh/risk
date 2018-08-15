extern crate itertools;

use itertools::Itertools;
use std::ops::Range;
use itertools::structs::MultiProduct;

fn dice_iterator_custom(n_dice: u32, n_faces: u32) -> MultiProduct<Range<u32>> {
    (0..n_dice).map(|_i| 1..(n_faces+1)).multi_cartesian_product()
}

pub fn dice_iterator(n_dice: u32) -> MultiProduct<Range<u32>> {
    dice_iterator_custom(n_dice, 6)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterate_3dice_2_faces() {
        let v: Vec<_> = dice_iterator_custom(3, 2).collect();
        let r = vec![
            vec![1, 1, 1],
            vec![1, 1, 2],
            vec![1, 2, 1],
            vec![1, 2, 2],
            vec![2, 1, 1],
            vec![2, 1, 2],
            vec![2, 2, 1],
            vec![2, 2, 2],
        ];
        assert_eq!(v, r);
    }

    #[test]
    fn iterate_2dice_3_faces() {
        let v: Vec<_> = dice_iterator_custom(2, 3).collect();
        let r = vec![
            vec![1, 1],
            vec![1, 2],
            vec![1, 3],
            vec![2, 1],
            vec![2, 2],
            vec![2, 3],
            vec![3, 1],
            vec![3, 2],
            vec![3, 3],
        ];
        assert_eq!(v, r);
    }
}
