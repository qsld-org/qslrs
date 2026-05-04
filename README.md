<div align="center">

# QSLRS

## Quantum Simulation Library in Rust

</div>

### Introduction

QSLRS is an alternative to the already existing QSLD or Quantum Simulation Library in D. However, this one is written in rust and will eventually have extra features.

### Dependencies for Building

- `cargo`

### Building 

```console
$ cargo build --release
```

### Benchmarking

Unlike QSLD, QSLRS is performance focused and therefore I have made some benchmarks (many more to come) in order to test different parts or the system and various different circuits with many gates and qubits.
If you would like to get an idea for the performance of the simulator overall you can do the following:

```console
$ cargo bench
```

**NOTE** not all benchmarks are equal, most as of right now, test a very small system with few qubits and basic gates but I also plan to expand my use of the `generate_random_circuit!` procedural macro in order to truly
stress test the simulator. I try to follow a strict naming convention with the benchmarks in order to explicitly give an idea of what they benchmark exactly so please pay attention to the name as well. The small benchmarks 
are more for just general performance for small workloads but the stress tests are the important benchmarks which you should pay attention to and compare with other simulators before deciding whether to use this one or 
another one. With this type of simulation, **performance is everything**.
