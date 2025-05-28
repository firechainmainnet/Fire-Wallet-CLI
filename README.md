<p align="center">
  <img src="assets/firechain_banner.png" alt="FireChain CLI" width="600px" />
</p>

---

<p align="center">
  <!-- 🔧 Tecnologia -->
  <img src="https://img.shields.io/badge/Rust-2021-934c97" />
  <img src="https://img.shields.io/badge/CLI-Clap%204.5-orange" />
  <img src="https://img.shields.io/badge/Hashing-SHA2%20%7C%20Keccak%20%7C%20RIPEMD160-FF6D00" />

  <!-- 🔐 Segurança & Execução -->
  <img src="https://img.shields.io/badge/execução-100%25%20local-blue" />
  <img src="https://img.shields.io/badge/memória-zeroizada-purple" />
  <img src="https://img.shields.io/badge/fingerprint-unico%20por%20carteira-9C27B0" />
  <img src="https://img.shields.io/badge/Testes-Reais%20via%20CLI-green" />
  <img src="https://img.shields.io/badge/Cobertura-100%25-4CAF50" />

  <!-- 📦 Repositório -->
  <img src="https://img.shields.io/badge/License-MIT-9C27B0" />
  <img src="https://img.shields.io/github/last-commit/firechainmainnet/fire-wallet-cli" />
  <img src="https://img.shields.io/github/issues/firechainmainnet/fire-wallet-cli" />
  <img src="https://img.shields.io/badge/version-0.1.2-blue" />
</p>

---

# 🔥 FireChain CLI

**🧱 Sua stack Web3 começa na linha de comando.**

CLI local, auditável e modular para gerenciamento seguro de carteiras Web3 — com foco em controle criptográfico real, execução offline e integração com pipelines DevOps.

---

## 📚 Índice

- [🧬 Sobre](#-sobre)
- [📊 Badges Técnicos](#-badges-técnicos)
- [🔑 Princípios](#-princípios)
- [⚙️ Funcionalidades Atuais](#️-funcionalidades-atuais)
- [📤 Casos de Uso Reais](#-casos-de-uso-reais)
- [🧠 Exemplo Real de Execução](#-exemplo-real-de-execução)
- [🖥️ Ajuda Rápida](#️-ajuda-rápida)
- [🔐 Arquitetura de Execução](#-arquitetura-de-execução)
- [🧱 Roadmap Técnico 2025](#-roadmap-técnico-2025)
- [📄 Documentação e Licenças](#-documentação-e-licenças)
- [🤝 Contribuições](#-contribuições)
- [🔚 Fechamento](#-fechamento)

---

## 🧬 Sobre

A **FireChain CLI** é uma carteira Web3 de linha de comando voltada para times que priorizam **segurança criptográfica real**, **execução auditável** e **integração com pipelines de produção**.

Projetada para:
- 🧑‍💻 Desenvolvedores Web3
- 🏦 Fintechs e instituições financeiras
- 🛡️ Times de auditoria e segurança
- 🎮 Jogos Web3 e backends automatizados
- 🎓 Educação técnica e universidades

---

## 🔑 Princípios

- 🔐 Segurança desde a primeira linha
- 🧩 Modularidade verdadeira (CLI, backend, integração)
- 🧪 Testabilidade real, sem mocks
- 📦 Independência total: execução local, sem rede
- 🧠 Código claro, Rust idiomático, auditável

---

## ⚙️ Funcionalidades Atuais

- ✅ Geração de carteira com ECDSA-secp256k1
- ✅ Derivação de endereços ETH / BTC / FireChain
- ✅ Fingerprint único por chave pública
- ✅ Testes CLI via `assert_cmd` + cobertura 100% via `llvm-cov`
- ✅ Compatível com frases BIP-39 (importação HD)
- 🟡 Exportação criptografada `.wallet` (AES-256-GCM) em desenvolvimento
- 🔲 Flags `--json`, `--show-private`, subcomandos `export`, `recover` planejados

---

## 📤 Casos de Uso Reais

- 🔄 Geração de carteiras automatizadas em CI/CD
- 🔍 Verificação de assinaturas offline
- 📥 Custódia temporária de chaves para bots e bridges
- 🧪 Testes de fuzzing em derivação e corrupção de payloads

---

## 🧠 Exemplo Real de Execução

```bash
$ firechain-cli wallet new

🧬 FireChain Wallet Generator
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ Chave privada gerada
🔓 Chave pública derivada
📬 Endereço (Fire)     : f1r3xyz...
🌐 Endereço (Ethereum) : 0xabc...
₿  Endereço (Bitcoin)  : 1xyz...
🧬 Fingerprint SHA256  : e0fa...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

---

## 🖥️ Ajuda Rápida

```bash
$ firechain-cli --help

FireChain CLI - Gerenciador de carteiras Web3

USAGE:
    firechain-cli [OPTIONS]

OPTIONS:
    --new                Gera nova carteira
    --show-private       Exibe chave privada (oculto por padrão)
    --json               Saída em JSON
    --help               Mostra esta mensagem
```

---

## 🔐 Arquitetura de Execução

```
Usuário → firechain-cli wallet new
       ↳ secp256k1::SecretKey::new()
       ↳ Derivação ETH, BTC, FireChain
       ↳ SHA256 Fingerprint
       ↳ (opcional) exportação criptografada
```

---

## 🧱 Roadmap Técnico 2025

| Funcionalidade                         | Status       |
|----------------------------------------|--------------|
| Exportação `.wallet` com AES-256-GCM   | 🟡 Em andamento |
| Subcomandos `export`, `inspect`, `recover` | 🔲 Planejado |
| Flag `--json` para stdout programável  | 🔲 Planejado |
| Integração com WebUSB (modo leitura)   | 🔲 Planejado |
| Suporte a Ledger / Trezor via FIDO     | 🔲 Em pesquisa |

---

## 📄 Documentação e Licenças

- 📚 [Documentação Técnica (docs.rs)](https://docs.rs/firechain-cli)
- 🔓 [LICENSE - MIT](./LICENSE)
- 🛡️ [Política de Segurança](./SECURITY.md)
- 🤝 [Solicitar Licença Comercial](https://www.linkedin.com/in/guilhermelimadev-web3/)

---

## 🤝 Contribuições

Pull requests são bem-vindos para testes, refinos de UX CLI, cobertura edge e novos comandos.  
Para propostas maiores, abra uma issue ou entre em contato via LinkedIn.

---

## 🔚 Fechamento

A maioria só usa carteiras.  
Poucos entendem como elas funcionam.

A FireChain CLI é para quem constrói com **segurança, consciência e propósito técnico**.

> Prefere confiar num clique ou entender cada byte?

🌐 [Comece agora com FireChain CLI](https://github.com/firechainmainnet/fire-wallet-cli)

---

**Guilherme Lima**  
Arquiteto do FireChain™  
🔗 [LinkedIn](https://www.linkedin.com/in/guilhermelimadev-web3/)

> FireChain™ é uma marca registrada. Todos os direitos reservados.
