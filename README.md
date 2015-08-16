This is an experiment in writing a dart native extension in rust.

Steps:

```shell
pub get
cd lib/src/
cargo build --release
cp target/release/librust_extension.so ../
cd ../..
dart bin/callrust.dart
```
