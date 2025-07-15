use roqoqo::{
    Circuit,
    operations::{DefinitionBit, MeasureQubit, PauliX, CNOT},
};
use roqoqo::backends::EvaluatingBackend;
use roqoqo_quest::Backend;

fn main() {
    let input = "Hi";
    println!("Original: {input}");

    let bits = string_to_bits(input);
    let n_qubits = bits.len();

    // ---------- build the circuit ----------
    let mut circuit = Circuit::new();
    circuit += DefinitionBit::new("ro".into(), n_qubits, true);

    // encode bits
    for (i, bit) in bits.iter().enumerate() {
        if *bit == 1 {
            circuit += PauliX::new(i);
        }
    }

    // measure all qubits into register "ro"
    for i in 0..n_qubits {
        circuit += MeasureQubit::new(i, "ro".into(), i);
    }

    // ---------- simulate ----------
    let backend = Backend::new(n_qubits, None);               // <- extra arg
    let (bit_regs, _float_regs, _complex_regs) =
        backend.run_circuit(&circuit).expect("circuit run");

    // The Quest backend returns Vec<Vec<bool>> for each register; we take the first shot
    let measured: Vec<u8> = bit_regs["ro"][0]
        .iter()
        .map(|b| if *b { 1 } else { 0 })
        .collect();

    let decoded = bits_to_string(&measured);
    println!("Decoded : {decoded}");
}

// ---------- helpers ----------
fn string_to_bits(s: &str) -> Vec<u8> {
    s.bytes()
        .flat_map(|b| (0..8).rev().map(move |i| ((b >> i) & 1) as u8))
        .collect()
}

fn bits_to_string(bits: &[u8]) -> String {
    bits.chunks(8)
        .map(|chunk| {
            chunk.iter().fold(0u8, |acc, &b| (acc << 1) | b) as char
        })
        .collect()
}
