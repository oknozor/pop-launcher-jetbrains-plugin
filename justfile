all:
    cargo build --release
    mkdir -p ~/.local/share/pop-launcher/plugins/jetbrains
    install -Dm0755 target/release/pop-launcher-intellij ~/.local/share/pop-launcher/plugins/jetbrains/jetbrains
    install -Dm644 plugin.ron ~/.local/share/pop-launcher/plugins/jetbrains/plugin.ron