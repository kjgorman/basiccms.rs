#![crate_name = "basiccms"]
#![crate_type = "lib"]

use std::hash::{ SipHasher, Hasher, Hash };
use std::iter::{ repeat };

pub struct CMS {
    width  : usize,
    depth  : usize,
    buckets: Vec<Vec<usize>>,
}

impl CMS {

    pub fn add<T: Hash>(&mut self, val: T) -> &CMS {
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

    fn index<T: Hash>(&mut self, val: &T) -> usize {
        // TODO(kjgorman): this shouldn't create a new hasher each time...
        let mut hasher = SipHasher::new();
        val.hash(&mut hasher);

        let hash  = hasher.finish();

        (hash % (self.width as u64)) as usize
    }

    pub fn new(epsilon: f64, delta: f64) -> CMS {
        if epsilon < 0.0 {
            panic!("CMS: epsilon must be positive");
        }

        if delta < 0.0  || delta > 1.0 {
            panic!("CMS: delta must be in (0.0, 1.0)");
        }

        let width   = (std::f64::consts::E / epsilon).ceil() as usize;
        let depth   = (1.0 / delta).ln().ceil() as usize;
        let buckets = make_buckets(width, depth);

        CMS {
            width: width,
            depth: depth,
            buckets: buckets
        }
    }
}

fn make_buckets(width: usize, depth: usize) -> Vec<Vec<usize>> {
    let mut rows: Vec<Vec<usize>> = Vec::with_capacity(depth);

    for _ in 0..depth {
        rows.push(repeat(0).take(width).collect());
    }

    rows
}
