pub trait Mutations {
    fn get_mutated_genes(&self) -> Vec<i32>;
}

pub struct MutantA;
impl Mutations for MutantA {
    fn get_mutated_genes(&self) -> Vec<i32> {
        let mutated_genes = vec![0,1];
        mutated_genes
    }
}

pub struct MutantB;
impl Mutations for MutantB {
    fn get_mutated_genes(&self) -> Vec<i32> {
        let mutated_genes = vec![2,3];
        mutated_genes
    }
}