# Anytype.rs Documentation

Welcome to the anytype_rs documentation directory! This is your navigation hub for all project documentation.

## 📚 Documentation Index

### User Guides

| Document | Description | Status |
|----------|-------------|--------|
| **[Nushell Plugin Guide](nushell-plugin.md)** | Complete guide to installing and using the Nushell plugin | ✅ Complete |
| **[Examples](examples.md)** | Rust library usage examples and patterns | ⚠️ Verify examples |

### Developer Guides

| Document | Description | Status |
|----------|-------------|--------|
| **[Development Guide](development.md)** | Project structure, setup, and contributing | ✅ Complete |
| **[Testing Guide](testing.md)** | Testing infrastructure (mock, snapshot, property tests) | ✅ Complete |
| **[HTTP Tracing Guide](HTTP_TRACING.md)** | Debugging HTTP requests/responses | ✅ Complete |

### Planning & Reference

| Document | Description | Status |
|----------|-------------|--------|
| **[Roadmap](roadmap.md)** | Project vision, priorities, and milestones | ✅ Complete |
| **[Archive](archive/)** | Completed planning documents (historical reference) | ✅ Archived |

## 🎯 Quick Start by Role

### I want to use the Nushell plugin
→ Read [nushell-plugin.md](nushell-plugin.md)

### I want to use the Rust library
→ See [examples.md](examples.md) and the [main README](../README.md)

### I want to contribute code
→ Start with [development.md](development.md)

### I want to understand testing
→ Check [testing.md](testing.md)

### I want to debug HTTP issues
→ Use [HTTP_TRACING.md](HTTP_TRACING.md)

### I want to see the project roadmap
→ Read [roadmap.md](roadmap.md)

## 📖 Also Available

- **[Main README](../README.md)** - Project overview, installation, and quick start
- **[CLAUDE.md](../CLAUDE.md)** - Quick reference for AI-assisted development
- **[CONTRIBUTING.md](../CONTRIBUTING.md)** - Contribution guidelines
- **[CODE_OF_CONDUCT.md](../CODE_OF_CONDUCT.md)** - Community guidelines

## 🏗️ Project Structure

For the complete workspace structure, see [development.md - Project Structure](development.md#project-structure).

**Quick Overview:**
```
anytype_rs/
├── bin/cli/                # CLI binary (atc)
├── crates/
│   ├── anytype_rs/         # Core library
│   └── nu_plugin_anytype/  # Nushell plugin
└── docs/                   # Documentation (you are here)
```

## 💡 Contributing to Documentation

Found an error or want to improve the docs?

1. **File an issue** describing the problem
2. **Submit a PR** with your improvements
3. **Test examples** - Ensure code examples actually work
4. **Update cross-references** - Keep links between docs accurate
5. **Keep it concise** - Clear and focused documentation is best

## 🔗 External Resources

- [Anytype Application](https://anytype.io/) - The app we're integrating with
- [Anytype API Docs](https://developers.anytype.io/) - Official API documentation
- [Nushell](https://www.nushell.sh/) - The shell we're extending
- [Rust Book](https://doc.rust-lang.org/book/) - Learn Rust
- [Tokio](https://tokio.rs/) - Async runtime we use
- [Tracing](https://docs.rs/tracing/) - Logging framework

---

**Last Updated:** 2025-10-11
**Documentation Status:** 67% Complete (10/15 files fully accurate)
