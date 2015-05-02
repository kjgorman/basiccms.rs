#![crate_name = "basiccms"]
#![crate_type = "lib"]

use std::hash::{ SipHasher, Hasher, Hash };
use std::iter::{ repeat };

pub struct Sketch {
    width  : usize,
    depth  : usize,
    buckets: Vec<Vec<usize>>,
}

impl Sketch {

    pub fn add<T: Hash>(&mut self, val: T) -> &Sketch {
        for row in 0..self.depth {
            let key = self.index(&val);
            let value = self.buckets[row][key];

            self.buckets[row][key] = value + 1;
        }

        self
    }

    pub fn point<T: Hash>(&mut self, val: T) -> usize {
        let mut counts = Vec::with_capacity(self.depth);

        for row in 0..self.depth {
            let key   = self.index(&val);
            let value = self.buckets[row][key];
            counts.push(value);
        }

        *counts.iter().min().unwrap()
    }

   pub fn new(epsilon: f64, delta: f64) -> Sketch {
        if epsilon < 0.0 {
            panic!("CMS: epsilon must be positive");
        }

        if delta < 0.0  || delta > 1.0 {
            panic!("CMS: delta must be in (0.0, 1.0)");
        }

        let width   = (std::f64::consts::E / epsilon).ceil() as usize;
        let depth   = (1.0 / delta).ln().ceil() as usize;
        let buckets = Sketch::make_buckets(width, depth);

        Sketch {
            width: width,
            depth: depth,
            buckets: buckets
        }
    }

    fn index<T: Hash>(&mut self, val: &T) -> usize {
        // TODO(kjgorman): this shouldn't create a new hasher each time...
        let mut hasher = SipHasher::new();
        val.hash(&mut hasher);

        let hash  = hasher.finish();

        (hash % (self.width as u64)) as usize
    }

    fn make_buckets(width: usize, depth: usize) -> Vec<Vec<usize>> {
        let mut rows: Vec<Vec<usize>> = Vec::with_capacity(depth);

        for _ in 0..depth {
            rows.push(repeat(0).take(width).collect());
        }

        rows
    }
}

fn elementwise_sum(left: &Vec<Vec<usize>>, right: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| l.iter().zip(r.iter()).map(|(x, y)| x + y).collect())
        .collect()
}

impl<'a> std::ops::Add<&'a Sketch> for &'a Sketch {
    type Output = Sketch;

    fn add(self, other: &'a Sketch) -> Sketch {
        assert_eq!(self.width, other.width);
        assert_eq!(self.depth, other.depth);

        Sketch {
            width: self.width,
            depth: self.depth,
            buckets: elementwise_sum(&self.buckets, &other.buckets)
        }
    }
}
