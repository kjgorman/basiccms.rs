extern crate basiccms;

#[cfg(test)]
mod tests {

    use basiccms::*;

    #[test]
    fn you_can_add_two_sketches_together () {
        let mut left  = Sketch::new(0.0001, 0.8);
        let mut right =  Sketch::new(0.0001, 0.8);

        left.add(&1);
        right.add(&1);

        let mut third = &left + &right;

        assert_eq!(1, left.point(&1));
        assert_eq!(1, right.point(&1));
        assert_eq!(2, third.point(&1));
    }
}
