use ark_poly::{Polynomial, multivariate::SparsePolynomial,multivariate::SparseTerm, MVPolynomial};
use ark_ff::Field;

/// The struct which represt a sumcheck verifier
struct Verifier<F: Field> {
    h_poly: SparsePolynomial<F, SparseTerm>,
    poly_sum_over_domain : F,
    rounds_verified : u32,
    random_values: Vec<F>
        
}
impl <F: Field> Verifier<F> {
    pub fn reset_state(&mut self) {
        self.rounds_verified = 0;
    }
    
    pub fn receive_sum_value(&mut self, eval_sum_from_prover: F) {
        poly_sum_over_domain = eval_sum_from_prover;
    }

    pub fn receive_univariate_poly_g(i : u32, g_i: DensePolynomial) -> Result<F, Err> {        
        if (rounds_verified != i - 1) { //we do not verify round i before all i-1th rounds are verified
            false
        }

        match (poly_sum_over_domain == g_i.evaluate(F::Zero) + g_i.evaluate(F::One) &&
               g_i.degree() <= deg_i_g(h_poly, i))
        {
            true => {                
                rounds_verified = i;
                Ok(F::rand())                
            }
            false => {
                Err()
            }
        }        
    }

    ///return the degree of g(X_1,..,X_v) in variable X_j
    fn deg_i_g(&self, j: u32) {
        h_poly.terms().iter().map(|(_,term)| term.powers()[j]).max()
    }

    
        
}
