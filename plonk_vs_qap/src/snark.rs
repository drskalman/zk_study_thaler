// This is a trait to encompass a general SNARK
// Based on polynomial commitment
ark_ff:Field;

pub trait SnarkProver<F: Field> {

    generate_proof() -> Vec<F>;
    
}

