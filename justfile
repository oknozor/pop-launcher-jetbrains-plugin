build:
    cargo build --release
build_sway:
    cargo build --release --features sway

_install:
    mkdir -p ~/.local/share/pop-launcher/plugins/jetbrains
    install -Dm0755 target/release/pop-launcher-jetbrains-plugin ~/.local/share/pop-launcher/plugins/jetbrains/jetbrains
    install -Dm644 plugin.ron ~/.local/share/pop-launcher/plugins/jetbrains/plugin.ron

install:
    just build
    just _install

install-sway:
    just build_sway
    just _install