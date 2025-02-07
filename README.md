# rustgoslin
Rust implementation of the Goslin Parsers and binding to R

**Disclaimer**: This project is in an early stage of development and is not yet ready for production use.

## Introduction
This project is a Rust implementation of the Goslin parsers. The Goslin parsers are a set of parsers that can be used to parse lipid names according to the LIPID MAPS nomenclature. The parsers are used in the [LipidCreator](https://lipidcreator.org) and [LipidHome](https://lipidhome.co.uk) projects.

This project was initiated at the EuBIC Developer's Meeting 2025, in Neustift, Italy.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

## Installation

### Prerequisites

To build this project you need to have Rust installed. You can install Rust by following the instructions on the [Rust website](https://www.rust-lang.org/tools/install).

### Building

To build the Rust part of the project, clone the repository and run the following command in the `src/rust` directory:

```bash
cargo build
```

To build the R part of the project, run the following command in the root directory:

```bash
R CMD INSTALL rustgoslin
```

or use the `devtools` package:

```R
devtools::install()
```
