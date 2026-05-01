use anyhow::Result;
use criterion::{Criterion, criterion_group, criterion_main};
use qslrs::quantum::pure_state::qc::QuantumCircuit;
use std::hint::black_box;

fn bell_state(qc: &mut QuantumCircuit) -> Result<()> {
    qc.hadamard(0).cnot(0, 1);
    qc.reset_state();
    Ok(())
}

fn bell_state_bench(c: &mut Criterion) {
    let mut qc = QuantumCircuit::builder().num_qubits(2).build().unwrap();

    c.bench_function("bell_state", |b| {
        b.iter(|| black_box(bell_state(&mut qc)));
    });
}

fn ghz_state_three_qubit(qc: &mut QuantumCircuit) -> Result<()> {
    qc.hadamard(0).cnot(0, 1).cnot(1, 2);
    qc.reset_state();
    Ok(())
}

fn ghz_state_three_qubit_bench(c: &mut Criterion) {
    let mut qc = QuantumCircuit::builder().num_qubits(3).build().unwrap();

    c.bench_function("ghz_state_three_qubit", |b| {
        b.iter(|| black_box(ghz_state_three_qubit(&mut qc)));
    });
}

fn ghz_state_five_qubit(qc: &mut QuantumCircuit) -> Result<()> {
    qc.hadamard(0).cnot(0, 1).cnot(1, 2).cnot(2, 3).cnot(3, 4);
    qc.reset_state();
    Ok(())
}

fn ghz_state_five_qubit_bench(c: &mut Criterion) {
    let mut qc = QuantumCircuit::builder().num_qubits(5).build().unwrap();

    c.bench_function("ghz_state_five_qubit", |b| {
        b.iter(|| black_box(ghz_state_five_qubit(&mut qc)));
    });
}

criterion_group!(
    benches,
    bell_state_bench,
    ghz_state_three_qubit_bench,
    ghz_state_five_qubit_bench
);
criterion_main!(benches);
