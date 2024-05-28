# [Choice] bionic (18.04), focal (20.04), jammy (22.04)
ARG VARIANT="jammy"

# First stage: build base image, just enough to build MTGO Collection Manager and run tests
FROM ubuntu:${VARIANT} as build-base

# Restate the variant to use it later on in the llvm and cmake installations
ARG VARIANT

ARG uid
ARG gid
ARG USER=dev

ENV TERM xterm-256color
ENV HOME=/home/$USER
ENV PATH="${PATH}:$HOME/.local/bin"

ARG DEBIAN_FRONTEND=noninteractive

RUN useradd -m -s /bin/bash $USER \
    && mkdir -p /work \
    && chown $USER:$USER /work \
    && echo "PS1='\e[32;1m\u: \e[34m\w\e[0m\$ '" >> $HOME/.bashrc \
    && usermod -u ${uid} ${USER} \
    && groupmod -g ${gid} ${USER} \
    && usermod -a -G ${gid} ${USER} 

# Install necessary packages available from standard repos
RUN touch /in_container \
    && apt-get update -qq \
    && export DEBIAN_FRONTEND=noninteractive \
    && apt-get install -y --no-install-recommends \
    # software-properties-common contains add-apt-repository
    software-properties-common \
    wget \
    curl \
    apt-utils \
    file \
    zip \
    openssh-client \
    gpg-agent \
    ninja-build \
    git \
    build-essential \
    # For cross compiling to windows
    gcc-mingw-w64-x86-64 \
    mingw-w64 \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Install just via curl just to lower container build times
ARG JUST_VERSION=1.27.0
RUN curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --tag "${JUST_VERSION}" --to /usr/bin
COPY Justfile .
COPY just-util just-util
COPY build-util build-util
RUN just install-debian-dev-deps

# Installation instructions on LLVM: https://apt.llvm.org/
ARG LLVM_VER="17"
# Add clang-${LLVM_VER}
ARG LLVM_URL="http://apt.llvm.org/${VARIANT}/"
ARG LLVM_PKG="llvm-toolchain-${VARIANT}-${LLVM_VER}"

RUN wget --progress=dot:giga -O - https://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add - 2>/dev/null \
    && add-apt-repository -y "deb ${LLVM_URL} ${LLVM_PKG} main" \
    && apt-get update -qq \
    && export DEBIAN_FRONTEND=noninteractive \
    && apt-get install -y --no-install-recommends \
    # LLVM
    libllvm${LLVM_VER} \
    llvm-${LLVM_VER} \
    llvm-${LLVM_VER}-dev \
    # Clang
    clang-${LLVM_VER} \
    clang-tools-${LLVM_VER} \
    libclang-common-${LLVM_VER}-dev \
    libclang-${LLVM_VER}-dev \
    libclang1-${LLVM_VER} \
    clangd-${LLVM_VER} \
    # Other tools
    lldb-${LLVM_VER} \
    lld-${LLVM_VER} \
    # libc++
    libc++-${LLVM_VER}-dev \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Set clang-${LLVM_VER} as default clang
RUN update-alternatives --install /usr/bin/clang clang $(which clang-${LLVM_VER}) 100
RUN update-alternatives --install /usr/bin/clang++ clang++ $(which clang++-${LLVM_VER}) 100
# Set ld.lld to point to a matching LLVM version
RUN update-alternatives --install /usr/bin/ld.lld ld.lld /usr/bin/ld.lld-${LLVM_VER} 100

ENV CC="clang"
ENV CXX="clang++"

# Add current cmake/ccmake, from Kitware
ARG CMAKE_URL="https://apt.kitware.com/ubuntu/"
ARG CMAKE_PKG=${VARIANT}
RUN wget --progress=dot:giga -O - https://apt.kitware.com/keys/kitware-archive-latest.asc 2>/dev/null \
    | gpg --dearmor - | tee /etc/apt/trusted.gpg.d/kitware.gpg >/dev/null \
    && apt-add-repository -y "deb ${CMAKE_URL} ${CMAKE_PKG} main" \
    && apt-get update -qq \
    && export DEBIAN_FRONTEND=noninteractive \
    && apt-get install -y --no-install-recommends \
    cmake \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

USER ${USER}

RUN just --version \
    && curl --version \
    && wget --version \
    && cmake --version \
    && zip --version \
    && ninja --version

# Download and install Rust using the official Rust installer script
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Add the Rust binaries to the system PATH
ENV PATH="${PATH}:$HOME/.cargo/bin"
RUN rustc --version

RUN cargo install --locked \
    cargo-xwin \
    cargo-audit


# For cross-compiling to windows
RUN rustup target add \
    x86_64-pc-windows-gnu \
    && rustup component add llvm-tools-preview

WORKDIR /work

RUN git config --global --add safe.directory /work


CMD ["/bin/bash"]
