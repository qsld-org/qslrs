use std::collections::HashMap;

use anyhow::anyhow;
use approx::relative_eq;
use rand::{RngExt, rngs::ThreadRng};

use crate::quantum::pure_state::qc::QuantumCircuit;

#[derive(Clone)]
pub struct DepolarizingNoiseConfig {
    pub qubit_idxs: Vec<u32>,
    pub probabilities: Vec<f64>,
}

impl DepolarizingNoiseConfig {
    /// Refer to the `QuantumCircuit` object documentation for how to use
    pub fn new(qubit_idxs: Vec<u32>, probabilities: Vec<f64>) -> Self {
        assert!(
            qubit_idxs.len() == probabilities.len(),
            "The length of the two arrays passed to the noise config must be the same size"
        );
        DepolarizingNoiseConfig {
            qubit_idxs,
            probabilities,
        }
    }
}

#[derive(Clone)]
pub struct PauliNoiseConfig {
    pub qubit_idxs: Vec<u32>,
    pub probability_map: HashMap<String, f64>,
}

impl PauliNoiseConfig {
    /// Refer to the `QuantumCircuit` object documentation for how to use
    pub fn new(qubit_idxs: Vec<u32>, probability_map: HashMap<String, f64>) -> Self {
        PauliNoiseConfig {
            qubit_idxs,
            probability_map,
        }
    }
}

#[derive(Clone)]
pub struct BitFlipNoiseConfig {
    pub qubit_idxs: Vec<u32>,
    pub probabilities: Vec<f64>,
}

impl BitFlipNoiseConfig {
    /// Refer to the `QuantumCircuit` object documentation for how to use
    pub fn new(qubit_idxs: Vec<u32>, probabilities: Vec<f64>) -> Self {
        assert!(
            qubit_idxs.len() == probabilities.len(),
            "The length of the two arrays passed to the noise config must be the same size"
        );
        BitFlipNoiseConfig {
            qubit_idxs,
            probabilities,
        }
    }
}

#[derive(Clone)]
pub struct PhaseFlipNoiseConfig {
    pub qubit_idxs: Vec<u32>,
    pub probabilities: Vec<f64>,
}

impl PhaseFlipNoiseConfig {
    /// Refer to the `QuantumCircuit` object documentation for how to use
    pub fn new(qubit_idxs: Vec<u32>, probabilities: Vec<f64>) -> Self {
        assert!(
            qubit_idxs.len() == probabilities.len(),
            "The length of the two arrays passed to the noise config must be the same size"
        );
        PhaseFlipNoiseConfig {
            qubit_idxs,
            probabilities,
        }
    }
}

#[derive(Clone)]
pub struct BitPhaseFlipNoiseConfig {
    pub qubit_idxs: Vec<u32>,
    pub probabilities: Vec<f64>,
}

impl BitPhaseFlipNoiseConfig {
    /// Refer to the `QuantumCircuit` object documentation for how to use
    pub fn new(qubit_idxs: Vec<u32>, probabilities: Vec<f64>) -> Self {
        assert!(
            qubit_idxs.len() == probabilities.len(),
            "The length of the two arrays passed to the noise config must be the same size"
        );
        BitPhaseFlipNoiseConfig {
            qubit_idxs,
            probabilities,
        }
    }
}

pub struct GateNoise {
    pub rng: ThreadRng,
}

impl GateNoise {
    pub(crate) fn new() -> Self {
        GateNoise { rng: rand::rng() }
    }

    pub(crate) fn depolarizing_noise(
        &mut self,
        qc: &mut QuantumCircuit,
        config: &DepolarizingNoiseConfig,
    ) -> Result<(), anyhow::Error> {
        let mut gates: HashMap<String, f64> = HashMap::new();
        gates.insert("X".to_string(), 1.0 / 3.0);
        gates.insert("Y".to_string(), 1.0 / 3.0);
        gates.insert("Z".to_string(), 1.0 / 3.0);

        for i in 0..config.qubit_idxs.len() {
            let r: f64 = self.rng.random();
            if r < config.probabilities[i] {
                let mut sum = 0.0;
                let r2: f64 = self.rng.random();

                for (_, (gate, prob)) in gates.iter().enumerate() {
                    sum += prob;
                    if r2 < sum {
                        match gate.as_str() {
                            "X" => {
                                qc.pauli_x(config.qubit_idxs[i]);
                                break;
                            }
                            "Y" => {
                                qc.pauli_y(config.qubit_idxs[i]);
                                break;
                            }
                            "Z" => {
                                qc.pauli_z(config.qubit_idxs[i]);
                                break;
                            }
                            _ => return Err(anyhow!("Invalid gate chosen for depolarizing noise")),
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub(crate) fn pauli_noise(
        &mut self,
        qc: &mut QuantumCircuit,
        config: &PauliNoiseConfig,
    ) -> Result<(), anyhow::Error> {
        assert!(
            config.probability_map.len() == 4,
            "The length of the probability array must be 4",
        );

        let value_sum: f64 = config.probability_map.values().sum();

        assert!(
            relative_eq!(value_sum, 1.0),
            "The total probability is not at least close to 1"
        );

        for key in config.probability_map.keys() {
            assert!(
                *key == key.to_uppercase(),
                "The key values are not capitalized, they must be",
            );
        }

        for i in 0..config.qubit_idxs.len() {
            let r: f64 = self.rng.random();
            let mut sum = 0.0;
            for (_, (op, prob)) in config.probability_map.iter().enumerate() {
                sum += prob;
                if r < sum {
                    match op.as_str() {
                        "I" => break,
                        "X" => {
                            qc.pauli_x(config.qubit_idxs[i]);
                            break;
                        }
                        "Y" => {
                            qc.pauli_y(config.qubit_idxs[i]);
                            break;
                        }
                        "Z" => {
                            qc.pauli_z(config.qubit_idxs[i]);
                            break;
                        }
                        _ => return Err(anyhow!("Unrecognized gate given to pauli noise")),
                    }
                }
            }
        }
        Ok(())
    }

    pub(crate) fn bit_flip_noise(&mut self, qc: &mut QuantumCircuit, config: &BitFlipNoiseConfig) {
        for i in 0..config.qubit_idxs.len() {
            let r: f64 = self.rng.random();

            if r < config.probabilities[i] {
                qc.pauli_x(config.qubit_idxs[i]);
            }
        }
    }

    pub(crate) fn phase_flip_noise(
        &mut self,
        qc: &mut QuantumCircuit,
        config: &PhaseFlipNoiseConfig,
    ) {
        for i in 0..config.qubit_idxs.len() {
            let r: f64 = self.rng.random();

            if r < config.probabilities[i] {
                qc.pauli_z(config.qubit_idxs[i]);
            }
        }
    }

    pub(crate) fn bit_phase_flip_noise(
        &mut self,
        qc: &mut QuantumCircuit,
        config: &BitPhaseFlipNoiseConfig,
    ) {
        for i in 0..config.qubit_idxs.len() {
            let r: f64 = self.rng.random();

            if r < config.probabilities[i] {
                qc.pauli_y(config.qubit_idxs[i]);
            }
        }
    }
}
