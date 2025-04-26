# 🏦 FDIC BankFind MCP Server

[![Rust](https://img.shields.io/badge/Rust-stable-blue?logo=rust)](https://www.rust-lang.org/) [![OpenAPI](https://img.shields.io/badge/OpenAPI-3.0-green?logo=openapi-initiative)](public/fdic/swagger.yaml)

Welcome to the **FDIC BankFind MCP Server**! 🚀

This project provides a modern, robust, and contributor-friendly Rust web server that wraps the FDIC BankFind API with a standardized MCP (Meta-Content Protocol) interface. It features:

- 🛠️ **Template-driven handler generation** for rapid endpoint development
- ✅ **MCP-compliant success and error responses**
- 🔒 **Centralized parameter validation**
- 📚 **Comprehensive documentation & open source governance**

---

## Table of Contents 📋
- [Features](#features) ✨
- [Architecture](#architecture) 🏗️
- [Getting Started](#getting-started) 🚀
- [Usage Examples](#usage-examples) 📦
- [Adding New Endpoints](#adding-new-endpoints) ➕
- [Contributing](#contributing) 🤝
- [Governance and Docs](#governance-and-docs) 📚
- [License](#license) ⚖️
- [Official FDIC Resources](#official-fdic-resources) 🔗

---

## Features
- **Template-Driven Handlers:** Add new FDIC endpoints in minutes using Tera templates.
- **MCP Protocol Compliance:** All responses use `MCPResponse<T>` for success and `MCPError` for errors—machine-readable, predictable, and debuggable.
- **Centralized Validation:** All parameter validation (fields, filters, sort, etc.) is reusable and standards-compliant.
- **OpenAPI/Swagger:** [OpenAPI spec](public/fdic/swagger.yaml) auto-generated for easy client integration.
- **Contributor-Friendly:** Clear docs, code ownership, and community standards.

---

## Architecture
- **Rust + Axum 0.7+** for async web APIs
- **Handler templates** in `templates/` generate endpoint stubs
- **Parameter utils** in `src/param_utils.rs` for validation/normalization
- **MCP response types** in `src/common.rs`
- **Comprehensive error handling** (no panics in production!)

---

## Getting Started

```sh
# Clone the repo
$ git clone https://github.com/clafollett/fdic-bank-find-mcp-server.git
$ cd fdic-bank-find-mcp-server

# Build and run
$ cargo run
```

- Server starts on `http://localhost:3000`
- See [OpenAPI docs](public/fdic/swagger.yaml) for endpoints

---

## Usage Examples

### Success Response
```json
{
  "type": "success",
  "data": [ { "CERT": "12345", ... } ],
  "meta": { "limit": 10, ... }
}
```

### Error Response
```json
{
  "type": "error",
  "error": {
    "type": "invalid_request_error",
    "message": "Invalid field(s): INVALIDFIELD",
    "status": 400
  }
}
```

---

## Adding New Endpoints
1. Place or update the official FDIC YAML definition (e.g., `institution_properties.yaml`, `summary_properties.yaml`, etc.) in the `public/fdic/` directory. The generator will honor whatever is present here—these files define the fields, data types, and structure for each endpoint, and are sourced directly from the FDIC. 
   - 📖 **Source:** [FDIC BankFind API Documentation](https://banks.data.fdic.gov/docs/)  
   - 📄 **YAML Definitions:** [FDIC BankFind API YAMLs](https://banks.data.fdic.gov/docs/yaml/)
2. Run the handler generator (`src/bin/generate_handlers.rs`). It will:
    - Parse the YAML definitions
    - Automatically generate new handler stubs in `src/handlers/`
    - Update and register endpoints in `src/handlers/mod.rs` (no manual edits needed!)
3. Add parameter validation logic in `src/param_utils.rs` if needed
4. Document your endpoint in OpenAPI (see `public/fdic/swagger.yaml`)

See [CONTRIBUTING.md](CONTRIBUTING.md) for the full process!

---

## Contributing
- Open issues or feature requests using the templates in `.github/ISSUE_TEMPLATE/`
- Fork and PR from a feature branch (see [CONTRIBUTING.md](CONTRIBUTING.md))
- Follow the [Prime Directive](.windsurfrules) and code style rules
- All PRs require code owner review (see [CODEOWNERS](.github/CODEOWNERS))

---

## Governance and Docs
- [CONTRIBUTING.md](CONTRIBUTING.md)
- [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)
- [SECURITY.md](SECURITY.md)
- [LICENSE](LICENSE)
- [OpenAPI Spec](public/fdic/swagger.yaml)

---

## Official FDIC Resources
- [FDIC BankFind Suite](https://banks.data.fdic.gov/docs/)
- [API Documentation](https://banks.data.fdic.gov/docs/)
- [YAML Definitions](https://banks.data.fdic.gov/docs/yaml/)
- [FDIC Data Download](https://banks.data.fdic.gov/data-download/)

---

## License
This project is licensed under the terms of the LICENSE file in this repo.

---

> Banking can be fun, too! 🦍🍌
