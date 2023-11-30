
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
  <img alt="GitHub Workflow Status (with event)" src="https://img.shields.io/github/actions/workflow/status/CramBL/mtgo-collection-manager/integration-ci.yml?label=CI">&thinsp;<a href="https://github.com/CramBL/mtgo-collection-manager/blob/main/LICENSE" title="Project License: MPLv2"><img src="https://img.shields.io/github/license/crambl/mtgo-collection-manager?style=flat&label=%20&color=grey" alt="License"></a>&thinsp;<img alt="CodeFactor Grade" src="https://img.shields.io/codefactor/grade/github/CramBL/mtgo-collection-manager?style=flat&logo=codefactor&logoColor=white&label=Code%20Quality">&thinsp;<img src="https://tokei.rs/b1/github/CramBL/mtgo-collection-manager?type=Rust,Go,C%2b%2b,C%2b%2b%20Header&style=flat&label=Total lines" alt="total lines"></a>&thinsp;<img src="https://tokei.rs/b1/github/CramBL/mtgo-collection-manager?type=C%2b%2b,C%2b%2b%20Header&style=flat&logo=https://simpleicons.org/icons/cplusplus.svg&label=&color=003aff" alt="C++ total lines"></a>&thinsp;<img src="https://tokei.rs/b1/github/CramBL/mtgo-collection-manager?type=Rust&style=flat&logo=https://simpleicons.org/icons/rust.svg&label=&color=e36705" alt="Rust total lines"></a>&thinsp;<img src="https://tokei.rs/b1/github/CramBL/mtgo-collection-manager?type=Go&style=flat&logo=https://simpleicons.org/icons/go.svg&label=&color=00a6e9" alt="Go total lines"></a>
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
  - [Features? Make an issue if you have suggestions.](#features-make-an-issue-if-you-have-suggestions)
    - [Most recent demo](#most-recent-demo)
- [Contributing](#contributing)
  - [Quickstart](#quickstart)
    - [Docker (with .devcontainer/Dockerfile)](#docker-with-devcontainerdockerfile)
    - [Unix-like (with Make)](#unix-like-with-make)
    - [Windows (with PowerShell)](#windows-with-powershell)
    - [Trouble shooting](#trouble-shooting)
      - [Compiling FLTK-rs on Linux](#compiling-fltk-rs-on-linux)
      - [Ubuntu](#ubuntu)
      - [Debian](#debian)


## Features? Make an issue if you have suggestions.
If you have a great idea, make a feature request via an issue, thanks!

### Most recent demo
The first time MTGO Collection Manager is started, a full trade list file is needed to start tracking price data etc. The initial processing takes a few seconds as a bunch of different downloads takes place to establish the basic data needed to parse and display data about the provided collection, along with price history from *Goatbots* and *Cardhoarder*. Parsing all the data is practically instantaneous as evident by subsequent launches of the app. If new data is available for the given collection, it is downloaded on startup (options and improvements are coming). The system time is used to determine if new data is available before attempting to download and parse it.
![Demo](.github/most-recent-demo.gif)

# Contributing
There's scripts for building and testing all projects described in the [Quickstart](#quickstart) section below.

You're welcome to submit PRs or make issues.

If you're serious about starting a collaboration, send me a mail at `mbkj@tutamail.com`.

## Quickstart
### Docker (with [.devcontainer/Dockerfile](.devcontainer/Dockerfile))
Build the image
```shell
docker build -t "mcm-dev:Dockerfile" .devcontainer
docker run --rm -it mcm-dev:Dockerfile
```
Now continuing from the container's shell
```shell
cd home
git clone https://github.com/CramBL/mtgo-collection-manager.git
cd mtgo-collection-manager
make all && make test
```
The very first build will take a while...
### Unix-like (with Make)
A Makefile lets you build and test any or all of the subprojects.

Check versions of Rust/Go/C++ (also display minimum required versions) and more
```shell
make show-versions
```

Build all projects `order: MTGO Getter -> MTGO Parser -> MTGO Updater -> MTGO GUI`
```shell
make
```
Test all projects `order: MTGO Getter -> MTGO Parser -> MTGO Updater -> MTGO GUI`
```shell
make test
```
Build/test a single subproject with the `-{projectname}`-suffix e.g.
```shell
make test-mtgogetter
```

### Windows (with PowerShell)

A PowerShell script lets you build and test any or all of the subprojects in a manner similar to a Makefile.

Check versions of Rust/Go/C++ (also display minimum required versions) and more
```shell
.\wmake.ps1 show-versions
```

Build all projects `order: MTGO Getter -> MTGO Parser -> MTGO Updater -> MTGO GUI`
```shell
.\wmake.ps1
```
Test all projects `order: MTGO Getter -> MTGO Parser -> MTGO Updater -> MTGO GUI`
```shell
.\wmake.ps1 test
```
Build/test a single subproject with the `-{projectname}`-suffix e.g.
```shell
.\wmake.ps1 test-mtgogetter
```

### Trouble shooting
#### Compiling FLTK-rs on Linux
Compiling FLTK requires some development headers on Linux.
#### Ubuntu
Everything should be in [ubuntu-fltk-dev-headers.txt](build-util/dev-ubuntu/ubuntu-fltk-dev-headers.txt) which the CI Linux runners install with the minimal [install-ubuntu-fltk-dev-headers.sh](build-util/dev-ubuntu/install-ubuntu-fltk-dev-headers.sh) script, you can invoke that script as well to install needed headers.

Tested on Ubuntu 22.04.3 and Lubuntu 22.04.3.

#### Debian
When compiling with GCC you will need headers for statically linking with **libstdc++**, which are installable via **dnf** and are found as `libstdc++-static.<CPU architecture>` or simiar, e.g. `libstdc++-static.x86_64`.

Tested on CentOS Stream 9.