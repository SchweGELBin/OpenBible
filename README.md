# This repository has been archived


# OpenBible
This project aims to provide the BIBLE with minimal distractions as an Android App.

## How does it work?
It is written in [rust](https://www.rust-lang.org/) and uses [slint](https://slint.dev/) for its ui.
It downloads and works with json files, provided by [getbible](https://getbible.net/docs), which uses crosswire's [sword](https://www.crosswire.org/sword) module.

## Build
### Dependencies
- Install Android's SDK and NDK and set its environment variables.
- Install Java's JDK and set its environment variables.
- Install rust and cargo
- `cargo install cargo-apk`
### Build
- Build: `cargo apk build`
- Build and run: `cargo apk run`
- Build for release:
```
export CARGO_APK_RELEASE_KEYSTORE=path/to/your/keystorename.keystore
export CARGO_APK_RELEASE_KEYSTORE_PASSWORD=your_keystore_password

cargo apk build -r
```
