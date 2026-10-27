import os, glob

# Organizar handlers dentro de um modulo publico se necessario ou reexportar no lib.rs
lib_path = "crates/neuropulse-core/src/lib.rs"

lib_content = """pub mod domain;
pub mod handlers;

pub use domain::*;
pub use handlers::*;
"""

with open(lib_path, "w") as f:
    f.write(lib_content)

# Criar o mod.rs em handlers se nao existir
handlers_dir = "crates/neuropulse-core/src/handlers"
os.makedirs(handlers_dir, exist_ok=True)

# Mover arquivos *_handler.rs soltos em src/ para src/handlers/ caso estejam na raiz de src/
src_dir = "crates/neuropulse-core/src"
for f in glob.glob(f"{src_dir}/*_handler.rs"):
    filename = os.path.basename(f)
    os.rename(f, os.path.join(handlers_dir, filename))

# Gerar mod.rs dentro de handlers
handler_files = glob.glob(f"{handlers_dir}/*.rs")
mod_lines = []
for h in handler_files:
    name = os.path.basename(h).replace(".rs", "")
    if name != "mod":
        mod_lines.append(f"pub mod {name};")
        mod_lines.append(f"pub use {name}::*;")

with open(os.path.join(handlers_dir, "mod.rs"), "w") as f:
    f.write("\n".join(mod_lines) + "\n")

print("neuropulse-core lib.rs e handlers/mod.rs reestruturados com sucesso.")
