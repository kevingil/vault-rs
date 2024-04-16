## Password vault in Rust with SQLite

Rust CLI to save passwords or secrets in a sqlite vault

### Run
```
cargo run
```

#### Dependencies

1. Rusqlite
2. Serde/serde_json


Menu Choices:

1. New service with username and password or secret.
2. List saved services from vault.
3. Search for service.
4. Quit.



Common issues:

In Windows, you need Visual Studio 2017 or later, or Build Tools for Visual Studio installed for `serde` macros.

```
note: the msvc targets depend on the msvc linker but `link.exe` was not found

note: please ensure that Visual Studio 2017 or later, or Build Tools for Visual Studio were installed with the Visual C++ option.

note: VS Code is a different product, and is not sufficient.
```
