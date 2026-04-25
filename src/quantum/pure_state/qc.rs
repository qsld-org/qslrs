use anyhow::{Result, anyhow};
use nalgebra::base::{DMatrix, DVector};
use nalgebra::{Complex, Normed, dmatrix};
use rand::RngExt;
use std::collections::HashMap;
use std::f64::consts::PI;

/// The main QuantumCircuit object used to execute gates on
/// qubits. Refer to the `QuantumCircuitBuilder` for how to create
/// this object.
#[allow(dead_code)]
pub struct QuantumCircuit {
    pub num_qubits: u32,
    pub state: DVector<Complex<f64>>,
    pub initial_state_idx: usize,
    pub(crate) visualization_arr: Vec<(String, Vec<u32>, bool)>,
    pub(crate) shots: u32,
    pub(crate) collapse: bool,
}

/// The object which builds the quantum circuit according to various
/// specifications using the `num_qubits` and `initial_state_idx` functions
///
/// # Example
///
/// ```
/// use qslrs::quantum::pure_state::qc::QuantumCircuit;
/// let mut qc = QuantumCircuit::builder()
///     .num_qubits(2)
///     .initial_state_idx(0)
///     .build()?;
/// ```
#[allow(dead_code)]
pub struct QuantumCircuitBuilder {
    num_qubits: Option<u32>,
    state: Option<DVector<Complex<f64>>>,
    initial_state_idx: Option<usize>,
    visualization_arr: Option<Vec<(String, Vec<u32>, bool)>>,
    shots: Option<u32>,
    collapse: bool,
}

impl QuantumCircuitBuilder {
    /// Specifies the number of qubits to use for the instance of
    /// `QuantumCircuit`
    ///
    /// # Example
    ///
    /// ```
    /// use qslrs::quantum::pure_state::qc::QuantumCircuit;
    /// let mut qc = QuantumCircuit::builder()
    ///     .num_qubits(1)
    ///     .build()?;
    /// ```
    pub fn num_qubits(&mut self, value: u32) -> &mut Self {
        self.num_qubits = Some(value);
        self
    }

    /// Specifies the basis state index to start in
    ///
    /// # Example
    ///
    /// ```
    /// use qslrs::quantum::pure_state::qc::QuantumCircuit;
    /// let mut qc = QuantumCircuit::builder()
    ///     .num_qubits(2)
    ///     .initial_state_idx(1)
    ///     .build()?;
    /// ```
    pub fn initial_state_idx(&mut self, value: usize) -> &mut Self {
        self.initial_state_idx = Some(value);
        self
    }

    /// Specifies the amount of times to measure the circuit
    pub fn shots(&mut self, value: u32) -> &mut Self {
        self.shots = Some(value);
        self
    }

    /// Specifies to collapse the quantum state after measurement.
    /// **NOtE** this function cannot be used in combination with
    /// shots
    pub fn collapse(&mut self) -> &mut Self {
        self.collapse = true;
        self
    }

    /// Builds the `QuantumCircuit` object based on the parameters
    /// specified by the builder functions
    #[allow(unused_assignments)]
    pub fn build(&mut self) -> Result<QuantumCircuit> {
        let mut num_qubits = 0;
        let mut initial_state_idx = 0;
        let mut measure_shots = 0;

        if let Some(nq) = self.num_qubits {
            num_qubits = nq;
        } else {
            return Err(anyhow!(
                "The number of qubits was not initialized correctly"
            ));
        }

        if let Some(isi) = self.initial_state_idx {
            initial_state_idx = isi;
        } else {
            initial_state_idx = 0;
        }

        if let Some(shots) = self.shots {
            if shots < 1 {
                return Err(anyhow!("Cannot have less than one shot"));
            }

            measure_shots = shots;
        } else {
            measure_shots = 1;
        }

        if self.collapse && measure_shots > 1 {
            return Err(anyhow!(
                "Cannot collapse state while also having many shots"
            ));
        }

        let mut state_arr =
            DVector::from_element(2u32.pow(num_qubits) as usize, Complex::new(0.0, 0.0));
        state_arr[initial_state_idx] = Complex::new(1.0, 0.0);

        let visualization_arr = Vec::new();

        Ok(QuantumCircuit {
            num_qubits: num_qubits,
            state: state_arr,
            initial_state_idx: initial_state_idx,
            visualization_arr: visualization_arr,
            shots: measure_shots,
            collapse: self.collapse,
        })
    }
}

impl QuantumCircuit {
    /// Gives a blank `QuantumCircuitBuilder` object
    /// to be constructed
    pub fn builder() -> QuantumCircuitBuilder {
        QuantumCircuitBuilder {
            num_qubits: None,
            state: None,
            initial_state_idx: None,
            visualization_arr: None,
            shots: None,
            collapse: false,
        }
    }

    // Updates the internal visualization array
    fn update_visualization_arr(&mut self, gate_name: String, qubit_idxs: Vec<u32>) {
        self.visualization_arr.push((gate_name, qubit_idxs, true));
    }

    /// Specifies not to visualize a specific gate in circuit diagrams
    /// generated
    ///
    /// # Example
    ///
    /// ```
    /// use qslrs::quantum::pure_state::qc::QuantumCircuit;
    /// let mut qc = QuantumCircuit::builder()
    ///     .num_qubits(1)
    ///     .build()?;
    ///
    /// qc.hadamard(0).no_visualization();
    /// ```
    ///
    /// **NOTE** this function only affects the previously called
    /// gate in the chain
    pub fn no_visualization(&mut self) -> &mut Self {
        if let Some(last_elem) = self.visualization_arr.last_mut() {
            last_elem.2 = false;
        }
        self
    }

    /// Puts the qubit specified into superposition
    ///
    /// # Example
    ///
    /// ```
    /// use qslrs::quantum::pure_state::qc::QuantumCircuit;
    /// let mut qc = QuantumCircuit::builder()
    ///     .num_qubits(2)
    ///     .build()?;
    ///
    /// qc.hadamard(0);
    /// ```
    pub fn hadamard(&mut self, qubit_idx: u32) -> &mut Self {
        self.update_visualization_arr(String::from("H"), Vec::from([qubit_idx]));

        let mut hadamard_mat: DMatrix<Complex<f64>> = dmatrix![Complex::new(1.0, 0.0), Complex::new(1.0, 0.0);
                                    Complex::new(1.0, 0.0), Complex::new(-1.0, 0.0)];

        hadamard_mat = hadamard_mat * Complex::new(1.0 / 2.0f64.sqrt(), 0.0);
        let mut pairs: Vec<(usize, usize)> = Vec::with_capacity(self.state.len() / 2);
        pairs.resize(self.state.len() / 2, (0, 0));
        let mut pairs_idx = 0;

        for i in 0..self.state.len() {
            let qubit_is_one = (i & (1 << qubit_idx)) != 0;
            if !qubit_is_one {
                let j = i ^ (1 << qubit_idx);
                pairs[pairs_idx] = (i, j);
                pairs_idx += 1;
            }
        }

        for pair in pairs.iter() {
            let amplitudes = DVector::from_vec(vec![self.state[pair.0], self.state[pair.1]]);
            let updated_amplitudes = hadamard_mat.clone() * amplitudes;
            self.state[pair.0] = updated_amplitudes[0];
            self.state[pair.1] = updated_amplitudes[1];
        }

        self
    }

    /// Puts the target qubit into superposition if and only if the control qubit is 1
    ///
    /// # Example
    ///
    /// ```
    /// use qslrs::quantum::pure_state::qc::QuantumCircuit
    /// let mut qc = QuantumCircuit::builder()
    ///     .num_qubits(2)
    ///     .initial_state_idx(1)
    ///     .build()?;
    ///
    /// qc.ch(0, 1);
    /// ```
    pub fn ch(&mut self, control_qubit_idx: u32, target_qubit_idx: u32) -> &mut Self {
        assert!(
            self.num_qubits >= 2,
            "The number of qubits must be greater than or equal to two in order to use controlled gate"
        );

        self.update_visualization_arr(
            String::from("CH"),
            Vec::from([control_qubit_idx, target_qubit_idx]),
        );

        for i in 0..self.state.len() {
            let cntl_qubit_is_one = (i & (1 << control_qubit_idx)) != 0;
            if cntl_qubit_is_one {
                let j = i ^ (1 << target_qubit_idx);
                if i < j {
                    let temp_i = self.state[i];
                    let temp_j = self.state[j];
                    self.state[i] = (temp_i + temp_j) / 2.0f64.sqrt();
                    self.state[j] = (temp_i + temp_j) / 2.0f64.sqrt();
                }
            }
        }

        self
    }

    /// Flips the value of the qubit specified, if it is 0 it becomes 1 and vice versa
    ///
    /// # Example
    ///
    /// ```
    /// use qslrs::quantum::pure_state::qc::QuantumCircuit
    /// let mut qc = QuantumCircuit::builder()
    ///     .num_qubits(2)
    ///     .build()?;
    ///
    /// qc.pauli_x(0);
    /// ```
    pub fn pauli_x(&mut self, qubit_idx: u32) -> &mut Self {
        self.update_visualization_arr(String::from("X"), Vec::from([qubit_idx]));

        let mut pairs: Vec<(usize, usize)> = Vec::with_capacity(self.state.len() / 2);
        pairs.resize(self.state.len() / 2, (0, 0));
        let mut pairs_idx = 0;

        for i in 0..self.state.len() {
            let qubit_is_zero = (i & (1 << qubit_idx)) == 0;
            if qubit_is_zero {
                let j = i ^ (1 << qubit_idx);
                pairs[pairs_idx] = (i, j);
                pairs_idx += 1;
            }
        }

        for pair in pairs.iter() {
            let temp = self.state[pair.0];
            self.state[pair.0] = self.state[pair.1];
            self.state[pair.1] = temp;
        }

        self
    }

    /// Flips the phase and value of the qubit specified
    ///
    /// # Example
    ///
    /// ```
    /// use qslrs::quantum::pure_state::qc::QuantumCircuit;
    /// let mut qc = QuantumCircuit::builder()
    ///     .num_qubits(2)
    ///     .build()?;
    ///
    /// qc.pauli_y(0);
    /// ```
    pub fn pauli_y(&mut self, qubit_idx: u32) -> &mut Self {
        self.update_visualization_arr(String::from("Y"), Vec::from([qubit_idx]));

        for i in 0..self.state.len() {
            let j = i ^ (1 << qubit_idx);
            if i < j {
                let temp = self.state[i];
                self.state[i] = self.state[j] * Complex::new(0.0, 1.0);
                self.state[j] = temp * Complex::new(0.0, -1.0);
            }
        }

        self
    }

    /// Flips the phase of the qubit specified
    ///
    /// # Example
    ///
    /// ```
    /// use qslrs::quantum::pure_state::qc::QuantumCircuit;
    /// let mut qc = QuantumCircuit::builder()
    ///     .num_qubits(2)
    ///     .build()?;
    ///
    /// qc.pauli_x(0).pauli_z(0);
    /// ```
    pub fn pauli_z(&mut self, qubit_idx: u32) -> &mut Self {
        self.update_visualization_arr(String::from("Z"), Vec::from([qubit_idx]));

        for i in 0..self.state.len() {
            if (i & (1 << qubit_idx)) != 0 {
                self.state[i] = self.state[i] * Complex::new(-1.0, 0.0);
            }
        }

        self
    }

    /// Flips the value of the target qubit specified if and only if the control qubit specified
    /// is 1
    ///
    /// # Example
    ///
    /// ```
    /// use qslrs::quantum::pure_state::qc::QuantumCircuit;
    /// let mut qc = QuantumCircuit::builder()
    ///     .num_qubits(2)
    ///     .build()?;
    ///
    /// // Create a bell state
    /// qc.hadamard(0).cnot(0, 1);
    /// ```
    pub fn cnot(&mut self, control_qubit_idx: u32, target_qubit_idx: u32) -> &mut Self {
        assert!(
            self.num_qubits >= 2,
            "The number of qubits must be greater than or equal to two in order to use controlled gates"
        );

        self.update_visualization_arr(
            String::from("CX"),
            Vec::from([control_qubit_idx, target_qubit_idx]),
        );

        for i in 0..self.state.len() {
            let control_is_one = i & (1 << control_qubit_idx) != 0;

            if control_is_one {
                let j = i ^ (1 << target_qubit_idx);
                if i < j {
                    let temp = self.state[i];
                    self.state[i] = self.state[j];
                    self.state[j] = temp;
                }
            }
        }

        self
    }

    /// Flips of the phase of the target qubit specified if and only if the control qubit specified
    /// is 1
    ///
    /// # Example
    ///
    /// ```
    /// use qslrs::quantum::pure_state::qc::QuantumCircuit;
    /// let mut qc = QuantumCircuit::builder()
    ///     .num_qubits(2)
    ///     .build()?;
    ///
    /// qc.pauli_x(0).pauli_x(1).cz(0, 1);
    /// ```
    pub fn cz(&mut self, control_qubit_idx: u32, target_qubit_idx: u32) -> &mut Self {
        assert!(
            self.num_qubits >= 2,
            "The number of qubits must be greater than or equal to two in order to use controlled gates"
        );

        self.update_visualization_arr(
            String::from("CZ"),
            Vec::from([control_qubit_idx, target_qubit_idx]),
        );

        for i in 0..self.state.len() {
            let cntl_qubit_is_one = i & (1 << control_qubit_idx) != 0;
            let tgt_qubit_is_one = i & (1 << target_qubit_idx) != 0;
            if cntl_qubit_is_one && tgt_qubit_is_one {
                self.state[i] = self.state[i] * Complex::new(-1.0, 0.0);
            }
        }

        self
    }

    /// Applies a phase shift of PI/4 to the qubit specified if it is in the 1 state
    ///
    /// # Example
    ///
    /// ```
    /// use qslrs::quantum::pure_state::qc::QuantumCircuit;
    /// let mut qc = QuantumCircuit::builder()
    ///     .num_qubits(1)
    ///     .build()?;
    ///
    /// qc.pauli_x(0).s(0);
    /// ```
    pub fn s(&mut self, qubit_idx: u32) -> &mut Self {
        self.update_visualization_arr(String::from("S"), Vec::from([qubit_idx]));

        for i in 0..self.state.len() {
            let qubit_is_one = i & (1 << qubit_idx) != 0;
            if qubit_is_one {
                self.state[i] = self.state[i] * Complex::new(0.0, 1.0);
            }
        }

        self
    }

    /// Applies a phase shift of PI/8 to the qubit specified if it is in the 1 state
    ///
    /// # Example
    ///
    /// ```
    /// use qslrs::quantum::pure_state::qc::QuantumCircuit;
    /// let mut qc = Quantumcircuit::builder()
    ///     .num_qubits(1)
    ///     .build()?;
    ///
    /// qc.pauli_x(0).t(0);
    /// ```
    pub fn t(&mut self, qubit_idx: u32) -> &mut Self {
        self.update_visualization_arr(String::from("T"), Vec::from([qubit_idx]));

        let i = Complex::new(0.0, 1.0);
        for j in 0..self.state.len() {
            let qubit_is_one = j & (1 << qubit_idx) != 0;
            if qubit_is_one {
                self.state[j] = self.state[j] * Complex::exp(i * Complex::new(PI / 4.0, 0.0));
            }
        }

        self
    }

    /// Flips the phase of a set of qubits if and only if all of them are in the 1 state
    ///
    /// # Example
    ///
    /// ```
    /// use qslrs::quantum::pure_state::qc::QuantumCircuit;
    /// let mut qc = QuantumCircuit::builder()
    ///     .num_qubits(3)
    ///     .initial_state_idx(7)
    ///     .build()?;
    ///
    /// qc.mcz(&[0, 1, 2]);
    /// ```
    pub fn mcz(&mut self, target_qubit_idxs: &[u32]) -> &mut Self {
        assert!(
            target_qubit_idxs.len() >= 3,
            "Their should be more than 3 qubits provided to the MCZ gate, if you want to provide less than that use the CZ or Z gate"
        );

        self.update_visualization_arr(String::from("MCZ"), Vec::from(target_qubit_idxs));

        let mut target_mask = 0;
        for qubit_idx in target_qubit_idxs.iter() {
            target_mask = target_mask | (1 << qubit_idx);
        }

        for i in 0..self.state.len() {
            if (i & target_mask) == target_mask {
                self.state[i] = self.state[i] * Complex::new(-1.0, 0.0);
            }
        }

        self
    }

    /// Swaps the qubits if and only if their states differ from each other
    ///
    /// # Example
    ///
    /// ```
    /// use qslrs::quantum::pure_state::qc::QuatumCircuit;
    /// let mut qc = QuantumCircuit::builder()
    ///     .num_qubits(2)
    ///     .initial_state_idx(1)
    ///     .build()?;
    ///
    /// qc.swap(0, 1);
    /// ```
    pub fn swap(&mut self, qubit1: u32, qubit2: u32) -> &mut Self {
        assert!(
            self.num_qubits >= 2,
            "The number of qubits must be greater than or equal to two in order to use the swap gate"
        );

        self.update_visualization_arr(String::from("SWAP"), Vec::from([qubit1, qubit2]));

        for i in 0..self.state.len() {
            let qubit1_val = (i >> qubit1) & 1;
            let qubit2_val = (i >> qubit2) & 1;
            if qubit1_val != qubit2_val {
                let j = i ^ ((1 << qubit1) | (1 << qubit2));
                if i < j {
                    let temp = self.state[i];
                    self.state[i] = self.state[j];
                    self.state[j] = temp;
                }
            }
        }

        self
    }

    /// Swaps the qubits specified if and only if they are in different states
    /// and adds a phase of i
    ///
    /// # Example
    ///
    /// ```
    /// use qslrs::quantum::pure_state::qc::QuantumCircuit;
    /// let mut qc = QuantumCircuit::builder()
    ///     .num_qubits(2)
    ///     .initial_state_idx(1)
    ///     .build()?;
    ///
    /// qc.iswap(0, 1);
    /// ```
    pub fn iswap(&mut self, qubit1: u32, qubit2: u32) -> &mut Self {
        self.update_visualization_arr(String::from("iSWAP"), Vec::from([qubit1, qubit2]));

        for i in 0..self.state.len() {
            let qubit1_val = (i >> qubit1) & 1;
            let qubit2_val = (i >> qubit2) & 1;
            if qubit1_val != qubit2_val {
                let j = i ^ ((1 << qubit1) | (1 << qubit2));
                if i < j {
                    let temp = self.state[i];
                    self.state[i] = self.state[j];
                    self.state[j] = temp;
                    self.state[i] = self.state[i] * Complex::new(0.0, 1.0);
                    self.state[j] = self.state[j] * Complex::new(0.0, 1.0);
                }
            }
        }

        self
    }

    /// Rotates the qubit specified about the x-axis of the bloch sphere by
    /// the angle theta in radians
    ///
    /// # Example
    ///
    /// ```
    /// use std::f64::consts::PI;
    /// use qslrs::quantum::pure_state::qc::QuantumCircuit;
    /// let mut qc = QuantumCircuit::builder()
    ///     .num_qubits(1)
    ///     .build()?;
    ///
    /// qc.rx(0, PI / 4.0);
    /// ```
    pub fn rx(&mut self, qubit_idx: u32, theta: f64) -> &mut Self {
        self.update_visualization_arr(String::from("R_X"), Vec::from([qubit_idx]));

        let c = Complex::new((theta / 2.0).cos(), 0.0);
        let s = Complex::new(0.0, -1.0) * Complex::new((theta / 2.0).sin(), 0.0);

        let mut psi: DVector<Complex<f64>> = DVector::zeros(2usize.pow(self.num_qubits));

        for i in 0..self.state.len() {
            let j = i ^ (1 << qubit_idx);
            if i < j {
                let a = self.state[i];
                let b = self.state[j];
                psi[i] = c * a + s * b;
                psi[j] = s * a + c * b;
            }
        }
        self.state = psi;

        self
    }

    /// Rotates the qubit specified about the y-axis of the bloch sphere by the
    /// angle theta in radians
    ///
    /// # Example
    ///
    /// ```
    /// use std::f64::consts::PI;
    /// use qslrs::quantum::pure_state::qc::QuantumCircuit;
    /// let mut qc = QuantumCircuit::builder()
    ///     .num_qubits(1)
    ///     .build()?;
    ///
    /// qc.ry(0, PI / 4.0);
    /// ```
    pub fn ry(&mut self, qubit_idx: u32, theta: f64) -> &mut Self {
        self.update_visualization_arr(String::from("R_Y"), Vec::from([qubit_idx]));

        let c = Complex::new((theta / 2.0).cos(), 0.0);
        let s = Complex::new((theta / 2.0).sin(), 0.0);

        let mut psi: DVector<Complex<f64>> = DVector::zeros(2usize.pow(self.num_qubits));

        for i in 0..self.state.len() {
            let j = i ^ (1 << qubit_idx);
            if i < j {
                let a = self.state[i];
                let b = self.state[j];

                psi[i] = c * a - s * b;
                psi[j] = s * a + c * b;
            }
        }
        self.state = psi;

        self
    }

    /// Rotates the qubit specified about the z-axis of the bloch sphere by the
    /// angle theta in radians
    ///
    /// # Example
    ///
    /// ```
    /// use std::f64::consts:PI;
    /// use qslrs::quantum:pure_state::qc::QuantumCircuit;
    /// let mut qc = QuantumCircuit::builder()
    ///     .num_qubits(1)
    ///     .build()?;
    ///
    /// qc.rz(0, PI / 4.0);
    /// ```
    pub fn rz(&mut self, qubit_idx: u32, theta: f64) -> &mut Self {
        self.update_visualization_arr(String::from("R_Z"), Vec::from([qubit_idx]));

        let z0 = (Complex::new(0.0, -1.0) * Complex::new(theta / 2.0, 0.0)).exp();
        let z1 = (Complex::new(0.0, 1.0) * Complex::new(theta / 2.0, 0.0)).exp();

        for i in 0..self.state.len() {
            let qubit_value = (i >> qubit_idx) & 1;
            if qubit_value == 0 {
                self.state[i] = self.state[i] * z0;
            } else {
                self.state[i] = self.state[i] * z1;
            }
        }

        self
    }

    /// Rotates the target qubit specified by some order k, if and only if both the control and target are
    /// in the 1 state
    ///
    /// # Example
    ///
    /// ```
    /// use qslrs::quantum::pure_state::qc::QuantumCircuit;
    /// let mut qc = QuantumCircuit::builder()
    ///     .num_qubits(2)
    ///     .build()?;
    ///
    /// qc.hadamard(0);
    /// qc.hadamard(1);
    /// qc.cr(0, 1, 2);
    /// qc.hadamard(1);
    /// ```
    pub fn cr(&mut self, control_qubit_idx: u32, target_qubit_idx: u32, k: f64) -> &mut Self {
        assert!(
            self.num_qubits >= 2,
            "The number of qubits must be greater than or equal to two in order to use controlled gates"
        );

        self.update_visualization_arr(
            String::from("CR"),
            Vec::from([control_qubit_idx, target_qubit_idx]),
        );

        let i = Complex::new(0.0, 1.0);
        for j in 0..self.state.len() {
            let cntl_qubit_val = (j >> control_qubit_idx) & 1;
            let tgt_qubit_val = (j >> target_qubit_idx) & 1;

            if cntl_qubit_val == 1 && tgt_qubit_val == 1 {
                self.state[j] =
                    self.state[j] * Complex::exp(i * Complex::new(2.0 * PI / 2.0f64.powf(k), 0.0));
            }
        }

        self
    }

    /// Rotates the target qubit specified by some order k, if and only if both the control and target are
    /// in the 1 state. This is the inverse version so the resulting state will differ by some factor.
    ///
    /// # Example
    ///
    /// ```
    /// use qslrs::quantum::pure_state::qc::QuantumCircuit;
    /// let mut qc = QuantumCircuit::builder()
    ///     .num_qubits(2)
    ///     .build()?;
    ///
    /// qc.hadamard(0);
    /// qc.hadamard(1);
    /// qc.cr_inv(0, 1, 2);
    /// qc.hadamard(1);
    /// ```
    pub fn cr_inv(&mut self, control_qubit_idx: u32, target_qubit_idx: u32, k: f64) -> &mut Self {
        assert!(
            self.num_qubits >= 2,
            "The number of qubits must be greater than or equal to two in order to use controlled gates"
        );

        self.update_visualization_arr(
            String::from("CR"),
            Vec::from([control_qubit_idx, target_qubit_idx]),
        );

        let i = Complex::new(0.0, 1.0);
        for j in 0..self.state.len() {
            let cntl_qubit_val = (j >> control_qubit_idx) & 1;
            let tgt_qubit_val = (j >> target_qubit_idx) & 1;

            if cntl_qubit_val == 1 && tgt_qubit_val == 1 {
                self.state[j] =
                    self.state[j] * Complex::exp(i * Complex::new(-2.0 * PI / 2.0f64.powf(k), 0.0));
            }
        }

        self
    }

    /// Measures the qubit specified and gives a singular state as the
    /// result unless the qubit is in superposition or some other arbitrary
    /// probability distribution of states
    ///
    /// # Example
    ///
    /// ```
    /// use qslrs::quantum::pure_state::qc::QuantumCircuit;
    /// let mut qc = QuantumCircuit::builder()
    ///     .num_qubits(1)
    ///     .shots(1000)
    ///     .build()?;
    ///
    /// println!("{}", qc.hadamard(0).measure(0));
    /// ```
    #[allow(unused_assignments, unused_variables)]
    pub fn measure(&mut self, qubit_idx: u32) -> String {
        self.update_visualization_arr(String::from("M"), Vec::from([qubit_idx]));

        let mut result = 0;
        let mut result_map: HashMap<String, u32> = HashMap::new();

        let mut rng = rand::rng();

        for _ in 0..self.shots {
            let mut probability_0 = 0.0;
            let mut probability_1 = 0.0;

            for i in 0..self.state.len() {
                let qubit_is_zero = (i & (1 << qubit_idx)) == 0;
                let qubit_is_one = (i & (1 << qubit_idx)) != 0;
                if qubit_is_zero {
                    let state_prob = self.state[i].norm_sqr();
                    probability_0 += state_prob;
                } else if qubit_is_one {
                    let state_prob = self.state[i].norm_sqr();
                    probability_1 += state_prob;
                }
            }

            let r: f64 = rng.random();

            if r < probability_0 {
                result = 0;
                *result_map.entry(result.to_string()).or_insert(0) += 1;
            } else if r >= probability_0 {
                result = 1;
                *result_map.entry(result.to_string()).or_insert(0) += 1;
            }
        }

        if self.shots == 1 {
            String::from(format!("{result}"))
        } else {
            format!("{result_map:?}")
        }
    }

    /// Measure multiple qubits which are a subset of all the qubits in the
    /// system
    ///
    /// # Example
    ///
    /// ```
    /// use qslrs::quantum::pure_state::qc::QuantumCircuit;
    /// let mut qc = QuantumCircuit::builder()
    ///     .num_qubits(2)
    ///     .shots(1000)
    ///     .build()?;
    ///
    /// println!("{}", qc.hadamard(0).cnot(0, 1).measure_many(&[0, 1]));
    /// ```
    pub fn measure_many(&mut self, qubit_idxs: &[u32]) -> String {
        assert!(
            qubit_idxs.len() <= self.num_qubits as usize,
            "The amount of qubit indices specified is too many"
        );

        self.update_visualization_arr(String::from("M"), Vec::from(qubit_idxs));

        let mut rng = rand::rng();
        let mut result = 0;
        let mut result_map: HashMap<String, u32> = HashMap::new();

        for _ in 0..self.shots {
            let mut b_probs: Vec<f64> = Vec::new();
            b_probs.resize(1 << qubit_idxs.len(), 0.0);

            let mut mask = 0;
            for idx in qubit_idxs.iter() {
                mask = mask | (1 << idx);
            }

            for b in 0..(1 << qubit_idxs.len()) {
                let mut target = 0;

                for (i, idx) in qubit_idxs.iter().enumerate() {
                    let bit = (b >> i) & 1;
                    if bit == 1 {
                        target = target | (1 << idx);
                    }
                }

                for i in 0..(1 << self.num_qubits) {
                    if (i & mask) == target {
                        b_probs[b] += self.state[i].norm_sqr();
                    }
                }
            }

            let r: f64 = rng.random();
            let mut sum: f64 = 0.0;

            for b in 0..b_probs.len() {
                sum += b_probs[b];
                if sum > r {
                    result = b;
                    *result_map
                        .entry(format!("{:0width$b}", result, width = qubit_idxs.len()))
                        .or_insert(0) += 1;
                    break;
                }
            }
        }

        if self.shots == 1 {
            format!("{:0width$b}", result, width = qubit_idxs.len())
        } else {
            format!("{result_map:?}")
        }
    }

    /// Measures all the qubits in the system
    ///
    /// # Example
    ///
    /// ```
    /// use qslrs::quantum::pure_state::qc::QuantumCircuit;
    /// let mut qc = QuantumCircuit::builder()
    ///     .num_qubits(2)
    ///     .shots(1000)
    ///     .build()?;
    ///
    /// println!("{}". qc.hadamard(0).cnot(0, 1).measure_all());
    /// ```
    pub fn measure_all(&mut self) -> String {
        self.update_visualization_arr(String::from("MA"), Vec::from_iter(0..self.num_qubits));

        let mut probs: Vec<f64> = Vec::new();
        probs.resize(self.state.len(), 0.0);

        let mut result = 0;
        let mut result_map: HashMap<String, u32> = HashMap::new();
        let mut rng = rand::rng();

        for _ in 0..self.shots {
            for i in 0..self.state.len() {
                let prob = self.state[i].norm_squared();
                probs[i] = prob;
            }

            let r: f64 = rng.random();
            let mut sum: f64 = 0.0;

            for i in 0..probs.len() {
                sum += probs[i];
                if r < sum {
                    result = i;
                    *result_map
                        .entry(format!(
                            "{:0width$b}",
                            result,
                            width = self.num_qubits as usize
                        ))
                        .or_insert(0) += 1;
                    break;
                }
            }
        }

        if self.shots == 1 {
            format!("{:0width$b}", result, width = self.num_qubits as usize)
        } else {
            format!("{result_map:?}")
        }
    }
}
