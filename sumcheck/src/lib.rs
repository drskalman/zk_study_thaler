extern crate ark_ff;
extern crate ark_poly;

pub mod prover;
pub mod verifier;

#[cfg(test)]
mod tests {
    use crate::{prover, verifier};
    use ark_ff::{biginteger::BigInteger64, fields::Fp64, BigInt, MontBackend, MontFp};
    use ark_poly::{MVPolynomial, multivariate::{SparsePolynomial, Term, SparseTerm } };

    //Tests for  a toy example field of 127 elements
    pub struct F127Config;
    pub type F127 = Fp64<MontBackend<F127Config, 1>>;

    impl ark_ff::MontConfig<1> for F127Config {
        // sage: FF(3)^63
        // 126
        #[rustfmt::skip]
        const TWO_ADIC_ROOT_OF_UNITY: F127 = MontFp!(F127, "126");

        /// MODULUS = 127
        #[rustfmt::skip]
        const MODULUS: BigInteger64 = BigInt!("127");

        // sage: FF(3).multiplicative_order()
        // 126
        // Montgomery conversion 3 * 2 = 6 % 127
        /// GENERATOR = 3
        #[rustfmt::skip]
        const GENERATOR: F127 = MontFp!(F127, "6");

        // T and T_MINUS_ONE_DIV_TWO, where MODULUS - 1 = 2^S * T
        // For T coprime to 2
    }

    const F127_ZERO: F127 = MontFp!(F127, "0");
    const F127_ONE: F127 = MontFp!(F127, "1");

    #[test]
    fn test_prover_proves_to_verifier() {
        let test_h_poly = SparsePolynomial::from_coefficients_slice(
            4,
            &[
                (MontFp!(F127, "2"), SparseTerm::new(vec![])),
                (MontFp!(F127, "4"), SparseTerm::new(vec![(0, 1), (1, 2)])),
                (MontFp!(F127, "8"), SparseTerm::new(vec![(0, 1), (0, 1)])),
                (F127_ONE, SparseTerm::new(vec![(3, 0)])),
            ]
        );

        let mut test_prover = prover::Prover { h_poly : test_h_poly.clone(), verifier_random_values : vec![] };
        let mut test_verifier =  verifier::Verifier {   h_poly: test_h_poly,
                                                        poly_sum_over_domain: F127_ZERO,
                                                        rounds_verified : 0,
                                                        random_values : vec![]
                                                        
        };

        test_prover.play_sum_check_protocol(test_verifier);
    }
}
