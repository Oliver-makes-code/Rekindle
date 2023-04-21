# Classes

We use the concept of classes to store and access data. 
A class at it's most basic defines a name

```
class Example()
```

A class can define methods and fields,
with a private-by-default policy.
```
class Example() {
    // Private!
    let someVar = 15; // Type inferred to be int

    // Public!
    pub fun someFunc() {
        print("owo")
    }
}
```

A class can implement a trait,
allowing functions requiring a specific trait to use it.
All fields and methods defined in a trait are public.

```
class Example() {
    impl ExampleTrait {
        fun log() {
            print("owo")
        }
    }
}
```

This might be a bit too many indents for some,
so we allow the use of `impl x for y` and `defer x for y`

We also allow developers to implement traits on foreign classes,
But you're only allowed access to public fields and methods.

```
impl ExampleTrait for Example {
    fun log() {
        print("owo")
    }
}

defer AnotherTrait for Example -> child
```

We do not have a standard inheritance structure,
instead deffering traits to children,
which are defined in the constructor.

If you defer a trait to a child,
you cannot override any methods in said trait.

```
class Example(pub child: ChildClass) {
    defer ExampleTrait -> child
}
```

You can also defer specific methods/fields in a trait to children

```
class Example(pub child: ChildClass) {
    impl ExampleTrait {
        defer let someVar -> child
        defer fun someFunc() -> child
    }
}
```

Fields and children are immutable by default,
making developers conscious of when they need to mutate values.

```
class Example(child: mut ChildClass) {
    let someVar: mut = 15
}
```
