use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("guid_lookup.rs");

    let output = fs::read_to_string("src/GUID_table.tsv")
        .expect("No GUI_table.tsv file found")
        .trim()
        .split('\n')
        .map(|line| {
            format!(
                "{:?}",
                line.split('\t')
                    .map(|item| item.trim().to_owned())
                    .collect::<Vec<String>>()
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(
        &dest_path,
        format!("guid_lookup! {{\n\
            {}\n\
        }}", output)
    ).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/GUID_table.tsv");
}
