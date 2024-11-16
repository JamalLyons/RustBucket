# Run cargo test for libraries, binaries, and tests
Write-Host "Running cargo tests..." -ForegroundColor Green
cargo test --lib --bins --tests

# Run cargo fmt to format the code
Write-Host "`nFormatting code..." -ForegroundColor Green
cargo fmt

Write-Host "`nDone!" -ForegroundColor Green