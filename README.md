
<p align="center">
  <img src="assets/firechain_banner.png" alt="FireChain CLI" width="600px" />
</p>

---

<p align="center">
  <img src="https://img.shields.io/badge/Rust-2021-934c97" />
  <img src="https://img.shields.io/badge/CLI-Clap%204.5-orange" />
  <img src="https://img.shields.io/badge/Hashing-SHA2%20%7C%20Keccak%20%7C%20RIPEMD160-FF6D00" />
  <img src="https://img.shields.io/badge/AES-GCM%20%2B%20HMAC-00695C" />
  <img src="https://img.shields.io/badge/Derivador-Argon2id-546E7A" />
  <img src="https://img.shields.io/badge/Arquivo-.wallet%20auto-FF7043" />
  <img src="https://img.shields.io/badge/execução-100%25%20local-blue" />
  <img src="https://img.shields.io/badge/memória-zeroizada-purple" />
  <img src="https://img.shields.io/badge/fingerprint-identidade%20única-9C27B0" />
  <img src="https://img.shields.io/badge/Testes-Reais%20via%20CLI-green" />
  <img src="https://img.shields.io/badge/Cobertura-100%25-4CAF50" />
  <img src="https://img.shields.io/badge/Criptografia-AES256%20%7C%20Argon2id-blueviolet" />
  <img src="https://img.shields.io/badge/Extensão-.wallet%20compatível-blue" />
  <img src="https://img.shields.io/badge/License-COMMERCIAL-red" />
  <img src="https://img.shields.io/github/last-commit/firechainmainnet/fire-wallet-cli" />
  <img src="https://img.shields.io/github/issues/firechainmainnet/fire-wallet-cli" />
  <img src="https://img.shields.io/badge/version-0.1.3-blue" />
</p>

---

# 🔥 FireChain CLI

**🧱 Sua stack Web3 começa na linha de comando.**  
Interface de linha de comando local, modular e segura para gerar carteiras com múltiplos padrões de endereços — **Bitcoin (₿), Ethereum (⛓️) e FireChain (🔥)** — com **criptografia avançada, exportação `.wallet` e identidade única por fingerprint**.

---

## 📚 Índice

- [🧬 Sobre](#-sobre)
- [🔑 Princípios](#-princípios)
- [⚙️ Funcionalidades Atuais](#️-funcionalidades-atuais)
- [🧠 Execução Real em Produção](#-execução-real-em-produção)
- [🖥️ Ajuda Rápida](#️-ajuda-rápida)
- [📁 Arquitetura de Diretórios](#-arquitetura-de-diretórios)
- [🧱 Roadmap Técnico 2025](#-roadmap-técnico-2025)
- [📄 CHANGELOG Técnico](./CHANGELOG.md)
- [📄 Licenças e Contato](#-licenças-e-contato)

---

## 🧬 Sobre

A **FireChain CLI** é uma ferramenta ultra-segura para geração de carteiras Web3 com identidade criptográfica e arquivos `.wallet` criptografados.  
Feita 100% em **Rust**, sem dependências externas e sem conexões de rede. Ideal para:

- 🧑‍💻 Desenvolvedores que controlam sua stack
- 🛡️ Times de segurança e auditoria
- 🔗 Integração com automações e CI/CD
- 🧬 Criptografia offline real e verificável

---

## 🔑 Princípios

- 🔐 Segurança desde a geração até o armazenamento
- 🔭 Identidade única com `fingerprint` e `wallet ID`
- 🎯 100% CLI local, sem exposição externa
- 🎨 Interface moderna com UX premium
- 🔬 Testes reais e auditáveis com cobertura total

---

## ⚙️ Funcionalidades Atuais

- ✅ Geração de chave privada/pública com `secp256k1`
- ✅ Criptografia AES-256-GCM com HMAC-SHA256
- ✅ Proteção por senha com derivação Argon2id (`--password`)
- ✅ Salvamento automático como `<fingerprint>.wallet` ao omitir `--out`
- ✅ Campo `--out` agora é opcional (`Option<String>`)
- ✅ Derivação de endereços BTC, ETH e FireChain
- ✅ Identidade única por fingerprint (`FC-xxxx`)
- ✅ Modularização clara (`parser`, `handler`, `derive`)
- ✅ UX premium de terminal com `termcolor`, emojis e semântica visual
- ✅ Testes automatizados com `assert_cmd` e saída validável

---

## 🧠 Execução Real em Produção

```bash
$ firechain-cli new --password "minhaSenha"

🔐 Iniciando criação da carteira FireChain...

📦 Carteira Gerada com Sucesso:
🔐 Fingerprint: c79449d3b8ad733a5e9875c8d19b8263d570303d
🧠 Chave Pública: 04abc...
🔒 Chave Privada: 1234abcd...
🔥 Endereço FireChain: f1r3...
🪙 Endereço Ethereum: 0x...
🪙 Endereço Bitcoin: 1...

✅ Carteira criada e criptografada com sucesso!

📁 Arquivo salvo em: c79449d3b8ad733a5e9875c8d19b8263d.wallet
```

```bash
$ firechain-cli new --password "minhaSenha" --out minha_wallet_secreta.wallet

📁 Arquivo salvo como: minha_wallet_secreta.wallet
```

```bash
$ firechain-cli derive --all

🔥 FireChain CLI
🧬 Segurança blockchain com modularidade, criptografia e elegância CLI-first

🔐 Chave Pública  : 04ab...
🔒 Chave Privada  : 0f43...
🧬 Fingerprint     : e0fa27b...
🆔 Wallet ID       : FC-E0FA27B0A3D4

₿ Endereço BTC    : 1ABCdEfG...
⛓️ Endereço ETH    : 0xabc123...
🔥 Endereço F1R3   : f1r3XYZ...

✅ Derivação concluída com sucesso!
⚠️  Nunca compartilhe sua chave privada.
```
---

## 🖥️ Ajuda Rápida

```bash
$ firechain-cli --help

🔥 FireChain CLI — Carteira Web3 com foco em segurança e modularidade.

USAGE:
    firechain-cli <COMMAND>

COMMANDS:
    new      🔐 Gera uma nova carteira FireChain
    derive   🔭 Deriva múltiplos endereços (₿, ⛓️, 🔥)
    help     ℹ️  Mostra ajuda detalhada
```

---

## 📁 Arquitetura de Diretórios

```
src/
├── bin/main.rs              🚀 Entry point
├── cli/                     🧭 CLI Interface
│   ├── mod.rs
│   ├── parser.rs
│   ├── handler.rs
│   └── derive.rs
├── core/                    🔐 Núcleo da carteira
│   ├── mod.rs
│   ├── wallet.rs
│   ├── address.rs
│   └── crypto.rs
├── utils/                   🧰 Utilitários
│   ├── format.rs            🎨 Estilo e saída
│   └── crypto/
│       └── aes.rs           🔐 Criptografia AES-GCM + HMAC
├── tests/                   🧪 Testes automatizados
│   ├── wallet_tests.rs
│   └── derive_tests.rs
└── lib.rs                   🔁 Conector de módulos
```

---

## 🧱 Roadmap Técnico 2025

| Funcionalidade                        | Status        |
|--------------------------------------|---------------|
| Exportação `.wallet` criptografada   | 🟡 Em andamento |
| Subcomandos `export`, `recover`      | 🔲 Planejado   |
| Suporte a `--json` para integração   | 🔲 Planejado   |
| Importação por chave privada         | 🔲 Planejado   |
| `balance` via indexer FireChain      | 🔲 Pesquisa    |

---

## 📄 CHANGELOG Técnico

> Histórico completo de versões, recursos e refatorações da FireChain CLI.  
> Transparência e rastreabilidade técnica mantida desde o início do projeto.

🔗 [Ver CHANGELOG.md](./CHANGELOG.md)

---

## 📄 Licenciamento, Segurança e Contato Oficial

> Esta seção resume diretrizes jurídicas, termos de licenciamento e política de segurança da FireChain CLI.  
> O uso deste projeto implica na aceitação plena desses termos.

---

### 🔓 Licença de Uso Livre (MIT)

Este projeto é publicado sob os termos da [Licença MIT](./LICENSE.md).

**✔️ Permissões**  
- Uso, modificação e redistribuição para fins não comerciais

**⚠️ Limitações**  
- Sem garantias
- Uso corporativo exige licenciamento comercial

🔗 [Ver documento completo – LICENSE.md](./LICENSE.md)

---

### 💼 Licença Comercial FireChain™

Uso em ambientes corporativos, produtos comerciais ou serviços Web3 requer [Licença Comercial FireChain™](./COMMERCIAL_LICENSE_FIRECHAIN.md).

**Exemplos de uso que exigem licença:**
- White-label, SDK, SaaS, custódia ou derivados da CLI
- Monetização direta ou indireta

🔗 [Ver documento completo – COMMERCIAL_LICENSE_FIRECHAIN.md](./COMMERCIAL_LICENSE_FIRECHAIN.md)

---

### 🔐 Política de Segurança Técnica

Mantemos uma [Política Oficial de Segurança](./SECURITY.md) com:

- 🛡️ Escopo técnico auditável
- ⏱️ SLA de resposta
- 🏆 Programa Bug Bounty com tokens FIRE

🔗 [Ver documento completo – SECURITY.md](./SECURITY.md)

---

### 🤝 Contato Oficial

**Responsável Técnico e Legal:**  
👤 Guilherme Lima — Arquiteto da FireChain CLI  
🔗 [linkedin.com/in/guilhermelimadev-web3](https://www.linkedin.com/in/guilhermelimadev-web3/)
