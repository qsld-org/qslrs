use std::f64::consts::PI;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rand::{RngExt, SeedableRng, rngs::StdRng, rngs::ThreadRng, seq::IteratorRandom};
use syn::{
    Ident, LitInt, Result, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

#[derive(Debug, Clone)]
struct GateSpec {
    name: Ident,
}

impl Parse for GateSpec {
    fn parse(input: ParseStream) -> Result<GateSpec> {
        let gate_name_ident: Ident = input.parse()?;

        Ok(GateSpec {
            name: gate_name_ident,
        })
    }
}

#[derive(Debug, Clone)]
struct NoiseSpec {
    name: Ident,
}

impl Parse for NoiseSpec {
    fn parse(input: ParseStream) -> Result<NoiseSpec> {
        let noise_config_ident: Ident = input.parse()?;

        Ok(NoiseSpec {
            name: noise_config_ident,
        })
    }
}

#[derive(Debug)]
struct RandomGateArrayInput {
    qc: Ident,
    gates: Vec<GateSpec>,
    qubits: u32,
    depth: usize,
    seed: Option<u64>,
    noises: Option<Vec<NoiseSpec>>,
}

impl Parse for RandomGateArrayInput {
    fn parse(input: ParseStream) -> Result<RandomGateArrayInput> {
        let qc_ident: Ident = input.parse()?;
        assert_eq!(qc_ident.to_string(), "qc");

        input.parse::<Token![=]>()?;

        let qc: Ident = input.parse()?;

        input.parse::<Token![,]>()?;

        let gates_ident: Ident = input.parse()?;
        assert_eq!(gates_ident.to_string(), "gates");

        input.parse::<Token![=]>()?;

        let gates_content;
        syn::bracketed!(gates_content in input);
        let gates = gates_content.parse_terminated(GateSpec::parse, Token![,])?;

        input.parse::<Token![,]>()?;

        let qubits_ident: Ident = input.parse()?;
        assert_eq!(qubits_ident.to_string(), "qubits");

        input.parse::<Token![=]>()?;

        let qubits: LitInt = input.parse()?;
        let qubits = qubits.base10_parse::<u32>()?;

        input.parse::<Token![,]>()?;

        let depth_ident: Ident = input.parse()?;
        assert_eq!(depth_ident.to_string(), "depth");

        input.parse::<Token![=]>()?;

        let depth: LitInt = input.parse()?;
        let depth = depth.base10_parse::<usize>()?;

        if let Ok(_) = input.parse::<Token![,]>() {
            let mut seed_value = None;
            if let Ok(seed_ident) = input.parse::<Ident>() {
                assert_eq!(seed_ident.to_string(), "seed");
                input.parse::<Token![=]>()?;

                let seed: LitInt = input.parse()?;
                seed_value = Some(seed.base10_parse::<u64>()?);
            }

            let mut noises = None;
            if let Ok(_) = input.parse::<Token![,]>() {
                if let Ok(noise_ident) = input.parse::<Ident>() {
                    assert_eq!(noise_ident.to_string(), "noises");
                    input.parse::<Token![=]>()?;

                    let noise_content;
                    syn::bracketed!(noise_content in input);
                    noises = Some(
                        noise_content
                            .parse_terminated(NoiseSpec::parse, Token![,])?
                            .into_iter()
                            .collect(),
                    );
                }

                return Ok(RandomGateArrayInput {
                    qc,
                    gates: gates.into_iter().collect(),
                    qubits,
                    depth,
                    seed: seed_value,
                    noises: noises,
                });
            } else {
                return Ok(RandomGateArrayInput {
                    qc,
                    gates: gates.into_iter().collect(),
                    qubits,
                    depth,
                    seed: seed_value,
                    noises: noises,
                });
            }
        } else {
            return Ok(RandomGateArrayInput {
                qc: qc,
                gates: gates.into_iter().collect(),
                qubits: qubits,
                depth: depth,
                seed: None,
                noises: None,
            });
        }
    }
}

fn create_single_qubit_gate_element(
    input: &RandomGateArrayInput,
    rng: &mut StdRng,
    random_gate_ident: Ident,
    mut gates: Vec<TokenStream2>,
) -> Vec<TokenStream2> {
    let qc = &input.qc;

    let random_qubit = (0..input.qubits)
        .choose(rng)
        .expect("Could not choose random number in random circuit generation");

    if random_gate_ident.to_string() == "measure".to_string() {
        gates.push(quote! {
            #qc.#random_gate_ident(#random_qubit);
        });

        return gates;
    }

    let noise_ident;
    if let Some(noises) = &input.noises {
        let insert_noise = rng.random_bool(0.5);
        if !insert_noise {
            gates.push(quote! {
                #qc.#random_gate_ident(#random_qubit);
            });

            return gates;
        }

        let noise_idx = rng.random_range(0..noises.len());
        noise_ident = &noises[noise_idx].name;

        match noise_ident.to_string() {
            val if val == "depolarizing_noise".to_string() => {
                gates.push(quote! {
                    #qc.#random_gate_ident(#random_qubit);
                    #qc.#noise_ident()?;
                });
            }
            val if val == "pauli_noise".to_string() => {
                gates.push(quote! {
                    #qc.#random_gate_ident(#random_qubit);
                    #qc.#noise_ident()?;
                });
            }
            val if val == "bit_flip_noise".to_string() => {
                gates.push(quote! {
                    #qc.#random_gate_ident(#random_qubit);
                    #qc.#noise_ident();
                });
            }
            val if val == "phase_flip_noise".to_string() => {
                gates.push(quote! {
                    #qc.#random_gate_ident(#random_qubit);
                    #qc.#noise_ident();
                });
            }
            val if val == "bit_phase_flip_noise".to_string() => {
                gates.push(quote! {
                    #qc.#random_gate_ident(#random_qubit);
                    #qc.#noise_ident();
                });
            }
            _ => panic!("Invalid noise type"),
        }
    } else {
        gates.push(quote! {
            #qc.#random_gate_ident(#random_qubit);
        });
    }

    gates
}

fn create_controlled_gate_element(
    input: &RandomGateArrayInput,
    rng: &mut StdRng,
    random_gate_ident: Ident,
    mut gates: Vec<TokenStream2>,
) -> Vec<TokenStream2> {
    let qc = &input.qc;

    let random_qubits = (0..input.qubits).sample(rng, 2);

    let noise_ident;
    if let Some(noises) = &input.noises {
        let insert_noise = rng.random_bool(0.5);
        if !insert_noise {
            gates.push(quote! {
                #qc.#random_gate_ident(#(#random_qubits),*);
            });

            return gates;
        }

        let noise_idx = rng.random_range(0..noises.len());
        noise_ident = &noises[noise_idx].name;

        match noise_ident.to_string() {
            val if val == "depolarizing_noise".to_string() => {
                gates.push(quote! {
                    #qc.#random_gate_ident(#(#random_qubits),*);
                    #qc.#noise_ident()?;
                });
            }
            val if val == "pauli_noise".to_string() => {
                gates.push(quote! {
                    #qc.#random_gate_ident(#(#random_qubits),*);
                    #qc.#noise_ident()?;
                });
            }
            val if val == "bit_flip_noise".to_string() => {
                gates.push(quote! {
                    #qc.#random_gate_ident(#(#random_qubits),*);
                    #qc.#noise_ident();
                });
            }
            val if val == "phase_flip_noise".to_string() => {
                gates.push(quote! {
                    #qc.#random_gate_ident(#(#random_qubits),*);
                    #qc.#noise_ident();
                });
            }
            val if val == "bit_phase_flip_noise".to_string() => {
                gates.push(quote! {
                    #qc.#random_gate_ident(#(#random_qubits),*);
                    #qc.#noise_ident();
                });
            }
            _ => panic!("Invalid noise type"),
        }
    } else {
        gates.push(quote! {
            #qc.#random_gate_ident(#(#random_qubits),*);
        });
    }

    gates
}

fn create_rotation_gate_element(
    input: &RandomGateArrayInput,
    rng: &mut StdRng,
    random_gate_ident: Ident,
    mut gates: Vec<TokenStream2>,
) -> Vec<TokenStream2> {
    let qc = &input.qc;

    let random_qubit = (0..input.qubits)
        .choose(rng)
        .expect("Could not choose random number in random circuit generation");

    let random_theta_denom: f64 = rng.random_range(1.0..=10.0);
    let random_theta = PI / random_theta_denom;

    let noise_ident;
    if let Some(noises) = &input.noises {
        let insert_noise = rng.random_bool(0.5);
        if !insert_noise {
            gates.push(quote! {
                #qc.#random_gate_ident(#random_qubit, #random_theta);
            });

            return gates;
        }

        let noise_idx = rng.random_range(0..noises.len());
        noise_ident = &noises[noise_idx].name;

        match noise_ident.to_string() {
            val if val == "depolarizing_noise".to_string() => {
                gates.push(quote! {
                    #qc.#random_gate_ident(#random_qubit, #random_theta);
                    #qc.#noise_ident()?;
                });
            }
            val if val == "pauli_noise".to_string() => {
                gates.push(quote! {
                    #qc.#random_gate_ident(#random_qubit, #random_theta);
                    #qc.#noise_ident()?;
                });
            }
            val if val == "bit_flip_noise".to_string() => {
                gates.push(quote! {
                    #qc.#random_gate_ident(#random_qubit, #random_theta);
                    #qc.#noise_ident();
                });
            }
            val if val == "phase_flip_noise".to_string() => {
                gates.push(quote! {
                    #qc.#random_gate_ident(#random_qubit, #random_theta);
                    #qc.#noise_ident();
                });
            }
            val if val == "bit_phase_flip_noise".to_string() => {
                gates.push(quote! {
                    #qc.#random_gate_ident(#random_qubit, #random_theta);
                    #qc.#noise_ident();
                });
            }
            _ => panic!("Invalid noise type"),
        }
    } else {
        gates.push(quote! {
            #qc.#random_gate_ident(#random_qubit, #random_theta);
        });
    }

    gates
}

#[proc_macro]
pub fn generate_random_circuit(args: TokenStream) -> TokenStream {
    let input = parse_macro_input!(args as RandomGateArrayInput);

    let mut rng;
    if let Some(seed) = input.seed {
        rng = StdRng::seed_from_u64(seed);
    } else {
        let seed: u64 = rand::random();
        rng = StdRng::seed_from_u64(seed);
    }

    let gates_arr_len = input.gates.len();
    let mut gates: Vec<TokenStream2> = Vec::new();

    for _ in 0..input.depth {
        let random_idx = rng.random_range(0..gates_arr_len);
        let random_gate = input.gates[random_idx].clone();
        let random_gate_ident = random_gate.name;
        match random_gate_ident.to_string() {
            val if val == "hadamard".to_string() => {
                gates =
                    create_single_qubit_gate_element(&input, &mut rng, random_gate_ident, gates);
            }
            val if val == "pauli_x".to_string() => {
                gates =
                    create_single_qubit_gate_element(&input, &mut rng, random_gate_ident, gates);
            }
            val if val == "pauli_y".to_string() => {
                gates =
                    create_single_qubit_gate_element(&input, &mut rng, random_gate_ident, gates);
            }
            val if val == "pauli_z".to_string() => {
                gates =
                    create_single_qubit_gate_element(&input, &mut rng, random_gate_ident, gates);
            }
            val if val == "s".to_string() => {
                gates =
                    create_single_qubit_gate_element(&input, &mut rng, random_gate_ident, gates);
            }
            val if val == "t".to_string() => {
                gates =
                    create_single_qubit_gate_element(&input, &mut rng, random_gate_ident, gates);
            }
            val if val == "rx".to_string() => {
                gates = create_rotation_gate_element(&input, &mut rng, random_gate_ident, gates);
            }
            val if val == "ry".to_string() => {
                gates = create_rotation_gate_element(&input, &mut rng, random_gate_ident, gates);
            }
            val if val == "rz".to_string() => {
                gates = create_rotation_gate_element(&input, &mut rng, random_gate_ident, gates);
            }
            val if val == "ch".to_string() => {
                gates = create_controlled_gate_element(&input, &mut rng, random_gate_ident, gates);
            }
            val if val == "cnot".to_string() => {
                gates = create_controlled_gate_element(&input, &mut rng, random_gate_ident, gates);
            }
            val if val == "cz".to_string() => {
                gates = create_controlled_gate_element(&input, &mut rng, random_gate_ident, gates);
            }
            val if val == "mcz".to_string() => {
                let qc = &input.qc;
                let qubit_amount: u32 = rng.random_range(3..=input.qubits);
                let random_qubits = (0..input.qubits).sample(&mut rng, qubit_amount as usize);

                let noise_ident;
                if let Some(noises) = &input.noises {
                    let insert_noise = rng.random_bool(0.5);
                    if !insert_noise {
                        gates.push(quote! {
                            #qc.#random_gate_ident(&[#(#random_qubits),*]);
                        });
                    } else {
                        let noise_idx = rng.random_range(0..noises.len());
                        noise_ident = &noises[noise_idx].name;

                        match noise_ident.to_string() {
                            val if val == "depolarizing_noise".to_string() => {
                                gates.push(quote! {
                                    #qc.#random_gate_ident(&[#(#random_qubits),*]);
                                    #qc.#noise_ident()?;
                                });
                            }
                            val if val == "pauli_noise".to_string() => {
                                gates.push(quote! {
                                    #qc.#random_gate_ident(&[#(#random_qubits),*]);
                                    #qc.#noise_ident()?;
                                });
                            }
                            val if val == "bit_flip_noise".to_string() => {
                                gates.push(quote! {
                                    #qc.#random_gate_ident(&[#(#random_qubits),*]);
                                    #qc.#noise_ident();
                                });
                            }
                            val if val == "phase_flip_noise".to_string() => {
                                gates.push(quote! {
                                    #qc.#random_gate_ident(&[#(#random_qubits),*]);
                                    #qc.#noise_ident();
                                });
                            }
                            val if val == "bit_phase_flip_noise".to_string() => {
                                gates.push(quote! {
                                    #qc.#random_gate_ident(&[#(#random_qubits),*]);
                                    #qc.#noise_ident();
                                });
                            }
                            _ => panic!("Invalid noise type"),
                        }
                    }
                } else {
                    gates.push(quote! {
                        #qc.#random_gate_ident(&[#(#random_qubits),*]);
                    });
                }
            }
            val if val == "swap".to_string() => {
                gates = create_controlled_gate_element(&input, &mut rng, random_gate_ident, gates);
            }
            val if val == "iswap".to_string() => {
                gates = create_controlled_gate_element(&input, &mut rng, random_gate_ident, gates);
            }
            val if val == "cr".to_string() => {
                let random_qubits = (0..input.qubits).sample(&mut rng, 2);
                let k: f64 = rng.random_range(1.0..=15.0);
                let qc = &input.qc;
                let noise_ident;
                if let Some(noises) = &input.noises {
                    let insert_noise = rng.random_bool(0.5);
                    if !insert_noise {
                        gates.push(quote! {
                            #qc.#random_gate_ident(#(#random_qubits),*, #k);
                        });
                    } else {
                        let noise_idx = rng.random_range(0..noises.len());
                        noise_ident = &noises[noise_idx].name;

                        match noise_ident.to_string() {
                            val if val == "depolarizing_noise".to_string() => {
                                gates.push(quote! {
                                    #qc.#random_gate_ident(#(#random_qubits),*, #k);
                                    #qc.#noise_ident()?;
                                });
                            }
                            val if val == "pauli_noise".to_string() => {
                                gates.push(quote! {
                                    #qc.#random_gate_ident(#(#random_qubits),*, #k);
                                    #qc.#noise_ident()?;
                                });
                            }
                            val if val == "bit_flip_noise".to_string() => {
                                gates.push(quote! {
                                    #qc.#random_gate_ident(#(#random_qubits),*, #k);
                                    #qc.#noise_ident();
                                });
                            }
                            val if val == "phase_flip_noise".to_string() => {
                                gates.push(quote! {
                                    #qc.#random_gate_ident(#(#random_qubits),*, #k);
                                    #qc.#noise_ident();
                                });
                            }
                            val if val == "bit_phase_flip_noise".to_string() => {
                                gates.push(quote! {
                                    #qc.#random_gate_ident(#(#random_qubits),*, #k);
                                    #qc.#noise_ident();
                                });
                            }
                            _ => panic!("Invalid noise type"),
                        }
                    }
                } else {
                    gates.push(quote! {
                        #qc.#random_gate_ident(#(#random_qubits),*, #k);
                    });
                }
            }
            val if val == "measure".to_string() => {
                gates =
                    create_single_qubit_gate_element(&input, &mut rng, random_gate_ident, gates);
            }
            val if val == "measure_many".to_string() => {
                let qubit_amount = rng.random_range(2..=input.qubits);
                let random_qubits = (0..input.qubits).sample(&mut rng, qubit_amount as usize);
                let qc = &input.qc;
                gates.push(quote! {
                    #qc.#random_gate_ident(&[#(#random_qubits),*])
                });
            }
            val if val == "measure_all".to_string() => {
                let qc = &input.qc;
                gates.push(quote! {
                    #qc.#random_gate_ident();
                });
            }
            _ => panic!("invalid gate"),
        }
    }

    quote! {
        #(#gates)*
    }
    .into()
}
