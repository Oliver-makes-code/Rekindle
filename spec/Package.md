# Packages

You can import a file from a path relative to the current file.

```
import "path/to/file.rk"
```

You can also qualify it under a namespace,
in case of name clashes.

```
import "path/to/file.rk" as SomeFile
```

For importing from libraries provided by a build tool,
you can use `from`.
This will use the path relative to the library root.

```
import "path/to/file.rk" from "some-lib" as SomeLib
```
