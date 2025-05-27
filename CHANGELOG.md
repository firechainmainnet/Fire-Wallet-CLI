# 📜 CHANGELOG — FireChain CLI

Evolução técnica, estratégica e arquitetural da FireChain CLI.  
Desde o início, o projeto foi concebido com foco em modularidade, segurança real, CLI-first UX e princípios sólidos de engenharia de software.

> A partir da versão `v0.1.1`, a FireChain CLI consolida sua transição de ferramenta de prototipação avançada para **infraestrutura empresarial de custódia Web3**.

Todas as versões abaixo refletem **marcos técnicos reais** no avanço da arquitetura.

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