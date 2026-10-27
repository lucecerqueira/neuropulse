with open("Cargo.toml", "r") as f:
    content = f.read()

if "default-run" not in content:
    lines = content.splitlines()
    new_lines = []
    for line in lines:
        new_lines.append(line)
        if line.strip() == "[workspace]":
            new_lines.append('default-run = "neuropulse-server"')
    
    with open("Cargo.toml", "w") as f:
        f.write("\n".join(new_lines) + "\n")
    print("default-run configurado com sucesso no Cargo.toml!")
else:
    print("default-run já configurado.")
