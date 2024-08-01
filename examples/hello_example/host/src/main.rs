use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

// These constants represent the RISC-V ELF and the image ID generated by risc0-build.
// The ELF is used for proving and the ID is used for verification.
use methods::{EXAMPLE_ELF, EXAMPLE_ID};
use risc0_zkvm::{default_prover, ExecutorEnv, ProverOpts, VerifierContext};

fn main() {
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    // An executor environment describes the configurations for the zkVM
    // including program inputs.
    // An default ExecutorEnv can be created like so:
    // `let env = ExecutorEnv::builder().build().unwrap();`
    // However, this `env` does not have any inputs.
    //
    // To add guest input to the executor environment, use
    // ExecutorEnvBuilder::write().
    // To access this method, you'll need to use ExecutorEnv::builder(), which
    // creates an ExecutorEnvBuilder. When you're done adding input, call
    // ExecutorEnvBuilder::build().

    // For example:
    let input: u32 = 15 * u32::pow(2, 27) + 1;
    let env = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .build()
        .unwrap();

    let receipt = default_prover()
        .prove_with_ctx(
            env,
            &VerifierContext::default(),
            EXAMPLE_ELF,
            &ProverOpts::groth16(),
        )
        .expect("failed to prove.")
        .receipt;

    let receipt_json = serde_json::to_vec(&receipt).unwrap();

    // Ensure the directory exists
    let dir_path = Path::new("../../test/data/receipt.json");
    let _dir = fs::create_dir_all(dir_path).unwrap();

    // Create the file
    let mut file = File::create(dir_path.join("receipt.bin")).unwrap();

    // Write the data
    file.write_all(&receipt_json).unwrap();

    // For example:
    let _output: u32 = receipt.journal.decode().unwrap();

    // The receipt was verified at the end of proving, but the below code is an
    // example of how someone else could verify this receipt.
    receipt.verify(EXAMPLE_ID).unwrap();
}