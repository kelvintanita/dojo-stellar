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
   cd back-end
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

O servidor estará rodando em `http://localhost.:8080/`.

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
http://localhost.:8080/swagger-ui/#
```

Acesse esse link no navegador para explorar os endpoints disponíveis e testar requisições diretamente pela interface do Swagger.


---

# Front-End - Stellar Explorador de Blocos

Este repositório contém o **Front-End** de um explorador de blocos, transações e saldos (Stellar). Foi construído utilizando [React](https://react.dev/) + [Vite](https://vitejs.dev/) + [TypeScript](https://www.typescriptlang.org/) + [SWC](https://swc.rs/) para oferecer uma experiência de desenvolvimento rápida e tipada.

---

## Sumário

1. [Visão Geral](#visão-geral)
2. [Funcionalidades](#funcionalidades)
3. [Arquitetura e Organização](#arquitetura-e-organização)
4. [Como Executar](#como-executar)
5. [Scripts Disponíveis](#scripts-disponíveis)
6. [Problemas de CORS](#problemas-de-cors)
7. [Próximos Passos e Melhorias](#próximos-passos-e-melhorias)
8. [Licença](#licença)

---

## Visão Geral

O Front-End é responsável por consumir uma API (descrita em um repositório/README separado) para exibir:

- **Informações de Blocos** (busca por número do bloco)  
- **Dados de Transações** (busca por hash da transação)  
- **Saldos** de um endereço (busca por endereço Stellar)

Para tal, criamos três componentes/páginas principais (`BlockPage`, `TransactionPage`, e `BalancePage`), todos unificados em uma tela inicial (`HomePage`) com abas para facilitar a navegação.

---

## Funcionalidades

1. **Busca de Blocos**  
   - Inserir o número do bloco e exibir campos como `id`, `hash`, `sequence`, entre outros retornados pela API.

2. **Busca de Transações**  
   - Inserir o hash da transação e exibir detalhes específicos, como `id`, `paging_token`, `successful`, `ledger`, etc.

3. **Busca de Saldos**  
   - Inserir um endereço Stellar e exibir o saldo correspondente.

4. **Layout Simples**  
   - `MainLayout` que contém o cabeçalho (header), o conteúdo (`main`) e o rodapé (footer).  
   - Layout responsivo básico para preencher toda a tela.

5. **Estilização Básica**  
   - Uso de estilos inline (demonstrativo) para inputs maiores, botões, e textos claros em fundo escuro.

---

## Arquitetura e Organização

A estrutura de pastas no Front-End segue este modelo (simplificado):

```
front-end
├─ public/
├─ src/
│  ├─ assets/                 # Imagens, ícones, etc.
│  ├─ components/             # Componentes reutilizáveis
│  ├─ layouts/                # Layout Principal (MainLayout)
│  ├─ pages/                  # Páginas (BlockPage, TransactionPage, BalancePage, HomePage)
│  ├─ services/               # Funções de chamada à API
│  ├─ types/                  # (opcional) Definições de tipos
│  ├─ App.tsx                 # Componente raiz
│  ├─ main.tsx                # Ponto de entrada React/Vite
│  └─ index.css               # CSS global
├─ index.html
├─ tsconfig.json
├─ package.json
├─ vite.config.ts
└─ README.md                  
```

### Principais Arquivos

- **`main.tsx`**: Ponto de entrada, monta o React no DOM.
- **`App.tsx`**: Contém o roteamento ou, em alguns casos, apenas importa o `MainLayout` e exibe o `HomePage`.
- **`MainLayout.tsx`**: Layout base com header, main e footer.
- **`HomePage.tsx`**: Página inicial.  
  - Possui **abas** para trocar entre `BlockPage`, `TransactionPage` e `BalancePage`.
- **`BlockPage.tsx`** / `TransactionPage.tsx` / `BalancePage.tsx`  
  - Exibem os formulários de busca e a renderização de dados retornados pela API.
- **`services/api.ts`**: Funções de fetch (ex.: `fetchBlock`, `fetchTransaction`, `fetchBalance`).

---

## Como Executar

1. **Instale as dependências**:

   ```bash
   npm install
   ```
   ou  
   ```bash
   yarn
   ```

2. **Rode em modo de desenvolvimento**:

   ```bash
   npm run dev
   ```
   O Vite iniciará um servidor em [http://127.0.0.1:5173](http://127.0.0.1:5173) (ou similar). Abra a URL no navegador.

3. **Crie a build de produção** (opcional):

   ```bash
   npm run build
   ```
   Isso gera uma pasta `dist/` com arquivos prontos para deploy.

4. **Visualize a build** (opcional):

   ```bash
   npm run preview
   ```
   Serve localmente a pasta `dist/` para inspecionar o resultado final.

---

## Scripts Disponíveis

No `package.json`, temos:

- **`npm run dev`**: Inicia o servidor de desenvolvimento com hot reload.  
- **`npm run build`**: Gera a build de produção.  
- **`npm run preview`**: Servidor local que mostra a build de produção gerada.  

*(Se você estiver usando `yarn`, basta trocar `npm run X` por `yarn X`.)*

---

## Problemas de CORS

Se ocorrerem erros de **CORS** ao fazer requisições, é preciso:

- **Configurar o cabeçalho `Access-Control-Allow-Origin`** no back-end.  
  OU  
- **Criar um proxy local** no `vite.config.ts` para redirecionar as chamadas e evitar bloqueios do navegador.

Exemplo de configuração de proxy no `vite.config.ts`:

```ts
export default defineConfig({
  server: {
    proxy: {
      '/api': {
        target: 'https://sua-api.com',
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api/, ''),
      },
    },
  },
});
```

---

## Próximos Passos e Melhorias

1. **Estilização Aprimorada**: migrar de estilos inline para:
   - CSS Modules, Styled Components, TailwindCSS, Material UI, etc.
2. **Gerenciador de Estado**: se o projeto crescer, considerar Redux, Zustand ou Recoil.
3. **Testes**: adicionar Jest + Testing Library para cobrir componentes e funcionalidades.
4. **Rotas Separadas** (opcional): caso deseje acesso direto via URL `/block`, `/transaction`, `/balance`, utilize o React Router e defina `<Routes>` no `App.tsx`.
5. **Responsividade Avançada**: incluir media queries ou breakpoints customizados para melhorar a experiência em telas menores.

---

## Considerações
Este projeto foi desenvolvido para estudos e pode ser expandido conforme necessário para atender novas funcionalidades. Contribuições são bem-vindas!

---

## Licença

Este projeto é livre para estudo e uso. Sinta-se à vontade para adaptar o texto conforme a **licença** que você deseja adotar (MIT, Apache, etc.) ou remover esta seção se for um projeto privado.

---
