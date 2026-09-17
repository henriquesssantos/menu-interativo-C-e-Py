# 📊 Sistema de Gerenciamento de Notas (C & Python)

> Projeto prático desenvolvido em equipe para a disciplina de **Estruturas de Dados**. O objetivo é aplicar na prática os conceitos de laços de repetição, vetores/listas paralelas, modularização e recursividade em duas linguagens de programação.

---

## 📌 Sobre o Projeto

O **Sistema de Gerenciamento de Notas** permite o cadastro de alunos e o processamento estatístico do rendimento acadêmico de uma turma. Toda a lógica de negócio foi implementada de forma idêntica e comparativa nas linguagens **C** e **Python**.

---

## 🚀 Funcionalidades Principais

- [x] **Menu Interativo:** Construído com `do-while`/`switch` (C) e `while`/`if-elif` (Python).
- [x] **Cadastro e Listagem:** Gerenciamento de vetores paralelos sincronizados (`nomes[]` e `notas[]`).
- [x] **Métricas da Turma (Iterativas):**
  - Cálculo da média geral.
  - Identificação da maior e menor nota registrada.
  - Contagem e listagem de discentes aprovados ($\text{Nota} \ge 7.0$).
- [x] **Estatísticas Recursivas:** Múltiplas rotinas desenvolvidas sem a utilização de laços tradicionais.
- [x] **Submenu de Ampliações (Extras):**
  - **S1:** Busca sequencial por nome.
  - **S2:** Validação modular de entradas.
  - **S3/S4:** Estatísticas complementares e módulos recursivos principais[cite: 1].

---

## 📁 Estrutura do Repositório

```text
.
├── c/
│   └── main.c           # Código-fonte completo e compilável em C
├── python/
│   └── main.py          # Código-fonte completo e executável em Python
└── README.md            # Documentação do projeto
