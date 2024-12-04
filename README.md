# rust wasm tax calculator
A prototype for calculating tax using rust and wasm

## Development Setup
### Prerequisites
- [Bun](https://bun.sh/) - JavaScript runtime and package manager
- [Rust](https://www.rust-lang.org/tools/install) - Systems programming language
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) - Rust's package manager (included with Rust)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) - WebAssembly build tool
- [Tailwind CSS](https://tailwindcss.com/) - Utility-first CSS framework

### Installation
1. Install project dependencies:

```bash
bun install
```

### Development Server
This project uses [`live-server`](https://github.com/tapio/live-server) for development with the following features:
- 🔄 Automatic page reloading when files change
- 📦 Proper WASM MIME type handling
- 🌐 Cross-browser compatibility
- 🚀 Default port 8080

### Tailwind CSS Setup
This project uses Tailwind CSS for styling. The setup includes:
- **Tailwind CLI**: For processing and building CSS
- **PostCSS**: For additional CSS processing
- **Autoprefixer**: For adding vendor prefixes automatically

#### Configuration Files
- `tailwind.config.js`: Tailwind configuration
- `postcss.config.js`: PostCSS configuration

#### Custom Styles
Custom styles are defined in `src/input.css` using Tailwind's utility classes and custom components.

#### Building CSS
To build and watch for changes in your CSS, run:

```bash
bun run tailwind
```

This will generate the `dist/output.css` file.

### Available Scripts
- `bun run build` - Build the WASM module
- `bun run build:dev` - Build the WASM module with development features
- `bun run serve` - Start the development server
- `bun run start` - Build and start the server
- `bun run test` - Run tests
- `bun run clean` - Clean build artifacts
- `bun run tailwind` - Build and watch Tailwind CSS

### Development Server Configuration
The server can be customized using the following options in package.json:
- `--port`: Change the port number (default: 8080)
- `--host`: Specify the host
- `--no-browser`: Prevent automatic browser opening
- `--quiet`: Suppress logging
- `--ignore`: Specify files to ignore for live reloading

### Local Development Workflow
For the best development experience, you'll need to run two commands in separate terminals:

#### Terminal 1: CSS Watcher
```bash
bun run tailwind
```
This command watches your CSS files for changes and rebuilds the output.css file automatically.

#### Terminal 2: Development Server
```bash
bun run serve
```
This starts the live-server that handles your application.

#### Why Two Terminals?
Running these commands separately ensures:
- 🔄 Live reloading works properly for both HTML/JS and CSS changes
- 📦 Tailwind can watch and rebuild CSS files immediately
- 🚀 The development server can pick up all changes instantly
- ⚡ Faster feedback loop during development

> **Note**: While you could combine these commands using a tool like `concurrently`, running them separately provides better visibility into any build errors that might occur.

## Tax Calculation Logic

The `calculate_tax` function in `src/lib.rs` is designed to compute the tax based on a given income using a progressive tax rate system. The function is accessible from JavaScript through WebAssembly, thanks to the `wasm_bindgen` annotation.

### Tax Brackets
The function uses the following tax brackets:

- **10%** on income up to $11,600
- **12%** on income over $11,600 up to $47,150
- **22%** on income over $47,150 up to $100,525
- **24%** on income over $100,525 up to $191,950
- **32%** on income over $191,950 up to $243,725
- **35%** on income over $243,725 up to $609,350
- **37%** on income over $609,350

### Calculation Method
The function calculates tax by applying the appropriate rate to the income within each bracket and summing the results. For example, if the income is $50,000, the tax is calculated as follows:

1. 10% on the first $11,600
2. 12% on the income between $11,600 and $47,150
3. 22% on the income between $47,150 and $50,000

This approach ensures that the tax is calculated progressively, with higher rates applied only to the income that falls within higher brackets.

## Roadmap
- [ ] Create a browser extension for a US tax calculator
  - [x] Inital project setup, dependencies, wasm-pack proto
  - [x] Build core WASM module for tax calculations
  - [ ] Implement browser extension architecture:
    - [ ] Background script for handling calculations
    - [ ] Content script for page integration
    - [ ] Popup interface for user interactions
  - [ ] Data persistence layer:
    - [ ] Utilize IndexedDB for storing tax calculation history
    - [ ] Cache frequently used tax rates and rules
  - [ ] Features:
    - [ ] Offline calculation support
    - [ ] History of previous calculations
    - [ ] Export functionality for tax records
    - [ ] Real-time tax calculation as users input values


## notes
Easily calculate your monthly income tax based on the latest tax brackets for salaried individuals in the USA.

The USA Tax Calculator is a browser extension designed to assist American individuals in calculating their income tax liability. It offers a user-friendly interface that allows users to input their monthly salary and select the tax year to estimate their tax obligations accurately. The extension supports tax calculations for different tax years and provides options for deductions and tax calculations specific to various professions.

