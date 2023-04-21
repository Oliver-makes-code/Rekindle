# Enumerations

Our enums are largely inspired by Rust's, with a few changes.

```
enum SomeEnum {
    SOME_VALUE
}
```

An enum instance can store values,
Allowing better control flow in code.

```
enum SomeEnum {
    SOME_VALUE(SomeClass)
}
```

To get a value of an enum,
you need to provide the stored value

```
let value = SomeEnum.SOME_VALUE(SomeClass())
```

An enum acts largely like a class,
allowing you to implement traits.

```
enum SomeEnum {
    SOME_VALUE(SomeClass)

    impl SomeTrait {
        //...
    }
}
```

For control flow, 
You check against different values in the enum

```
fun useEnum(enum: SomeEnum) {
    let SOME_VALUE(someClass) -> enum else {
        //...
    }

    if SOME_VALUE(someClass) -> enum {
        someClass.method()
    }

    when enum {
        SOME_VALUE(someClass) {
            someClass.method()
        }
        else {
            //...
        }
    }
}
```
