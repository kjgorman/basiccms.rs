#![crate_name = "basiccms"]
#![crate_type = "lib"]

pub struct CMS {
    width: isize,
    depth: isize
}

impl CMS {
    pub fn new(epsilon: f64, delta: f64) -> CMS {
        if epsilon < 0.0 {
            panic!("CMS: epsilon must be positive");
        }

        if delta < 0.0  || delta > 1.0 {
            panic!("CMS: delta must be in (0.0, 1.0)");
        }

        CMS {
            width: 0,
            depth: 0
        }
    }
}
