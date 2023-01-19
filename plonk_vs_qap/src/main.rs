use std::env;
mod plonk;
    
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mut verbose = false;

    match args.len() {
        1 => return Err("Proof system not specified: plonk, r1cs")?,
        2 => return Err("Witness not specified: e.g. 3,4,5")?,
        3 => (),
        4 => match args[3].as_str() {
            "--verbose" => { verbose = true; },
            _ => return Err(["Invalid option: ", args[3].as_str()].concat())?,
        }
        _ => return Err("Too many args")?,
    }

    let witness: Vec<u32> =
        args[2]
        .as_str()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    
    let mut (prover,verifier) = match args[1].as_str() {
        "plonk"=> (plonk::Prover(),plonk::Verifier)                                    
        //"r1cs"=> r1cs::verify(r1cs1::generate_prove(witness)),
        _ => Err(["invalid proof system {}", args[1].as_str()].concat())?
    }

     match verifier.verify_proof(prover.generate_proof(witness)) {
        true => Ok(()),
        false => Err("proof verification failed for the given witness")?,
    };

