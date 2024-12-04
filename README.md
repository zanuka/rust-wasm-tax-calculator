# rust wasm tax calculator
A prototype for calculating tax using rust and wasm

## Roadmap
- Create a browser extension for the tax calculator feature
  - Build core WASM module for tax calculations
  - Implement browser extension architecture:
    - Background script for handling calculations
    - Content script for page integration
    - Popup interface for user interactions
  - Data persistence layer:
    - Utilize IndexedDB for storing tax calculation history
    - Cache frequently used tax rates and rules
  - Features:
    - Offline calculation support
    - History of previous calculations
    - Export functionality for tax records
    - Real-time tax calculation as users input values


