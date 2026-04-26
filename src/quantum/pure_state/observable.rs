use nalgebra::{Complex, DVector};

pub struct Observable {
    pub(crate) coeffs: Vec<Complex<f64>>,
    pub(crate) observable_decomp: Vec<String>,
    pub(crate) num_qubits: u32,
}

impl Observable {
    /// Makes a new `Observable` object
    pub fn new(observable_decomp: Vec<String>, coeffs: Vec<Complex<f64>>, num_qubits: u32) -> Self {
        assert!(
            observable_decomp.len() == coeffs.len(),
            "Array of coefficients and array of pauli strings must be the same length"
        );

        Observable {
            coeffs,
            observable_decomp,
            num_qubits,
        }
    }

    /// Applies an observable to a quantum state
    #[allow(unused_assignments)]
    pub(crate) fn apply(&self, psi: &DVector<Complex<f64>>) -> DVector<Complex<f64>> {
        let mut phi = DVector::zeros(psi.len());
        for (i, term) in self.observable_decomp.iter().enumerate() {
            assert!(
                term.len() == self.num_qubits as usize,
                "Each term in the observable decomposition must be the same length as the number of qubits"
            );

            let mut j_prime = 0;

            for j in 0..psi.len() {
                let mut phase_acc = Complex::new(1.0, 0.0);
                let bit_str = format!("{:0width$b}", j, width = self.num_qubits as usize);
                let mut bit_arr = bit_str.chars().collect::<Vec<_>>();

                for q in 0..self.num_qubits {
                    let q_idx = q as usize;
                    // This is not great but its the only way I figured how to
                    let cur_term = term.chars().collect::<Vec<_>>()[q_idx];

                    if cur_term == 'I' {
                        continue;
                    } else if cur_term == 'X' {
                        let bit_q = bit_arr[q_idx]
                            .to_digit(2)
                            .expect("Could not convert char to integer");
                        let bit_prime = char::from_u32((bit_q ^ 1) + '0' as u32)
                            .expect("Could not convert integer to char");
                        bit_arr[q_idx] = bit_prime;
                    } else if cur_term == 'Y' {
                        let bit_q = bit_arr[q_idx]
                            .to_digit(2)
                            .expect("Could not convert integer to char");
                        let bit_prime = char::from_u32((bit_q ^ 1) + '0' as u32)
                            .expect("Could not convert integer to char");
                        bit_arr[q_idx] = bit_prime;

                        if bit_q == 0 {
                            phase_acc *= Complex::new(0.0, 1.0);
                        } else {
                            phase_acc *= Complex::new(0.0, -1.0);
                        }
                    } else if cur_term == 'Z' {
                        if bit_arr[q_idx] == '1' {
                            phase_acc *= Complex::new(-1.0, 0.0);
                        }
                    }
                }
                let temp_j = bit_arr.iter().collect::<String>();
                j_prime = u32::from_str_radix(&temp_j, 2)
                    .expect("Could not convert bit string into integer")
                    as usize;

                phi[j_prime] = phi[j_prime] + self.coeffs[i] * phase_acc * psi[j];
            }
        }
        phi
    }
}
