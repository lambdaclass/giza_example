use std::path::PathBuf;

use giza_air::ProofOptions;
use giza_runner::ExecutionTrace;

fn main() {
    let trace = ExecutionTrace::from_file(
        PathBuf::from("../trace/program.json"),
        PathBuf::from("../trace/trace.bin"),
        PathBuf::from("../trace/memory.bin"),
        Some(3),
    );

    // generate the proof of execution
    let proof_options = ProofOptions::with_proof_options(None, None, None, None, None);
    let (_proof, _pub_inputs) = giza_prover::prove_trace(trace, &proof_options).unwrap();
}
