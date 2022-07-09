# Fantastic

![plugin_demo](./assets/ui.png)

Steam Deck fan controls.

This plugin is a bit special because it doesn't use the standard Python back-end.
Instead, it uses [USDPL](https://github.com/NGnius/usdpl-rs) written in Rust.
This means Fantastic needs a compiled binary to work correctly, which is not included in this repository.
If you run `./build.sh` from the `backend-rs` directory on a Linux system with `cargo` installed, the binary will be built for you.

This is generated from the template plugin for the [SteamOS Plugin Loader](https://github.com/SteamDeckHomebrew/decky-plugin-template).

Once Plugin Loader is installed, you can get this plugin from the [plugin store](https://plugins.deckbrew.xyz/) (that shopping basket at the top of the plugin menu).

## License

This is licensed under GNU GPLv3.
