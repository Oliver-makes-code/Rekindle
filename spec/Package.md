# Packages

You can import a file from a path relative to the current file.

```rk
import "path/to/file.rk"
```

You can also qualify it under a namespace,
in case of name clashes.

```rk
import "path/to/file.rk" as SomeFile
```

For importing from libraries provided by a build tool,
you can use `from`.
This will use the path relative to the library root.

```rk
import "path/to/file.rk" from "some-lib" as SomeLib
```

You can also import libraries from a URL.

```rk
import "mod.rk" from "https://example.com/rekindle-lib"
```
