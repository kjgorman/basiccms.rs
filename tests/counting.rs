extern crate basiccms;

#[cfg(test)]
mod tests {

    use basiccms::*;

    #[test]
    fn can_add_any_hashable_type () {
        let mut sketch = Sketch::new(0.0001, 0.8);

        sketch.add(&1);
        sketch.add(&"foo");
    }

    #[test]
    fn we_should_be_able_to_keep_track_of_an_addition () {
        let mut sketch = Sketch::new(0.0001, 0.8);

        sketch.add(&1);

        assert_eq!(1, sketch.point(&1));
    }

    #[test]
    fn we_should_be_able_to_add_heterogenuously () {
        let mut sketch = Sketch::new(0.0001, 0.8);

        sketch.add(&1); sketch.add(&2); sketch.add(&3);
        sketch.add(&"foo"); sketch.add(&"bar"); sketch.add(&"quux");

        assert_eq!(1, sketch.point(&"foo"));
        sketch.add(&"foo");

        assert_eq!(2, sketch.point(&"foo"));
    }

    #[test]
    fn we_should_be_able_to_add_very_many_elements () {
        let mut sketch = Sketch::new(0.0001, 0.8);

        for n in 1..10000 {
            sketch.add(&n);
        }

        assert!(sketch.point(&5000) > 0);
        assert_eq!(0, sketch.point(&0));
    }

    #[test]
    fn we_should_be_able_to_count_repetitions () {
        let mut sketch = Sketch::new(0.0001, 0.8);

        sketch.add(&1);
        sketch.add(&1);
        sketch.add(&1);
        sketch.add(&1);

        assert_eq!(4, sketch.point(&1));
    }
}
