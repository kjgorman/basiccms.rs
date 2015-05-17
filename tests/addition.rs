extern crate basiccms;

#[cfg(test)]
mod tests {

    use basiccms::*;

    #[test]
    #[should_panic]
    fn you_cannot_add_two_sketches_together_if_they_have_different_hashers () {
        let mut left  = Sketch::new(0.0001, 0.99);
        let mut right =  Sketch::new(0.0001, 0.99);

        left.add(1); right.add(1);

        let mut third = &left + &right;

        third.point(1);
    }

    #[test]
    fn but_you_can_add_together_two_sketches_from_a_common_base () {
        let mut left  = Sketch::new(0.0001, 0.99);
        let mut right = left.clone();

        left.add(1);
        right.add(1);

        let mut third = &left + &right;

        assert_eq!(1, left.point(1));
        assert_eq!(1, right.point(1));
        assert_eq!(2, third.point(1));
    }
}
