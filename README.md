# Airoro

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/perororoo/Airoro)

Discord bot built on `serenity-next` and uhhhhhhhh I forgot...

## Notes

- /media is not in the repo so any code block that uses the `crate::utils::msg_utils::builders::audio_attachment_builder()` won't work ;-;
- `openssl-sys` makes build really really really slow, if you want you can change to rustls backend. Switch out `native_tls_backend` feature to `rustls-backend` on serenity over at `Cargo.toml` (might need additional setup; you can also try using the `rustls_backend_no_provider` feature flag if it works) so it won't use openssl (need openssl to work on termux that's why I switched it out I think but u can try with rustls if it will work)

## Building

### Cross-compiling for Termux

See [termux-cross-compile.md](./termux-cross-compile.md) if you want to compile for Termux (from a Windows environment).

### Building within Termux

This takes a lot less time and effort than cross-compiling, but does require ~2 GB of storage on your Android device. Make sure you have Cargo installed by running `pkg install rust`, then simply run `cargo build`.

## Random Stuff

- This version of Airoro is a rewrite from JavaScript to Rust!!
