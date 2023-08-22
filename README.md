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
My euristics is a lookahead algorithm that tries to find implications between literals.
First, let's define some terms:

let n be the number of literals in the formula

foreach i in i to n let Phi'_i be a subset of Phi where the clauses of phi' are the ones from Phi that contain the literal xi

Example:
Phi = (x1 v x2 v x3) ^ (x1 v x3 v x4) ^ (x1 v x3 v x5) ^ (x2 v x3 v x4)
Phi'_1 = (x1 v x2 v x3) ^ (x1 v x3 v x4) ^ (x1 v x3 v x5)
Phi'_2 = (x1 v x2 v x3) ^ (x2 v x3 v x4)
Phi'_3 = (x1 v x2 v x3) ^ (x1 v x3 v x4) ^ (x2 v x3 v x4)
...

Now let's observe some properties of Phi'_i:
If some literal x_i satisfies its Phi'_i with only one assignment A={T|F}, then that literal is implied to be A in the current branch of Phi.

Example:
Phi'_1 = (x1 v x2 v x3) ^ (x1 v x3 v x4) ^ (x1 v x3 v x5)
x1 is satisfied with only one assignment (T) in Phi'_1, so x1 is implied to be T in the current branch of Phi.

While resolving some Phi'_i, if some other literal x_j takes the same assignment A in all satisfing assignments of Phi', then x_j is also implied to be A in the current branch of Phi.

Example:
Phi'_2 = (¬x2 v x3) ^ (x2 v x3) ^ (x2 v x3 v x4)
x2 is satisfied with both assignments (T,F) in Phi'_2, so we can say nothing about x2 in Phi.
x3 is satisfied with only one assignment (T) in Phi'_2, so x3 is implied to be T in the current branch of Phi.

While resolving some Phi'_i let x_i the implication found , if some other literal x_j takes the same assignment in all satisfing assignments of Phi', then x_i => x_j in the current branch of Phi.

Example:
Phi'_3 = (x3) ^ (x1 v ¬x3) ^ (x2 v x3 v x4)
x3 is satisfied with only one assignment (T) in Phi'_3, so x3 is implied to be T in the current branch of Phi.
At the same time x3 = T means X1 = T in all satisfing assignments of Phi'_3, so x1 is implied to be T in the current branch of Phi.

