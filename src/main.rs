use fastnbt::{to_writer, Value};
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;

fn prompt_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

fn main() -> io::Result<()> {
    let size_x: i32 = prompt_input("Enter size_x: ").parse().unwrap_or(64);
    let size_y: i32 = prompt_input("Enter size_y: ").parse().unwrap_or(1);
    let size_z: i32 = prompt_input("Enter size_z: ").parse().unwrap_or(64);
    let block_name = prompt_input("Enter block name (e.g., minecraft:netherrack):");

    let mut blocks = vec![];
    for y in 0..size_y {
        for z in 0..size_z {
            for x in 0..size_x {
                blocks.push(Value::Compound(
                    vec![
                        (
                            "pos".to_string(),
                            Value::List(vec![Value::Int(x), Value::Int(y), Value::Int(z)]),
                        ),
                        ("state".to_string(), Value::Int(0)),
                    ]
                    .into_iter()
                    .collect(),
                ));
            }
        }
    }

    let structure = Value::Compound(
        vec![
            (
                "size".to_string(),
                Value::List(vec![
                    Value::Int(size_x),
                    Value::Int(size_y),
                    Value::Int(size_z),
                ]),
            ),
            (
                "palette".to_string(),
                Value::List(vec![Value::Compound(
                    vec![("Name".to_string(), Value::String(block_name.clone()))]
                        .into_iter()
                        .collect(),
                )]),
            ),
            ("paletteMax".to_string(), Value::Int(1)),
            ("entities".to_string(), Value::List(vec![])),
            ("blocks".to_string(), Value::List(blocks)),
            ("DataVersion".to_string(), Value::Int(3465)),
        ]
        .into_iter()
        .collect(),
    );

    let file_name = format!(
        "{}_structure_{}x{}x{}.nbt",
        block_name.replace("minecraft:", ""),
        size_x,
        size_y,
        size_z
    );

    let path = Path::new(&file_name);
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    let mut gz = GzEncoder::new(writer, Compression::default());
    to_writer(&mut gz, &structure).unwrap();

    println!("File saved as {}", file_name);
    Ok(())
}
