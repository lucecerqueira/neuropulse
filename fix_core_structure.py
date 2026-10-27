import os

# 1. Reescrever lib.rs limpo para neuropulse-core
lib_path = "crates/neuropulse-core/src/lib.rs"
lib_content = """pub mod analytics;
pub mod domain;

pub use analytics::*;
pub use domain::*;
"""
with open(lib_path, "w") as f:
    f.write(lib_content)

# 2. Garantir domain/mod.rs correto
domain_mod_path = "crates/neuropulse-core/src/domain/mod.rs"
os.makedirs("crates/neuropulse-core/src/domain", exist_ok=True)

# Copiar domain do projeto original de forma limpa se necessário
os.system("cp -r ~/Documents/Neurosync_platform_Rust/src/domain/* crates/neuropulse-core/src/domain/ 2>/dev/null || true")

# Reescrever domain/mod.rs com apenas o que existe
domain_files = [f.replace(".rs", "") for f in os.listdir("crates/neuropulse-core/src/domain") if f.endswith(".rs") and f != "mod.rs"]
domain_mod_content = "\n".join([f"pub mod {f};" for f in domain_files]) + "\n"

with open(domain_mod_path, "w") as f:
    f.write(domain_mod_content)

print("Estrutura do neuropulse-core sincronizada!")
