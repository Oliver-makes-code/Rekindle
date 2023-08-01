# Enumerations

Our enums are largely inspired by Rust's, with a few changes.

```rk
enum SomeEnum {
    SomeValue
}
```

An enum instance can store values,
Allowing better control flow in code.

```rk
enum SomeEnum {
    SomeValue(SomeClass)
}
```

To get a value of an enum,
you need to provide the stored value

```rk
let value = SomeEnum.SomeValue(SomeClass())
```

An enum acts largely like a class,
allowing you to implement traits.

```rk
enum SomeEnum {
    SomeValue(SomeClass)

    impl SomeTrait {
        //...
    }
}
```

For control flow,
You check against different values in the enum

For example, cast-else, which casts to a certain value,
and if it's impossible it runs a block, which must return

```rk
let SomeValue(someClass) -> someEnum else {
    return
}
```

There's also if-let, which runs a block if the cast can be done

```rk
if let SomeValue(someClass) -> someEnum {
    someClass.method()
}
```

And finally, there's usage in pattern matching, with the `when` statement

```rk
when someEnum {
    SomeValue(someClass) {
        someClass.method()
    }

    else {
        //...
    }
}
```
