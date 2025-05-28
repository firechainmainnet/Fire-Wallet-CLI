# 🔐 Política Oficial de Segurança – FireChain CLI

A segurança é um pilar inegociável da FireChain CLI.  
Nosso compromisso vai além de boas práticas: entregamos **criptografia real, execução auditável e controle local soberano**, em conformidade com os padrões mais exigentes de segurança da informação e soberania digital Web3.

Este documento define nossa **política de resposta a vulnerabilidades**, escopo técnico de análise, conduta ética esperada de pesquisadores e detalhes do nosso **programa de Bug Bounty oficial com tokens FIRE**.

---

## 📬 Reporte Seguro de Vulnerabilidades

Encontrou uma possível falha de segurança?  
**Nunca compartilhe detalhes publicamente.** Utilize um dos canais abaixo:

- 🔐 Mensagem direta no [LinkedIn (Guilherme Lima)](https://www.linkedin.com/in/guilhermelimadev-web3/)
- 🛡️ Issue privada no GitHub com a tag `security`

Não exigimos KYC para colaborador nem formulários extensos: você fala diretamente com o autor e responsável técnico.

---

## 🧠 Escopo Técnico de Interesse

Aceitamos análises responsáveis e reprodutíveis que envolvam:

- Comprometimento na geração ECDSA-secp256k1
- Vazamento de chaves privadas (direto ou via derivação)
- Bypass de zeroização de memória (`zeroize`)
- Fragilidades em hashing (SHA256, Keccak, RIPEMD160)
- Criptografia insegura na exportação `.wallet` (AES-256-GCM, Argon2id)
- Falhas em validação de argumentos CLI (`--unsafe-*`)
- Injeções via STDIN ou manipulação do modo `--json` (planejado)

---

## 🚫 Fora de Escopo

- Bugs de UI/UX ou sugestões de CLI
- Relatórios genéricos com ferramentas automatizadas sem PoC
- Ações esperadas por design quando `--unsafe-*` for usado
- Vulnerabilidades dependentes de sistemas externos (SO, shell, hardware)

---

## ⏱️ Política de Resposta

Nos comprometemos com os seguintes prazos:

| Tipo de Relato     | Tempo de Resposta | Patch/Correção         |
|--------------------|-------------------|-------------------------|
| Crítica (exploit)  | 24h úteis         | Correção imediata       |
| Média              | 48–72h úteis      | Correção prioritária    |
| Baixa              | Até 5 dias úteis  | Correção em release     |

Todos os casos terão acompanhamento pessoal do autor, com confidencialidade garantida.  
Quando autorizado, nomes dos contribuidores serão publicados no `CHANGELOG.md`.

---

## 🏆 Bug Bounty — Recompensas FIRE

Recompensamos quem fortalece nossa stack.  
As recompensas são pagas em nosso token nativo **FIRE** e transferidas on-chain:

| Severidade | Exemplo Técnico                                      | Recompensa Estimada (FIRE) |
|------------|-------------------------------------------------------|-----------------------------|
| ⚠️ Baixa   | Hash fraco, argumentação insegura, observações      | 50–100 FIRE                |
| 🚨 Média   | Vazamento de chave privada via uso comum            | 200–400 FIRE               |
| 🔥 Crítica | Exploit funcional, desvio completo de segurança     | 500–1000+ FIRE + destaque  |

Sem burocracia, sem bancos, sem barreiras.  
Isso é Web3 como deveria ser.

---

## 🤝 Código de Ética do Pesquisador

Para garantir recompensa e reconhecimento público:

- ❌ Não divulgar publicamente antes da correção oficial
- ✅ Manter conduta responsável e ética
- ✅ Garantir reprodutibilidade mínima com descrição clara

Pesquisadores que cooperam de forma construtiva são tratados como parceiros, não adversários.

---

## 📄 Compliance e Licença

Este projeto é licenciado sob os termos da [Licença MIT (Português)](./LICENSE).

Colaboradores que reportam vulnerabilidades e recebem recompensa aceitam os termos desta licença, inclusive a cláusula de **isenção de responsabilidade legal**.

---

## 💬 Última mensagem

Você não precisa ser hacker para entender segurança.  
Mas aqui, recompensamos quem pensa como um — para proteger os demais.

> “Segurança é quando você sabe exatamente o que cada byte faz.”  
> – FireChain™

---

**Guilherme Lima**  
Arquiteto e responsável técnico  
🔗 [LinkedIn](https://www.linkedin.com/in/guilhermelimadev-web3/)
