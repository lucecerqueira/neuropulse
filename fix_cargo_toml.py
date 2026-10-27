# Fix root Cargo.toml
with open("Cargo.toml", "r") as f:
    lines = f.readlines()

new_root = [l for l in lines if 'default-run = "neuropulse-server"' not in l]
with open("Cargo.toml", "w") as f:
    f.writelines(new_root)

# Fix server Cargo.toml
server_path = "crates/neuropulse-server/Cargo.toml"
with open(server_path, "r") as f:
    s_lines = f.readlines()

if not any("default-run" in line for line in s_lines):
    new_server = []
    for line in s_lines:
        new_server.append(line)
        if line.strip() == "[package]":
            new_server.append('default-run = "neuropulse-server"\n')
    with open(server_path, "w") as f:
        f.writelines(new_server)

print("Cargo.toml ajustado com sucesso.")
