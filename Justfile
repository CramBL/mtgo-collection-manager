PWD := `pwd`
USE_CLANG := env('USE_CLANG', '1')
GCOV_EXECUTABLE := if USE_CLANG == "1" { "llvm-cov gcov" } else { "gcov" }
CC  := if USE_CLANG == "1" { "/usr/bin/clang"   } else { "gcc" }
CXX := if USE_CLANG == "1" { "/usr/bin/clang++" } else { "g++" }
DEVCONTAINER_NAME := "mtgo-cm-devcontainer"
CMD := if path_exists('/in_container') == "true" {
"eval"
} else {
"docker run" \
+ " -e CC=" + CC \
+ " -e CXX=" + CXX \
+ " -e XWIN_CACHE_DIR=/work/xwin_cache"
+ " -v " + PWD + ":/work" \
+ " --rm" \
+ " -t " + DEVCONTAINER_NAME \
+ " /bin/bash -lc "
}

CMD_IT := if path_exists('/in_container') == "true" {
"eval"
} else {
"docker run" \
+ " -e CC=" + CC \
+ " -e CXX=" + CXX \
+ " -v " + PWD + ":/work" \
+ " --rm" \
+ " -it " + DEVCONTAINER_NAME \
+ " /bin/bash -l "
}

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
		--build-arg USE_CLANG={{ USE_CLANG }} \
		--build-arg VARIANT={{ UBUNTU_VARIANT }} \
		--build-arg HOST_USER=${USER} \
		--build-arg HOST_GID=$( id -g ) \
		--build-arg HOST_UID=$( id -u ) \
		-f .devcontainer/Dockerfile .

[no-exit-message]
run-devcontainer:
	{{CMD_IT}}


ci-install-cross-compile-windows-deps:
    rustup target add x86_64-pc-windows-gnu 
    sudo apt-get install gcc-mingw-w64-x86-64 ninja-build cmake

cross-compile-windows PROFILE="dev":
    just cmd 'cd mtgogui && cargo build --profile={{PROFILE}} --target=x86_64-pc-windows-gnu'

cross-compile-windows-xwin PROFILE="dev":
    just cmd 'cd mtgogui && cargo xwin build --profile={{PROFILE}} --target x86_64-pc-windows-msvc'

archive-cross-compile-windows PACKAGE_NAME="windows-mtgo-collection-manager":
    mkdir -p mtgo-collection-manager
    cp mtgogui/target/x86_64-pc-windows-gnu/release/mtgogui.exe mtgo-collection-manager/mtgo-collection-manager
    zip -r {{PACKAGE_NAME}}.zip mtgo-collection-manager

archive-cross-compile-windows-xwin PACKAGE_NAME="windows-mtgo-collection-manager":
    mkdir -p mtgo-collection-manager
    cp mtgogui/target/x86_64-pc-windows-msvc/release/mtgogui.exe mtgo-collection-manager/mtgo-collection-manager
    zip -r {{PACKAGE_NAME}}.zip mtgo-collection-manager


clean: (cmd "cd mtgogui && cargo clean")