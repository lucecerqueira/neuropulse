path = "crates/neuropulse-server/Cargo.toml"
with open(path, "r") as f:
    content = f.read()

new_content = content.replace(
    'tower-http = { version = "0.5", features = ["cors"] }',
    'tower-http = { version = "0.5", features = ["cors", "fs"] }'
)

with open(path, "w") as f:
    f.write(new_content)

print("Feature 'fs' ativada no tower-http com sucesso.")
