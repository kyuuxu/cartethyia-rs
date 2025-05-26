use std::{
    fs,
    io::{self, BufRead},
    path::Path,
};

const CODEGEN_OUT_DIR: &str = "generated/";

pub fn main() {
    let _ = fs::create_dir(CODEGEN_OUT_DIR);

    let proto_file = "proto/internal.proto";
    if Path::new(&proto_file).exists() {
        println!("cargo:rerun-if-changed={proto_file}");

        prost_build::Config::new()
            .out_dir(CODEGEN_OUT_DIR)
            .type_attribute(".", "#[derive(wicked_waifus_protocol_derive::MessageID)]")
            .compile_protos(&[proto_file], &["internal"])
            .unwrap();

        impl_message_id(Path::new("generated/internal.rs")).unwrap();
    }

    let proto_file = "proto/data.proto";
    if Path::new(&proto_file).exists() {
        println!("cargo:rerun-if-changed={proto_file}");

        prost_build::Config::new()
            .out_dir(CODEGEN_OUT_DIR)
            .compile_protos(&[proto_file], &["data"])
            .unwrap();
    }
}

pub fn impl_message_id(path: &Path) -> io::Result<()> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut output = Vec::new();

    let mut attr = None;
    for line in reader.lines() {
        let line = line?;

        if line.contains("MessageId:") {
            attr = Some(make_message_id_attr(&line).unwrap());
        } else {
            output.push(line);
            if let Some(attr) = attr.take() {
                output.push(attr);
            }
        }
    }

    fs::write(path, output.join("\n").as_bytes())?;
    Ok(())
}

fn make_message_id_attr(line: &str) -> Option<String> {
    let id = line.trim_start().split(' ').nth(2)?.parse::<u16>().ok()?;
    Some(format!("#[message_id({id})]"))
}