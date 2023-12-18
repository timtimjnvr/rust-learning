- enums allows to group types and attach data to variants.
- each variant can have its own data.
# Option instead of null
- rust does not have ```null``` values. It uses ```Option``` instead (which is an enum, calleable in any scope without using ```Option::None``` just ```None``````)
```
// generic enum (can hold any type)
enum Option<T> {
    None,
    Some(T),
}```
- Why Option instead of null : because Option<T> and T (where T can be any type) are different types, the compiler won’t let us use an Option<T> if the value is not a valid value specified in the enum definition.