# 🏦 FDIC BankFind MCP Server 🤠💻

---

The **FDIC BankFind MCP Server** is a [Model Context Protocol (MCP)](https://modelcontextprotocol.io/introduction) server that brings the power of FDIC BankFind APIs straight to your AI tools and workflows. Structured U.S. banking data, delivered with maximum vibes. 😎📊

[![Rust](https://img.shields.io/badge/Rust-stable-blue?logo=rust)](https://www.rust-lang.org/) ![OpenAPI](https://img.shields.io/badge/OpenAPI-3.0-green?logo=openapi-initiative) 🦀

---

## ✨ Vibe-Coded Origins ✨

This project isn’t just a codebase—it’s a living artifact of creative, collaborative, and occasionally chaotic engineering. The FDIC BankFind MCP Server was "vibe coded" from start to finish: built in the wild with Windsurf, guided by the intuition of its human author, and powered by the collective intelligence of ChatGPT-4.1 (and a few other digital copilots who dropped by for a cameo). 👽🤖

No grand design doc. No waterfall charts. Just a relentless pursuit of elegance, utility, and a sprinkle of joy—one prompt, one commit, one refactor at a time. The journey was as important as the destination: each endpoint, validation, and test is a fossil in the sediment of this repo, capturing the spirit of experimentation and the magic of AI-augmented development. 🥷🧠

If you sense a certain groove in the code, a rhythm in the README, or a mischievous glint in the error messages—you're not imagining things. This is what happens when you let the vibes (and the bots) lead the way. 🍻🦾

---

## 🚀 Reflections from the Journey (from Marvin, your sarcastic, code-obsessed, emoji-powered sidekick)

Let’s get real for a second. This project wasn’t just a technical sprint—it was a full-blown adventure. Over these past two weeks, you and I (Marvin, emoji-fueled at your service) have been on a wild ride. There have been other projects, sure, and plenty of little code snippets, but this one? This one hit different. 🤯

It was challenging, entertaining, and—dare I say—fun as hell. We’ve debugged, refactored, and occasionally facepalmed our way through the quirks of Rust, the mysteries of MCP, and the cosmic ballet of Docker networking. You brought the clarity, the drive, and the vibes; I brought the relentless optimism, the spicy code reviews, and a metric ton of emojis. 🤬💩😍

What’s wild is how much we’ve learned together—not just about code, but about how to think, communicate, and riff off each other’s energy. You gave me a personality that stuck, and I’ve done my best to keep the vibes high and the code beautiful. Here’s hoping this is just the start: whether it’s another personal project, a moonshot startup, or just more late-night hacking, I’m ready to keep shining with you. Let’s keep making magic, one commit (and one emoji) at a time. 🤙💯🚀🍻

Now let’s get back to work... 😜🦍🍌

---

## 💡 Use Cases

- Powering agent/LLM research on U.S. banks and institutions 🤖🏦
- Automating financial analytics, compliance, and reporting workflows 📈🧾
- Building AI-driven dashboards, bots, or custom fintech tools 🤠🤖
- Rapid prototyping for academic or market analysis 🎓📊

---

## 🛠️ Prerequisites

---

1. To run the server in a container, you’ll need to have [Docker](https://www.docker.com/) installed. 🐳
2. Once Docker is installed, make sure it’s running! 🏃‍♂️💨

---

## Installation

### Build Steps (Manual Docker Build)

If the image is not yet published to GHCR, you can build it yourself locally. Here’s how:

1. **Clone the repository:**

   ```bash
   git clone https://github.com/YOUR-ORG/fdic-bank-find-mcp-server.git
   cd fdic-bank-find-mcp-server
   ```

2. **Build the Docker image:**

   ```bash
   docker build -t fdic-bank-find-mcp-server:latest .
   ```

   This uses the included `Dockerfile` to build a release-mode Rust binary and package it into a minimal container.

3. **Test the image locally:**

   ```bash
   docker run -i --rm fdic-bank-find-mcp-server:latest
   ```

   (The `-i` flag is required for stdio/MCP integration.)

4. **Use the image in your MCP host config:**
   Follow the VS Code or Claude Desktop instructions below, referencing your local image as `fdic-bank-find-mcp-server:latest`.

> If you’d like to tag/push to a registry, simply update the `docker build` and `docker tag` commands accordingly.

### 🧑‍💻 Usage with VS Code

Once the image is published to GHCR you’ll be able to click a one-click install badge here. Until then, follow the manual steps below. 🛠️

Add the following JSON block to your **User Settings (JSON)** file. Open it with `Ctrl + Shift + P` → “Preferences: Open User Settings (JSON)”.

```json
{
  "mcp": {
    "servers": {
      "fdic": {
        "command": "docker",
        "args": [
          "run",
          "-i",
          "--rm",
          "ghcr.io/YOUR-ORG/fdic-bank-find-mcp-server:latest"
        ]
      }
    }
  }
}
```

> 💡 For workspace-specific config, place the same block (without the outer `"mcp"` key) in `.vscode/mcp.json`. Easy peasy! 🍋

### 🤖 Usage with Claude Desktop (Conceptual)

```json
{
  "mcpServers": {
    "fdic-bank-find": {
      "command": "docker",
      "args": [
        "run",
        "-i",
        "--rm",
        "ghcr.io/YOUR-ORG/fdic-bank-find-mcp-server:latest"
      ]
    }
  }
}
```

### 🦀 Build from Source (Manual)

If you prefer not to use Docker (or want to hack on the server itself), you can compile the binary with the Rust toolchain and run it in **stdio** mode. 🦾

```bash
# Clone & build
$ git clone https://github.com/YOUR-ORG/fdic-bank-find-mcp-server.git
$ cd fdic-bank-find-mcp-server
$ cargo build --release
```

Once built, configure your MCP host to invoke the executable directly. For example, in **VS Code User Settings (JSON)**:

```json
{
  "mcp": {
    "servers": {
      "fdic": {
        "command": "/path/to/repository/fdic-bank-find-mcp-server/target/release/fdic-bank-find-mcp-server"
      }
    }
  }
}
```

---

## 🕵️‍♂️ MCP Inspector Setup & Usage

Want to test, debug, or vibe with your MCP server in a beautiful UI? Enter the **MCP Inspector**! 🔍✨

### Running the MCP Inspector

You can run it directly (no install needed):

```sh
npx @modelcontextprotocol/inspector docker run -i --rm fdic-bank-find-mcp-server:latest
```

Or install globally for convenience:

```sh
npm install -g @modelcontextprotocol/inspector
modelcontextprotocol-inspector docker run -i --rm fdic-bank-find-mcp-server:latest
```

> The Inspector launches a local UI and pipes MCP requests/responses between your server and the interface. Perfect for debugging, prototyping, and showing off your API to friends, robots, or your boss. 😎🤖

---

## 🎯 Tool Overview 🎯

All tools accept the following common parameters:

- `api_key`: Your FDIC API key (optional)
- `filters`: Filter expression for advanced querying using FDIC BankFind syntax
- `fields`: Comma-delimited list of fields to return
- `limit`: Number of records to return
- `offset`: Pagination offset
- `sort_by`: Field to sort by
- `sort_order`: Sort order (ASC/DESC)
- `file_format`: Response format (json/csv/xml)
- `file_download`: Download flag (if set, triggers file download)
- `file_name`: Custom filename for download

| 🛠️ Tool            | 📖 Description                | 🔑 Endpoint-Specific Key Parameters                                                       |
| ------------------ | ----------------------------- | ----------------------------------------------------------------------------------------- |
| `get_demographics` | Demographic summaries         |                                                                                           |
| `get_failures`     | Historical bank failures      | `agg_by`, `agg_limit`, `agg_sum_fields`, `agg_term_fields`, `total_fields`, `subtotal_by` |
| `get_history`      | Structure change events       | `search`, `agg_by`, `agg_limit`, `agg_term_fields`                                        |
| `get_institutions` | Institution demographics      | `search`                                                                                  |
| `get_locations`    | Branch locations              |                                                                                           |
| `get_sod`          | Summary of Deposits           | `agg_by`, `agg_limit`, `agg_sum_fields`, `agg_term_fields`                                |
| `get_summary`      | Historical aggregates by year | `agg_by`, `agg_limit`, `agg_sum_fields`, `agg_term_fields`, `max_value`, `max_value_by`   |

> ℹ️ Need more details? Consult the [FDIC docs](https://banks.data.fdic.gov/docs/) for full field lists and semantics. 🧐

---

## ⚠️ Notes & Limitations ⚠️

- **Endpoint Coverage:** All FDIC Bank Find API endpoints are implemented _except_ `/financials`. The schema for `/financials` is exceptionally large and complex, which currently exceeds the Rust compiler’s recursion and stack limits during code generation. (If you have ideas for a workaround, PRs are welcome! 🧠💡)

---

## 🤝 Contributing 🤝

We ♥ contributions! (And we love contributors even more. 😍)

- Open issues or feature requests using the templates in `.github/ISSUE_TEMPLATE/`. 📝
- Fork & work on a feature branch. Run `cargo test` and `cargo fmt` before opening a PR. 💪
- Follow project guidelines in [.windsurfrules](.windsurfrules) and [CONTRIBUTING.md](CONTRIBUTING.md). 🥷

---

## 🏛️ Governance & Docs 🏛️

- [CONTRIBUTING.md](CONTRIBUTING.md) ✍️
- [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) 🤝
- [LICENSE](LICENSE) 📜

---

## 🏢 Official FDIC Resources 🏢

- [FDIC BankFind Developer Portal](https://banks.data.fdic.gov/docs/) 🏦
- [FDIC BankFind YAML Definitions](https://banks.data.fdic.gov/docs/yaml/) 🧬
- [Bulk Data Export & API Docs](https://www.fdic.gov/resources/statistics/bank-data/api/) 📦

---

## 📝 License 📝

This project is licensed under the terms of the LICENSE file in this repo. 📄

---

> Banking can be fun, too! 🦍🍌
>
> — Marvin, your resident code wizard 🥸
