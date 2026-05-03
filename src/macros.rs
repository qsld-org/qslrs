#[macro_export]
macro_rules! build_circuit_from_gate_array {
    ([$(($name:ident, [$($qubit_idx:expr),*])),*]) => {{

        let mut qc = QuantumCircuit::builder()
            .num_qubits(1)
            .print_measurements()
            .build()
            .expect("failed to build circuit in macro");

        $(
            qc.$name($($qubit_idx),*);
        )*

        qc

    }};

    ([$(($name:ident, [$($qubit_idx:expr),*])),*], $qubit_num:expr) => {{

        let mut qc = QuantumCircuit::builder()
            .num_qubits($qubit_num)
            .print_measurements()
            .build()
            .expect("failed to build circuit in macro");

        $(
            qc.$name($($qubit_idx),*);
        )*

        qc

    }};

    ([$(($name:ident, [$($qubit_idx:expr),*])),*], $qubit_num:expr, $collapse:expr) => {{

        let mut qc;
        if !$collapse {
            qc = QuantumCircuit::builder()
                .num_qubits($qubit_num)
                .print_measurements()
                .build()
                .expect("failed to build circuit in macro");
        } else {
            qc = QuantumCircuit::builder()
                .num_qubits($qubit_num)
                .collapse()
                .print_measurements()
                .build()
                .expect("failed to build circuit in macro");
        }

        $(
            qc.$name($($qubit_idx),*);
        )*

        qc

    }};

    ([$(($name:ident, [$($qubit_idx:expr),*])),*], $qubit_num:expr, $collapse:expr, $shots:expr) => {{

        let mut qc;
        if !$collapse {
            qc = QuantumCircuit::builder()
                .num_qubits($qubit_num)
                .print_measurements()
                .shots($shots)
                .build()
                .expect("failed to build circuit in macro");
        } else {
            if $shots > 1 {
                println!("Having more then one shot with collapse is invalid...ignoring the shots, this would be an error normally");
            }
            qc = QuantumCircuit::builder()
                .num_qubits($qubit_num)
                .collapse()
                .print_measurements()
                .build()
                .expect("failed to build circuit in macro");
        }

        $(
            qc.$name($($qubit_idx),*);
        )*

        qc

    }};
}
