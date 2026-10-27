import glob

files = glob.glob("crates/neuropulse-core/src/analytics/*.rs")

for filepath in files:
    with open(filepath, "r") as f:
        content = f.read()
    
    # Ajustar ambiguidades comuns em tipos float no Rust
    content = content.replace(".max(0.0)", ".max(0.0f64)")
    content = content.replace("(1.0 + variance).ln()", "(1.0f64 + variance).ln()")
    content = content.replace(".round() / 100.0", ".round() / 100.0f64")
    
    with open(filepath, "w") as f:
        f.write(content)

print("Tipos literais float ajustados nos algoritmos analiticos.")
