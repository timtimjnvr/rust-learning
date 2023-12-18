# Enums
- enums allows to group types and attach data to variants.
- each variant can have its own data.

# Option instead of null
- rust does not have ```null``` values. It uses ```Option``` instead (which is an enum, callable in any scope without using ```Option::None``` just ```None```)
```
// generic enum (can hold any type)
enum Option<T> {
    None,
    Some(T),
}
```

- Why ```Option``` instead of ```null``` : because ```Option<T>``` and ```T``` (where ```T``` can be any type) are different types, the compiler won’t let us use an ```Option<T>``` if the value is not a valid value specified in the enum definition.
- This prevents the [billion-dollar mistake](https://www.infoq.com/presentations/Null-References-The-Billion-Dollar-Mistake-Tony-Hoare/) !

# Match & let if
- match list need to be exaustive !
- ```other``` matches all non enumerated preceeding cases and bind the value in the other variable.
- ```_```  matches all non enumerated preceeding cases but does not bind any value.
- ```let if``` can be useful when having one or two cases (more consise).