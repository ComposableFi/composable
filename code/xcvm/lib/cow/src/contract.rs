

use  crate::msg::*;

/// enriches `intention` with execution context to form `problem`
pub fn define(intention: &Intention) -> Problem {
    todo!()
}


/// for each solution do ranking for specific problem
pub fn rank(problems : &Problem, solutions : &[Solution] ) ->  &Solution {
    todo!()
} 

pub fn solve() {
    match_intentions();
    match_solutions();
    rank(panic!(), panic!());
}

/// match intention among each other if can solve these without solutions - basically on chain solver
pub fn match_intentions() {}

// matches solutions with external
pub fn match_solutions() {}