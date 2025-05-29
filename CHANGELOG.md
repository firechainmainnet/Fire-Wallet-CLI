# 📜 CHANGELOG — FireChain CLI

Evolução técnica, estratégica e arquitetural da FireChain CLI.  
Desde o início, o projeto foi concebido com foco em modularidade, segurança real, CLI-first UX e princípios sólidos de engenharia de software.

> A partir da versão `v0.1.1`, a FireChain CLI consolida sua transição de ferramenta de prototipação avançada para **infraestrutura empresarial de custódia Web3**.

Todas as versões abaixo refletem **marcos técnicos reais** no avanço da arquitetura.

Todos os updates importantes neste projeto serão documentados aqui.

O formato é baseado em [Keep a Changelog](https://keepachangelog.com/pt-BR/1.0.0/)
e segue os princípios de versionamento semântico ([SemVer](https://semver.org/lang/pt-BR/)).

---

## [v0.1.3] - 2025-05-29

### 🔐 Segurança e Criptografia Reforçada
- 🔑 Adicionada criptografia client-side com AES-256-GCM e autenticação por HMAC-SHA256
- 🧠 Proteção obrigatória por senha via `--password`, utilizando derivação Argon2id com parâmetros ajustáveis
- ♻️ Aplicado Zeroizing para apagar da memória dados sensíveis como chaves privadas e senhas derivadas
- ✅ Todos os dados são serializados com `serde_json` e criptografados localmente sem dependência externa

### 💾 Persistência Inteligente e Compatível
- 📁 Novo sistema de nomeação automática: se `--out` for omitido, o arquivo `.wallet` é salvo como `<fingerprint>.wallet`, garantindo unicidade e rastreabilidade
- 🔄 Extensão `.wallet` adotada como padrão interoperável com futuros carregadores de DEX, scripts e ferramentas self-custodial
- 🧱 Campo `--out` reestruturado para `Option<String>`, permitindo flexibilidade máxima sem quebra de compatibilidade

### 🌈 UX Premium de Linha de Comando
- 🎨 Terminal com visual aprimorado: `.bold()`, `.dimmed()`, `.green()`, `.yellow()` e mensagens contextuais
- 📦 Exibição organizada da fingerprint, chaves e endereços (BTC, ETH, FireChain) com ícones por tipo
- ✅ UX 100% sem ruído: mensagens só aparecem quando necessárias e sempre com semântica clara

### 🧱 Refatorações Técnicas de Arquitetura
- 🧠 Atualização completa em `parser.rs` e `handler.rs` com separação de responsabilidades e lógica declarativa
- 📦 Removido fallback legado de `wallet.fire` para abrir caminho ao modelo baseado em fingerprint
- 🔍 Modularização e comentários no código para maior manutenibilidade e onboarding de contribuidores

---

🔄 Esta atualização marca a consolidação da FireChain CLI como ferramenta segura, interoperável e pronta para integração em pipelines Web3 e validações em produção.

---

## [v0.1.3] - 2025-05-28

### ✨ Adicionado
- Comando `derive` com suporte completo aos formatos de endereço:
  - **₿ Bitcoin (Base58Check, prefixo 0x00, compatível com P2PKH)**.
  - **⛓️ Ethereum (últimos 20 bytes do Keccak-256 da chave pública sem prefixo 0x04)**.
  - **🔥 FireChain (base58 com checksum SHA-256 + prefixo `f1r3`)**.
- Campo `fingerprint` derivado do SHA-256 da chave pública (6 bytes iniciais) + ID no formato `FC-<FINGERPRINT>`.
- CLI modular separada em `cli/parser.rs` e `cli/handler.rs`, elevando coesão e escalabilidade.
- Estilização premium de terminal com suporte a cores (`termcolor`) e emojis intuitivos.
- Novos testes automatizados cobrindo funcionalidades críticas.

### 🧠 Alterado
- **Remoção do campo `address` da estrutura `Wallet`**.
  - A geração de endereço agora é externa e modular (via `derive` e `address.rs`).
- **Refatoração completa de `main.rs`**:
  - Simplificação da execução com `.parse()` e encaminhamento de comando.
- `cli.rs` agora é dividido entre parsing (`parser.rs`) e execução (`handler.rs`), seguindo padrão Command Handler.

### ✅ Testes
- `tests/wallet_tests.rs`: Garante geração consistente de chaves privadas/públicas via `secp256k1`.
- `tests/derive_tests.rs`: Valida a integridade dos formatos BTC, ETH e F1R3 com entradas hexadecimais.

### 🛠️ Infraestrutura
- Atualização de `Cargo.toml` com dependências essenciais:
  - [`clap`](https://docs.rs/clap/4.0.0) (`Parser` para linha de comando)
  - [`termcolor`](https://docs.rs/termcolor/) (colorização cross-platform para terminal)
  - [`bs58`](https://docs.rs/bs58/) (codificação Base58)
  - `hex`, `secp256k1`, `rand` (cripto e geração de chave)
- Build 100% funcional com `cargo build --release` e `cargo install --path .`

### 📁 Estrutura Final de Diretórios

```bash
src/
├── bin/
│   └── main.rs              🚀 Ponto de entrada
├── cli/
│   ├── mod.rs               📦 Declaração CLI
│   ├── parser.rs            🧭 CLI com `clap`
│   ├── handler.rs           🧠 Lógica por comando
│   └── derive.rs            📡 Comando de derivação de endereços
├── core/
│   ├── mod.rs               📦 Núcleo lógico da FireChain
│   ├── wallet.rs            🔐 Geração e gestão de carteiras
│   ├── address.rs           🔗 Hashes e formatação de endereços
│   └── crypto.rs            ♻️  Hashing centralizado (keccak, ripemd, sha256)
├── utils/
│   └── format.rs            🧰 Estilo de saída e termcolor
├── tests/
│   ├── wallet_tests.rs      🧪 Testes de geração de carteira
│   └── derive_tests.rs      🧪 Testes de formatação de endereço
├── lib.rs                   🔁 Conecta todos os módulos
```
---

## [v0.1.2] — 2025-05-27

> Redefinimos o formato de endereçamento FireChain com foco em integridade criptográfica, legibilidade institucional e interoperabilidade de nível enterprise. Esta versão fortalece a identidade técnica da FireChain CLI, prepara a base para integrações futuras e posiciona o endereço FireChain como um padrão seguro e auditável.

---

### 🔗 Destaques Estratégicos

- **Novo padrão de endereço FireChain (`f1r3`)**
  - Redesenho completo do esquema de endereçamento com foco em segurança, leitura e brand awareness.
  - Prefixo simbólico `"f1r3"` com base em Base58Check e binário `0x77`, removendo separadores e ambiguidades visuais.

- **Segurança com significado: Prefixo `0x77`**
  - Binário `0x77` carrega significados de espiritualidade (7), multiplicação (x) e soberania digital — reforçando identidade e segurança.

- **Cobertura de testes 100% com E2E real**
  - Atualização completa dos testes `address_formats.rs`, `wallet_generate.rs` e `cli_generate.rs` com validações reais em produção.

- **Atualização institucional do README**
  - Exemplo de saída CLI reformulado, emojis removidos das partes técnicas, e foco em confiabilidade e visual profissional mantido.

---

### 🧱 Mudanças Técnicas

#### ✅ Adicionado
- Novo formato de endereço FireChain com `Base58Check` e prefixo `0x77`.
- Validação de comprimento base58 e prefixo textual (`f1r3`) em testes.
- Changelog institucional baseado em arquitetura e branding técnico.

#### 🔁 Alterado
- `address.rs`: Derivação de endereço FireChain refatorada para gerar `f1r3<base58check>` limpo.
- `wallet.rs`: Novo padrão FireChain aplicado nas funções principais de geração.
- Testes `wallet_generate` e `cli_generate` adaptados ao novo formato.

#### 🧹 Removido
- Uso do separador `:` no endereço FireChain.
- Emojis em trechos de código e validação (preservados apenas no layout visual).

#### 🛠 Corrigido
- Inconsistência no teste anterior que esperava prefixo `f1r3:` e sufixo com símbolos.

#### 🧬 Auditoria e Design
- Nova convenção de endereço reforça alinhamento com segurança e design técnico.
- Arquitetura interna do `README` e dos testes reforçada para refletir visão institucional.

---

### 📈 Recomendação de Upgrade

Todos os usuários devem atualizar para `v0.1.2` o quanto antes.  
Essa versão estabelece um novo padrão de endereço FireChain estável e auditável, fundamental para integração com:

- Multisig determinístico
- Exportação segura de `.wallet`
- Modo STDIN/JSON
- Ledger/Trezor/WebUSB

---

## [v0.1.1] — 2025-05-26

### 🔒 Marco Enterprise: Modelo Comercial Consolidado

#### 🔧 Arquitetura consolidada:
- CLI estável com derivação multi-chain: BTC (P2PKH), ETH (Keccak + checksum), FireChain (`f1r3:` com digest composto)
- Fingerprint exclusivo ativo por design, com SHA2 → RIPEMD160 → Base58
- Modularidade isolada por responsabilidade (`wallet.rs`, `address.rs`, `cli.rs`)

#### 📈 Expansão institucional:
- Estrutura de licenciamento formalizada (MIT + Comercial)
- Planos segmentados: Dev Solo, Cursos, Elite, Starter, Enterprise
- Curso mentorado: acesso direto à base arquitetural com mentoria ativa

#### 🔐 Segurança e Responsabilidade:
- Bug Bounty ativo com recompensa em FIRE + canal de reporte formal
- Política de uso seguro, SLA de correção e escopo de vulnerabilidade documentado

---

## [v0.1.0] — 2025-05-25

### 📦 Modularização avançada (internamente validada)
- Criação de `mod.rs` centralizando exportações
- Exportação direta de comandos via `pub use`, isolando lógica de CLI

### 📊 Design público
- Roadmap publicado com status por funcionalidade (entregue, roteirizado, P&D)
- Comparativo técnico de mercado com 5 carteiras CLI/GUI (Ledger, Metamask, Specter...)

---

## [v0.0.10] — 2025-05-25

### 🔐 Segurança avançada ativada
- `zeroize` aplicado em memória sensível (`private_key`, `seed`, `entropy`)
- Fingerprint exclusivo finalizado com hashing SHA256 do pubkey
- Detecção de colisão rara e handling silencioso implementado

---

## [v0.0.9] — 2025-05-24

### 🧠 UX de linha de comando refinada
- Migração completa para `clap` com derive + descrições longas
- Subcomandos integrados com nomenclatura intuitiva (`new`, `--unsafe-dump`, `--help`)

---

## [v0.0.8] — 2025-05-24

### 🔥 Suporte nativo à FireChain
- Endereço `f1r3:` derivado por hashing duplo + base58 + prefixo customizado
- Implementação de encoding próprio com checksum exclusivo FireChain

---

## [v0.0.7] — 2025-05-23

### ₿ Derivação Bitcoin P2PKH
- Conversão pubkey → SHA256 → RIPEMD160 → Base58Check com prefixo `0x00`
- Compatibilidade garantida com validadores externos (Electrum compat)

---

## [v0.0.6] — 2025-05-23

### 🧪 Derivação Ethereum (EIP-55)
- Geração de endereço `0x...` via Keccak-256 sobre pubkey
- Checksum adicionado com letras mistas (EIP-55)
- Derivação baseada em path HD padrão ETH

---

## [v0.0.5] — 2025-05-22

### 🧩 Pipeline unificado de derivação
- Criação de função genérica pubkey → address com adapter para cada chain
- Validação cruzada de fingerprint para prevenir duplicidade por algoritmo

---

## [v0.0.4] — 2025-05-22

### 🧱 Modularidade Base
- Separação dos motores CLI, wallet e address em arquivos independentes
- Organização semântica: derivação, hashing, exibição, criptografia

---

## [v0.0.3] — 2025-05-21

### 🧪 Testes automatizados e cobertura
- Configuração de `cargo llvm-cov` com cobertura em tempo real
- Introdução de testes CLI com `assert_cmd` + `predicates` + `float-cmp`

---

## [v0.0.2] — 2025-05-21

### 🧬 Primeira versão CLI funcional
- Comando `firechain-cli new` com geração multi-chain
- Output de chave privada, chave pública, endereços ETH, BTC, FIRE

---

## [v0.0.1] — 2025-05-20

### ⚙️ Genesis
- Criação do projeto com `cargo init`
- Inclusão de bibliotecas criptográficas core: `secp256k1`, `bs58`, `sha2`, `keccak`, `ripemd`, `rand`, `clap`
- Primeira execução funcional com `--help` e `--version`