# Crates & packages
- crate : smallest amount of code that the compiler can consider at a time. It can have two forms :
    - a binary crate (program you can compile and run).
    - a library crate (don't have a main function and don't compile to an executable).
- crate root : it's the starting point of your package.
- package is a bundle of one or more crates (containes a ```Cargo.toml``` at the root directory of your package). Underlyingly, it contains the binary crate for the ```cargo``` command, and a library crate the package depends on. A package can contain as much binary crates as you want but only one library crate.
- The root crate of a package is ```src/main.rs```.
- if the package directory contains ```src/lib.rs```, the package contains a library crate with the same name as the package, and ```src/lib.rs``` is its crate root.
- package can have multiple binary crates by placing files in the ```src/bin``` directory.

# Modules
-  ```src/main.rs``` and ```src/lib.rs``` are called crate roots. They form a module named ```crate``` at the root of the crate's module tree.
- private/public: code within a module is private for its parent modules by default. Use ```pub``` to make a module public or to make items within a module public.
- making a module public does not make its content public.

# Paths

- paths can be :
    - absolute : starting from the crate root.
    - relative : starting from the current module (use ```self```, self, ```super```)

# Exporting 
- when importing with ```use```. The name available is private. You can associate it with ```pub``` to make it public.

# Separating modules into files
- ```mod <path>``` load a module into scope. Rust looks in a file with the same name as the module for the code that goes into that module.