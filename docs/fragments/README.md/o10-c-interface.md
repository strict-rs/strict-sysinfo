### C interface

It's possible to use this crate directly from C. Take a look at the `Makefile` and at the
`examples/simple.c` file.

To build the C example, just run:

```bash
> make
> ./simple
# If needed:
> LD_LIBRARY_PATH=target/debug/ ./simple
```
