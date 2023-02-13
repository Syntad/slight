dev:
    cd ./crates/backend && cargo tauri dev

build:
    cd ./crates/backend && cargo tauri build

setup:
    npm i -g pnpm --silent
    cd ./crates/frontend && pnpm i

# Install package to frontend (pnpm)
if package:
    cd ./crates/frontend && pnpm i {{package}}

# Uninstall package from frontend
uf package:
    cd ./crates/frontend && pnpm uninstall {{package}}

# Install package to backend (cargo)
ib package:
    cd ./crates/backend && cargo add {{package}}

# Uninstall package from backend
ub package:
    cd ./crates/backend && cargo remove {{package}}