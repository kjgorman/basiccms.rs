### basic count-min sketch

![](https://travis-ci.org/kjgorman/basiccms.rs.svg?branch=master)

It's the count-min sketch structure implemented in rust.

You can use it to count frequencies of events with some degree of
uncertainty. It uses sub-linear space, but allows for possible hash
collisions that could lead to over counting.

Conceptually, it's like a bloom filter, but for multi-sets rather than
strict sets.

In order to use it, you need to provide an implementation of the trait
`basiccms::IntoSketch` that describes how to turn some type into the
underlying `u64` used by the hash buckets.

Ostensibly we could just use an implementation like this:

```rust
impl<T: Hash> IntoSketch for T {
    fn asu64(&self) -> u64 {
        let mut hasher = SipHasher::new();
        self.hash(&mut hasher);

        hasher.finish()
    }
}
```

But that precludes occasions where we might not want an extra hashing
pass, and there's an "obvious" way to convert to a u64 (for example a
u32 might just get widened...). It's not defined in the crate itself,
because so many types implement `Hash` it would cause annoying
collisions, but no reason you can't add it yourself in another scope.

Once you've actually got a method of putting your data into the sketch
by implementing the trait, you can use it heterogenuously if you feel
like it (unlike, say sketchy):

```rust
extern crate basiccms;

use basiccms::Sketch;

#[test]
fn we_should_be_able_to_add_heterogenuously () {
        let mut sketch = Sketch::new(0.0001, 0.8);

        sketch.add(1); sketch.add(2); sketch.add(3);
        sketch.add("foo"); sketch.add("bar"); sketch.add("quux");

        assert_eq!(1, sketch.point("foo"));
        sketch.add("foo");

        assert_eq!(2, sketch.point("foo"));
}

```

At the moment all that's offered is the `sketch.point` point query
method, which returns the class "min count" frequency estimation.

Patches welcome!