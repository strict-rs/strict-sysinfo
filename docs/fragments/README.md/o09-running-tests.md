### Running tests

Because we're looking at system information, some tests have a better chance to succeed when there is
a limited number of parallel running tests. To ensure they all pass, use:

```bash
cargo test -- --test-threads=1
```
