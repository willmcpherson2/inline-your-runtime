https://willmcpherson2.com/2025/05/18/inline-your-runtime.html

# Inline Your Runtime

Enter the shell:

```sh
nix develop
```

Build the runtime system:

```sh
make
```

Run the JIT:

```sh
cargo run -- -e
```

Compile to a binary:

```sh
cargo run
cc main.o -o main
./main
```
