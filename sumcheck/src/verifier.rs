use ark_poly::{Polynomial, univariate::DensePolynomial, multivariate::SparsePolynomial,multivariate::{SparseTerm, Term}, MVPolynomial};
use ark_ff::Field;
use rand::{thread_rng};

/// The struct which represt a sumcheck verifier
pub struct Verifier<F: Field> {
    pub h_poly: SparsePolynomial<F, SparseTerm>,
    pub poly_sum_over_domain : F,
    pub rounds_verified : usize,
    pub random_values: Vec<F>
        
}
impl <F: Field> Verifier<F> {
    pub fn reset_state(&mut self) {
        self.rounds_verified = 0;
        self.poly_sum_over_domain = F::zero();
        self.random_values = vec![];
    }
    
    pub fn receive_sum_value(&mut self, eval_sum_from_prover: F) {
        self.reset_state();
        self.poly_sum_over_domain = eval_sum_from_prover;
    }

    pub fn receive_univariate_poly_g(&mut self, i : usize, g_i: DensePolynomial<F>) -> Result<F, &'static str> {
        if (self.rounds_verified != i - 1) {
            return Err("we do not verify round i before all i-1th rounds are verified");
            
        }

        match (self.poly_sum_over_domain == g_i.evaluate(&F::zero()) + g_i.evaluate(&F::one()) &&
               g_i.degree() <= self.deg_i_g(i))
        {
            true => {                
                self.rounds_verified = i;
                Ok(F::rand(&mut thread_rng()))                
            }
            false => {
                Err("verification failed")
            }
        }        
    }

    ///return the degree of g(X_1,..,X_v) in variable X_j
    fn deg_i_g(&self, j: usize) -> usize {
        self.h_poly.terms().iter().map(|(_,term)| term.powers()[j]).max().unwrap()
    }

    
        
}
