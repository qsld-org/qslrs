use anyhow::Result;
use criterion::{Criterion, criterion_group, criterion_main};
use qslrs::quantum::pure_state::{
    gate_noise::{
        BitFlipNoiseConfig, BitPhaseFlipNoiseConfig, DepolarizingNoiseConfig, PauliNoiseConfig,
        PhaseFlipNoiseConfig,
    },
    qc::QuantumCircuit,
};
use std::collections::HashMap;
use std::hint::black_box;

// Depolarizing noise benchmarks

fn depolarizing_noise_bell_state(qc: &mut QuantumCircuit) -> Result<()> {
    qc.hadamard(0).depolarizing_noise()?.cnot(0, 1);
    qc.reset_state();
    Ok(())
}

fn depolarizing_noise_bell_state_bench(c: &mut Criterion) {
    let d_config = DepolarizingNoiseConfig::new(Vec::from([0]), Vec::from([0.7]));
    let mut qc = QuantumCircuit::builder()
        .num_qubits(2)
        .depolarizing_noise_config(d_config)
        .build()
        .unwrap();

    c.bench_function("depolarinzing_noise_bell_state", |b| {
        b.iter(|| black_box(depolarizing_noise_bell_state(&mut qc)));
    });
}

fn depolarizing_noise_bell_state_multi_qubit(qc: &mut QuantumCircuit) -> Result<()> {
    qc.hadamard(0).depolarizing_noise()?.cnot(0, 1);
    qc.reset_state();
    Ok(())
}

fn depolarizing_noise_bell_state_multi_qubit_bench(c: &mut Criterion) {
    let d_config = DepolarizingNoiseConfig::new(Vec::from([0, 1]), Vec::from([0.7, 0.7]));
    let mut qc = QuantumCircuit::builder()
        .num_qubits(2)
        .depolarizing_noise_config(d_config)
        .build()
        .unwrap();

    c.bench_function("depolarinzing_noise_bell_state_multi_qubit", |b| {
        b.iter(|| black_box(depolarizing_noise_bell_state_multi_qubit(&mut qc)));
    });
}

fn depolarizing_noise_ghz_state(qc: &mut QuantumCircuit) -> Result<()> {
    qc.hadamard(0).cnot(0, 1).depolarizing_noise()?.cnot(1, 2);
    qc.reset_state();
    Ok(())
}

fn depolarizing_noise_ghz_state_bench(c: &mut Criterion) {
    let d_config = DepolarizingNoiseConfig::new(Vec::from([0]), Vec::from([0.7]));
    let mut qc = QuantumCircuit::builder()
        .num_qubits(3)
        .depolarizing_noise_config(d_config)
        .build()
        .unwrap();

    c.bench_function("depolarizing_noise_ghz_state", |b| {
        b.iter(|| black_box(depolarizing_noise_ghz_state(&mut qc)));
    });
}

fn depolarizing_noise_ghz_state_multi_qubit(qc: &mut QuantumCircuit) -> Result<()> {
    qc.hadamard(0).cnot(0, 1).depolarizing_noise()?.cnot(1, 2);
    qc.reset_state();
    Ok(())
}

fn depolarizing_noise_ghz_state_multi_qubit_bench(c: &mut Criterion) {
    let d_config = DepolarizingNoiseConfig::new(Vec::from([0, 1]), Vec::from([0.7, 0.7]));
    let mut qc = QuantumCircuit::builder()
        .num_qubits(3)
        .depolarizing_noise_config(d_config)
        .build()
        .unwrap();

    c.bench_function("depolarizing_noise_ghz_state_multi_qubit", |b| {
        b.iter(|| black_box(depolarizing_noise_ghz_state_multi_qubit(&mut qc)));
    });
}

// Pauli noise benchmarks

fn pauli_noise_bell_state(qc: &mut QuantumCircuit) -> Result<()> {
    qc.hadamard(0).pauli_noise()?.cnot(0, 1);
    qc.reset_state();
    Ok(())
}

fn pauli_noise_bell_state_bench(c: &mut Criterion) {
    let mut prob_map: HashMap<String, f64> = HashMap::new();
    prob_map.insert("I".to_string(), 1.0 / 4.0);
    prob_map.insert("X".to_string(), 1.0 / 4.0);
    prob_map.insert("Y".to_string(), 1.0 / 4.0);
    prob_map.insert("Z".to_string(), 1.0 / 4.0);
    let p_config = PauliNoiseConfig::new(Vec::from([0]), prob_map);
    let mut qc = QuantumCircuit::builder()
        .num_qubits(2)
        .pauli_noise_config(p_config)
        .build()
        .unwrap();

    c.bench_function("pauli_noise_bell_state", |b| {
        b.iter(|| black_box(pauli_noise_bell_state(&mut qc)));
    });
}

fn pauli_noise_bell_state_multi_qubit(qc: &mut QuantumCircuit) -> Result<()> {
    qc.hadamard(0).pauli_noise()?.cnot(0, 1);
    qc.reset_state();
    Ok(())
}

fn pauli_noise_bell_state_multi_qubit_bench(c: &mut Criterion) {
    let mut prob_map: HashMap<String, f64> = HashMap::new();
    prob_map.insert("I".to_string(), 1.0 / 4.0);
    prob_map.insert("X".to_string(), 1.0 / 4.0);
    prob_map.insert("Y".to_string(), 1.0 / 4.0);
    prob_map.insert("Z".to_string(), 1.0 / 4.0);
    let p_config = PauliNoiseConfig::new([0, 1].to_vec(), prob_map);
    let mut qc = QuantumCircuit::builder()
        .num_qubits(2)
        .pauli_noise_config(p_config)
        .build()
        .unwrap();

    c.bench_function("pauli_noise_bell_state_multi_qubit", |b| {
        b.iter(|| black_box(pauli_noise_bell_state_multi_qubit(&mut qc)));
    });
}

fn pauli_noise_ghz_state(qc: &mut QuantumCircuit) -> Result<()> {
    qc.hadamard(0).cnot(0, 1).pauli_noise()?.cnot(0, 2);
    qc.reset_state();
    Ok(())
}

fn pauli_noise_ghz_state_bench(c: &mut Criterion) {
    let mut prob_map: HashMap<String, f64> = HashMap::new();
    prob_map.insert("I".to_string(), 1.0 / 4.0);
    prob_map.insert("X".to_string(), 1.0 / 4.0);
    prob_map.insert("Y".to_string(), 1.0 / 4.0);
    prob_map.insert("Z".to_string(), 1.0 / 4.0);
    let p_config = PauliNoiseConfig::new([0].to_vec(), prob_map);
    let mut qc = QuantumCircuit::builder()
        .num_qubits(3)
        .pauli_noise_config(p_config)
        .build()
        .unwrap();

    c.bench_function("pauli_noise_ghz_state", |b| {
        b.iter(|| black_box(pauli_noise_ghz_state(&mut qc)))
    });
}

fn pauli_noise_ghz_state_multi_qubit(qc: &mut QuantumCircuit) -> Result<()> {
    qc.hadamard(0).cnot(0, 1).pauli_noise()?.cnot(0, 2);
    qc.reset_state();
    Ok(())
}

fn pauli_noise_ghz_state_multi_qubit_bench(c: &mut Criterion) {
    let mut prob_map: HashMap<String, f64> = HashMap::new();
    prob_map.insert("I".to_string(), 1.0 / 4.0);
    prob_map.insert("X".to_string(), 1.0 / 4.0);
    prob_map.insert("Y".to_string(), 1.0 / 4.0);
    prob_map.insert("Z".to_string(), 1.0 / 4.0);
    let p_config = PauliNoiseConfig::new([0, 1].to_vec(), prob_map);
    let mut qc = QuantumCircuit::builder()
        .num_qubits(3)
        .pauli_noise_config(p_config)
        .build()
        .unwrap();

    c.bench_function("pauli_noise_ghz_state_multi_qubit", |b| {
        b.iter(|| black_box(pauli_noise_ghz_state_multi_qubit(&mut qc)))
    });
}

// bit flip noise benchmarks

fn bit_flip_noise_bell_state(qc: &mut QuantumCircuit) {
    qc.hadamard(0).bit_flip_noise().cnot(0, 1);
    qc.reset_state();
}

fn bit_flip_noise_bell_state_bench(c: &mut Criterion) {
    let b_config = BitFlipNoiseConfig::new([0].to_vec(), [0.7].to_vec());
    let mut qc = QuantumCircuit::builder()
        .num_qubits(2)
        .bit_flip_noise_config(b_config)
        .build()
        .unwrap();

    c.bench_function("bit_flip_noise_bell_state", |b| {
        b.iter(|| black_box(bit_flip_noise_bell_state(&mut qc)))
    });
}

fn bit_flip_noise_bell_state_multi_qubit(qc: &mut QuantumCircuit) {
    qc.hadamard(0).bit_flip_noise().cnot(0, 1);
    qc.reset_state();
}

fn bit_flip_noise_bell_state_multi_qubit_bench(c: &mut Criterion) {
    let b_config = BitFlipNoiseConfig::new([0, 1].to_vec(), [0.7, 0.7].to_vec());
    let mut qc = QuantumCircuit::builder()
        .num_qubits(2)
        .bit_flip_noise_config(b_config)
        .build()
        .unwrap();

    c.bench_function("bit_flip_noise_bell_state_multi_qubit", |b| {
        b.iter(|| black_box(bit_flip_noise_bell_state_multi_qubit(&mut qc)))
    });
}

fn bit_flip_noise_ghz_state(qc: &mut QuantumCircuit) {
    qc.hadamard(0).cnot(0, 1).bit_flip_noise().cnot(1, 2);
    qc.reset_state();
}

fn bit_flip_noise_ghz_state_bench(c: &mut Criterion) {
    let b_config = BitFlipNoiseConfig::new([0].to_vec(), [0.7].to_vec());
    let mut qc = QuantumCircuit::builder()
        .num_qubits(3)
        .bit_flip_noise_config(b_config)
        .build()
        .unwrap();

    c.bench_function("bit_flip_noise_ghz_state", |b| {
        b.iter(|| black_box(bit_flip_noise_ghz_state(&mut qc)));
    });
}

fn bit_flip_noise_ghz_state_multi_qubit(qc: &mut QuantumCircuit) {
    qc.hadamard(0).cnot(0, 1).bit_flip_noise().cnot(1, 2);
    qc.reset_state();
}

fn bit_flip_noise_ghz_state_multi_qubit_bench(c: &mut Criterion) {
    let b_config = BitFlipNoiseConfig::new([0, 1].to_vec(), [0.7, 0.7].to_vec());
    let mut qc = QuantumCircuit::builder()
        .num_qubits(3)
        .bit_flip_noise_config(b_config)
        .build()
        .unwrap();

    c.bench_function("bit_flip_noise_ghz_state_multi_qubit", |b| {
        b.iter(|| black_box(bit_flip_noise_ghz_state_multi_qubit(&mut qc)))
    });
}

// Phase flip noise benchmarks

fn phase_flip_noise_bell_state(qc: &mut QuantumCircuit) {
    qc.hadamard(0).phase_flip_noise().cnot(0, 1);
    qc.reset_state();
}

fn phase_flip_noise_bell_state_bench(c: &mut Criterion) {
    let p_config = PhaseFlipNoiseConfig::new([0].to_vec(), [0.7].to_vec());
    let mut qc = QuantumCircuit::builder()
        .num_qubits(2)
        .phase_flip_noise_config(p_config)
        .build()
        .unwrap();

    c.bench_function("phase_flip_noise_bell_state", |b| {
        b.iter(|| black_box(phase_flip_noise_bell_state(&mut qc)))
    });
}

fn phase_flip_noise_bell_state_multi_qubit(qc: &mut QuantumCircuit) {
    qc.hadamard(0).phase_flip_noise().cnot(0, 1);
    qc.reset_state();
}

fn phase_flip_noise_bell_state_multi_qubit_bench(c: &mut Criterion) {
    let p_config = PhaseFlipNoiseConfig::new([0, 1].to_vec(), [0.7, 0.7].to_vec());
    let mut qc = QuantumCircuit::builder()
        .num_qubits(2)
        .phase_flip_noise_config(p_config)
        .build()
        .unwrap();

    c.bench_function("phase_flip_noise_bell_state_multi_qubit", |b| {
        b.iter(|| black_box(phase_flip_noise_bell_state_multi_qubit(&mut qc)))
    });
}

fn phase_flip_noise_ghz_state(qc: &mut QuantumCircuit) {
    qc.hadamard(0).cnot(0, 1).phase_flip_noise().cnot(1, 2);
    qc.reset_state();
}

fn phase_flip_noise_ghz_state_bench(c: &mut Criterion) {
    let p_config = PhaseFlipNoiseConfig::new([0].to_vec(), [0.7].to_vec());
    let mut qc = QuantumCircuit::builder()
        .num_qubits(3)
        .phase_flip_noise_config(p_config)
        .build()
        .unwrap();

    c.bench_function("phase_flip_noise_ghz_state", |b| {
        b.iter(|| black_box(phase_flip_noise_ghz_state(&mut qc)))
    });
}

fn phase_flip_noise_ghz_state_multi_qubit(qc: &mut QuantumCircuit) {
    qc.hadamard(0).cnot(0, 1).phase_flip_noise().cnot(1, 2);
    qc.reset_state();
}

fn phase_flip_noise_ghz_state_multi_qubit_bench(c: &mut Criterion) {
    let p_config = PhaseFlipNoiseConfig::new([0, 1].to_vec(), [0.7, 0.7].to_vec());
    let mut qc = QuantumCircuit::builder()
        .num_qubits(3)
        .phase_flip_noise_config(p_config)
        .build()
        .unwrap();

    c.bench_function("phase_flip_noise_ghz_state_multi_qubit", |b| {
        b.iter(|| black_box(phase_flip_noise_ghz_state_multi_qubit(&mut qc)))
    });
}

// bit phase flip noise benchmarks

fn bit_phase_flip_noise_bell_state(qc: &mut QuantumCircuit) {
    qc.hadamard(0).bit_phase_flip_noise().cnot(0, 1);
    qc.reset_state();
}

fn bit_phase_flip_noise_bell_state_bench(c: &mut Criterion) {
    let bp_config = BitPhaseFlipNoiseConfig::new([0].to_vec(), [0.7].to_vec());
    let mut qc = QuantumCircuit::builder()
        .num_qubits(2)
        .bit_phase_flip_noise_config(bp_config)
        .build()
        .unwrap();

    c.bench_function("bit_phase_flip_noise_bell_state", |b| {
        b.iter(|| black_box(bit_phase_flip_noise_bell_state(&mut qc)));
    });
}

fn bit_phase_flip_noise_bell_state_multi_qubit(qc: &mut QuantumCircuit) {
    qc.hadamard(0).bit_phase_flip_noise().cnot(0, 1);
    qc.reset_state();
}

fn bit_phase_flip_noise_bell_state_multi_qubit_bench(c: &mut Criterion) {
    let bp_config = BitPhaseFlipNoiseConfig::new([0, 1].to_vec(), [0.7, 0.7].to_vec());
    let mut qc = QuantumCircuit::builder()
        .num_qubits(2)
        .bit_phase_flip_noise_config(bp_config)
        .build()
        .unwrap();

    c.bench_function("bit_phase_flip_noise_bell_state_multi_qubit", |b| {
        b.iter(|| black_box(bit_phase_flip_noise_bell_state_multi_qubit(&mut qc)));
    });
}

fn bit_phase_flip_noise_ghz_state(qc: &mut QuantumCircuit) {
    qc.hadamard(0).cnot(0, 1).bit_phase_flip_noise().cnot(1, 2);
    qc.reset_state();
}

fn bit_phase_flip_noise_ghz_state_bench(c: &mut Criterion) {
    let bp_config = BitPhaseFlipNoiseConfig::new([0].to_vec(), [0.7].to_vec());
    let mut qc = QuantumCircuit::builder()
        .num_qubits(3)
        .bit_phase_flip_noise_config(bp_config)
        .build()
        .unwrap();

    c.bench_function("bit_phase_flip_noise_ghz_state", |b| {
        b.iter(|| black_box(bit_phase_flip_noise_ghz_state(&mut qc)))
    });
}

fn bit_phase_flip_noise_ghz_state_multi_qubit(qc: &mut QuantumCircuit) {
    qc.hadamard(0).cnot(0, 1).bit_phase_flip_noise().cnot(1, 2);
    qc.reset_state();
}

fn bit_phase_flip_noise_ghz_state_multi_qubit_bench(c: &mut Criterion) {
    let bp_config = BitPhaseFlipNoiseConfig::new([0, 1].to_vec(), [0.7, 0.7].to_vec());
    let mut qc = QuantumCircuit::builder()
        .num_qubits(3)
        .bit_phase_flip_noise_config(bp_config)
        .build()
        .unwrap();

    c.bench_function("bit_phase_flip_noise_ghz_state_multi_qubit", |b| {
        b.iter(|| black_box(bit_phase_flip_noise_ghz_state_multi_qubit(&mut qc)))
    });
}

criterion_group!(
    depolarizing_noise_benches,
    depolarizing_noise_bell_state_bench,
    depolarizing_noise_bell_state_multi_qubit_bench,
    depolarizing_noise_ghz_state_bench,
    depolarizing_noise_ghz_state_multi_qubit_bench,
);

criterion_group!(
    pauli_noise_benches,
    pauli_noise_bell_state_bench,
    pauli_noise_bell_state_multi_qubit_bench,
    pauli_noise_ghz_state_bench,
    pauli_noise_ghz_state_multi_qubit_bench,
);

criterion_group!(
    bit_flip_noise_benches,
    bit_flip_noise_bell_state_bench,
    bit_flip_noise_bell_state_multi_qubit_bench,
    bit_flip_noise_ghz_state_bench,
    bit_flip_noise_ghz_state_multi_qubit_bench,
);

criterion_group!(
    phase_flip_noise_benches,
    phase_flip_noise_bell_state_bench,
    phase_flip_noise_bell_state_multi_qubit_bench,
    phase_flip_noise_ghz_state_bench,
    phase_flip_noise_ghz_state_multi_qubit_bench,
);

criterion_group!(
    bit_phase_flip_noise_benches,
    bit_phase_flip_noise_bell_state_bench,
    bit_phase_flip_noise_bell_state_multi_qubit_bench,
    bit_phase_flip_noise_ghz_state_bench,
    bit_phase_flip_noise_ghz_state_multi_qubit_bench,
);

criterion_main!(
    depolarizing_noise_benches,
    pauli_noise_benches,
    bit_flip_noise_benches,
    phase_flip_noise_benches,
    bit_phase_flip_noise_benches,
);
