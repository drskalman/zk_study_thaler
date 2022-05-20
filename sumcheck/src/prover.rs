use ark_poly::{Polynomial, multivariate::SparsePolynomial,multivariate::SparseTerm, MVPolynomial};
use ark_ff::Field;

/// The struct which contains all data needed for the prover to play its part
struct Prover<F: Field> {
    h_poly: SparsePolynomial<F, SparseTerm>

}

impl <F: Field> Prover<F> {
    ///sum of the evaulation of the multivariate polynomial
    /// over all possible binary input
    pub fn evaluate_sum(&self)->F {
        self.evaluate_one_variable(&Vec::<F>::new())
    }

    ///recursive evalutation of the polynomial one variable
    ///at the time
    fn evaluate_one_variable(&self,  point_vec: &Vec<F>) -> F {
        if point_vec.len() < self.h_poly.num_vars() {
            //evaluate one variable at the time
            let mut point_vec_0 = point_vec.clone();
            point_vec_0.push(F::zero());
            let value_at_0 = self.evaluate_one_variable(&point_vec_0);

            let mut point_vec_1 = point_vec.clone();
            point_vec_1.push(F::one());
            let value_at_1 = self.evaluate_one_variable(&point_vec_1);

            value_at_0 + value_at_1

        } else {
            //we have all variable value and we can simply evaluate the polynomial
            self.h_poly.evaluate(&point_vec)
        }
        
    }
        
}
