use crate::clause::Set;
use crate::cnf;

pub struct Solver {
    depth: usize,
    history: Vec<u32>,
	cnf: cnf::Cnf,
}

impl Solver {
    pub fn new(cnf: cnf::Cnf) -> Solver {
        Solver { 
            depth: 0,
            history: Vec::with_capacity(cnf.length),
			cnf: cnf,
        }
    }

    pub fn dpll(&mut self, t: &Set, f: &Set) -> bool {
        self.depth = self.history.len();
        let mut t = t.clone();
        let mut f = f.clone();

        let mut flag: bool = true;
        while flag {
            flag = false;
            
            // find unit clause
            for (idx, clause) in self.cnf.formula.iter_mut().enumerate() {
                if self.cnf.mask[idx] == true { continue }
                let (l, b) = clause.find_unit_clause();
                if l > 0 { 
                    if b { t.insert(l); } else {f.insert(l); }
                    self.assign(l, b); 
                    flag = true;
                    break;
                } 
            }

            // find pure variable
            // for idx in cnf.variables() {
            //     let t_b = cnf.count_t[idx] > 0;
            //     let f_b = cnf.count_f[idx] > 0;
            //     if t_b && !f_b && !f.contains(idx) { t.insert(idx); flag = true; }
            //     if f_b && !t_b && !t.contains(idx) { f.insert(idx); flag = true; }
            // }
        }

        if self.is_empty() { 
            return true 
        }
        if self.include_empty_clause(&t, &f) { 
            self.backtrack();
            return false
        }

        let (l, b) = self.strategy_select(&t, &f);

        self.assign(l, b);

        if b { t.insert(l); } else { f.insert(l); }
        self.down();
        if self.dpll(&t, &f) == true { return true }
        if b { t.remove(&l); } else { f.remove(&l); }
        self.up();
        
        self.assign(l, !b);

        if !b { t.insert(l); } else { f.insert(l); }
        self.down();
        if self.dpll(&t, &f) == true { return true }
        f.remove(&l);
        if !b { t.remove(&l); } else { f.remove(&l); }
        self.up();

        self.backtrack();
        false
    }

    pub fn strategy_select(&self, t: &Set, f: &Set) -> (usize, bool) {
        let mut max: u16 = 0;
        let mut result: usize = 0;
        for idx in self.cnf.variables() {
            let item = self.cnf.count_t[idx] + self.cnf.count_f[idx];
            if item > max && !t.contains(&idx) && !f.contains(&idx) {
                max = item;
                result = idx;
            }
        }
        if self.cnf.count_t[result] >= self.cnf.count_f[result] {
            return (result, true) 
        } else {
            return (result, false) 
        }
    }
    
    pub fn assign(&mut self, l: usize, b: bool) {
        if b == true {
            for (idx, c) in self.cnf.formula.iter_mut().enumerate() {
                if self.cnf.mask[idx] == true { continue; }
                
                // Mask all clcauses including l(true)
                if c.t.contains(&l) { 
                    self.cnf.mask[idx] = true; 
                    self.history.push(idx as u32);
                    continue;
                } 
            }
        } else {
            for (idx, c) in self.cnf.formula.iter_mut().enumerate() {
                if self.cnf.mask[idx] == true { continue; }
                
                // Mask all clcauses including l(false)
                if c.f.contains(&l) { 
                    self.cnf.mask[idx] = true; 
                    self.history.push(idx as u32);
                    continue;
                } 
            }
        }
    }

    fn is_empty(&self) -> bool {
        self.history.len() == self.cnf.length
    }

    fn include_empty_clause(&self, t: &Set, f: &Set) -> bool {
        for (idx, c) in self.cnf.formula.iter().enumerate() {
            if self.cnf.mask[idx] == true { continue }
            if c.is_empty(&t, &f) == true { return true }
        }
        false
    }

    fn backtrack(&mut self) {
        for idx in self.history[self.depth..].iter() {
            let idx = *idx as usize;
            self.cnf.mask[idx] = false;
            for t_e in &self.cnf.formula[idx].t {
                self.cnf.count_t[*t_e] += 1;
            }
            for t_f in &self.cnf.formula[idx].f {
                self.cnf.count_f[*t_f] += 1;
            }
        }
        self.history.truncate(self.depth);
    }

    fn down(&mut self) {
        self.depth += 1;
    }

    fn up(&mut self) {
        self.depth -= 1;
    }
}
