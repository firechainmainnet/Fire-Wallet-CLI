
# 🔐 Política Oficial de Segurança – FireChain CLI

**Projeto:** FireChain CLI  
**Autor:** Guilherme Lima  
**Versão:** 2.1 (2025)  
**Jurisdicação Legal:** República Federativa do Brasil  
**Conformidade Técnica:**  
ISO/IEC 27001:2022, ISO/IEC 27002:2022, ISO/IEC 27400:2022, ISO/IEC 29100, ISO/IEC 29147, ISO/IEC 30111,  
NIST SP 800-63B, NIST SP 800-90A, OWASP Top 10, CWE Top 25, GDPR (EU), LGPD (BR), RFC 6979, RFC 4880, RFC 8446,  
PCI DSS v4.0, SOC 2 (Trust Service Principles), HIPAA (aplicações compatíveis)

---

## 🧾 1. Compromisso Institucional

A FireChain CLI adota o princípio de **segurança por design**, com execução 100% local, sem dependências de rede, sem REST e com criptografia aplicada.  
Esta política formaliza os processos de resposta, tratamento de vulnerabilidades e responsabilidade institucional com base em normas de segurança reconhecidas internacionalmente.

---

## 🧠 2. Escopo Técnico de Interesse

Vulnerabilidades relevantes incluem, mas não se limitam a:

- Falhas em ECDSA secp256k1
- Vazamentos de chave privada (direto ou via side-channel)
- Quebra de `AES-256-GCM`, `HMAC`, `Argon2id`
- Colisões ou fraquezas em `SHA-256`, `Keccak`, `RIPEMD160`
- Injeção via `STDIN` ou vetores CLI
- Mau funcionamento de fingerprint/wallet ID

Todos os testes devem ser reproduzíveis, éticos e tecnicamente embasados.

---

## 🚫 3. Fora de Escopo

Excluem-se da política:

- Bugs visuais, UX ou CLI
- Scanners automáticos sem PoC
- Falhas dependentes de shell ou hardware
- Abusos voluntários com `--unsafe-*`
- Demandas fora do modelo de ameaça

---

## 🛡️ 4. Processo Seguro de Reporte

### 📫 Canais Oficiais

- Mensagem privada via [LinkedIn](https://www.linkedin.com/in/guilhermelimadev-web3/)
- Issues privadas no GitHub com tag `security` (criptografadas)

### 📋 Requisitos

- Prova de conceito
- Comportamento reprodutível
- Explicação clara e técnica

🚫 **Divulgação pública sem coordenação resultará em desqualificação e possível ação legal.**

---

## ⏱️ 5. SLA e Resposta Técnica

| Severidade | Resposta       | Correção                 |
|------------|----------------|--------------------------|
| 🔥 Crítica | ≤ 24h úteis    | Mitigação ou hotfix      |
| 🚨 Média   | ≤ 72h úteis    | Próxima release          |
| ⚠️ Baixa   | ≤ 5 dias úteis | Em versão futura         |

---

## 🏆 6. Bug Bounty – Recompensa em $FIRE

| Severidade | Exemplo Técnico                          | Recompensa em FIRE      |
|------------|-------------------------------------------|--------------------------|
| ⚠️ Baixa   | Validação de argumento fraca             |  0–100 FIRE              |
| 🚨 Média   | Bypass de autenticação, vazamento        | 200–400 FIRE             |
| 🔥 Crítica | Controle total, leitura de `.wallet`     | 500–1000+ FIRE + destaque|

---

## 🧬 7. Código de Ética

Para elegibilidade:

- Ethical disclosure
- Conduta profissional
- Não publicar antes da correção
- PoC funcional obrigatória

---

## 📐 8. Validação Criptográfica Interna

- `secp256k1` com geração randômica via `rand_core`
- `fingerprint` por SHA-256 → RIPEMD160 → Base58
- `wallet_id` derivado verificável
- `.wallet` criptografado com AES-GCM + HMAC
- `Argon2id` com parâmetros seguros para hashing de senha
- Zeroização de memória com `zeroize`
- Parâmetros auditáveis (sem `unsafe`)

---

## 🚫 9. Política Anti-Backdoor

A FireChain CLI:

- Não possui fallback oculto
- Não inclui telemetria
- Não realiza chamadas externas
- Não possui portas REST
- É validada por `cargo audit`, `deny`, `geiger`

---

## 💼 10. Conformidade e Licenciamento

- Licença: [MIT com restrições comerciais](./LICENSE.md)
- Licença comercial: [COMMERCIAL_LICENSE_FIRECHAIN.md](./COMMERCIAL_LICENSE_FIRECHAIN.md)
- Política de segurança é parte integral do compliance FireChain

---

## ⚖️ 11. Validade Jurídica

Este documento tem validade legal com base em:

- Lei nº 9.609/98 (Software)
- Lei nº 13.709/18 (LGPD)
- GDPR (EU) – Art. 32–34
- ISO/IEC 27001, 27400, 29100, 29147, 30111
- RFCs e padrões criptográficos aplicáveis

---

## 📬 12. Contato Técnico e Legal

Responsável: Guilherme Lima  
🔗 [LinkedIn – Contato direto](https://www.linkedin.com/in/guilhermelimadev-web3/)  
📩 Canal de resposta avançada disponível sob NDA

---

**FireChain CLI © Guilherme Lima 2025 – Soberania não é recurso. É fundação.**
