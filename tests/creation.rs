extern crate basiccms;

#[cfg(test)]
mod tests {

    use basiccms::*;

    #[test]
    #[should_panic]
    fn rejects_negative_epsilon () {
        CMS::new(-1.0, 0.0);
    }

    #[test]
    #[should_panic]
    fn rejects_negative_delta () {
        CMS::new(1.0, -1.0);
    }

    #[test]
    #[should_panic]
    fn rejects_delta_above_one () {
        CMS::new(1.0, 2.0);
    }

}
