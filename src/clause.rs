use std::collections::BTreeSet;

pub type Set = BTreeSet<usize>;
pub type Bag = Vec<usize>;

#[derive(Debug, Clone, Default)]
pub struct Clause {
    pub t: Bag,
    pub f: Bag,
}

impl Clause {
    pub fn new() -> Clause {
        Clause {
            t: vec![],
            f: vec![],
        }
    }

    pub fn is_empty(&self, t: &Set, f: &Set) -> bool {
        // if self.t.len() == 0 && self.f.len() == 0 { return true; }

        for t_e in self.t.iter() {
            if !f.contains(t_e) { return false; }
        }
        for f_e in self.f.iter() {
            if !t.contains(f_e) { return false; }
        }
        true
    }

    pub fn find_unit_clause(&self) -> (usize, bool) {
        if self.t.len() + self.f.len() == 1 {
            if self.t.len() == 1 {
                return (self.t[0], true)
            } else {
                return (self.f[0], false)
            }
        }
        (0, true)
    }
}
