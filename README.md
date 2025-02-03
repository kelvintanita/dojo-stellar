# Dojo Stellar - Back-End

Este projeto implementa um back-end em Rust utilizando Actix-web e Utoipa para documentação de APIs. Ele expõe endpoints para interação com a blockchain Stellar.

## Requisitos

Certifique-se de ter os seguintes requisitos instalados antes de rodar o projeto:

- [Rust](https://www.rust-lang.org/) (Instale com `rustup`)
- [Cargo](https://doc.rust-lang.org/cargo/) (gerenciador de pacotes do Rust)
- [Docker](https://www.docker.com/) (caso queira rodar serviços auxiliares)

## Instalação

1. Clone este repositório:
   ```sh
   git clone https://github.com/kelvintanita/dojo-stellar.git
   cd dojo-stellar
   ```

2. Instale as dependências do projeto:
   ```sh
   cargo build
   ```

3. Crie um arquivo `.env` com as configurações necessárias (exemplo de variável usada no projeto):
   ```sh
   RPC_URL=http://34.60.10.29:8000/
   ```

## Executando o Servidor

Para iniciar o servidor de desenvolvimento, execute:
```sh
cargo run
```

O servidor estará rodando em `http://127.0.0.1:8080/`.

## Endpoints Principais

### Interação com a Blockchain Stellar
- `GET /block/{sequence}`
  - Obtém um bloco pelo número de sequência
- `GET /transaction/{hash}`
  - Obtém uma transação pelo hash
- `GET /balance/{account_id}`
  - Obtém o saldo de uma conta Stellar

## Documentação da API

Após iniciar o servidor com `cargo run`, a documentação Swagger estará disponível no seguinte endereço:
```
http://127.0.0.1:8080/swagger-ui/#/
```

Acesse esse link no navegador para explorar os endpoints disponíveis e testar requisições diretamente pela interface do Swagger.

## Considerações
Este projeto foi desenvolvido para estudos e pode ser expandido conforme necessário para atender novas funcionalidades. Contribuições são bem-vindas!

