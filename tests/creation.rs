extern crate basiccms;

#[cfg(test)]
mod tests {

    use basiccms::*;

    #[test]
    #[should_panic]
    fn rejects_negative_epsilon () {
        Sketch::new(-1.0, 0.0);
    }

    #[test]
    #[should_panic]
    fn rejects_negative_delta () {
        Sketch::new(1.0, -1.0);
    }

    #[test]
    #[should_panic]
    fn rejects_delta_above_one () {
        Sketch::new(1.0, 2.0);
    }

}
