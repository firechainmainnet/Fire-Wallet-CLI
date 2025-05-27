# 🔐 Política de Segurança — FireChain CLI

A segurança da FireChain CLI é parte central do nosso compromisso com a soberania digital. Este documento define as diretrizes para reporte de vulnerabilidades, escopo de análise, políticas de resposta e nosso programa oficial de **Bug Bounty baseado em tokens nativos FireChain**.

---

## 📬 Como reportar uma vulnerabilidade

Se você encontrou um possível problema de segurança envolvendo:

- Derivação de chaves
- Exportação de carteiras
- Fingerprints, verificações ou criptografia
- Argumentos perigosos (`--unsafe-*`)
- Ações que comprometem privacidade, sigilo ou integridade do binário

Por favor, **NÃO publique em issues públicas ou redes sociais.**  
Entre em contato diretamente via:

🔗 [LinkedIn (mensagem direta)](https://www.linkedin.com/in/guilhermelimadev-web3/)  
🛡️ GitHub: crie uma issue privada com a tag `security` se necessário

---

## 🎯 Escopo de interesse

Análises bem-vindas incluem (mas não se limitam a):

- Falhas em derivação ECDSA, seed ou mnemonics
- Vazamento de chaves privadas via CLI
- Falhas na zeroização de memória
- Criptografia insegura ou fraca (ex: AES, Argon2id, futuras integrações)
- Bypass de proteções como `--unsafe-dump`
- Desvios no modo STDIN / JSON (futuro)

---

## 🚫 Fora de escopo

- Bugs visuais ou UX sem impacto de segurança
- Solicitações de features ainda não implementadas
- Comportamentos esperados por design (ex: uso avançado com `--unsafe`)
- Questões relacionadas a sistemas operacionais externos ou shells

---

## ⏱️ SLA de Resposta

- A FireChain CLI responde vulnerabilidades reportadas **em até 72h úteis**.
- Casos críticos podem receber **patch privado antes do anúncio público**.
- Atualizações serão documentadas no `CHANGELOG.md` com agradecimentos (caso autorizado).

---

## 🏆 Bug Bounty (Recompensa de Vulnerabilidade)

A FireChain CLI possui um **programa oficial de Bug Bounty com recompensas reais**, sempre pagas em nosso **token nativo FireChain (FIRE)**.

- Todos os valores são transferidos on-chain com transparência
- Você pode **reter, trocar (swap) ou transferir** os tokens conforme preferir
- Isso é Web3 de verdade: **auto-custódia sem burocracia, sem KYC, sem limitações**

🏅 Recompensas são baseadas em severidade (valores aproximados):

| Nível         | Descrição Técnica                        | Recompensa Estimada (FIRE)     |
|---------------|------------------------------------------|---------------------------------|
| ⚠️ Baixa      | Falhas teóricas ou não exploráveis       | 25–50 FIRE                      |
| 🚨 Média      | Vazamentos indiretos, inconsistência CLI  | 100–250 FIRE                    |
| 🔥 Crítica    | Acesso a chaves, bypass real ou exploit   | 500+ FIRE + nome no release     |

---

## 🤝 Ética & Conduta do Pesquisador

Para receber recompensa e ser reconhecido publicamente, você deve:

- **Não divulgar publicamente antes de correção oficial**
- **Evitar exploração ativa ou uso indevido das falhas descobertas**
- Relatar de forma ética, clara e com reprodutibilidade

---

## 🔐 Um novo padrão em segurança local

A FireChain CLI é o primeiro projeto Web3 que **traz segurança real com auto-custódia local desde o primeiro byte**.  
Nem Bitcoin Core, Metamask ou Ledger oferecem esse nível de controle direto com acesso auditável.

> Se você acredita que encontrou uma falha — colabore.  
> Aqui, recompensamos quem fortalece o que importa.
