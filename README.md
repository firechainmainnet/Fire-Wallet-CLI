
<p align="center">
  <img src="assets/firechain_banner.png" alt="FireChain CLI" width="600px" />
</p>

---

<p align="center">
  <img src="https://img.shields.io/badge/Rust-2021-934c97" />
  <img src="https://img.shields.io/badge/CLI-Clap%204.5-orange" />
  <img src="https://img.shields.io/badge/Hashing-SHA2%20%7C%20Keccak%20%7C%20RIPEMD160-FF6D00" />
  <img src="https://img.shields.io/badge/execução-100%25%20local-blue" />
  <img src="https://img.shields.io/badge/memória-zeroizada-purple" />
  <img src="https://img.shields.io/badge/fingerprint-unico%20por%20carteira-9C27B0" />
  <img src="https://img.shields.io/badge/Testes-Reais%20via%20CLI-green" />
  <img src="https://img.shields.io/badge/Cobertura-100%25-4CAF50" />
  <img src="https://img.shields.io/badge/License-COMMERCIAL-red" />
  <img src="https://img.shields.io/github/last-commit/firechainmainnet/fire-wallet-cli" />
  <img src="https://img.shields.io/github/issues/firechainmainnet/fire-wallet-cli" />
  <img src="https://img.shields.io/badge/version-0.1.3-blue" />
</p>

---

# 🔥 FireChain CLI

**🧱 Sua stack Web3 começa na linha de comando.**  
Interface de linha de comando local, modular e segura para gerar carteiras com múltiplos padrões de endereços — **Bitcoin (₿), Ethereum (⛓️) e FireChain (🔥)** — com **criptografia de ponta e identidade única por fingerprint**.

---

## 📚 Índice

- [🧬 Sobre](#-sobre)
- [🔑 Princípios](#-princípios)
- [⚙️ Funcionalidades Atuais](#️-funcionalidades-atuais)
- [🧠 Execução Real](#-execução-real)
- [🖥️ Ajuda Rápida](#️-ajuda-rápida)
- [📁 Arquitetura de Diretórios](#-arquitetura-de-diretórios)
- [🧱 Roadmap Técnico 2025](#-roadmap-técnico-2025)
- [📄 Licenças e Contato](#-licenças-e-contato)

---

## 🧬 Sobre

A **FireChain CLI** é uma ferramenta ultra-segura para geração de carteiras Web3 com identidade criptográfica.  
Feita 100% em **Rust**, zero dependência externa, zero rede, perfeita para:

- 🧑‍💻 Desenvolvedores que controlam sua stack
- 🛡️ Times de segurança e auditoria
- 🔗 Integração com automações e CI/CD
- 🧬 Criptografia offline e testável

---

## 🔑 Princípios

- 🔐 Segurança desde a geração até a derivação
- 🔭 Identidade única com `fingerprint` e `wallet ID`
- 🎯 100% CLI local, sem exposição externa
- 🎨 Interface moderna com `termcolor` + emojis
- 🔬 Testes reais com cobertura total

---

## ⚙️ Funcionalidades Atuais

- ✅ Geração de chave privada/publica com `secp256k1`
- ✅ Derivação de endereços BTC, ETH e FireChain
- ✅ Identidade por fingerprint (`FC-xxxx`)
- ✅ Modularização da CLI (`parser`, `handler`, `derive`)
- ✅ Estilo premium para terminal (`termcolor`)
- ✅ Testes automatizados com `assert_cmd`

---

## 🧠 Execução Real

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
├── utils/format.rs          🧰 Estilização e helpers
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

## 📄 Licenciamento, Segurança e Contato Oficial

> Esta seção reúne as diretrizes jurídicas, permissões de uso, termos de licenciamento e política de segurança aplicáveis à FireChain CLI.  
> Toda utilização do projeto implica na aceitação integral dos documentos aqui vinculados.

---

### 🔓 Licença de Uso Livre (MIT)

Este projeto é publicado sob os termos da [Licença MIT](./LICENSE.md), amplamente adotada em software open-source.  

**✔️ Permissões**  
- Uso, modificação e redistribuição para fins pessoais, educacionais e técnicos
- Integração livre em projetos não comerciais

**⚠️ Limitações**  
- **Sem garantias** (como previsto na própria licença)
- **Não cobre uso institucional, corporativo ou com monetização direta ou indireta**

🔗 [Ver documento completo – LICENSE.md](./LICENSE.md)

---

### 💼 Licença Comercial FireChain™

O uso da FireChain CLI em **ambientes corporativos, produtos comerciais, serviços Web3 com fins lucrativos ou integração OEM** requer o aceite e contratação sob os termos da nossa [Licença Comercial FireChain™](./COMMERCIAL_LICENSE_FIRECHAIN.md).

**🏛️ Jurisdição Legal**  
- Aplicável sob o direito brasileiro, com validade internacional

**📌 Exemplos de uso que requerem licença**  
- Distribuição como SDK, white-label, SaaS ou custódia
- Prestação de serviços com base na CLI
- Branding derivado da marca FireChain™

🔗 [Ver documento completo – COMMERCIAL_LICENSE_FIRECHAIN.md](./COMMERCIAL_LICENSE_FIRECHAIN.md)

---

### 🔐 Política de Segurança Técnica

A segurança da FireChain CLI é tratada como requisito **não funcional essencial**.  
Mantemos uma [Política Oficial de Segurança](./SECURITY.md) pública com:

- 🧠 Escopo técnico auditável
- 🛡️ Processo de resposta a incidentes
- 🏆 Programa Bug Bounty nativo (com tokens FIRE)
- ⏱️ SLAs documentados para tratamento de vulnerabilidades

🔗 [Ver documento completo – SECURITY.md](./SECURITY.md)

---

### 🤝 Suporte, Parcerias e Licenciamento

**Responsável técnico e legal:**  
👤 Guilherme Lima – Arquiteto da FireChain CLI

**Contato direto para propostas comerciais, auditorias ou parcerias:**  
🔗 [https://www.linkedin.com/in/guilhermelimadev-web3/](https://www.linkedin.com/in/guilhermelimadev-web3/)