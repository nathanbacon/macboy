# MacBoy - GameBoy Emulator

[![CI](https://github.com/nathanbacon/macboy/actions/workflows/ci.yml/badge.svg)](https://github.com/nathanbacon/macboy/actions/workflows/ci.yml)
[![Coverage](https://github.com/nathanbacon/macboy/actions/workflows/coverage.yml/badge.svg)](https://github.com/nathanbacon/macboy/actions/workflows/coverage.yml)

A GameBoy emulator written in Rust.

## Features

- CPU emulation with comprehensive instruction set support
- Memory management unit (MMU)
- Graphics processing unit (GPU) simulation
- Type-safe opcode enumeration (512 opcodes)
- Comprehensive test suite with coverage tracking

## Development

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Coverage Reports

Generate coverage reports locally using the provided script:

```bash
./scripts/coverage.sh
```

Or manually:

```bash
# Install required tools
cargo install grcov
rustup component add llvm-tools-preview

# Generate coverage
RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test
grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o coverage/html/
```

Open `coverage/html/index.html` in your browser to view the detailed coverage report.

### CI/CD

This project uses GitHub Actions for:

- **Continuous Integration**: Automated builds and testing on every push/PR
- **Coverage Tracking**: Generates coverage reports and uploads them as artifacts
- **Code Quality**: Linting with clippy and formatting checks with rustfmt
- **Dependency Updates**: Automated dependency updates via Dependabot

#### Viewing Coverage in GitHub

1. **PR Comments**: Coverage percentage is automatically commented on pull requests
2. **Artifacts**: Detailed HTML coverage reports are uploaded as build artifacts
3. **Download**: Go to Actions → Latest workflow run → Artifacts → Download "coverage-report"

#### Setting up External Coverage Services (Optional)

To integrate with external coverage services like Codecov:

1. Sign up for [Codecov](https://codecov.io) and connect your repository
2. Add your Codecov token as a repository secret named `CODECOV_TOKEN`
3. The workflow will automatically upload coverage data

## Coverage

Current coverage is tracked automatically in CI/CD. View coverage reports by:

1. **GitHub Actions Artifacts**: Download coverage reports from workflow runs
2. **PR Comments**: Coverage percentage shown automatically on pull requests
3. **Local Generation**: Use `./scripts/coverage.sh` to generate reports locally

## Project Structure

```
src/
├── main.rs              # Entry point
├── cpu.rs               # CPU implementation
├── opcodes.rs           # Complete opcode enumeration
├── mmu.rs               # Memory management
├── gpu.rs               # Graphics processing
├── registers.rs         # Register definitions
├── cartridge.rs         # Cartridge/ROM handling
├── joypad.rs           # Input handling
├── interrupts.rs       # Interrupt system
└── cpu_comprehensive_tests.rs  # Test suite
```

## Architecture

The emulator follows a modular design:

- **Type-safe opcodes**: All 512 GameBoy CPU instructions are enumerated as Rust enums
- **Comprehensive testing**: Systematic testing of CPU instructions with edge cases
- **Coverage tracking**: Automated coverage analysis to ensure instruction completeness
- **Clean separation**: CPU, MMU, GPU, and other components are clearly separated

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass and coverage is maintained
6. Submit a pull request

## License

This project is open source. Please see the LICENSE file for details.
