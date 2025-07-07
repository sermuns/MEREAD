<a href="https://github.com/sermuns/meread">
  <img alt="MEREAD" src="media/banner.png">
</a>

<div align="center">
  <p>
  <em>
      Locally preview how GitHub will render your Markdown files.
  </em>
  </p>
  <a href="https://github.com/sermuns/meread/releases/latest">
    <img alt="release-badge" src="https://img.shields.io/github/v/release/sermuns/meread.svg"></a>
  <a href="https://github.com/sermuns/meread/blob/main/LICENSE">
    <img alt="WTFPL" src="https://img.shields.io/badge/License-WTFPL-brightgreen.svg"></a>
  <a href="https://crates.io/crates/meread"><img src="https://img.shields.io/crates/v/meread.svg" alt="Version info"></a>
</div>

---

MEREAD is a command-line tool for previewing Markdown files as they will be presented on GitHub, all completely locally and offline.

## Motivation

I was surprised to find no _simple_ tool that would allow me to preview Markdown files as they would be rendered on GitHub.

I wanted a tool that is:

- shipped as a single binary
- fast
- accurate to GitHub's rendering
- offline

There are other tools that get the job done, better or worse, but they all have some drawbacks that I wanted to avoid:

| Tool                                                                     | Written in | Biggest drawback                                                                                |
| ------------------------------------------------------------------------ | ---------- | ----------------------------------------------------------------------------------------------- |
| [grip](https://github.com/joeyespo/grip)                                 | Python     | Uses GitHub:s markdown API to render Markdown files, causing unnecessary usage of web requests. |
| [gh markdown-preview](https://github.com/yusukebe/gh-markdown-preview)   | Go         | Is meant to be used as extension in `gh`, GitHub's CLI.                                         |
| [markdown-preview.nvim](https://github.com/iamcco/markdown-preview.nvim) | Typescript | Requires Neovim.                                                                                |

## Installation

### From prebuilt binaries

For each version, prebuilt binaries are automatically built for Linux, MacOS and Windows.

- You can download and unpack the
  latest release from the [releases page](https://github.com/sermuns/meread/releases/latest).

- Using [`cargo-binstall`](https://github.com/cargo-bins/cargo-binstall)

  ```bash
  cargo binstall meread
  ```

- Using [`ubi`](https://github.com/houseabsolute/ubi):

  ```bash
  ubi -p sermuns/meread
  ```

- Using [`mise`](https://github.com/jdx/mise):
  ```bash
  # `ubi` under the hood
  mise use -g ubi:sermuns/meread
  ```
  ```bash
  # `cargo-binstall` under the hood
  mise use -g cargo:meread
  ```

### From source

- ```bash
  cargo install meread
  ```

- ```bash
  git clone https://github.com/sermuns/meread
  cd meread
  cargo install
  ```
