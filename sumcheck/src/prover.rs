use ark_ff::{Field, Zero};
use ark_poly::{
    multivariate::{SparsePolynomial, SparseTerm, Term},
    univariate::DensePolynomial,
    DenseMVPolynomial, DenseUVPolynomial, Polynomial,
};

use crate::verifier::Verifier;
use std::cmp::Ordering;

/// The struct which contains all data needed for the prover to play its part
pub struct Prover<F: Field> {
    pub h_poly: SparsePolynomial<F, SparseTerm>,
    pub verifier_random_values: Vec<F>,
}

impl<F: Field> Prover<F> {
    ///The main function which trigger and represents sum-check interactive protocol
    pub fn play_sum_check_protocol(&mut self, mut cur_verifier: Verifier<F>) {
        self.verifier_random_values = vec![];
        cur_verifier.receive_sum_value(self.evaluate_one_variable(&Vec::<F>::new()));

        //porotocol rounds start here, one round per variable
        for i in 0..self.h_poly.num_vars() + 1 {
            match cur_verifier.receive_univariate_poly_g(i, self.compute_univariate_poly_g(i)) {
                Ok(next_random_value) => self.verifier_random_values.push(next_random_value),
                Err(_) => {
                    break;
                }
            }
        }

        println!("played protocol for {}", self.verifier_random_values.len());
    }
    ///sum of the evaulation of the multivariate polynomial
    /// over all possible binary input
    pub fn evaluate_sum(&self) -> F {
        self.evaluate_one_variable(&Vec::<F>::new())
    }

    ///recursive evalutation of the polynomial one variable
    ///at the time
    fn evaluate_one_variable(&self, point_vec: &Vec<F>) -> F {
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

    fn compute_univariate_poly_g(&mut self, i: usize) -> DensePolynomial<F> {
        let derived_uni_poly = self.evaluate_one_variable_except_one(i, Vec::<F>::new());
        println!("dervied uni poly g_i: {:?} for i: {}", derived_uni_poly, i);
        derived_uni_poly
    }

    /// play one round of sum check that is to evaluate at the given point for  all variables
    /// up to exception index and run sum on the domain 0,1 on the remaining after excetion index
    fn evaluate_one_variable_except_one(
        &mut self,
        exception_index: usize,
        mut point_vec: Vec<F>,
    ) -> DensePolynomial<F> {
        assert!(
            self.h_poly.num_vars() != 0,
            "not expecting a multi-variate polynomial of zero variable"
        );
        if point_vec.len() < self.h_poly.num_vars() {
            //because we don't deal (and skip) with exception index in this clause  {
            match point_vec.len().cmp(&exception_index) {
                // if we are less than the exception then we put verifier's provided value
                Ordering::Less => {
                    point_vec.push(self.verifier_random_values[point_vec.len()]);
                    self.evaluate_one_variable_except_one(exception_index, point_vec)
                }

                // if we are equal to exception index  we just simply put one as place holde and deal with it later
                Ordering::Equal => {
                    point_vec.push(F::one());
                    self.evaluate_one_variable_except_one(exception_index, point_vec)
                }

                Ordering::Greater => {
                    // if we are greater than the exception indexwe recursively branch out
                    let mut point_vec_0 = point_vec.clone();
                    point_vec_0.push(F::zero());
                    let value_at_0 =
                        self.evaluate_one_variable_except_one(exception_index, point_vec_0);

                    let mut point_vec_1 = point_vec.clone();
                    point_vec_1.push(F::one());
                    let value_at_1 =
                        self.evaluate_one_variable_except_one(exception_index, point_vec_1);

                    value_at_0 + value_at_1
                }
            } //atch
        } else {
            //we have all variable value and we can simply evaluate the polynomial
            self.evaluate_to_univariate(exception_index, &point_vec)
        }
    }

    /// Evaluates `self` at the given `point` in `Self::Point`.
    fn evaluate_to_univariate(
        &mut self,
        variable_index_to_keep: usize,
        mut point: &Vec<F>,
    ) -> DensePolynomial<F> {
        assert!(
            point.len() != self.h_poly.num_vars() - 1,
            "wrong evaluation vector size"
        );
        let mut sum: DensePolynomial<F> = DensePolynomial::<F>::zero();
        for cur_term in &self.h_poly.terms {
            //we are going to evaluate the term with our variable equal to 1 then multiply
            //the resulting coefficient with correct pover of X
            //let power_of_var = cur_term.1.powers()[variable_index_to_keep];
            let power_of_var: usize = cur_term
                .1
                .iter()
                .map(|(v, p)| if *v == variable_index_to_keep { *p } else { 0 })
                .sum();

            let mut coeff_vect: Vec<F> = vec![F::zero(); power_of_var + 1]; //making just a monomial
            coeff_vect[power_of_var] = cur_term.0 * cur_term.1.evaluate(&point);

            let cur_monomial = DensePolynomial::<F>::from_coefficients_vec(coeff_vect);
            sum = sum + cur_monomial;
        }
        sum
    }
}
