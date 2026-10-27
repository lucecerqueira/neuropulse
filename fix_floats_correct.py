import glob

files = glob.glob("crates/neuropulse-core/src/analytics/*.rs")

for filepath in files:
    with open(filepath, "r") as f:
        content = f.read()
    
    # Ajustar para f32 para bater com as structs das metricas
    content = content.replace("100.0f64", "100.0f32")
    content = content.replace(".max(0.0)", ".max(0.0f32)")
    content = content.replace("(1.0 + variance).ln()", "(1.0f32 + variance).ln()")
    
    with open(filepath, "w") as f:
        f.write(content)

print("Decimais convertidos para f32 com sucesso.")
