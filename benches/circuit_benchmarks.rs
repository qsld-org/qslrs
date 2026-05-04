use anyhow::Result;
use criterion::{Criterion, criterion_group, criterion_main};
use qslrs::quantum::pure_state::qc::QuantumCircuit;
use qslrs_macros::generate_random_circuit;
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

fn random_circuit_three_qubit_50_depth(qc: &mut QuantumCircuit) {
    generate_random_circuit!(
        qc = qc,
        gates = [hadamard, s, cnot],
        qubits = 3,
        depth = 50,
        seed = 1337
    );

    qc.reset_state();
}

fn random_circuit_three_qubit_50_depth_bench(c: &mut Criterion) {
    let mut qc = QuantumCircuit::builder().num_qubits(3).build().unwrap();

    c.bench_function("random_circuit_three_qubit_50_depth", |b| {
        b.iter(|| black_box(random_circuit_three_qubit_50_depth(&mut qc)))
    });
}

fn random_circuit_seven_qubit_100_depth(qc: &mut QuantumCircuit) {
    generate_random_circuit!(
        qc = qc,
        gates = [hadamard, s, cnot],
        qubits = 7,
        depth = 100,
        seed = 1337
    );

    qc.reset_state();
}

fn random_circuit_seven_qubit_100_depth_bench(c: &mut Criterion) {
    let mut qc = QuantumCircuit::builder().num_qubits(7).build().unwrap();

    c.bench_function("random_circuit_seven_qubit_100_depth", |b| {
        b.iter(|| black_box(random_circuit_seven_qubit_100_depth(&mut qc)))
    });
}

fn random_circuit_fifteen_qubit_50_depth(qc: &mut QuantumCircuit) {
    generate_random_circuit!(
        qc = qc,
        gates = [hadamard, s, cnot],
        qubits = 15,
        depth = 50,
        seed = 1337
    );

    qc.reset_state();
}

fn random_circuit_fifteen_qubit_50_depth_bench(c: &mut Criterion) {
    let mut qc = QuantumCircuit::builder().num_qubits(15).build().unwrap();

    c.bench_function("random_circuit_fifteen_qubit_50_depth", |b| {
        b.iter(|| black_box(random_circuit_fifteen_qubit_50_depth(&mut qc)))
    });
}

criterion_group!(
    benches,
    bell_state_bench,
    ghz_state_three_qubit_bench,
    ghz_state_five_qubit_bench,
    random_circuit_three_qubit_50_depth_bench,
    random_circuit_seven_qubit_100_depth_bench,
    random_circuit_fifteen_qubit_50_depth_bench,
);
criterion_main!(benches);
