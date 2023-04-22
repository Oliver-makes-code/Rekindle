# Visability

We go for an immutable-by-default and private-by-default structure,
making developers consciously decide what they what public and mutable.

You can make something mutable/public by adding it in the declaration

```
pub let mut something = 15 // Type inferred to int

class SomeClass(pub mut child SomeOtherClass)
```
