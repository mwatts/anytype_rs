<!-- omit in toc -->
# Contributing to anytype_rs

First off, thanks for taking the time to contribute! ❤️

All types of contributions are encouraged and valued. See the [Table of Contents](#table-of-contents) for different ways to help and details about how this project handles them. Please make sure to read the relevant section before making your contribution. It will make it a lot easier for us maintainers and smooth out the experience for all involved. The community looks forward to your contributions. 🎉

> And if you like the project, but just don't have time to contribute, that's fine. There are other easy ways to support the project and show your appreciation, which we would also be very happy about:
> - Star the project
> - Tweet about it
> - Refer this project in your project's readme
> - Mention the project at local meetups and tell your friends/colleagues

<!-- omit in toc -->
## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [I Have a Question](#i-have-a-question)
- [I Want To Contribute](#i-want-to-contribute)
  - [Reporting Bugs](#reporting-bugs)
  - [Suggesting Enhancements](#suggesting-enhancements)
  - [Your First Code Contribution](#your-first-code-contribution)
    - [1. Set Up Your Development Environment](#1-set-up-your-development-environment)
    - [2. Find an Issue to Work On](#2-find-an-issue-to-work-on)
    - [3. Make Your Changes](#3-make-your-changes)
    - [4. Submit a Pull Request](#4-submit-a-pull-request)
  - [Improving The Documentation](#improving-the-documentation)
    - [Types of Documentation Contributions](#types-of-documentation-contributions)
    - [Documentation Structure](#documentation-structure)
    - [Making Documentation Changes](#making-documentation-changes)
    - [Documentation Guidelines](#documentation-guidelines)
    - [Building Documentation Locally](#building-documentation-locally)
- [Styleguides](#styleguides)
  - [Commit Messages](#commit-messages)
    - [Commit Message Format](#commit-message-format)
    - [Types](#types)
    - [Subject Line](#subject-line)
    - [Body (Optional)](#body-optional)
    - [Examples](#examples)
- [Join The Project Team](#join-the-project-team)
  - [Paths to Joining](#paths-to-joining)
    - [Active Contributors](#active-contributors)
    - [Areas of Expertise](#areas-of-expertise)
  - [Maintainer Responsibilities](#maintainer-responsibilities)
  - [Recognition](#recognition)
  - [Getting Started](#getting-started)
  - [Contact](#contact)


## Code of Conduct

This project and everyone participating in it is governed by the
[anytype_rs Code of Conduct](https://github.com/lanesawyer/anytype_rs/blob/main/CODE_OF_CONDUCT.md).
By participating, you are expected to uphold this code. Please report unacceptable behavior
by [opening an issue](https://github.com/lanesawyer/anytype_rs/issues) or contacting the maintainers.


## I Have a Question

> If you want to ask a question, we assume that you have read the available [Documentation](https://github.com/lanesawyer/anytype_rs/tree/main/docs).

Before you ask a question, it is best to search for existing [Issues](https://github.com/lanesawyer/anytype_rs/issues) that might help you. In case you have found a suitable issue and still need clarification, you can write your question in this issue. It is also advisable to search the internet for answers first.

If you then still feel the need to ask a question and need clarification, we recommend the following:

- Open an [Issue](https://github.com/lanesawyer/anytype_rs/issues/new).
- Provide as much context as you can about what you're running into.
- Provide project and platform versions (e.g. Library version, Rust version, Anytype version), depending on what seems relevant.

We will then take care of the issue as soon as possible.

<!--
You might want to create a separate issue tag for questions and include it in this description. People should then tag their issues accordingly.

Depending on how large the project is, you may want to outsource the questioning, e.g. to Stack Overflow or Gitter. You may add additional contact and information possibilities:
- IRC
- Slack
- Gitter
- Stack Overflow tag
- Blog
- FAQ
- Roadmap
- E-Mail List
- Forum
-->

## I Want To Contribute

> ### Legal Notice <!-- omit in toc -->
> When contributing to this project, you must agree that you have authored 100% of the content, that you have the necessary rights to the content and that the content you contribute may be provided under the project licence.

### Reporting Bugs

<!-- omit in toc -->
#### Before Submitting a Bug Report

A good bug report shouldn't leave others needing to chase you up for more information. Therefore, we ask you to investigate carefully, collect information and describe the issue in detail in your report. Please complete the following steps in advance to help us fix any potential bug as fast as possible.

- Make sure that you are using the latest version.
- Determine if your bug is really a bug and not an error on your side e.g. using incompatible environment components/versions (Make sure that you have read the [documentation](https://github.com/lanesawyer/anytype_rs/tree/main/docs). If you are looking for support, you might want to check [this section](#i-have-a-question)).
- To see if other users have experienced (and potentially already solved) the same issue you are having, check if there is not already a bug report existing for your bug or error in the [bug tracker](https://github.com/lanesawyer/anytype_rs/issues?q=label%3Abug).
- Also make sure to search the internet (including Stack Overflow) to see if users outside of the GitHub community have discussed the issue.
- Collect information about the bug:
  - Stack trace (Traceback)
  - OS, Platform and Version (Windows, Linux, macOS, x86, ARM)
  - Version of the interpreter, compiler, SDK, runtime environment, package manager, depending on what seems relevant.
  - Possibly your input and the output
  - Can you reliably reproduce the issue? And can you also reproduce it with older versions?

<!-- omit in toc -->
#### How Do I Submit a Good Bug Report?

> You must never report security related issues, vulnerabilities or bugs including sensitive information to the issue tracker, or elsewhere in public. Instead, please report security issues by opening a private security advisory on GitHub.
<!-- You may add a PGP key to allow the messages to be sent encrypted as well. -->

We use GitHub issues to track bugs and errors. If you run into an issue with the project:

- Open an [Issue](https://github.com/lanesawyer/anytype_rs/issues/new). (Since we can't be sure at this point whether it is a bug or not, we ask you not to talk about a bug yet and not to label the issue.)
- Explain the behavior you would expect and the actual behavior.
- Please provide as much context as possible and describe the *reproduction steps* that someone else can follow to recreate the issue on their own. This usually includes your code. For good bug reports you should isolate the problem and create a reduced test case.
- Provide the information you collected in the previous section.

Once it's filed:

- The project team will label the issue accordingly.
- A team member will try to reproduce the issue with your provided steps. If there are no reproduction steps or no obvious way to reproduce the issue, the team will ask you for those steps and mark the issue as `needs-repro`. Bugs with the `needs-repro` tag will not be addressed until they are reproduced.
- If the team is able to reproduce the issue, it will be marked `needs-fix`, as well as possibly other tags (such as `critical`), and the issue will be left to be [implemented by someone](#your-first-code-contribution).

<!-- You might want to create an issue template for bugs and errors that can be used as a guide and that defines the structure of the information to be included. If you do so, reference it here in the description. -->


### Suggesting Enhancements

This section guides you through submitting an enhancement suggestion for anytype_rs, **including completely new features and minor improvements to existing functionality**. Following these guidelines will help maintainers and the community to understand your suggestion and find related suggestions.

<!-- omit in toc -->
#### Before Submitting an Enhancement

- Make sure that you are using the latest version.
- Read the [documentation](https://github.com/lanesawyer/anytype_rs/tree/main/docs) carefully and find out if the functionality is already covered, maybe by an individual configuration.
- Perform a [search](https://github.com/lanesawyer/anytype_rs/issues) to see if the enhancement has already been suggested. If it has, add a comment to the existing issue instead of opening a new one.
- Find out whether your idea fits with the scope and aims of the project. It's up to you to make a strong case to convince the project's developers of the merits of this feature. Keep in mind that we want features that will be useful to the majority of our users and not just a small subset. If you're just targeting a minority of users, consider writing an add-on/plugin library.

<!-- omit in toc -->
#### How Do I Submit a Good Enhancement Suggestion?

Enhancement suggestions are tracked as [GitHub issues](https://github.com/lanesawyer/anytype_rs/issues).

- Use a **clear and descriptive title** for the issue to identify the suggestion.
- Provide a **step-by-step description of the suggested enhancement** in as many details as possible.
- **Describe the current behavior** and **explain which behavior you expected to see instead** and why. At this point you can also tell which alternatives do not work for you.
- You may want to **include screenshots or screen recordings** which help you demonstrate the steps or point out the part which the suggestion is related to. You can use [LICEcap](https://www.cockos.com/licecap/) to record GIFs on macOS and Windows, and the built-in [screen recorder in GNOME](https://help.gnome.org/users/gnome-help/stable/screen-shot-record.html.en) or [SimpleScreenRecorder](https://github.com/MaartenBaert/ssr) on Linux. <!-- this should only be included if the project has a GUI -->
- **Explain why this enhancement would be useful** to most anytype_rs users. You may also want to point out the other projects that solved it better and which could serve as inspiration.

<!-- You might want to create an issue template for enhancement suggestions that can be used as a guide and that defines the structure of the information to be included. If you do so, reference it here in the description. -->

### Your First Code Contribution

Ready to make your first contribution? Here's how to get started:

#### 1. Set Up Your Development Environment

Follow the [Development Guide](docs/development.md) for detailed setup instructions. Quick start:

```bash
# Clone the repository
git clone https://github.com/lanesawyer/anytype_rs.git
cd anytype_rs

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build the project
cargo build --workspace

# Run tests
cargo test --workspace
```

#### 2. Find an Issue to Work On

- Look for issues labeled [`good first issue`](https://github.com/lanesawyer/anytype_rs/labels/good%20first%20issue) - these are perfect for newcomers
- Check [`help wanted`](https://github.com/lanesawyer/anytype_rs/labels/help%20wanted) issues for areas where contributions are especially welcome
- Comment on the issue to let others know you're working on it

#### 3. Make Your Changes

- Create a feature branch: `git checkout -b feature/my-feature` or `git checkout -b fix/my-bugfix`
- Follow the code style guidelines (run `cargo fmt` and `cargo clippy`)
- Add tests for your changes
- Update documentation if needed
- Make sure all tests pass: `cargo test --workspace`

#### 4. Submit a Pull Request

- Push your branch to your fork
- Open a pull request against the `main` branch
- Fill in the pull request template
- Link to any related issues
- Wait for review and address any feedback

See [docs/development.md](docs/development.md) for detailed development workflows.

### Improving The Documentation

Documentation improvements are always welcome! Here's how you can help:

#### Types of Documentation Contributions

1. **Fix Errors**: Found a typo, broken link, or incorrect information? Please fix it!
2. **Add Examples**: More code examples help users understand how to use the library
3. **Clarify Explanations**: Make complex topics easier to understand
4. **Add Missing Info**: Document undocumented features or behaviors

#### Documentation Structure

Our documentation is organized as follows:

- **README.md** - Project overview and quick start
- **docs/development.md** - Development guide for contributors
- **docs/nushell-plugin.md** - Nushell plugin user guide
- **docs/examples.md** - Rust library usage examples
- **docs/testing.md** - Testing infrastructure guide
- **docs/HTTP_TRACING.md** - HTTP debugging guide
- **docs/roadmap.md** - Project vision and roadmap

#### Making Documentation Changes

1. **Small fixes** (typos, broken links): Feel free to submit a PR directly
2. **Larger changes** (new sections, restructuring): Open an issue first to discuss

#### Documentation Guidelines

- **Test code examples** - Ensure all code examples actually compile and run
- **Keep it concise** - Clear and focused is better than comprehensive but confusing
- **Update cross-references** - When moving content, update links in other docs
- **Use proper markdown** - Follow GitHub-flavored markdown conventions
- **Add code comments** - Explain what code examples are doing

#### Building Documentation Locally

```bash
# Build API documentation
cargo doc --workspace --no-deps --open

# This will open the generated docs in your browser
```

## Styleguides
### Commit Messages

We follow these guidelines for commit messages to maintain a clear and useful project history:

#### Commit Message Format

```
<type>: <subject>

<body> (optional)

<footer> (optional)
```

#### Types

- **feat**: A new feature
- **fix**: A bug fix
- **docs**: Documentation only changes
- **style**: Changes that don't affect the meaning of the code (formatting, missing semicolons, etc.)
- **refactor**: A code change that neither fixes a bug nor adds a feature
- **perf**: A code change that improves performance
- **test**: Adding missing tests or correcting existing tests
- **chore**: Changes to the build process or auxiliary tools

#### Subject Line

- Use the imperative mood ("Add feature" not "Added feature")
- Don't capitalize the first letter
- No period at the end
- Limit to 50 characters
- Be specific and descriptive

#### Body (Optional)

- Separate from subject with a blank line
- Wrap at 72 characters
- Explain **what** and **why**, not **how**
- Reference issues and pull requests

#### Examples

```
feat: add bulk import command for markdown files

Implements a new command that can import multiple markdown files
at once by scanning a directory. Includes dry-run mode for safety.

Closes #42
```

```
fix: resolve relative file paths in import command

The import command now properly resolves relative paths against
the current working directory using Nushell's engine APIs.
```

```
docs: update development guide with correct workspace structure

The guide had outdated paths referencing anytype-core/ which
doesn't exist. Updated to reflect actual bin/cli/ and crates/
structure.
```

## Join The Project Team

We're always looking for dedicated contributors to join the core team! Here's how you can become more involved:

### Paths to Joining

#### Active Contributors
If you've made several quality contributions and are interested in a larger role:
1. Demonstrate consistent, high-quality contributions over time
2. Show understanding of the project's architecture and goals
3. Help review other contributors' pull requests
4. Participate in issue discussions and help other users

#### Areas of Expertise
We especially value contributors with expertise in:
- **API Development**: Rust, async programming, HTTP clients
- **CLI Tools**: Command-line interface design and implementation
- **Nushell Plugin Development**: Nu plugin system, custom values
- **Documentation**: Technical writing, examples, tutorials
- **Testing**: Property-based testing, snapshot testing, integration tests
- **Anytype Integration**: Understanding of Anytype's data model and use cases

### Maintainer Responsibilities

Core maintainers help with:
- Reviewing and merging pull requests
- Triaging and labeling issues
- Guiding project direction and roadmap
- Mentoring new contributors
- Release management
- Community engagement

### Recognition

We believe in recognizing all contributions:
- **Contributors**: Listed in release notes for their contributions
- **Regular Contributors**: Acknowledged in the README
- **Core Maintainers**: Listed as project maintainers with commit access

### Getting Started

1. **Start Contributing**: Make quality contributions following this guide
2. **Engage with Community**: Participate in discussions, help others
3. **Express Interest**: Open an issue or discussion expressing your interest in joining the team
4. **Review Process**: Current maintainers will review your contributions and involvement

### Contact

Interested in joining the team? Reach out by:
- Opening a [GitHub Discussion](https://github.com/lanesawyer/anytype_rs/discussions)
- Commenting on issues where you'd like to take ownership
- Mentioning your interest in pull requests

We look forward to working with you!

<!-- omit in toc -->
## Attribution
This guide is based on the **contributing-gen**. [Make your own](https://github.com/bttger/contributing-gen)!
