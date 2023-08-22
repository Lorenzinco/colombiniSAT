# Colombini-SAT
[![MacOS](https://github.com/Lorenzinco23/colombiniSAT/actions/workflows/macos.yml/badge.svg)](https://github.com/Lorenzinco23/colombiniSAT/actions/workflows/macos.yml)
[![Ubuntu](https://github.com/Lorenzinco23/colombiniSAT/actions/workflows/ubuntu.yml/badge.svg)](https://github.com/Lorenzinco23/colombiniSAT/actions/workflows/ubuntu.yml)
[![Windows](https://github.com/Lorenzinco23/colombiniSAT/actions/workflows/rust.yml/badge.svg)](https://github.com/Lorenzinco23/colombiniSAT/actions/workflows/rust.yml)

A simple 3-SAT solver written in Rust.

#### Links
[GitHub](https://github.com/Lorenzinco23/colombiniSAT "GitHub Repository page of the project.")


## How to use
Create an istance of the Solver struct passing it the path to the DMACS file you wish to solve.

```rust
let solver = Solver::new("path/to/file.cnf");
```

Then call the solve method on the solver istance.

```rust
let solution = solver.solve();
```

The solve method returns an Option enum, which can be either Some or None.
If the solve method returns Some, it means that the formula is satisfiable and the solution is contained in the Option.
If the solve method returns None, it means that the formula is unsatisfiable.

```rust
match solution{
        Some(solution) => println!("SAT: ({:?})", solution),
        None => println!("UNSAT")
    }
```

## Compiling
To compile the project you need to have Rust installed on your machine.
You can download Rust from [here](https://www.rust-lang.org/tools/install "Rust download page").

Once you have Rust installed, you can run the following command to see if everything is working:

```bash
cargo test
```

If everything is working correctly you can now run the following command to compile and run the project:

```bash
cargo run --release
```

## How it works
The solver uses the DPLL algorithm to solve the formula. (provisory)
### TODO: Full explaination of the algorithm
