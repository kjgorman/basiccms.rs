#![crate_name = "basiccms"]
#![crate_type = "lib"]

extern crate rand;

use std::hash::{ SipHasher, Hasher, Hash };

pub struct Sketch {
    width  : usize,
    depth  : usize,
    buckets: Vec<Vec<u64>>,
    hash_offsets: Vec<u64>
}

impl Sketch {

    pub fn add<T: IntoSketch>(&mut self, val: T) -> &Sketch {
        for row in 0..self.depth {
            let key = self.index(&val, row);
            let value = self.buckets[row][key];

            self.buckets[row][key] = value + 1;
        }

        self
    }

    pub fn point<T: IntoSketch>(&mut self, val: T) -> u64 {
        let mut counts = Vec::with_capacity(self.depth);

        for row in 0..self.depth {
            let key   = self.index(&val, row);
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

        let width        = (std::f64::consts::E / epsilon).ceil() as usize;
        let depth        = (1.0 / delta).ln().ceil() as usize;
        let hash_offsets = Sketch::make_hash_offsets(depth);

        Sketch {
            width: width,
            depth: depth,
            buckets: vec![vec![0; width]; depth],
            hash_offsets: hash_offsets
        }
    }

    fn index<T: IntoSketch>(&mut self, val: &T, row: usize) -> usize {
        let sketched = val.asu64();
        let offset   = self.hash_offsets[row];
        let hashed   = sketched.wrapping_mul(offset);

        (hashed % (self.width as u64)) as usize
    }

    fn make_hash_offsets(depth: usize) -> Vec<u64> {
        let mut hashers: Vec<u64> = Vec::with_capacity(depth);

        for _ in 0..depth {
            hashers.push(rand::random::<u64>());
        }

        hashers
    }
}

fn elementwise_sum(left: &Vec<Vec<u64>>, right: &Vec<Vec<u64>>) -> Vec<Vec<u64>> {
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
        assert_eq!(self.hash_offsets, other.hash_offsets);

        Sketch {
            width: self.width,
            depth: self.depth,
            hash_offsets: self.hash_offsets.clone(),
            buckets: elementwise_sum(&self.buckets, &other.buckets)
        }
    }
}

impl std::clone::Clone for Sketch {
    fn clone(&self) -> Sketch {
        Sketch {
            width: self.width,
            depth: self.depth,
            hash_offsets: self.hash_offsets.clone(),
            buckets: self.buckets.clone()
        }
    }
}

pub trait IntoSketch {
    fn asu64(&self) -> u64;
}

impl IntoSketch for u64 {
    fn asu64(&self) -> u64 {
        *self
    }
}

impl IntoSketch for String {
    fn asu64(&self) -> u64 {
        let mut hasher = SipHasher::new();
        self.hash(&mut hasher);

        hasher.finish()
    }
}

impl<'a> IntoSketch for &'a str {
    fn asu64(&self) -> u64 {
        let mut hasher = SipHasher::new();
        self.hash(&mut hasher);

        hasher.finish()
    }
}
