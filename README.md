# `win32-version-info`: Retrieve file version info (file description, file version, etc.) from Windows files

## Usage

```rust
use win32_version_info::VersionInfo;

let info = VersionInfo::from_file("path/to/your/file.exe")
    .expect("Failed to retrieve version information");

println!("File description: {}", info.file_description);
println!("File version: {}", info.file_version);
```

## Considerations

This crate is built upon the *official* Rust bindings of Win32 APIs provided
by the [`windows`](https://crates.io/crates/windows) crate [maintained by
Microsoft](https://github.com/microsoft/windows-rs).

This crate is highly inspired by the implementation of the
`System.Diagnostics.FileVersionInfo` class in the
[Microsoft .NET Reference Source](https://github.com/microsoft/referencesource/blob/master/System/services/monitoring/system/diagnosticts/FileVersionInfo.cs).

This crate should work in most cases but may have trouble with some rare
edge cases. If you encounter any issues, please report it on the GitHub
repository and I'm glad to help.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT).