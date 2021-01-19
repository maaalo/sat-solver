pub mod cnf;
pub mod clause;
pub mod solver;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strategy_test() {
        let cnf = cnf::read_file("./cnf/uf75-01.cnf".to_string());
        let solver = solver::Solver::new(cnf);
        assert_eq!(solver.strategy_select(&clause::Set::new(), &clause::Set::new()), (50, false));
    }
    
    #[test]
    fn test_dpll_01() {
        let cnf = cnf::read_file("./cnf/quinn.cnf".to_string());
        let mut solver = solver::Solver::new(cnf);
        assert_eq!(solver.dpll(&clause::Set::new(), &clause::Set::new()), true);
    }

    #[test]
    fn test_dpll_02() {
        let cnf = cnf::read_file("./cnf/uf75-01.cnf".to_string());
        let mut solver = solver::Solver::new(cnf);
        assert_eq!(solver.dpll(&clause::Set::new(), &clause::Set::new()), true);
    }
}
