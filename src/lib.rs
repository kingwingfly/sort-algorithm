mod solutions;

use rand::Rng;

pub trait SortSolution {
    fn describle(&self) -> String;
    fn sort(&self, input: Vec<isize>) -> Vec<isize>;
}

pub trait SortCheck: SortSolution {
    fn check(&self, input: &[isize]) {
        let mut ans = Vec::from(input);
        ans.sort();
        assert_eq!(self.sort(Vec::from(input)), ans, "{:?}", input);
    }
}

impl<T: SortSolution> SortCheck for T {}

#[derive(Default)]
pub struct Checker<'c> {
    /// solutions
    solus: Vec<&'c dyn SortCheck>,
}

impl<'c> Checker<'c> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_solu(&mut self, solu: &'c [&dyn SortCheck]) {
        self.solus.extend_from_slice(solu)
    }

    pub fn check(&self) {
        let inputs: [[isize; 10]; 10] = rand::thread_rng().gen();
        self.solus.iter().for_each(|solu| {
            println!("checking: {}", solu.describle());
            inputs.iter().for_each(|seq| solu.check(seq));
        });
    }

    pub fn check_with_input(&self, input: &[isize]) {
        self.solus.iter().for_each(|solu| {
            println!("checking: {}", solu.describle());
            solu.check(input);
        });
    }

    pub fn check_with_inputs(&self, inputs: &[&[isize]]) {
        self.solus.iter().for_each(|solu| {
            println!("checking: {}", solu.describle());
            inputs.iter().for_each(|seq| solu.check(seq));
        });
    }
}
