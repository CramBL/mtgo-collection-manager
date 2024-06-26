
<p align="center">
<img src="mtgogui/assets/logo-card-pile.png" alt="logo" width="150"/>
</p>
<h1 align="center">
MTGO Collection Manager
</h1>

<!-- navbar -->
<div align="center">
  <a href="https://github.com/CramBL/mtgo-collection-manager/releases" title="Latest Stable GitHub Release"><img src="https://img.shields.io/github/release/CramBL/mtgo-collection-manager/all.svg?style=flat&logo=github&logoColor=white&colorB=blue&label=" alt="GitHub release"></a>&thinsp;<img src="https://img.shields.io/badge/-Windows-6E46A2.svg?style=flat&logo=windows-11&logoColor=white" alt="Windows" title="Supported Platform: Windows">&thinsp;<img src="https://img.shields.io/badge/-Linux-9C2A91.svg?style=flat&logo=linux&logoColor=white" alt="Linux" title="Supported Platform: Linux">&thinsp;<img src="https://img.shields.io/badge/-macOS-red.svg?style=flat&logo=apple&logoColor=white" alt="macOS" title="Supported Platform: macOS">
  <br>
  <img alt="GitHub Workflow Status (with event)" src="https://img.shields.io/github/actions/workflow/status/CramBL/mtgo-collection-manager/CI.yml?label=CI">&thinsp;<a href="https://github.com/CramBL/mtgo-collection-manager/blob/main/LICENSE" title="Project License: GPLv3"><img src="https://img.shields.io/github/license/crambl/mtgo-collection-manager?style=flat&label=%20&color=grey" alt="License"></a>&thinsp;<img alt="CodeFactor Grade" src="https://img.shields.io/codefactor/grade/github/CramBL/mtgo-collection-manager?style=flat&logo=codefactor&logoColor=white&label=Code%20Quality">&thinsp;<img src="https://tokei.rs/b1/github/CramBL/mtgo-collection-manager?type=Rust&style=flat&logo=https://simpleicons.org/icons/rust.svg&label=&color=e36705" alt="Rust total lines"></a>
</div>

## Purpose

To automate some tasks regarding effective management of [MTGO](https://www.mtgo.com/en/mtgo) collection, that are too cumbersome for anyone to actually do them manually.

**MTGO Collection Manager** aims to be as effecient and accurate as possible, while still being easy to install and use, meaning:

* **Blazingly fast**
* **As few downloads as possible** to get all relevant data
* **Lightweight app with tiny memory footprint** (e.g. the [demo](#most-recent-demo) with a fairly large collection uses [4.2 MB RAM on Windows 11](.github/doc/mtgo-cm-process-view-windows.png))
* Installation limited to **downloading a single binary** and just running it
* **No runtime dependencies** on *MacOS* and *Windows* (very few on *Linux*)
* Installation is fully contained within the directory the binary is run from (deleting the directory leaves no trace of MTGO Collection Manager)
* **No login required** - All it needs is the `Full Trade List.dek`-file generated by exporting an MTGO collection.

# Table of contents

- [Table of contents](#table-of-contents)
  - [Features? Make an issue if you have suggestions](#features-make-an-issue-if-you-have-suggestions)
    - [Most recent demo](#most-recent-demo)
- [Contributing](#contributing)
  - [Quickstart](#quickstart)
    - [Development in with a Docker container](#development-in-with-a-docker-container)


## Features? Make an issue if you have suggestions

If you have a great idea, make a feature request via an issue, thanks!

### Most recent demo

The first time MTGO Collection Manager is started, a full trade list file is needed to start tracking price data etc. The initial processing takes a few seconds as a bunch of different downloads takes place to establish the basic data needed to parse and display data about the provided collection, along with price history from *Goatbots* and *Cardhoarder*. Parsing all the data is practically instantaneous as evident by subsequent launches of the app. If new data is available for the given collection, it is downloaded on startup (options and improvements are coming). The system time is used to determine if new data is available before attempting to download and parse it.
![Demo](.github/most-recent-demo.gif)

# Contributing

There's scripts for building and testing the project described in the [Quickstart](#quickstart) section below.

You're welcome to submit PRs or make issues.

## Quickstart

### Development in with a Docker container

First install [Just](https://github.com/casey/just).

Then build the development container.

```shell
just build-devcontainer
```

```shell
just build
```

```shell
just test
```

```shell
cargo run # GUI currently has to be run outside the container
```
