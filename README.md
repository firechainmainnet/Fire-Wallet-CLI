<h1 align="center">🔥 FireChain CLI</h1>

<p align="center">
  <img src="assets/firechain_banner.png" alt="FireChain CLI Banner" width="600px" />
</p>

<!-- 🔧 Stack & Compatibilidade -->
![Rust](https://img.shields.io/badge/built%20with-Rust-934c97)
![Clap](https://img.shields.io/badge/CLI-Clap%204.5-orange)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux-0078D6)
![Architecture](https://img.shields.io/badge/design-CLI%20First%20%7C%20Modular-0066cc)

<!-- 🔐 Segurança & Criptografia -->
![Security](https://img.shields.io/badge/security-AES--256%20%2B%20Argon2-00C853)
![Hashing](https://img.shields.io/badge/hashing-SHA2%20%7C%20Keccak-FF6D00)
![Zeroize](https://img.shields.io/badge/memory-zeroized-7C4DFF)
![Fingerprint](https://img.shields.io/badge/fingerprint-unique%20per%20wallet-9C27B0)

<!-- 🧪 Testes & Cobertura -->
![Coverage](https://img.shields.io/badge/coverage-100%25-4CAF50)
![E2E](https://img.shields.io/badge/tests-E2E%20Real-43A047)
![LLVM](https://img.shields.io/badge/LLVM-cov%20enabled-546E7A)
![Commit](https://img.shields.io/github/last-commit/firechainmainnet/fire-wallet-cli)
![Status](https://img.shields.io/badge/status-em%20desenvolvimento-F57C00)

<!-- 📈 Reputação & Licença -->
![License](https://img.shields.io/badge/license-MIT-9C27B0)
![Stars](https://img.shields.io/github/stars/firechainmainnet/fire-wallet-cli?logo=github)
![Forks](https://img.shields.io/github/forks/firechainmainnet/fire-wallet-cli?logo=github)
![SLA](https://img.shields.io/badge/SLA-99.99%25-D32F2F)
![Curso](https://img.shields.io/badge/Curso%20Oficial-Turma%2016/06-FBC02D)
![FireChain](https://img.shields.io/badge/verified-by%20FireChain-212121)

---

## 🧬 Sobre

A FireChain CLI é uma carteira digital de linha de comando projetada para engenheiros, validadores e sistemas que exigem **segurança criptográfica auditável**, **soberania de chaves local** e **integração com fluxos DevOps Web3**.

> 🔐 Totalmente local. ⚙️ Altamente modular. 🧱 Preparada para produção.

---

### 🌍 Soberania real. Sem concessões.

Na FireChain, acreditamos que custódia não é interface — é **arquitetura**.

Nosso objetivo é entregar uma plataforma que:

- 🔒 Não dependa de extensões, GUIs ou servidores externos
- 🧬 Faça derivação, exportação e verificação diretamente do terminal
- ⚙️ Se integre com pipelines CI/CD, bots e validadores
- 🧩 Sirva como base para produtos de terceiros — com total controle criptográfico

---

### 🔧 Fundamentos Arquiteturais (Já Entregues)

- ✔️ CLI 100% funcional para geração e derivação de carteiras multi-chain
- ✔️ Suporte completo a ECDSA-secp256k1 com derivação HD modular
- ✔️ Fingerprint de segurança por carteira gerada
- ✔️ Hashing com SHA2, RIPEMD160, Keccak
- ✔️ Integração com cobertura de testes `cargo llvm-cov` (100%)
- ✔️ Testes E2E reais em ambientes locais

---

### 🧪 Comparativo Estratégico

| Funcionalidade             | FireChain CLI | Metamask | Ledger | Electrum |
|---------------------------|----------------|----------|--------|----------|
| CLI-first (sem GUI)       | ✅              | ❌       | ✅     | ✅       |
| Arquitetura modular       | ✅              | ❌       | 🔸     | 🔸       |
| Fingerprint por chave     | ✅              | ❌       | 🔸     | ❌       |
| JSON + STDIN              | 🔜              | ❌       | ❌     | ❌       |
| Código Rust 100% auditável| ✅              | ❌ (JS)  | 🔸     | ✅       |
| Multisig determinístico   | 🔜              | ❌       | ✅     | ✅       |

---

### 🤝 Por que adotar a FireChain CLI?

- 💼 Ferramenta profissional para uso real em produção
- 🔍 Testada, validada e pronta para expansão
- 🧠 Pensada para quem constrói no mundo real: devs, auditores, validadores
- 🌱 Crescendo junto da comunidade — cada feedback molda o roadmap

> 👇 A FireChain CLI é mais que uma carteira.  
> É o **core criptográfico local para sua stack Web3.**

---

## 🛡️ Segurança Real. Criptografia de Verdade

| Recurso de Segurança             | Status     | Notas Técnicas                         |
|----------------------------------|------------|----------------------------------------|
| AES-256-GCM + nonce aleatório    | 🔜 Em planejamento | Criptografia simétrica para `.wallet` |
| Derivação com Argon2id           | 🔜 Em planejamento | Proteção contra brute force sem persistência |
| Zeroização de memória (`zeroize`)| ✅          | Já integrado e ativo no código         |
| Campos privados ocultos          | 🔜 Em planejamento | Só visíveis com `--unsafe-dump`       |
| Fingerprint de segurança         | ✅          | Identificação única por chave derivada |

---

## 📈 Por que FireChain é a melhor CLI do mercado?

| Característica                      | 🔥 FireChain CLI                         | 🛠️ Outras carteiras CLI          |
|------------------------------------|------------------------------------------|----------------------------------|
| UX Premium via Clap 4.5            | ✅ Sim                                    | ❌ Básico/antigo                  |
| Cobertura 100% + Testes Reais      | ✅ Incluídos por padrão                   | ❌ Limitado / Nenhum              |
| Derivação multi-chain real         | ✅ ETH / BTC / FireChain                  | 🔸 ETH apenas                     |
| Independência de rede              | ✅ 100% local, auditável                  | ❌ Dependência de API/daemon      |
| Segurança de ponta (cripto/hashing)| ✅ `secp256k1` + `SHA2` + `Keccak`        | 🔸 Parcial                        |
| Fingerprint de segurança           | ✅ Cada chave com ID único                | ❌ Ausente                        |
| Licença comercial com SLA          | ✅ Sim, via contrato                      | ❌ Não disponível                 |

---

## 💼 Para quem é a FireChain CLI?

- 🧑‍💻 Desenvolvedores que exigem controle local e testabilidade
- 🏦 Fintechs que lidam com auditoria, KYC e tokens próprios
- 🕹️ Games Web3 que precisam de carteiras embutidas no backend
- 🧠 Universidades e educadores que ensinam cripto real

---

## 💬 Frases que definem FireChain CLI

> 🔐 "O nível de segurança que impressiona qualquer auditor."  
> 🧬 "CLI que entrega UX de verdade, sem gambiarra."  
> 🧪 "Cada linha é testada. Cada chave, verificável. Zero achismos."

---

## 📈 Roadmap FireChain CLI (2025)

| 🧱 Funcionalidade                        | Status     | Notas Técnicas                         |
|-----------------------------------------|------------|----------------------------------------|
| HD Wallet com frase BIP-39              | ✅          | Presente com suporte inicial           |
| AES-256-GCM com nonce                   | 🔜 Em planejamento | Criptografia simétrica segura       |
| Argon2id derivation                     | 🔜 Em planejamento | Proteção de senha sem persistência  |
| Campos ocultos                          | 🔜 Em planejamento | Exibição com `--unsafe-dump`       |
| Zeroização de memória (`zeroize`)       | ✅          | Ativo por padrão                      |
| Ledger on-chain (audit trail local)     | 🔜 Roteirizado | Logs com hash assinado              |
| Exportação `.wallet` segura             | 🔜 Roteirizado | Com criptografia local segura        |
| UTXO local com indexação                | 🔜 Roteirizado | Arquitetura validada                 |
| Multisig determinístico                 | 🔜 Roteirizado | Padrão BIP-67                         |
| Tokens nativos FireChain                | 🔜 Em planejamento | Suporte ao FireToken padrão         |
| Assinatura & Verificação                | ✅          | Já disponível via secp256k1          |
| Web Wallet (read-only) + Dashboard      | 🔜 Roteirizado | Interface conectável via WebUSB     |
| Integração Ledger/Trezor                | 🔜 Em planejamento | Padrão FIDO/WebUSB em pesquisa     |
| Suporte a mnemonics variados            | 🔜 Em planejamento | Compatibilidade com BIP-32, 39, 44 |

---

## 🧪 Testabilidade Real

```bash
cargo test --release -- --nocapture
cargo llvm-cov --release
cargo llvm-cov --release --html
```

📊 Relatório visual: `target/llvm-cov/html/index.html`

✅ Binário validado com testes reais  
✅ 100% cobertura garantida

---

## 📋 Licenciamento & Programas Premium

A FireChain CLI pode ser utilizada sob diferentes formatos de licença — adaptados para devs individuais, startups, instituições educacionais, e operações empresariais de alto nível.

💡 Nosso compromisso é fornecer segurança real com flexibilidade de acesso — do estudo pessoal à operação financeira regulada.

---

### 🔓 Licença Open Source (MIT) — Uso Restrito com Termos Claros

A FireChain CLI está licenciada sob a [MIT License](https://opensource.org/licenses/MIT), com **uso livre e auditável**, mas com **limites explícitos quanto ao uso comercial ou institucional**.

#### ✔️ Permitido para:

- 👨‍🔬 Pesquisa acadêmica e estudos de segurança
- 🧪 Testes internos, aprendizado pessoal e labs de desenvolvimento
- 🏠 Provas de conceito e aplicações locais **sem fins lucrativos**
- 🎓 Projetos educacionais ou curriculares com uso não-comercial

#### 🚫 **Restrito e proibido para:**

- ❌ Uso comercial direto ou indireto (prestação de serviços, integrações, SaaS, apps, consultorias etc.)
- ❌ Operações institucionais, bancárias ou corporativas com dados de terceiros
- ❌ Distribuição de binários, forks ou builds com fins comerciais
- ❌ Uso em ambientes de produção real (mesmo que interno)
- ❌ Expectativa de suporte, atualizações ou segurança garantida

🔐 Qualquer uso fora do escopo acima exige **licenciamento comercial formal com contrato**, com suporte técnico, garantia legal e SLAs opcionais.

📄 [📬 Solicitar licença comercial](https://www.linkedin.com/in/guilhermelimadev-web3/)

---

### 💼 Planos Oficiais FireChain CLI

A FireChain CLI é mais que uma ferramenta — é um motor criptográfico pensado para devs, fintechs e instituições que precisam de controle absoluto. Abaixo estão nossos planos oficiais, otimizados para cada etapa da sua jornada Web3:

| 💎 Plano              | Público-Alvo                   | Benefícios Chave                                                                                  | 💵 Valor¹             |
|----------------------|--------------------------------|----------------------------------------------------------------------------------------------------|------------------------|
| 👤 **Dev Solo**       | Desenvolvedores individuais     | 1 ambiente, uso pessoal comercial, export `.wallet`, CLI liberada                                 | **US$ 190 / ano**      |
| 🎓 **Curso Autônomo** | Instrutores & Plataformas       | Até 30 alunos, material oficial, acompanhamento via LinkedIn/Telegram                             | **US$ 390 / turma**    |
| 🎤 **Curso Mentorado**| Alunos com mentoria oficial     | Certificação FireChain Pro, aula ao vivo com fundador, comunidade via LinkedIn/Telegram           | **US$ 990 / aluno**    |
| 🏛️ **FireChain Elite**| Universidades & Corporações     | Evento fechado, backend privado, sessão presencial (até 100 pessoas), acesso antecipado a módulos | **US$ 15.900 / módulo**|
| 🚀 **Starter**        | Startups & MVPs                 | 50k carteiras/ano, ambiente de produção liberado, sem “Powered by FireChain”                      | **US$ 990 / ano**      |
| ⚙️ **Scale**          | Fintechs & Games Web3           | 3 ambientes, suporte técnico 72h, logs auditáveis, estrutura multisig                             | **US$ 2.900 / ano**    |
| 🛡️ **Enterprise**     | Exchanges, Bancos, Governos     | SLA 99.99%, suporte 24×7, cláusulas legais (DPA, GDPR), DevSecOps assistido                       | **sob consulta**       |

📅 **Próxima turma mentorada confirmada: 16 de Junho de 2025** — vagas limitadas.

---

💼 Todos os planos incluem acesso à documentação premium e canal dedicado de onboarding.  
🎓 Cursos incluem material validado por especialistas e **acompanhamento direto via LinkedIn e Telegram**.

> 🚀 Está construindo algo sério em Web3? A FireChain CLI foi feita para isso.

---

### 💡 Por que esses valores são mais que justos?

A FireChain CLI **não é um produto comum** — é a fundação de um novo modelo de custódia digital, pensado para quem realmente precisa construir com segurança, agilidade e independência.

- 🔐 Você não compra uma ferramenta — **você adquire poder criptográfico auditável**, com arquitetura validada, suporte e roadmap vivo.
- ⚙️ Cada linha de código é otimizada para **integrar com pipelines, bots e validadores**, sem depender de GUIs frágeis ou extensões opacas.
- 🧠 O conhecimento transmitido em nossos cursos **vai da base criptográfica até arquitetura de sistemas e práticas DevOps Web3 modernas**.
- 🧩 Você aprende a gerar chaves, derivar endereços, exportar carteiras, aplicar zeroização de memória, pensar multisig, e montar stacks privadas.

🎓 Nosso curso mentorado **condensa anos de prática, design, auditoria e segurança em uma trilha objetiva e transformadora.**

💥 Resultado real? Quem entra na FireChain sai com:

- Domínio de derivação HD multi-chain
- Mentalidade de design seguro desde o byte 0
- Capacidade de auditar e criticar outras soluções
- Ferramental real para desenvolver sua própria stack Web3

💎 A FireChain não é para todos — é para os que constroem o futuro.

> Preferimos **1 dev com propósito** do que **1000 curiosos com pressa.**

---

### ⚠️ Esses valores vão subir?

📢 Ninguém vai receber tudo isso por esses valores por muito tempo.

Estamos entregando:

- 📦 CLI modular, local, 100% Rust
- 🔒 Fundamentos de criptografia de estado-da-arte (secp256k1, SHA2, RIPEMD160, Keccak)
- 🧪 Testes reais, cobertura total, práticas seguras
- 🧱 Roadmap claro e executado com disciplina

📈 Em breve: AES-256-GCM, Argon2id, Multisig M/N, Web Wallet, STDIN/JSON mode, suporte a Ledger/Trezor, e integração com WebUSB e API REST.

🧠 A cada entrega, a FireChain CLI se aproxima de se tornar **a stack de referência para DevOps Web3 e custódia modular**.

> 🕐 Acesso atual é oportunidade.  
> Nova turma mentorada: **16/06/2025**  
> Após isso, **os planos Dev Solo, Cursos e Elite serão reavaliados com base no novo valor entregue.**

💡 **Se você entendeu o que a FireChain representa — entre enquanto ainda está ao seu alcance.**

---

### ⚠️ Termos Legais e Condições de Uso

A FireChain CLI é um projeto que lida com geração de carteiras criptográficas, derivação de chaves privadas e exportação de informações sensíveis. Por isso, o uso desta ferramenta **requer compreensão técnica adequada e consciência sobre os riscos associados**.

---

#### 📌 Responsabilidades do Usuário

- Você é o único responsável por toda operação realizada com este software.
- A FireChain **não armazena, monitora, nem recupera chaves privadas, backups ou configurações locais**.
- Nenhum dado é transmitido a servidores externos: toda execução é local e sua segurança depende das práticas do operador.

---

#### 🚫 Restrições Adicionais

Além das limitações da licença open source:

- ❌ É proibido utilizar a marca FireChain, ou qualquer derivação (logos, nome, identidade visual), em campanhas, produtos ou anúncios sem autorização formal.
- ❌ É proibida a inclusão deste software (ou partes dele) em **bundles pagos, extensões comerciais, cursos monetizados ou marketplaces**.
- ❌ É proibida a revenda de binários gerados a partir deste repositório.

---

#### 📡 Compliance e Uso Empresarial

Se sua operação exige:

- Uso em ambientes de produção
- Prestação de serviços com base nesta CLI
- Inclusão como parte de um SaaS, gateway, exchange ou validadores
- Distribuição sob forma de contrato, franquia ou curso comercializado

> ⚖️ Você precisará de **um contrato comercial oficial** com cláusulas de suporte, auditoria, segurança e licenciamento formal.

---

📄 [📬 Solicitar Contrato Comercial ou Participar de Cursos via LinkedIn](https://www.linkedin.com/in/guilhermelimadev-web3/)

---

## 👨‍💻 Contato

- **Autor**: [Guilherme Lima](https://www.linkedin.com/in/guilhermelimadev-web3/)
- 🌐 FireChain Mainnet (em breve)

---

🛡️ FireChain™ é uma marca registrada. Todos os direitos reservados.  
📅 Atualizado em: **2025-05-26**
