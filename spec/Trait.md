# Traits

We have traits, which are an abstract set of methods and fields

```
trait SomeTrait {
    let someField: int
    fun someFunc()
}
```

Classes and enums can implement traits,
and external code can implement traits on foreign classes.
and foreign traits on their classes.

```
impl SomeTrait for SomeClass {
    let someField = 15
    fun someFunc() {
        someField += 1
    }
}
```
