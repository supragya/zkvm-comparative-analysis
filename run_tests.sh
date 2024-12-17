# Create TestData
echo "Creating TestData"
cargo run

# Run for SHA256 non-precompile mode
echo "Running RISC-0 SHA256 non-precompile mode, output: testoutput/r0_sha256_noprecompile.log"
cd r0_sha256_noprecompile && \
RUST_BACKTRACE=1 RUST_LOG="info" RISC0_DEV_MODE=1 cargo run 2&> ../testoutput/r0_sha256_noprecompile.log