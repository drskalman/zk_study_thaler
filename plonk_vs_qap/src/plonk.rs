use ark_ff::Field;
/// Plonk Prover
struct PythaProver<F: Field> {
    mut witness_length: usize,
    mut full_witness: (Vec<F>, Vec<F>, Vec<F>),
    mut q_vectors: {(Vec<F>, Vec<F>, Vec<F>, Vec<F>, Vec<F>)}
       
    ,
}

impl<F: Field> PythaProver<F> {
    /// The q vectors are part of the setup and so they are constant
    fn generate_q_vectors() {
        let q_a = [0, 0, 0, 1];
        let q_b = [0, 0, 0, 1];
        let q_m = [1, 1, 1, 0];
        let q_c = [0, 0, 0, 0];
        let q_o = [-1, -1, -1, -1];

        self.q_vectors = [q_a, q_b, q_m, q_c, q_o];
    }
    
    fn generate_full_witness(&self, witness: Vec<F>) -> Result<Vec<F>, Box<dyn std::error::Error>> {
        if witness.len() != 3 {
            Err("pythagorus witness should consists of 3 elements")?
        }

        //plonk has 3 vectors and the full circuit is as follows
        // q_a*a + q_b*b + q_m*a*b +q_o*c + q_c = 0
        //sym_1 = x * x   -sym_1 + x * x = 0
        //sym_2 = y * y   -sym_2 + y * y = 0
        //sym_3 = z * z   -sym_3 + z * z = 0
        //sym_3 = sym_1 + sym_2 -sym_3 + sym_1 + sym_2 = 0
        //a = [x, y, z, x * x]
        //b = [x, y, z, y * y]
        //o = [x * x, y * y, z * z, z * z]
        //q_a = [0, 0, 0, 1]
        //q_b = [0, 0, 0, 1]
        //q_o = [-1, -1, -1, -1]
        //q_m = [1, 1, 1, 0]
        //q_c = [0, 0, 0, 0]
        self.full_witness : (Vec<F>, Vec<F>, Vec<F>) = (
            [witness[0], witness[1], witness[3], witness[0]*witness[0]],
            [witness[0], witness[1], witness[3], witness[1]*witness[1]],
            [witness[0]*witness[0], witness[1]*witness[1], witness[3]*witness[3], witness[3]*witness[3]],
        );

        assert!(does_witness_satisfy_constrian())
    }

    /// Return true 
    fn does_witness_satisfy_constrain() -> bool {
        //sym_1 = x * x   -sym_1 + x * x = 0
        //sym_2 = y * y   -sym_2 + y * y = 0
        //sym_3 = z * z   -sym_3 + z * z = 0
        //sym_3 = sym_1 + sym_2 -sym_3 + sym_1 + sym_2 = 0
        asert!(self.
        
    }
}

impl<F: Field> SnarkProver<F> for PythoProver<F> {
    pub fn generate_proof(witness: Vec<u32>) -> Vec<F> {
        Self::genenrate_full_witness();
        vec![]
    }
}

///Plonk Verifier
pub fn verify_proof(proof: Vec<u32>)-> bool {
    false
}
