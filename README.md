This is an experiment in writing a dart native extension in rust. There's no C involved.

You should have both the dart vm and rust and cargo installed.

Steps to build and run:

```shell
pub get
cd lib/src/
cargo build --release
cp target/release/librust_extension.so ../
cd ../..
dart bin/callrust.dart
```
