# 🔥 FireChain Wallet CLI

> A revolução da custódia digital começa aqui. A FireChain Wallet CLI entrega **soberania total, criptografia impenetrável e liberdade programável** para quem leva o mundo cripto a sério. Construída em Rust para velocidade, segurança e controle absoluto.

![Status](https://img.shields.io/badge/status-em%20desenvolvimento-orange)
![Rust](https://img.shields.io/badge/built%20with-Rust-934c97)
![Secure](https://img.shields.io/badge/security-AES--256%20%2B%20Argon2-green)

<p align="center">
  <img src="https://img.shields.io/badge/f1r3-powered-blueviolet?logo=fire" alt="f1r3 badge">
</p>

---

## 🌍 A Nova Geração da Soberania Digital

Imagine um mundo onde você controla cada chave, cada assinatura e cada transação — sem confiar em servidores, extensões ou interfaces gráficas obscuras.

Na FireChain Wallet, **você é o guardião de seus próprios fundos**. Nós acreditamos em um futuro sem intermediários, onde sua carteira é um terminal poderoso — capaz de executar ações, derivar novos caminhos, verificar múltiplas assinaturas, e exportar backups seguros em segundos.

**Isso não é uma carteira. É uma fundação para sua liberdade financeira.**

---

## 📜 Whitepaper Integrado

A **FireChain Wallet** nasce com a missão de redefinir o conceito de carteiras cripto. Somos uma **plataforma de custódia digital de código aberto**, projetada para usuários exigentes — desenvolvedores, investidores, validadores e projetos DAO — que precisam de ferramentas de nível profissional, seguras e 100% auditáveis.

### 🔧 Fundamentos Técnicos
- Zero dependência de servidores: 100% local & auditável
- Algoritmo de assinatura: ECDSA-secp256k1
- Criptografia simétrica: AES-256-GCM com salt + nonce aleatório
- Derivação de chave: Argon2id + HMAC-SHA512 (HD Path `m/77/N`)
- Endereços `f1r3...` com checksum baseado em RIPEMD160 + Base58

### 🔍 Visão de Expansão
- 🧱 UTXO Engine com persistência local
- 🪙 Suporte a múltiplas moedas/token standards FireChain
- 🔗 Ledger On-Chain: sincronia, assinaturas e eventos
- 🌍 Web Wallet (read-only) + API REST JSON

---

## 🥇 Diferenciais vs Outras Wallets

| 🔥 FireChain Wallet | 🔐 Metamask | 🧊 Ledger | 💼 Electrum | 🧠 Specter |
|---------------------|-------------|-----------|-------------|------------|
| 100% CLI/JSON       | ❌          | ❌        | ✅          | ✅         |
| Open Source Rust    | ❌ (JS)     | Parcial   | ✅          | ✅         |
| Multisig M/N        | ❌          | ✅        | ✅          | ✅         |
| AES-256 + Argon2id  | ❌          | ✅        | ❌          | ❌         |
| JSON + STDIN Mode   | ❌          | ❌        | ❌          | ❌         |
| Sem GUI obrigatória | ✅          | ❌        | ✅          | ✅         |
| Integração com bots | ✅          | ❌        | ❌          | ❌         |

> **Conclusão:** a FireChain é a primeira CLI wallet **com foco em automação, segurança, multiusuário e DevOps Web3.**

---

## 🤝 Por que Adotar a FireChain Wallet?

- 💯 **Open Source** sob licença MIT — use, fork, contribua
- 🧠 Pensada para Devs, Auditores e Validators
- 🧩 Fácil de integrar com pipes CI/CD e bots Web3
- 💣 Segurança padrão militar (Argon2id + AES)
- 🌱 Crescendo com a comunidade: seu feedback constrói o roadmap

---

## 📦 Instalação

```bash
cargo install --path .
```

---

## 🧪 Exemplos Práticos

```bash
# Criar carteira aleatória
firechain-cli new --label minha_wallet

# Criar carteira mnemônica (12 palavras)
firechain-cli mnemonic --words 12 --label minha_hd

# Derivar próximo endereço
firechain-cli derive --input-wallet minha_hd.json --password 123 --output minha_hd_out.json

# Exportar com senha forte
firechain-cli export --input-json minha_hd_out.json --password senha123 --output backup.wallet

# Importar em modo inseguro (exibir frase/chaves)
firechain-cli --unsafe-dump import --password senha123 --path backup.wallet

# Criar multisig 2 de 3
firechain-cli multisig --m-required 2 --public-keys PUB1 PUB2 PUB3

# Assinar mensagem
firechain-cli sign --privkey HEX_PRIV --message "mensagem"

# Verificar M/N multisig
firechain-cli verify --message "msg" --m-required 2 --public-keys P1 P2 --signatures S1 S2
```

---

## 🧱 Arquitetura Modular

- `wallet.rs`: HD wallets, mnemônicos e derivação
- `secure.rs`: exportação criptografada
- `signing.rs`: assinatura e verificação ECDSA
- `multisig.rs`: endereços M-de-N determinísticos
- `commands/*.rs`: comandos CLI & JSON via STDIN
- `utils/fs.rs`: salvamento com id+timestamp seguro

---

## 🛡️ Segurança Zero Trust

- AES-256-GCM + nonce aleatório
- Derivação com Argon2id (senha nunca armazenada)
- Campos privados ocultos por padrão
- Frases/chaves só exibidas com `--unsafe-dump`
- Memória crítica zeroizada com `zeroize`

---

## 🌐 JSON Mode para Automação

```bash
echo '{
  "action": "Mnemonic",
  "words": 12,
  "label": "auto_wallet",
  "unsafe_dump": false
}' | firechain-cli --json
```

> 🤖 Ideal para bots de transação, integração em CI, backups e sistemas com múltiplos signatários.

---

## 📍 Roadmap Estratégico

| ✅ Já Disponível           | 🔜 Em Andamento                              |
|---------------------------|-----------------------------------------------|
| HD Wallet com frase       | Ledger On-Chain para auditoria e histórico   |
| Exportação `.wallet` segura | Sistema UTXO local com indexação            |
| Multisig determinístico   | Tokens nativos FireChain                     |
| Assinatura & Verificação  | Web Wallet (read-only) + Dashboard           |
| CLI + JSON                | Integração com Ledger/Trezor (FIDO/WebUSB)   |

---

## 🤝 Comunidade FireChain

Queremos construir isso com você.

- 💬 Sugira ideias ou melhorias
- 🧪 Teste novos recursos
- 📢 Compartilhe com outros builders
- ✍️ Escreva tutoriais ou traduções
- 🛠️ Contribua com código, testes ou documentação

Siga-nos no Twitter/X, LinkedIn, Discord (em breve). O mundo precisa de ferramentas seguras — e **FireChain Wallet CLI é só o começo.**

---

## 🚀 Junte-se à Missão FireChain

> O mundo está migrando para contratos, blocos, consenso e descentralização. Mas a maior parte das carteiras ainda vive no século passado.

A **FireChain Wallet CLI** é o começo de algo maior. Estamos construindo a base para um novo ecossistema: **com tokens nativos, ledger on-chain, múltiplas identidades, oráculos, e mais.**

Se você chegou até aqui, é porque **também sente que o mundo cripto merece algo melhor.**

👉 **Clone, contribua, compartilhe.**
👉 **Use. Dobre. Hackeie. Expanda.**
👉 **FireChain é para builders. Para pioneiros. Para quem pensa em código e vive em cripto.**

Junte-se à 🔥 revolução.

---

## 👨‍💼 Arquiteto

**Guilherme Lima** — Arquiteto de Sistemas Blockchain & Engenheiro Full Stack

> “Nossa missão é democratizar o acesso à segurança cripto. A FireChain Wallet é mais do que uma ferramenta: é um movimento.”

Conecte-se no [LinkedIn](https://www.linkedin.com/in/guilhermelimadev-web3/) ou contribua com PRs e ideias.

---

## 📄 Licença

MIT © 2025 — FireChain Wallet CLI • Liberdade e soberania digital para todos.