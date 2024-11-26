#![allow(unused)]

use crate::SortSolution;

struct StdSort;

impl SortSolution for StdSort {
    fn describle(&self) -> String {
        "std::sort".to_string()
    }

    fn sort(&self, mut input: Vec<isize>) -> Vec<isize> {
        input.sort();
        input
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Checker;

    #[test]
    fn test() {
        let mut chcker = Checker::new();
        chcker.add_solu(&StdSort);
        chcker.check();
    }
}
