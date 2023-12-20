- `Cargo.lock` contains versions used to build a project for the first time. The second build, cargo use the version specified in the `Cargo.lock` file.

- to update a crate : use ```cargo update```. It will ignore the `Cargo.lock` file and figure out all the latest version compatible with your ```Cargo.toml``` specification.

For example :
If you have ```rand = "0.8.5"``` in your ```Cargo.toml```
```cargo update``` will try to find versions greater than 0.8.5 and less than 0.9.0
to use 0.9.x you need to manually update the ```Cargo.toml``` file.

- ```cargo doc --open``` will build and open the documentation for your project in your browser.