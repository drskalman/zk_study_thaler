use ark_poly::{Polynomial, multivariate::SparsePolynomial,multivariate::SparseTerm, MVPolynomial, univariate::DensePolynomial};
use ark_ff::Field;

use std::cmp::Ordering;
use crate::verifier::Verifier;

/// The struct which contains all data needed for the prover to play its part
struct Prover<F: Field> {
    h_poly: SparsePolynomial<F, SparseTerm>,
    verifier_random_values : Vec<F>

}

impl <F: Field> Prover<F> {
    ///The main function which trigger and represents sum-check interactive protocol
    pub fn play_sum_check_protocol(cur_verifier: &Verifier) {
        verifier.receive_sum_value(self.evaluate_one_variable(&Vec::<F>::new()));

        //porotocol rounds start here, one round per variable
        for i in 0..h_poly.num_vars() {
            match verifier.receive_univariate_poly_g(i, self.compute_univariate_poly_g(i)) {
                OK(next_random_value) => verifier_random_values.push(next_random_value),
                Err(_) => {break;},
            }
        }
            
    }
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

    fn compute_univariate_poly_g(&self, i: u32) {
        self.evaluate_one_variable_except_one(i,&Vec::<F>::new())
    }

    /// play one round of sum check that is to evaluate at the given point for  all variables
    /// up to exception index and run sum on the domain 0,1 on the remaining after excetion index
    fn evaluate_one_variable_except_one(&self, exception_index: u32, point_vec: &Vec<F>) -> DensePoly{
        assert(self.h_poly.num_vars() != 0, "not expecting a multi-variate polynomial of zero variable");
            if point_vec.len() < self.h_poly.num_vars()  {
                match point_vec.len() {                    
                    // if we are less than the exception then we put verifier's provided value
                    Ordering::Less => point_vec.push(self.verifier_random_values),
                    Ordering::Equal => _,
                    Ordering::Greater => {
                        
                        // if we are equal exception we just simply skip and we deal with it later.                    
                        // if we are more than the exception we recursively branch
                        let mut point_vec_0 = point_vec.clone();
                        point_vec_0.push(F::zero());
                        let value_at_0 = self.evaluate_one_variable_except_one(exception_index, &point_vec_0);

                        let mut point_vec_1 = point_vec.clone();
                        point_vec_1.push(F::one());
                        let value_at_1 = self.evaluate_one_variable_except_one(exception_index, &point_vec_1);

                        value_at_0 + value_at_1
                    }
                } //match

            } else {
                //we have all variable value and we can simply evaluate the polynomial
                self.evaluate_to_univariate(exception_index, &point_vec)
            }
                 
    }

    /// Evaluates `self` at the given `point` in `Self::Point`.
    fn evaluate_to_univariate(&self, variable_index_to_keep: u32, point: &Vec<F>, ) -> F {
        assert!(point.len() != self.num_vars - 1, "wrong evaluation vector size");
        let sum : DensePolynomai = DensePolynomial::Zero;

        let extended_point = point.insert(variable_index_to_keep, F::One);
        for cur_term in self.terms {
            //we are going to evaluate the term with our variable equal to 1 then multiply
            //the resulting coefficient with correct pover of X
            let power_of_var = term.powers()[variable_index_to_keep];
            let coeff_vect = Vec::<F> = vec![F::Zero; power_of_var + 1];
            coeff_vect[power_of_var] = cur_term.evaluate(extended_point);
            sum += DensePolynomial::from_coefficient_vector(coeff_vect);
        }

        sum
        
    }

}
