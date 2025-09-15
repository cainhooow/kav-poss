# Kav-Poss

Kav é um sistema(back-end) de ponto de venda em desenvolvimento. É um sistema local ou baseado na web projetado para fornecer uma interface de vendas para pequenas (ou grandes) empresas

## Por que Rust?

Uma base do projeto foi desenvolvida usando `tauri`, com isso, para manter uma stack continua foi tomada a decisão de usar a linguagem rust. O kav-poss tem como objetivo de futuramente ser um sistema maior, usado na web ou em servidores locais.

## Requistos

- Rust && Rustc `1.89.0`
  - Base para rodar o projeto e compilação;
- Node/Bun;
  - Para futuras implementações/build de assets, etc;
- `10Gb` de espaço livre;

### **Visual Studio Code**

- Extensões recomendadas:
  - Dependi `*`
  - Even Better TOML
  - PowerShell
  - Rust Doc Viewer
  - Rust Mod Generator/Rust Automod
  - rust-analzyer `*`

- Extensões opcionais:
  - DevContainers

## Configuração Inicial

Para manter um ambiente de desenvolvimento sem conflitos
e clean, recomenda-se usar um container docker/dev containers

1. Copiar `.env.example` para .env

   ```sh
   cp .env.example .env
   ```

2. Configure o `.env` com a conexão com o banco de dados
e um `JWT_AUTH_SECRET`

   ```.env
   DATABASE_URL=driver://<user>:<password>@<host>:<port>/<schema>

   JWT_AUTH_SECRET=my_secret
   ```

3. Build do projeto(padrão com `rust-analyzer`)

   ```sh
   cargo build
   ```

4. Instalar o `sea-orm-cli` - [sea-orm](https://www.sea-ql.org/SeaORM/docs/generate-entity/sea-orm-cli/)

   ```sh
   cargo install sea-orm-cli@1.1.0
   ```

5. Rodar migrations

   ```sh
   sea-orm-cli migrate up
   ```
