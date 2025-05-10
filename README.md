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
- [MCP Inspector Setup & Usage](#mcp-inspector-setup--usage-) 
- [MCP IDE Configuration](#mcp-ide-configuration) 
- [Usage Examples](#usage-examples) 📦
- [Handler & OpenAPI Generation](#handler--openapi-generation) 🛠️
- [Adding New Endpoints](#adding-new-endpoints) ➕
- [Vibe-Architected with Windsurf & GPT-4.1](#vibe-architected-with-windsurf-gpt-41) 🚀
- [About Dynamic Search Response Fields](#about-dynamic-search-response-fields) ⚡️
- [Contributing](#contributing) 🤝
- [Official FDIC Resources](#official-fdic-resources) 🔗
- [Governance and Docs](#governance-and-docs) 📚
- [License](#license) ⚖️

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

- Server starts on `http://localhost:5000`
- See [OpenAPI docs](public/fdic/swagger.yaml) for endpoints


### Docker

```sh
docker build -t fdic-bank-find-mcp-server:latest .
docker run --rm \
  -e "RUST_BACKTRACE=1" \
  -e "BIND_ADDR=0.0.0.0" \
  -e "BIND_PORT=5000" \
  -e "RUST_LOG=debug" \
  -p "5000:5000" \
  --name "fdic-bank-find-mcp-server" \
  "fdic-bank-find-mcp-server:latest"
```

---

## MCP Inspector Setup & Usage 🕵️‍♂️

### 1. Install the MCP Inspector

You can run it directly (no install needed):
```sh
npx @modelcontextprotocol/inspector cargo run --bin fdic-bank-find-mcp-server
```

Or install globally for convenience:
```sh
npm install -g @modelcontextprotocol/inspector
```

### 2. Start the Inspector with your Rust MCP server

```sh
npx @modelcontextprotocol/inspector cargo run --bin fdic-bank-find-mcp-server
```
Or, if installed globally:
```sh
modelcontextprotocol-inspector cargo run --bin fdic-bank-find-mcp-server
```

### 3. Troubleshooting

- If you see `PORT IS IN USE`, kill any previous inspector processes (`pkill -f inspector` on Mac/Linux).
- If you get `spawn stdio ENOENT`, check your command: it must be `cargo`, not `stdio`, and don’t use `--` between arguments.
- If you see `Unexpected token 'M'`, make sure your Rust server prints only valid MCP protocol JSON to stdout (send logs to stderr).

---

## MCP IDE Configuration

### Windsurf
```json
{
  "mcpServers": {
    "fdic-bank-find": {
      "command": "docker",
      "args": [
        "run", "--rm",
        "-e", "RUST_BACKTRACE=1", 
        "-e", "BIND_ADDR=0.0.0.0",
        "-e", "BIND_PORT=5000",
        "-e", "RUST_LOG=debug",
        "-p", "5000:5000",
        "--name", "fdic-bank-find-mcp-server",
        "fdic-bank-find-mcp-server:latest"
      ]
    }
  }
}
```

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

## 🛠️ Handler & OpenAPI Generation

This project uses **automated code and OpenAPI generation** to keep Rust handlers, docs, and API schemas perfectly in sync with the YAML specs.

### Generating Handlers & OpenAPI

1. **Edit your YAML specs** (in `public/fdic/*.yaml`).
2. **Run the generator:**

   ```sh
   cargo run --bin generate_handlers
   ```

   This will:
   - Regenerate all handler stubs in `src/handlers/`
   - Update the OpenAPI JSON at `public/openapi.json`

3. **Rebuild & retest:**

   ```sh
   cargo check && cargo test
   ```

### Why Do This?
- **No manual sync**: Your Rust code, OpenAPI, and YAML are always aligned
- **Instant metadata**: Any new field-level capabilities (e.g., searchability) are exposed everywhere
- **Agent/automation ready**: UIs and clients can introspect all field capabilities

**Pro tip:** Regenerate handlers after any YAML/spec change to keep your API and docs in perfect harmony! 🎶

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

## ⚡️ About Dynamic Search Response Fields

Some endpoints, like `/institutions`, may return additional fields in each search result—such as `highlight` (for term highlighting) and `score` (for match relevance)—in addition to the standard `data` object. These fields are passed directly from the underlying search engine (e.g., Elasticsearch) and are not always present for every query.

### Example Response
```json
{
  "type": "success",
  "data": [
    {
      "data": { "NAME": "Wells Fargo Bank", ... },
      "highlight": { "NAME.raw": ["<em>Wells</em> ..."] },
      "score": 1091.385
    }
  ],
  "meta": { ... }
}
```

### What Clients/Agents Should Know
- Always expect a `data` object, but be prepared for optional `highlight` and `score` fields.
- These fields help agents/UIs explain and rank results, but may not appear for every query.
- If your client or agent only expects the core `data` fields, simply ignore any unknown fields at the top level of each array entry.
- For agent/AI use: leverage `highlight` for explanations and `score` for ranking or filtering.

### OpenAPI Note
Our OpenAPI spec describes the core response shape, but additional fields may be present in the result objects. This is intentional for agent/automation flexibility.

**Pro tip:** If you want to always see these fields, use a query that matches search terms directly (e.g., `NAME:WELLS FARGO`).

---

## 🚀 Vibe-Architected with Windsurf & GPT-4.1

This project was fully architected, coded, and iterated using the **Windsurf Editor** with the help of GPT-4.1 and 04-mini-high models. The entire development experience was:

- **Conversational, collaborative, and lightning-fast**
- **Zero context lost**—AI paired on every design and code decision
- **100% code generated, reviewed, and refactored in real-time**
- **No manual boilerplate**: All endpoints, docs, and metadata (like `x-elastic-type` for search/filter discovery) were vibe-architected and auto-wired
- **Agent/AI-friendly by design**: Every feature was built with future automation and discoverability in mind

> _"This has been the most amazing development experience I've had in years!!!"_

If you’re reading this, you’re looking at a codebase that’s not just modern, but **future-native**. All thanks to the Windsurf + GPT-4.1/04-mini-high workflow. Give it a try and you’ll never want to code solo again! 😎🤙✨

---

## Contributing
- Open issues or feature requests using the templates in `.github/ISSUE_TEMPLATE/`
- Fork and PR from a feature branch (see [CONTRIBUTING.md](CONTRIBUTING.md))
- Follow the [Prime Directive](.windsurfrules) and code style rules
- All PRs require code owner review (see [CODEOWNERS](.github/CODEOWNERS))

---

## Official FDIC Resources
- [FDIC BankFind Suite](https://banks.data.fdic.gov/docs/)
- [API Documentation](https://banks.data.fdic.gov/docs/)
- [YAML Definitions](https://banks.data.fdic.gov/docs/yaml/)
- [FDIC Data Download](https://banks.data.fdic.gov/data-download/)

---

## Governance and Docs
- [CONTRIBUTING.md](CONTRIBUTING.md)
- [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)
- [SECURITY.md](SECURITY.md)
- [LICENSE](LICENSE)
- [OpenAPI Spec](public/fdic/swagger.yaml)

---

## License
This project is licensed under the terms of the [LICENSE](LICENSE) file in this repo.

---

> Banking can be fun, too! 🦍🍌
