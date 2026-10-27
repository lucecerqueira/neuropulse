path = "crates/neuropulse-server/src/app_router.rs"
with open(path, "r") as f:
    content = f.read()

content = content.replace("use crate::domain", "use neuropulse_core::domain")
content = content.replace("use crate::handlers", "use neuropulse_core::handlers")

with open(path, "w") as f:
    f.write(content)

print("Importacoes do app_router.rs redirecionadas para neuropulse_core com sucesso.")
