# 🔐 Política Oficial de Segurança – FireChain CLI

**Projeto:** FireChain CLI  
**Autor:** Guilherme Lima  
**Versão:** 2.0 (2025)  
**Jurisdicação Legal:** República Federativa do Brasil  
**Conformidade Técnica:** ISO/IEC 27001:2022, ISO/IEC 27400:2022, OWASP, RFC 6979, GDPR (EU)

---

## 🧾 1. Compromisso Institucional

A FireChain CLI é construída sob o princípio de **segurança por design**, com foco em **soberania digital, criptografia de ponta, execução local e independência de rede**.  
Esta política estabelece regras formais de análise, reporte e tratamento de vulnerabilidades em conformidade com padrões internacionais e boas práticas regulatórias.

---

## 🧠 2. Escopo Técnico de Interesse

Aceitamos contribuições e auditorias técnicas que envolvam:

- Comprometimento da curva elíptica ECDSA `secp256k1`
- Vazamento direto ou indireto de chaves privadas
- Falhas de zeroização de memória (`zeroize`)
- Colisões, preimage attacks ou fraqueza nos algoritmos: `SHA-256`, `Keccak-256`, `RIPEMD160`
- Quebra ou má implementação de `AES-256-GCM` ou `Argon2id` (criptografia de `.wallet`)
- Injeções via `STDIN`, abusos de `--unsafe-*`, manipulação do modo `--json`
- Desvios de fingerprint ou wallet ID via inputs maliciosos

**Todos os testes devem ser reproduzíveis, éticos e tecnicamente embasados.**

---

## 🚫 3. Fora de Escopo

Os seguintes itens são excluídos da nossa política de resposta:

- Bugs visuais ou estéticos no terminal (CLI/UX)
- Alertas genéricos de scanners sem PoC funcional
- Vulnerabilidades que dependam exclusivamente de sistemas externos (SO, shell, hardware)
- Ações esperadas ao usar opções de risco (`--unsafe-*`)
- Critérios fora do modelo de ameaça documentado

---

## 🛡️ 4. Processo Seguro de Reporte

### 📫 Canais Oficiais de Reporte

- 🔐 Mensagem privada para [Guilherme Lima – LinkedIn](https://www.linkedin.com/in/guilhermelimadev-web3/)
- 🛡️ Issue privada com tag `security` no GitHub (criptografar dados sensíveis)

### 📋 Requisitos para análise
- Prova de conceito (PoC)
- Descrição técnica clara, sem scripts obscuros
- Comportamento reprodutível no ambiente local FireChain CLI

**🚫 Não aceitamos divulgação pública sem coordenação prévia.**

---

## ⏱️ 5. Política de Resposta e SLA

| Severidade        | Resposta Inicial     | Correção Oficial         |
|-------------------|----------------------|--------------------------|
| 🔥 Crítica        | ≤ 24h úteis          | Patch imediato ou mitigação emergencial |
| 🚨 Média          | ≤ 72h úteis          | Inclusão em próxima release prioritária |
| ⚠️ Baixa          | ≤ 5 dias úteis       | Correção em release futura planejada    |

Todos os relatórios serão tratados com **confidencialidade, rastreabilidade e respeito técnico**.

---

## 🏆 6. Programa de Recompensas (Bug Bounty)

A FireChain recompensa contribuições reais com seu token nativo **FIRE**, sem burocracia, KYC ou intermediários.

| Severidade | Exemplos Técnicos                                | Recompensa Estimada (FIRE) |
|------------|--------------------------------------------------|-----------------------------|
| ⚠️ Baixa   | Má validação de argumentos, hash fraco           | 50–100 FIRE                |
| 🚨 Média   | Vazamento de chaves privadas, bypass de segurança| 200–400 FIRE               |
| 🔥 Crítica | Exploit funcional, controle total da CLI         | 500–1000+ FIRE + destaque  |

Recompensas são pagas **on-chain**, com prova pública de transferência.

---

## 🤝 7. Código de Ética do Pesquisador

Para qualificação no programa:

- ✅ Conduta responsável e ética
- ✅ Divulgação coordenada (responsible disclosure)
- ✅ Reprodutibilidade técnica mínima
- ❌ Nunca publicar vulnerabilidades antes da correção oficial

**Colaboradores éticos são tratados como parceiros, não adversários.**

---

## 💼 8. Conformidade e Licenciamento

- Este projeto é licenciado sob os termos da [Licença MIT](./LICENSE.md)
- Para usos comerciais, consultar [COMMERCIAL_LICENSE_FIRECHAIN.md](./COMMERCIAL_LICENSE_FIRECHAIN.md)
- A política de segurança integra o modelo de **compliance técnico e jurídico da FireChain CLI**

---

## ⚖️ 9. Validade Jurídica

Esta política possui **validade jurídica vinculante**.  
Seu conteúdo está em conformidade com os seguintes marcos regulatórios e normativos:

- 🇧🇷 Legislação Brasileira (Lei nº 9.609/98 – Software)
- 🇧🇷 Lei Geral de Proteção de Dados – LGPD (Lei nº 13.709/18)
- 🇪🇺 GDPR (EU) – Art. 32, 33 e 34
- 📘 ISO/IEC 27001:2022 – Gestão de Segurança da Informação
- 📘 ISO/IEC 27400:2022 – Cibersegurança em IoT
- 🛠️ OWASP Top 10 – Boas práticas de segurança de software
- 📜 RFC 6979 – Deterministic ECDSA

---

## 📬 10. Contato Técnico e Legal

Responsável: **Guilherme Lima** – Arquiteto e mantenedor do projeto

- 🔗 [LinkedIn – Contato direto](https://www.linkedin.com/in/guilhermelimadev-web3/)
- 📩 Solicitações formais e relatórios avançados podem ser discutidos sob NDA

---

**FireChain CLI © Guilherme Lima 2025 – Segurança não é uma opção, é fundação.**