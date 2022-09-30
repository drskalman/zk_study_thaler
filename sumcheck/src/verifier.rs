use ark_ff::Field;
use ark_poly::{
    multivariate::SparsePolynomial,
    multivariate::{SparseTerm, Term},
    univariate::DensePolynomial,
    DenseMVPolynomial, Polynomial,
};
use rand::thread_rng;

/// The struct which represt a sumcheck verifier
pub struct Verifier<F: Field> {
    pub h_poly: SparsePolynomial<F, SparseTerm>,
    pub poly_sum_over_domain: F,
    pub rounds_verified: isize,
    pub random_values: Vec<F>,
    pub univar_polys: Vec<DensePolynomial<F>>,
}
impl<F: Field> Verifier<F> {
    pub fn reset_state(&mut self) {
        self.rounds_verified = -1;
        self.poly_sum_over_domain = F::zero();
        self.random_values = vec![];
        self.univar_polys = vec![];
    }

    pub fn receive_sum_value(&mut self, eval_sum_from_prover: F) {
        self.reset_state();
        self.poly_sum_over_domain = eval_sum_from_prover;
        println!("sum is {}", self.poly_sum_over_domain);
    }

    pub fn receive_univariate_poly_g(
        &mut self,
        i: usize,
        g_i: DensePolynomial<F>,
    ) -> Result<F, &'static str> {
        if (self.rounds_verified != i as isize - 1) {
            return Err("we do not verify round i before all i-1th rounds are verified");
        }

        let ref_value = if (i == 0) {
            self.poly_sum_over_domain
        } else {
            self.univar_polys[i - 1].evaluate(&self.random_values[i - 1])
        };

        match (ref_value == g_i.evaluate(&F::zero()) + g_i.evaluate(&F::one())
            && g_i.degree() <= self.deg_i_g(i))
        {
            true => {
                self.rounds_verified = i as isize;
                self.random_values.push(F::rand(&mut thread_rng()));
                self.univar_polys.push(g_i);
                Ok(self.random_values[i])
            }
            false => Err("verification failed"),
        }
    }

    ///return the degree of g(X_1,..,X_v) in variable X_j
    fn deg_i_g(&self, j: usize) -> usize {
        self.h_poly
            .terms()
            .iter()
            .map(|(_, term)| term.iter().map(|(v, p)| if *v == j { *p } else { 0 }).sum())
            .max()
            .unwrap()
    }
}
