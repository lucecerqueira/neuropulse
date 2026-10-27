path = "crates/neuropulse-server/src/app_router.rs"
with open(path, "r") as f:
    content = f.read()

content = content.replace('"../../templates/', '"../../../templates/')

with open(path, "w") as f:
    f.write(content)

print("Caminhos dos templates ajustados para ../../../templates/ com sucesso.")
