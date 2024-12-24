use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};

#[derive(Debug)]
struct Gate<'a> {
    logic: &'a str,
    input1: &'a str,
    input2: &'a str,
    output: &'a str,
}

fn parse_gate(line: &str) -> Option<Gate<'_>> {
    let mut parts = line.split(' ');
    let input1 = parts.next()?;
    let logic = parts.next()?;
    let input2 = parts.next()?;
    let _ = parts.next()?;
    let output = parts.next()?;

    Some(Gate {
        logic,
        input1,
        input2,
        output,
    })
}

fn is_connected_to_inputs<'a>(
    gate: &'a Gate,
    gates: &'a [Gate],
    visited: &mut HashSet<&'a str>,
    x_input: &str,
    y_input: &str,
) -> bool {
    // If we've already visited this gate, skip it to avoid cycles
    if !visited.insert(gate.output) {
        return false;
    }

    // Direct connection to our input pair
    if gate.input1 == x_input
        || gate.input1 == y_input
        || gate.input2 == x_input
        || gate.input2 == y_input
    {
        return true;
    }

    // Check connections through other gates
    gates
        .iter()
        .filter(|g| g.output == gate.input1 || g.output == gate.input2)
        .any(|g| is_connected_to_inputs(g, gates, visited, x_input, y_input))
}

fn sort_inputs(inputs: &mut [String]) {
    inputs.sort_by(|a, b| {
        // Sort by the number after the 'x' or 'y'
        let a_num = a.strip_prefix('x').or_else(|| a.strip_prefix('y')).unwrap();
        let b_num = b.strip_prefix('x').or_else(|| b.strip_prefix('y')).unwrap();

        // If the numbers are equal, sort by the character before the number
        a_num
            .parse::<usize>()
            .unwrap()
            .cmp(&b_num.parse::<usize>().unwrap())
            .then(a.chars().next().unwrap().cmp(&b.chars().next().unwrap()))
    });
}

fn sort_outputs(outputs: &mut [String]) {
    outputs.sort_by(|a, b| {
        a[1..]
            .parse::<usize>()
            .unwrap()
            .cmp(&b[1..].parse::<usize>().unwrap())
    });
}

fn write_ranked_nodes<S: AsRef<str>>(dot: &mut String, nodes: &[S]) {
    dot.push_str("    { rank=same;\n");

    dot.push_str("    ");
    for (i, node) in nodes.iter().enumerate() {
        if i > 0 {
            dot.push_str(" -> ");
        }
        dot.push_str(node.as_ref());
    }
    dot.push_str(" [style=invis];\n");
    dot.push_str("    rankdir=LR;\n");

    dot.push_str("    }\n");
}

fn write_gates<'a>(dot: &mut String, gates: impl Iterator<Item = &'a Gate<'a>>) {
    for gate in gates {
        dot.push_str(&format!(
            r#"    "{}" [label="{} {}"];"#,
            gate.output, gate.logic, gate.output
        ));
        dot.push('\n');
        dot.push_str(&format!(r#"    "{}" -> "{}";"#, gate.input1, gate.output));
        dot.push('\n');
        dot.push_str(&format!(r#"    "{}" -> "{}";"#, gate.input2, gate.output));
        dot.push('\n');
    }
}

fn generate_subgraph_dot(gates: &[Gate], bit: usize) -> String {
    let mut dot = String::new();
    dot.push_str("digraph {\n");

    // Input pair nodes
    let x_input = format!("x{:02}", bit);
    let y_input = format!("y{:02}", bit);
    write_ranked_nodes(&mut dot, &[&x_input, &y_input]);

    // Find connected gates
    let connected_gates: Vec<&Gate> = gates
        .iter()
        .filter(|gate| {
            let mut visited = HashSet::new();
            is_connected_to_inputs(gate, gates, &mut visited, &x_input, &y_input)
        })
        .collect();

    let mut outputs: Vec<_> = connected_gates
        .iter()
        .map(|gate| gate.output.to_string())
        .filter(|output| output.starts_with('z'))
        .collect();
    sort_outputs(&mut outputs);
    write_ranked_nodes(&mut dot, &outputs);

    write_gates(&mut dot, connected_gates.into_iter());

    dot.push('}');
    dot
}

fn generate_dot(gates: &[Gate]) -> String {
    let mut dot = String::new();
    dot.push_str("digraph {\n");

    let mut inputs: Vec<_> = gates
        .iter()
        .flat_map(|gate| [gate.input1.to_string(), gate.input2.to_string()])
        .filter(|input| input.starts_with('x') || input.starts_with('y'))
        .unique()
        .collect();
    sort_inputs(&mut inputs);
    write_ranked_nodes(&mut dot, &inputs);
    let mut outputs: Vec<_> = gates
        .iter()
        .map(|gate| gate.output.to_string())
        .filter(|output| output.starts_with('z'))
        .collect();
    sort_outputs(&mut outputs);
    write_ranked_nodes(&mut dot, &outputs);

    write_gates(&mut dot, gates.iter());

    dot.push('}');
    dot
}

fn find_outputs<'a>(
    gates: &'a [Gate],
    input1: &'a str,
    input2: &'a str,
) -> impl Iterator<Item = &'a Gate<'a>> {
    gates.iter().filter(move |gate| {
        (gate.input1 == input1 && gate.input2 == input2)
            || (gate.input1 == input2 && gate.input2 == input1)
    })
}

fn find_gates_with_input<'a>(
    gates: &'a [Gate],
    input: &'a str,
) -> impl Iterator<Item = &'a Gate<'a>> {
    gates
        .iter()
        .filter(move |gate| gate.input1 == input || gate.input2 == input)
}

fn check_full_adder(gates: &[Gate], bit: usize) -> bool {
    let x_input = format!("x{:02}", bit);
    let y_input = format!("y{:02}", bit);
    let sum_out = format!("z{:02}", bit);

    // Step 1: Find initial XOR and AND gates connected to x and y
    let first_and_gates: Vec<_> = find_outputs(gates, &x_input, &y_input)
        .filter(|gate| gate.logic == "AND")
        .collect();
    let first_xor_gates: Vec<_> = find_outputs(gates, &x_input, &y_input)
        .filter(|gate| gate.logic == "XOR")
        .collect();

    // Verify we have exactly one of each
    if first_and_gates.len() != 1 || first_xor_gates.len() != 1 {
        println!(
            "Invalid number of initial gates for bit {}: {} AND, {} XOR",
            bit,
            first_and_gates.len(),
            first_xor_gates.len()
        );
        return false;
    }

    let first_and = &first_and_gates[0];
    let first_xor = &first_xor_gates[0];

    // Step 2: Find XOR and AND gates that take the first XOR output as input
    let connected_to_first_xor: Vec<_> = find_gates_with_input(gates, first_xor.output).collect();

    let second_xor_gates: Vec<_> = connected_to_first_xor
        .iter()
        .filter(|gate| gate.logic == "XOR")
        .collect();
    let second_and_gates: Vec<_> = connected_to_first_xor
        .iter()
        .filter(|gate| gate.logic == "AND")
        .collect();

    if second_xor_gates.len() != 1 || second_and_gates.len() != 1 {
        println!(
            "Invalid number of secondary gates for bit {}: {} AND, {} XOR",
            bit,
            second_and_gates.len(),
            second_xor_gates.len()
        );
        return false;
    }

    let second_and = &second_and_gates[0];
    let second_xor = &second_xor_gates[0];

    // Step 3: Find OR gate that combines the AND outputs
    let or_gates: Vec<_> = find_outputs(gates, first_and.output, second_and.output)
        .filter(|gate| gate.logic == "OR")
        .collect();

    if or_gates.len() != 1 {
        println!(
            "Invalid number of OR gates for bit {}: {}",
            bit,
            or_gates.len()
        );
        return false;
    }

    // Step 4: Verify that the second XOR produces the sum output
    if second_xor.output != sum_out {
        println!(
            "Invalid sum output for bit {}: expected {}, got {}",
            bit, sum_out, second_xor.output
        );
        return false;
    }

    // Step 5: Verify that the carry input is shared between the second XOR and AND
    let carry_in = if second_xor.input1 == first_xor.output {
        &second_xor.input2
    } else {
        &second_xor.input1
    };

    let second_and_carry = if second_and.input1 == first_xor.output {
        &second_and.input2
    } else {
        &second_and.input1
    };

    if carry_in != second_and_carry {
        println!(
            "Mismatched carry inputs: XOR uses {}, AND uses {}",
            carry_in, second_and_carry
        );
        return false;
    }

    true
}

pub fn solution(input: &str) {
    let (inputs, gates) = input.split_once("\n\n").unwrap();

    // Parse all our gates
    let gates = gates
        .lines()
        .map(parse_gate)
        .collect::<Option<Vec<_>>>()
        .unwrap();

    // Read the provided inputs
    let inputs = inputs
        .lines()
        .map(|line| {
            let (input, value) = line.split_once(": ")?;
            let value = match value {
                "0" => false,
                "1" => true,
                _ => return None,
            };
            Some((input, value))
        })
        .collect::<Option<Vec<_>>>()
        .unwrap();

    let mut values: HashMap<&str, bool> = inputs.into_iter().collect();

    loop {
        let mut changed = false;
        for gate in &gates {
            if let (Some(&input1), Some(&input2)) =
                (values.get(gate.input1), values.get(gate.input2))
            {
                let result = match gate.logic {
                    "AND" => input1 & input2,
                    "OR" => input1 | input2,
                    "XOR" => input1 ^ input2,
                    _ => panic!("Invalid logic"),
                };

                if values.insert(gate.output, result) != Some(result) {
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
    }

    let part1: u64 = values
        .iter()
        .filter_map(|(k, v)| {
            k.strip_prefix('z')
                .and_then(|suffix| suffix.parse::<usize>().ok())
                .map(|num| (num, *v))
        })
        .fold(
            0,
            |acc, (bit, value)| if value { acc | (1 << bit) } else { acc },
        );
    println!("Part 1: {}", part1);

    // We need to find the combination of swaps that will give us the correct output.
    // The graph forms a 44-bit adder, so we want every pair of bits to form a full adder circuit.
    // It's easier to do this visually. We can generate a graph of each bit and its connections
    // to the inputs, and visually inspect it to find incorrect full adder circuits.

    // Spit out obvious errors
    for bit in 0..45 {
        check_full_adder(&gates, bit);
    }

    // Create output dir
    std::fs::create_dir_all("day24-graphs").unwrap();

    // Generate the full graph
    let dot = generate_dot(&gates);
    std::fs::write("day24-graphs/full.dot", dot).unwrap();

    // Generate subgraphs for each bit
    for bit in 0..45 {
        println!("Generating subgraph for bit {}", bit);
        let dot = generate_subgraph_dot(&gates, bit);
        std::fs::write(format!("day24-graphs/bit{:02}.dot", bit), dot).unwrap();
    }

    // Render all graphs in parallel
    std::fs::read_dir("day24-graphs")
        .unwrap()
        .par_bridge()
        .for_each(|result| {
            let entry = result.unwrap();
            println!("Rendering graph {:?}", entry.file_name());
            std::process::Command::new("dot")
                .arg("-Tsvg")
                .arg("-o")
                .arg(format!(
                    "day24-graphs/{}.svg",
                    entry.file_name().to_string_lossy()
                ))
                .arg(entry.path())
                .output()
                .unwrap();
        })
}
