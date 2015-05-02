### basic count-min sketch

It's the count-min sketch structure implemented in rust.

You can use it to count frequencies of events with some degree of
uncertainty. It uses sub-linear space, but allows for possible hash
collisions that could lead to over counting.

Conceptually, it's like a bloom filter, but for multi-sets rather than
strict sets.

It's still a work in progess, in particular the hashing step just
creates a new `SipHasher` for every insert, when there are some
stronger guarantees about randomness that need to hold for some of the
papers presented guarantees around error bounds to hold (although
apparently it [might not actually need to be that
strong](https://github.com/ezyang/ocaml-cminsketch/blob/master/cminsketch.ml#L16-L19)). On
the flipside, that means you can use it with anything that implements
`Hash`, like this:

```
extern crate basiccms;

use basiccms::Sketch;

#[test]
fn we_should_be_able_to_add_heterogenuously () {
    let mut sketch = Sketch::new(0.0001, 0.8);

    sketch.add(&1); sketch.add(&2); sketch.add(&3);
    sketch.add(&"foo"); sketch.add(&"bar"); sketch.add(&"quux");

    assert_eq!(1, sketch.point(&"foo"));

    sketch.add(&"foo");

     assert_eq!(2, sketch.point(&"foo"));
}

```