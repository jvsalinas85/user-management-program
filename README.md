# 🧩 Programa de Gestión de Usuarios en Solana / User Management Program on Solana

Este proyecto es un programa en Rust para la blockchain de Solana que permite gestionar perfiles de usuario. Usa Borsh para serialización, y puede interactuar con un cliente que ejecuta instrucciones como inicializar perfiles, saludar y actualizar mensajes.

This project is a Rust-based smart contract for the Solana blockchain that manages user profiles. It uses Borsh for serialization and can be interacted with via a client that sends instructions to initialize profiles, greet users, and update messages.

---

## 📦 Requisitos / Requirements

- ✅ Rust (>= 1.70)  
- ✅ Solana CLI (`solana --version` ≥ 1.18)  
- ✅ `cargo-build-sbf` (instalado con `cargo install --git https://github.com/solana-labs/solana --bin cargo-build-sbf`)  
- ✅ [Optional] Anchor (si decides migrar a Anchor)

---

## 🚀 Instrucciones para ejecutar localmente / How to Run Locally

### 🧪 1. Inicia el validador local / Start the local validator

```bash
solana-test-validator --reset
```

> Deja esta terminal abierta para mantener el nodo local corriendo.  
> Leave this terminal open to keep the local validator running.

---

### 🛠️ 2. Compila y despliega el programa / Build and deploy the program

```bash
cargo build-sbf
solana program deploy ./target/deploy/user-management-program.so
```

> Guarda el **program ID** que te devuelve este comando.  
> Keep the **program ID** returned by the deploy command.

---

### 📡 3. Configura Solana CLI en modo local / Set Solana CLI to local mode

```bash
solana config set --url http://localhost:8899
```

---

### 👨‍💻 4. Corre el cliente / Run the client

```bash
cd client/
cargo run
```

> Este cliente inicializa una cuenta, la rellena con datos y luego los muestra.  
> This client initializes a user account, fills it with data, and displays it.

---

## 🧠 Funcionalidades / Features

- 📥 Crear e inicializar un perfil de usuario  
- 🔁 Incrementar un contador de saludos  
- ✏️ Actualizar el mensaje de bienvenida

---

## 📂 Estructura del Proyecto / Project Structure

```
src/
├── instruction.rs       # Define las instrucciones disponibles
├── processor.rs         # Lógica principal del programa
├── state.rs             # Estructura de los datos almacenados
├── lib.rs               # Punto de entrada del programa
client/
├── main.rs              # Cliente que interactúa con el programa
```

---

## 🧾 Licencia / License

Este proyecto está bajo licencia MIT.  
This project is licensed under the MIT License.

---

Made with ❤️ using Rust and Solana.