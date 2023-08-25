# Sedenion Engine
![npm](https://img.shields.io/npm/v/sedenion_engine)
![GitHub](https://img.shields.io/github/license/SedenionCas/sedenion-engine)
[![codecov](https://codecov.io/gh/SedenionCas/sedenion-engine/graph/badge.svg?token=6SIKVQCIAQ)](https://codecov.io/gh/SedenionCas/sedenion-engine)

Sedenion Engine is an open-source project that offers a scientific calculator and CAS (Computer Algebra System) solver. This repository contains the Rust source code used by Sedenion to parse and evaluate expressions.

## Building the Project

To build the Sedenion engine to WebAssembly (wasm), follow these steps:

1. Make sure you have `wasm-pack` installed on your system.
2. Execute the following command: `wasm-pack build --target bundler`.
3. The NPM package will be generated and stored in the `pkg` directory.

## License

This project is licensed under GNU GPL-v3. You can find more detailed information in the `LICENSE` file located in the root of this repository.

## Features
Currently Sedenion engine only supports numerical evaluation.

## Getting Started

To use the Sedenion Engine in your project, follow the steps below:

1. Install the NPM package generated during the build process.
   ```
   npm install sedenion_engine
   ```

2. Import the Sedenion module in your JavaScript or TypeScript code.
   ```javascript
   import {} from 'sedenion-engine';
   ```

3. Call the functions in your app.
   ```javascript
   const result = evaluate('2 + 2');
   console.log(result); // Output: 4
   ```

## Contributing

We welcome contributions from the community to enhance Sedenion Engine further. If you want to contribute, please follow the guidelines outlined in the `contributing.md` file.

## Support

If you encounter any issues or have questions regarding the Sedenion Engine, please [open an issue](https://github.com/SedenionCas/sedenion-engine/issues) on our GitHub repository.

## Acknowledgments

We would like to express our gratitude to all the developers and contributors who have made this project possible.