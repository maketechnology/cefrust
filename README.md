## Build

download CEF binaries from http://opensource.spotify.com/cefbuilds/index.html
unzip to ./cef_{linux¦win¦mac}
cargo build

Note: check target/debug for cef files (resources, dlls, locales)

### For release

cargo build --release

## Run sample app

cargo run --bin cefrust