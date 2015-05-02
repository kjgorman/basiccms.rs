extern crate basiccms;

#[cfg(test)]
mod tests {

    use basiccms::*;

    #[test]
    fn can_add_any_hashable_type () {
        let mut cms = CMS::new(0.0001, 0.8);

        cms.add(&1);
        cms.add(&"foo");
    }

    #[test]
    fn we_should_be_able_to_keep_track_of_an_addition () {
        let mut cms = CMS::new(0.0001, 0.8);

        cms.add(&1);

        assert_eq!(1, cms.point(&1));
    }
}
