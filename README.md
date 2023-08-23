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

# How it works
My euristics is a lookahead algorithm that tries to find implications between literals.

### First, let's define some terms:
let $`n`$ be the number of literals in $`\phi`$

$`\forall i \in [1, n]`$ let $`\phi'_i \subseteq \phi`$ be the subformula containing only the clauses of $`\phi`$ that contain the literal $`x_i`$.



#### Example:
```math
\begin{align}

\phi\ = (x_1 \vee x_2 \vee x_3) \wedge (x_1 \vee x_3 \vee x_4) \wedge (x_1 \vee x_3 \vee x_5) \wedge (x_2 \vee x_3 \vee x_4)\\
\phi'_1 = (x_1 \vee x_2 \vee x3) \wedge (x_1 \vee x_3 \vee x_4) \wedge (x_1 \vee x_3 \vee x_5)\\
\phi'_2 = (x_1 \vee x_2 \vee x3) \wedge (x_2 \vee x_3 \vee x_4)\\
\phi'_3 = (x_1 \vee x_2 \vee x3) \wedge (x_1 \vee x_3 \vee x_4) \wedge (x_2 \vee x_3 \vee x_4)\\
\vdots
\end{align}
```


Now let's observe some properties of $`\phi'_i`$:
* ### If the literal $`x_i`$ satisfies its $`\phi'_i`$ with only one assignment $`A`$ either $`true\ or\ false`$, then that literal is implied to be $`A`$ in the current branch of $`\phi`$.

#### Example:
$`\phi'_1 = (x_1 \vee \neg x_5) \wedge (\neg x_1 \vee \neg x_3) \wedge (x_1 \vee x_3 \vee x_5) \wedge (x_1 \vee \neg x_3)`$ 


$`x_1`$ is satisfied with only one assignment $`true`$ in $`\phi'_1`$, so $`x_1`$ is implied to be $`true`$ in the current branch of $`\phi`$.

* ### While solving some $`\phi'_i`$, if some other literal $`x_j`$ takes the same assignment in all satisfing assignments of $`\phi'`$, then $`x_j`$ is also implied in the current branch of $`\phi`$.

#### Example:
$`\phi'_2 = (\neg x_2 \vee x_3) \wedge (x_2 \vee x_3) \wedge (x_2 \vee x_3 \vee x_4)`$


$`x_2`$ appears with both assignments in each satisfing assignment in $`\phi'_2`$, so we can say nothing about $`x_2`$,
however $`x_3`$ must be true to satisfy $`\phi'_2`$, so $`x_3`$ is implied to be T in the current branch of $`\phi`$.

* ### While solving some $`\phi'_i`$, let $`x_i`$ the implication found , if some other literal $`x_j`$ takes the same assignment in all satisfing assignments of Phi', then $`x_i\implies x_j`$ in the current branch of $`\phi`$.

#### Example:
$`\phi'_3 = (x_3) \wedge (x_1 \vee \neg x_3) \wedge (x_2 \vee x_3 \vee x_4)`$


$`x_3`$ is satisfied with only one assignment (T) in $`\phi'_3`$, so $`x_3`$ is implied to be T in the current branch of $`\phi`$.
At the same time $`x_3`$ = T means $`x_1`$ = T in all satisfing assignments of $`\phi'_3`$, so $`x_1`$ is implied to be T in the current branch of $`\phi`$.

