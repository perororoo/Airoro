# Airoro

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/perororoo/Airoro)

Discord bot built on `serenity-next` and uhhhhhhhh I forgot...

# Notes

- /media is not in the repo so any code block that uses the `crate::utils::msg_utils::builders::audio_attachment_builder()` won't work ;-;
- `openssl-sys` makes build really really really slow, if you want you can change to rustls backend. Switch out `native_tls_backend` feature to `rustls-backend` on serenity over at `Cargo.toml` (might need additional setup; you can also try using the `rustls_backend_no_provider` feature flag if it works) so it won't use openssl (need openssl to work on termux that's why I switched it out I think but u can try with rustls if it will work)

# Building

`cargo build` works but see the section below if you want to compile for Termux (from a Windows environment)

# How to Configure/Cross-Compile Airoro for Termux

## Requirements

- Android NDK (rn it is at build r-30)
- MSYS2 (MINGW64)
- Folders without spaces in name (IMPORTANT!!!!!)

## Summary of setup for cross-compilation

- Switch out serenity-next's "rustls-backend" feature to "native_tls_backend" (rustls-backend uses the package rustls-platform-verifier, which doesn't play nice with CLI-only applications on Android)
- Expose the "openssl" and "openssl-sys" crate packages onto Cargo.toml, and add "vendored" on the features field (this allows openssl to compile with GCC or Clang which works when using MSYS2 (MINGW64)
- Install "Make" and "Perl" MSYS2 (MINGW64)
- Installing Android NDK and setting up environment variables within MSYS2 (MINGW64)
- Download the OpenSSL-3.6.5 binary inside MSYS2 (MINGW64)
- Building a cross-compiled version of OpenSSL-3.6.5 on MSYS2 (MINGW64)
- Install MINGW64 toolchain
- Install Rust on MSYS2 (MINGW64)
- Compilation through MSYS2 (MINGW64), with Perl, Make, OpenSSL, and Android NDK as dependencies
- Install certificates on termux

## Installing "Make" and "Perl" within MSYS2 (MINGW64)

```bash
pacman -Syu
pacman -S make perl tar wget
```

## Installing Android NDK and setting up environment variables within MSYS2 (MINGW64)

- MAKE SURE THE ANDROID NDK IS INSIDE A FOLDER WITHOUT SPACES ON ITS NAME.
- After installing the Android NDK, setup these environment variables

```bash
export ANDROID_NDK_ROOT="<the directory to the Android NDK folder>"
export TOOLCHAIN="<the directory to the Android NDK folder>/toolchains/llvm/prebuilt/windows-x86_64"
export PATH="$TOOLCHAIN/bin:/c/Android/ndk/android-ndk-r26b/prebuilt/windows-x86_64/bin:$PATH"
```

- check if the paths can be seen running this command: which clang

## Downloading the OpenSSL-3.6.5 binary inside MSYS2 (MINGW64)

```bash
wget https://openssl.org tar -xf openssl-3.6.5 .tar.gz
```

- note: OpenSSL-3.6.5 is the latest version on termux package manager, idk if it makes a difference but install the same version as what's inside the termux package manager just to be safe ovo)b

## Building a cross-compiled version of OpenSSL-3.6.5 on MSYS2 (MINGW64)

- navigate to the openssl directory

```bash
cd openssl-3.6.5
```

- Make sure the environment variables are properly configured, then run this (This is configuring the MakeFile I think)

```
./Configure android-arm64 \
  -D__ANDROID_API__= <put 30, if NDK is r30 otherwise change number> \
  --prefix=/opt/openssl-android-arm64 \
  no-shared
```

- build it

```
make -j$(nproc)
make install_sw
```

## Installing MINGW64 toolchain (so we can install rust)

```bash
pacman -S --needed base-devel mingw-w64-x86_64-toolchain
```

## Installing Rust on MSYS2 (MINGW64)

```bash
pacman -S --needed mingw-w64-x86_64-rustup
```

## Airoro cross-compilation

```bash
- cargo ndk -t aarch64-linux-android build --release
```

## Installing certificates on termux

```bash
pkg update && pkg install ca-certificates
```

- set up environment variable pointing to the certificates

```bash
export SSL_CERT_FILE=/data/data/com.termux/files/usr/etc/ssl/certs/ca-certificates.crt
```

## Notes

- This is not a 0-100 documentation, this is assumes that rust is installed on termux, termux apk was downloaded from f-droid and not from the play store, initial setup is done on termux like the .env & start script, etc.

# Random Stuff

- This version of Airoro is a rewrite from JavaScript to Rust!!
