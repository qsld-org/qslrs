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

fn depolarizing_noise_bell_state(
    qc: &mut QuantumCircuit,
    d_config: DepolarizingNoiseConfig,
) -> Result<()> {
    qc.hadamard(0).depolarizing_noise(d_config)?.cnot(0, 1);
    qc.reset_state();
    Ok(())
}

fn depolarizing_noise_bell_state_bench(c: &mut Criterion) {
    let mut qc = QuantumCircuit::builder().num_qubits(2).build().unwrap();
    let d_config = DepolarizingNoiseConfig::new(Vec::from([0]), Vec::from([0.7]));

    c.bench_function("depolarinzing_noise_bell_state", |b| {
        b.iter(|| {
            black_box(depolarizing_noise_bell_state(
                &mut qc,
                black_box(d_config.clone()),
            ))
        });
    });
}

fn depolarizing_noise_bell_state_multi_qubit(
    qc: &mut QuantumCircuit,
    d_config: DepolarizingNoiseConfig,
) -> Result<()> {
    qc.hadamard(0).depolarizing_noise(d_config)?.cnot(0, 1);
    qc.reset_state();
    Ok(())
}

fn depolarizing_noise_bell_state_multi_qubit_bench(c: &mut Criterion) {
    let mut qc = QuantumCircuit::builder().num_qubits(2).build().unwrap();
    let d_config = DepolarizingNoiseConfig::new(Vec::from([0, 1]), Vec::from([0.7, 0.7]));

    c.bench_function("depolarinzing_noise_bell_state_multi_qubit", |b| {
        b.iter(|| {
            black_box(depolarizing_noise_bell_state_multi_qubit(
                &mut qc,
                black_box(d_config.clone()),
            ))
        });
    });
}

fn depolarizing_noise_ghz_state(
    qc: &mut QuantumCircuit,
    d_config: DepolarizingNoiseConfig,
) -> Result<()> {
    qc.hadamard(0)
        .cnot(0, 1)
        .depolarizing_noise(d_config)?
        .cnot(1, 2);
    qc.reset_state();
    Ok(())
}

fn depolarizing_noise_ghz_state_bench(c: &mut Criterion) {
    let mut qc = QuantumCircuit::builder().num_qubits(3).build().unwrap();
    let d_config = DepolarizingNoiseConfig::new(Vec::from([0]), Vec::from([0.7]));

    c.bench_function("depolarizing_noise_ghz_state", |b| {
        b.iter(|| {
            black_box(depolarizing_noise_ghz_state(
                &mut qc,
                black_box(d_config.clone()),
            ))
        });
    });
}

fn depolarizing_noise_ghz_state_multi_qubit(
    qc: &mut QuantumCircuit,
    d_config: DepolarizingNoiseConfig,
) -> Result<()> {
    qc.hadamard(0)
        .cnot(0, 1)
        .depolarizing_noise(d_config)?
        .cnot(1, 2);
    qc.reset_state();
    Ok(())
}

fn depolarizing_noise_ghz_state_multi_qubit_bench(c: &mut Criterion) {
    let mut qc = QuantumCircuit::builder().num_qubits(3).build().unwrap();
    let d_config = DepolarizingNoiseConfig::new(Vec::from([0, 1]), Vec::from([0.7, 0.7]));

    c.bench_function("depolarizing_noise_ghz_state_multi_qubit", |b| {
        b.iter(|| {
            black_box(depolarizing_noise_ghz_state_multi_qubit(
                &mut qc,
                black_box(d_config.clone()),
            ))
        });
    });
}

// Pauli noise benchmarks

fn pauli_noise_bell_state(qc: &mut QuantumCircuit, p_config: PauliNoiseConfig) -> Result<()> {
    qc.hadamard(0).pauli_noise(p_config)?.cnot(0, 1);
    qc.reset_state();
    Ok(())
}

fn pauli_noise_bell_state_bench(c: &mut Criterion) {
    let mut qc = QuantumCircuit::builder().num_qubits(2).build().unwrap();
    let mut prob_map: HashMap<String, f64> = HashMap::new();
    prob_map.insert("I".to_string(), 1.0 / 4.0);
    prob_map.insert("X".to_string(), 1.0 / 4.0);
    prob_map.insert("Y".to_string(), 1.0 / 4.0);
    prob_map.insert("Z".to_string(), 1.0 / 4.0);
    let p_config = PauliNoiseConfig::new(Vec::from([0]), prob_map);

    c.bench_function("pauli_noise_bell_state", |b| {
        b.iter(|| black_box(pauli_noise_bell_state(&mut qc, black_box(p_config.clone()))));
    });
}

fn pauli_noise_bell_state_multi_qubit(
    qc: &mut QuantumCircuit,
    p_config: PauliNoiseConfig,
) -> Result<()> {
    qc.hadamard(0).pauli_noise(p_config)?.cnot(0, 1);
    qc.reset_state();
    Ok(())
}

fn pauli_noise_bell_state_multi_qubit_bench(c: &mut Criterion) {
    let mut qc = QuantumCircuit::builder().num_qubits(2).build().unwrap();
    let mut prob_map: HashMap<String, f64> = HashMap::new();
    prob_map.insert("I".to_string(), 1.0 / 4.0);
    prob_map.insert("X".to_string(), 1.0 / 4.0);
    prob_map.insert("Y".to_string(), 1.0 / 4.0);
    prob_map.insert("Z".to_string(), 1.0 / 4.0);
    let p_config = PauliNoiseConfig::new([0, 1].to_vec(), prob_map);

    c.bench_function("pauli_noise_bell_state_multi_qubit", |b| {
        b.iter(|| {
            black_box(pauli_noise_bell_state_multi_qubit(
                &mut qc,
                p_config.clone(),
            ))
        });
    });
}

fn pauli_noise_ghz_state(qc: &mut QuantumCircuit, p_config: PauliNoiseConfig) -> Result<()> {
    qc.hadamard(0).cnot(0, 1).pauli_noise(p_config)?.cnot(0, 2);
    qc.reset_state();
    Ok(())
}

fn pauli_noise_ghz_state_bench(c: &mut Criterion) {
    let mut qc = QuantumCircuit::builder().num_qubits(3).build().unwrap();
    let mut prob_map: HashMap<String, f64> = HashMap::new();
    prob_map.insert("I".to_string(), 1.0 / 4.0);
    prob_map.insert("X".to_string(), 1.0 / 4.0);
    prob_map.insert("Y".to_string(), 1.0 / 4.0);
    prob_map.insert("Z".to_string(), 1.0 / 4.0);
    let p_config = PauliNoiseConfig::new([0].to_vec(), prob_map);

    c.bench_function("pauli_noise_ghz_state", |b| {
        b.iter(|| black_box(pauli_noise_ghz_state(&mut qc, black_box(p_config.clone()))))
    });
}

fn pauli_noise_ghz_state_multi_qubit(
    qc: &mut QuantumCircuit,
    p_config: PauliNoiseConfig,
) -> Result<()> {
    qc.hadamard(0).cnot(0, 1).pauli_noise(p_config)?.cnot(0, 2);
    qc.reset_state();
    Ok(())
}

fn pauli_noise_ghz_state_multi_qubit_bench(c: &mut Criterion) {
    let mut qc = QuantumCircuit::builder().num_qubits(3).build().unwrap();
    let mut prob_map: HashMap<String, f64> = HashMap::new();
    prob_map.insert("I".to_string(), 1.0 / 4.0);
    prob_map.insert("X".to_string(), 1.0 / 4.0);
    prob_map.insert("Y".to_string(), 1.0 / 4.0);
    prob_map.insert("Z".to_string(), 1.0 / 4.0);
    let p_config = PauliNoiseConfig::new([0, 1].to_vec(), prob_map);

    c.bench_function("pauli_noise_ghz_state_multi_qubit", |b| {
        b.iter(|| {
            black_box(pauli_noise_ghz_state_multi_qubit(
                &mut qc,
                black_box(p_config.clone()),
            ))
        })
    });
}

// bit flip noise benchmarks

fn bit_flip_noise_bell_state(qc: &mut QuantumCircuit, b_config: BitFlipNoiseConfig) {
    qc.hadamard(0).bit_flip_noise(b_config).cnot(0, 1);
    qc.reset_state();
}

fn bit_flip_noise_bell_state_bench(c: &mut Criterion) {
    let mut qc = QuantumCircuit::builder().num_qubits(2).build().unwrap();
    let b_config = BitFlipNoiseConfig::new([0].to_vec(), [0.7].to_vec());

    c.bench_function("bit_flip_noise_bell_state", |b| {
        b.iter(|| {
            black_box(bit_flip_noise_bell_state(
                &mut qc,
                black_box(b_config.clone()),
            ))
        })
    });
}

fn bit_flip_noise_bell_state_multi_qubit(qc: &mut QuantumCircuit, b_config: BitFlipNoiseConfig) {
    qc.hadamard(0).bit_flip_noise(b_config).cnot(0, 1);
    qc.reset_state();
}

fn bit_flip_noise_bell_state_multi_qubit_bench(c: &mut Criterion) {
    let mut qc = QuantumCircuit::builder().num_qubits(2).build().unwrap();
    let b_config = BitFlipNoiseConfig::new([0, 1].to_vec(), [0.7, 0.7].to_vec());

    c.bench_function("bit_flip_noise_bell_state_multi_qubit", |b| {
        b.iter(|| {
            black_box(bit_flip_noise_bell_state_multi_qubit(
                &mut qc,
                black_box(b_config.clone()),
            ))
        })
    });
}

fn bit_flip_noise_ghz_state(qc: &mut QuantumCircuit, b_config: BitFlipNoiseConfig) {
    qc.hadamard(0)
        .cnot(0, 1)
        .bit_flip_noise(b_config)
        .cnot(1, 2);
    qc.reset_state();
}

fn bit_flip_noise_ghz_state_bench(c: &mut Criterion) {
    let mut qc = QuantumCircuit::builder().num_qubits(3).build().unwrap();
    let b_config = BitFlipNoiseConfig::new([0].to_vec(), [0.7].to_vec());

    c.bench_function("bit_flip_noise_ghz_state", |b| {
        b.iter(|| {
            black_box(bit_flip_noise_ghz_state(
                &mut qc,
                black_box(b_config.clone()),
            ))
        });
    });
}

fn bit_flip_noise_ghz_state_multi_qubit(qc: &mut QuantumCircuit, b_config: BitFlipNoiseConfig) {
    qc.hadamard(0)
        .cnot(0, 1)
        .bit_flip_noise(b_config)
        .cnot(1, 2);
    qc.reset_state();
}

fn bit_flip_noise_ghz_state_multi_qubit_bench(c: &mut Criterion) {
    let mut qc = QuantumCircuit::builder().num_qubits(3).build().unwrap();
    let b_config = BitFlipNoiseConfig::new([0, 1].to_vec(), [0.7, 0.7].to_vec());

    c.bench_function("bit_flip_noise_ghz_state_multi_qubit", |b| {
        b.iter(|| {
            black_box(bit_flip_noise_ghz_state_multi_qubit(
                &mut qc,
                black_box(b_config.clone()),
            ))
        })
    });
}

// Phase flip noise benchmarks

fn phase_flip_noise_bell_state(qc: &mut QuantumCircuit, p_config: PhaseFlipNoiseConfig) {
    qc.hadamard(0).phase_flip_noise(p_config).cnot(0, 1);
    qc.reset_state();
}

fn phase_flip_noise_bell_state_bench(c: &mut Criterion) {
    let mut qc = QuantumCircuit::builder().num_qubits(2).build().unwrap();
    let p_config = PhaseFlipNoiseConfig::new([0].to_vec(), [0.7].to_vec());

    c.bench_function("phase_flip_noise_bell_state", |b| {
        b.iter(|| {
            black_box(phase_flip_noise_bell_state(
                &mut qc,
                black_box(p_config.clone()),
            ))
        })
    });
}

fn phase_flip_noise_bell_state_multi_qubit(
    qc: &mut QuantumCircuit,
    p_config: PhaseFlipNoiseConfig,
) {
    qc.hadamard(0).phase_flip_noise(p_config).cnot(0, 1);
    qc.reset_state();
}

fn phase_flip_noise_bell_state_multi_qubit_bench(c: &mut Criterion) {
    let mut qc = QuantumCircuit::builder().num_qubits(2).build().unwrap();
    let p_config = PhaseFlipNoiseConfig::new([0, 1].to_vec(), [0.7, 0.7].to_vec());

    c.bench_function("phase_flip_noise_bell_state_multi_qubit", |b| {
        b.iter(|| {
            black_box(phase_flip_noise_bell_state_multi_qubit(
                &mut qc,
                black_box(p_config.clone()),
            ))
        })
    });
}

fn phase_flip_noise_ghz_state(qc: &mut QuantumCircuit, p_config: PhaseFlipNoiseConfig) {
    qc.hadamard(0)
        .cnot(0, 1)
        .phase_flip_noise(p_config)
        .cnot(1, 2);
    qc.reset_state();
}

fn phase_flip_noise_ghz_state_bench(c: &mut Criterion) {
    let mut qc = QuantumCircuit::builder().num_qubits(3).build().unwrap();
    let p_config = PhaseFlipNoiseConfig::new([0].to_vec(), [0.7].to_vec());

    c.bench_function("phase_flip_noise_ghz_state", |b| {
        b.iter(|| {
            black_box(phase_flip_noise_ghz_state(
                &mut qc,
                black_box(p_config.clone()),
            ))
        })
    });
}

fn phase_flip_noise_ghz_state_multi_qubit(qc: &mut QuantumCircuit, p_config: PhaseFlipNoiseConfig) {
    qc.hadamard(0)
        .cnot(0, 1)
        .phase_flip_noise(p_config)
        .cnot(1, 2);
    qc.reset_state();
}

fn phase_flip_noise_ghz_state_multi_qubit_bench(c: &mut Criterion) {
    let mut qc = QuantumCircuit::builder().num_qubits(3).build().unwrap();
    let p_config = PhaseFlipNoiseConfig::new([0, 1].to_vec(), [0.7, 0.7].to_vec());

    c.bench_function("phase_flip_noise_ghz_state_multi_qubit", |b| {
        b.iter(|| {
            black_box(phase_flip_noise_ghz_state_multi_qubit(
                &mut qc,
                black_box(p_config.clone()),
            ))
        })
    });
}

// bit phase flip noise benchmarks

fn bit_phase_flip_noise_bell_state(qc: &mut QuantumCircuit, bp_config: BitPhaseFlipNoiseConfig) {
    qc.hadamard(0).bit_phase_flip_noise(bp_config).cnot(0, 1);
    qc.reset_state();
}

fn bit_phase_flip_noise_bell_state_bench(c: &mut Criterion) {
    let mut qc = QuantumCircuit::builder().num_qubits(2).build().unwrap();
    let bp_config = BitPhaseFlipNoiseConfig::new([0].to_vec(), [0.7].to_vec());

    c.bench_function("bit_phase_flip_noise_bell_state", |b| {
        b.iter(|| {
            black_box(bit_phase_flip_noise_bell_state(
                &mut qc,
                black_box(bp_config.clone()),
            ))
        });
    });
}

fn bit_phase_flip_noise_bell_state_multi_qubit(
    qc: &mut QuantumCircuit,
    bp_config: BitPhaseFlipNoiseConfig,
) {
    qc.hadamard(0).bit_phase_flip_noise(bp_config).cnot(0, 1);
    qc.reset_state();
}

fn bit_phase_flip_noise_bell_state_multi_qubit_bench(c: &mut Criterion) {
    let mut qc = QuantumCircuit::builder().num_qubits(2).build().unwrap();
    let bp_config = BitPhaseFlipNoiseConfig::new([0, 1].to_vec(), [0.7, 0.7].to_vec());

    c.bench_function("bit_phase_flip_noise_bell_state_multi_qubit", |b| {
        b.iter(|| {
            black_box(bit_phase_flip_noise_bell_state_multi_qubit(
                &mut qc,
                black_box(bp_config.clone()),
            ))
        });
    });
}

fn bit_phase_flip_noise_ghz_state(qc: &mut QuantumCircuit, bp_config: BitPhaseFlipNoiseConfig) {
    qc.hadamard(0)
        .cnot(0, 1)
        .bit_phase_flip_noise(bp_config)
        .cnot(1, 2);
    qc.reset_state();
}

fn bit_phase_flip_noise_ghz_state_bench(c: &mut Criterion) {
    let mut qc = QuantumCircuit::builder().num_qubits(3).build().unwrap();
    let bp_config = BitPhaseFlipNoiseConfig::new([0].to_vec(), [0.7].to_vec());

    c.bench_function("bit_phase_flip_noise_ghz_state", |b| {
        b.iter(|| {
            black_box(bit_phase_flip_noise_ghz_state(
                &mut qc,
                black_box(bp_config.clone()),
            ))
        })
    });
}

fn bit_phase_flip_noise_ghz_state_multi_qubit(
    qc: &mut QuantumCircuit,
    bp_config: BitPhaseFlipNoiseConfig,
) {
    qc.hadamard(0)
        .cnot(0, 1)
        .bit_phase_flip_noise(bp_config)
        .cnot(1, 2);
    qc.reset_state();
}

fn bit_phase_flip_noise_ghz_state_multi_qubit_bench(c: &mut Criterion) {
    let mut qc = QuantumCircuit::builder().num_qubits(3).build().unwrap();
    let bp_config = BitPhaseFlipNoiseConfig::new([0, 1].to_vec(), [0.7, 0.7].to_vec());

    c.bench_function("bit_phase_flip_noise_ghz_state_multi_qubit", |b| {
        b.iter(|| {
            black_box(bit_phase_flip_noise_ghz_state_multi_qubit(
                &mut qc,
                black_box(bp_config.clone()),
            ))
        })
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
