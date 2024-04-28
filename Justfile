
@_default:
    just --list

ci-install-cross-compile-windows-deps:
    rustup target add x86_64-pc-windows-gnu 
    sudo apt-get install gcc-mingw-w64-x86-64 ninja-build cmake

cross-compile-windows PROFILE="dev":
    cd mtgogui && cargo build --profile={{PROFILE}} --target=x86_64-pc-windows-gnu


archive-cross-compile-windows PACKAGE_NAME="windows-mtgo-collection-manager":
    mkdir -p mtgo-collection-manager
    cp mtgogui/target/x86_64-pc-windows-gnu/release/mtgogui.exe mtgo-collection-manager/mtgo-collection-manager
    zip -r {{PACKAGE_NAME}}.zip mtgo-collection-manager