use std::fs;

fn collect(dir: &str, out: &mut String) {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            collect(path.to_str().unwrap(), out);
        } else if path.extension().and_then(|e| e.to_str()) == Some("pest") {
            out.push_str(&fs::read_to_string(&path).unwrap());
            out.push_str("\n\n");
        }
    }
}

fn main() {
    let mut merged = String::new();

    collect("src/parser/grammar", &mut merged);

    fs::write("src/parser/grammar.pest", merged).unwrap();

    println!("cargo:rerun-if-changed=src/parser/grammar");
}
