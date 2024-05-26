# [Choice] bionic (18.04), focal (20.04), jammy (22.04)
ARG VARIANT="jammy"

# First stage: build base image, just enough to build MTGO Collection Manager and run tests
FROM ubuntu:${VARIANT} as build-base

# Restate the variant to use it later on in the llvm and cmake installations
ARG VARIANT

ARG HOST_USER
ARG HOST_UID
ARG HOST_GID

RUN groupadd --gid ${HOST_GID} ${HOST_USER} \
    && useradd --uid ${HOST_UID} --gid ${HOST_GID} -m ${HOST_USER}

# Install necessary packages available from standard repos
RUN touch /in_container \
    && apt-get update -qq \
    && export DEBIAN_FRONTEND=noninteractive \
    && apt-get install -y --no-install-recommends \
    # software-properties-common contains add-apt-repository
    software-properties-common \
    wget \
    apt-utils \
    file \
    zip \
    openssh-client \
    gpg-agent \
    socat \
    rsync \
    make \
    ninja-build \
    git \
    build-essential \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# User-settable versions:
# This Dockerfile should support gcc-[7, 8, 9, 10, 11, 12, 13] and clang-[10, 11, 12, 13, 14, 15, 16, 17]
# Earlier versions of clang will require significant modifications to the IWYU section
ARG GCC_VER="13"
# Add gcc-${GCC_VER}
RUN add-apt-repository -y ppa:ubuntu-toolchain-r/test \
    && apt-get update -qq \
    && export DEBIAN_FRONTEND=noninteractive \
    && apt-get install -y --no-install-recommends \
    gcc-${GCC_VER} \
    g++-${GCC_VER} \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Set gcc-${GCC_VER} as default gcc
RUN update-alternatives --install /usr/bin/gcc gcc $(which gcc-${GCC_VER}) 100
RUN update-alternatives --install /usr/bin/g++ g++ $(which g++-${GCC_VER}) 100


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
    clang-tidy-${LLVM_VER} \
    # Other tools
    lldb-${LLVM_VER} \
    lld-${LLVM_VER} \
    # Compiler runtime contains the sanitizers
    libclang-rt-${LLVM_VER}-dev \
    # libc++
    libc++-${LLVM_VER}-dev \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Set the default clang-tidy, so CMake can find it
RUN update-alternatives --install /usr/bin/clang-tidy clang-tidy $(which clang-tidy-${LLVM_VER}) 1

# Set clang-${LLVM_VER} as default clang
RUN update-alternatives --install /usr/bin/clang clang $(which clang-${LLVM_VER}) 100
RUN update-alternatives --install /usr/bin/clang++ clang++ $(which clang++-${LLVM_VER}) 100
# Set ld.lld to point to a matching LLVM version
RUN update-alternatives --install /usr/bin/ld.lld ld.lld /usr/bin/ld.lld-${LLVM_VER} 100

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

# Download and install Rust using the official Rust installer script
RUN wget --progress=dot:giga -O - https://sh.rustup.rs | sh -s -- -y

# Add the Rust binaries to the system PATH
ENV PATH=/root/.cargo/bin:${PATH}
RUN rustc --version
COPY Justfile .
COPY build-util build-util
RUN cargo install --locked \
    cargo-xwin \
    just \
    cargo-audit \
    && just install-debian-dev-deps

# Allow the user to set compiler defaults
ARG USE_CLANG
# if --build-arg USE_CLANG=1, set CC to 'clang' or set to null otherwise.
ENV CC=${USE_CLANG:+"clang"}
ENV CXX=${USE_CLANG:+"clang++"}
# if CC is null, set it to 'gcc' (or leave as is otherwise).
ENV CC=${CC:-"gcc"}
ENV CXX=${CXX:-"g++"}

RUN rustup target add \
    x86_64-pc-windows-gnu \
    x86_64-pc-windows-msvc \
    && rustup component add llvm-tools-preview \
    && apt-get update -qq \
    && apt-get install -y gcc-mingw-w64-x86-64 mingw-w64 \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /work

RUN git config --global --add safe.directory /work


CMD ["/bin/bash"]
