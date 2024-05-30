import? 'just-util/mod.just'

@_default:
    just --list

# Entry point to ensure commands are run within the container
[no-exit-message]
cmd *ARGS:
    {{CMD}} '{{ ARGS }}'

[no-exit-message]
build-devcontainer UBUNTU_VARIANT="jammy":
	docker build \
		-t {{ DEVCONTAINER_NAME }} \
		--build-arg VARIANT={{ UBUNTU_VARIANT }} \
		--build-arg USER=${USER} \
		--build-arg gid=$( id -g ) \
		--build-arg uid=$( id -u ) \
		-f .devcontainer/Dockerfile .

[no-exit-message]
run-devcontainer:
	{{CMD_IT}}

[unix]
launch:
    ./mtgogui/target/release/mtgogui

cross-compile-windows PROFILE="dev":
    just cmd 'cargo build --profile={{PROFILE}} --target=x86_64-pc-windows-gnu'

cross-compile-windows-xwin PROFILE="dev" CLIB="gnu" ARGS="--features bundled":
    just cmd 'cargo xwin build --profile={{PROFILE}} --target x86_64-pc-windows-{{CLIB}} {{ARGS}}'

archive-cross-compile-windows PACKAGE_NAME="windows-mtgo-collection-manager":
    mkdir -p mtgo-collection-manager
    cp target/x86_64-pc-windows-gnu/release/mtgogui.exe mtgo-collection-manager/mtgo-collection-manager
    zip -r {{PACKAGE_NAME}}.zip mtgo-collection-manager

archive-cross-compile-windows-xwin PACKAGE_NAME="windows-mtgo-collection-manager" CLIB="gnu":
    mkdir -p mtgo-collection-manager
    cp target/x86_64-pc-windows-{{CLIB}}/release/mtgogui.exe mtgo-collection-manager/mtgo-collection-manager.exe
    zip -r {{PACKAGE_NAME}}.zip mtgo-collection-manager

archive-bin PACKAGE_NAME:
    #!/usr/bin/env bash
    BIN=$( find target/ -type f -executable \
            \( -name 'mtgogui' -o -name 'mtgogui.exe' \) \
            -print -quit )
    mkdir -p target/mtgo-collection-manager
    cp ${BIN} target/mtgo-collection-manager/
    if [[ "{{os_family()}}" == "unix" ]]; then
        tar --create -vv --file={{PACKAGE_NAME}}.tar --directory=target/mtgo-collection-manager mtgo-collection-manager
    else
        cd target
        zip -r {{PACKAGE_NAME}}.zip mtgo-collection-manager
        mv mtgo-collection-manager ..
    fi


install-debian-dev-deps:
    cd build-util/deps && ./install-debian-deps.sh

build *ARGS: (cmd "cargo build " + ARGS)
test *ARGS: (cmd "cargo test " + ARGS)
clippy *ARGS: (cmd "cargo clippy " + ARGS)

clean: (cmd "cargo clean")

[group("CI")]
ci-lint:
    cargo check --verbose
    cargo fmt -- --check
    cargo audit
    cargo doc
    cargo clean
    cargo clippy -- -D warnings --no-deps

# Print tool versions
env:
    #!/usr/bin/env bash
    set -eou pipefail
    tools=(
        just
        rustc
        cargo
        cmake
        clang
        gcc
        ninja
        docker
        curl
    )
    for t in "${tools[@]}"; do
        if [[ "{{os_family()}}" == "unix" ]]; then
            {{PRINT_RGB}} 255 155 100 "==> ${t}: "
        else
            echo "==> ${t}:"
        fi
        ${t} --version 2>/dev/null || echo "not found"
    done
