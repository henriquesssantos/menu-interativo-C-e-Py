# Sistema de Gerenciamento de Notas: Relatório Técnico

**EQUIPE:** FRANCO LUDVIG, HENRIQUE FERNANDES, JOEL ROBLES

**Disciplina:** : Estruturas de Dados
**Atividade:** : A1  Avaliação Prática em Equipe
**Entregas:** :`av1.cpp` (C++), `av1.py` (Python), `av1.rs` (Rust) + `pyinput_unificado.rs` >> Sendo `pyinput_unificado.rs` um conjunto de utilitátios que evita boylerplates e replicar em Rust o mesmo comportamento da função "input()" do Python, desse modo aumentando a produtividade, reduzindo grau de dificuldade e economizando linhas evitando uso de replicado de códigoss obrigatórios de Rust para entrada de dados via teclado.

> Nota sobre o escopo: a atividade pede C e Python. A versão em **C++** foi entregue no lugar de C por já ser prática recorrente do autor nas aulas do professor (com o conhecimento dele). A linguagem **Rust** foi incluído como terceira implementação, por interesse pessoal em aprofundar a linguagem mantendo, em todas as três, a mesma lógica e o mesmo comportamento observável.

## 1. Ideia central do algoritmo (independente de linguagem)

O sistema gira em torno de **registros paralelos**: cada aluno ocupa a mesma posição `i` em todos os vetores (nome, N1, N2, N3, média). Não existe uma "struct de aluno" nas versões C++/Python — a correspondência é 100% posicional, exigindo que toda inserção, leitura ou remoção mantenha os vetores sincronizados. Em Rust, os mesmos vetores paralelos foram agrupados dentro de uma `struct Turma` apenas para evitar passar 5 parâmetros soltos entre funções — a lógica de indexação permanece idêntica.

O menu principal roda em laço contínuo (`do-while` em C++, `while True` em Python, `while` em Rust) até que o usuário confirme a saída R01.

## 2. Onde entra a iteração (R04)

Quatro operações são deliberadamente **iterativas**, por exigência do escopo:

- Média da turma (soma acumulada / total)
- Maior e menor média (varredura linear com comparação)
- Contagem/listagem de aprovados (nota final ≥ 7.0)
- (extra) Estatísticas complementares: percentual de aprovação e contagem de alunos acima da média

Todas seguem o mesmo padrão: checagem de "lista vazia" → laço `for` que acumula ou compara → impressão do resultado.

## 3. Onde entra a recursão (R05)

A ideia usada em todas as funções recursivas do projeto é a mesma discutida no material de apoio ("Recursividade na sua Simplicidade"): **reduzir o problema a uma versão menor de si mesmo até atingir um caso-base**, e deixar o "retorno" (a volta da pilha de chamadas) fazer o trabalho de acumular ou compor o resultado.

Funções recursivas puras implementadas (mínimo exigido: 2 — entregues: **4**):

| Função | Caso-base | Redução | O que acontece na volta |
|---|---|---|---|
| `somaMediasAprovadosRecursivo(indice)` | `indice == total` → retorna 0 | `indice + 1` | soma a média do aluno atual (se aprovado) ao resultado das chamadas seguintes |
| `listarAprovadosRecursivo(indice)` | `indice == total` → retorna 0 | `indice + 1` | imprime o aluno aprovado *na descida* e soma o contador *na volta* |
| `somaDigitosRecursivo(n)` *(extra)* | `n == 0` → retorna 0 | `n / 10` | soma o dígito das unidades (`n % 10`) ao resultado da chamada com o número truncado |
| `exibirInvertidoRecursivo(indice)` *(extra)* | `indice < 0` → retorna | `indice - 1` | imprime o aluno *antes* de chamar o próximo índice — como a impressão acontece na descida (não na volta), a ordem final observada é invertida |

As duas primeiras satisfazem diretamente o requisito R05 (chamadas dentro de `emitirEstatisticas`, mostrando **apenas os aprovados**, conforme pedido no enunciado). As duas últimas foram adicionadas como "Módulos Recursivos Adicionais", exatamente como sugerido na seção 7 do documento da atividade (soma de dígitos e exibição invertida da listagem).

## 4. Funcionalidades extras implementadas

Seguindo a seção "Funcionalidades EXTRAS e Oportunidades de Ampliação" do enunciado, foram adicionadas — sem alterar nenhuma linha da lógica ou do menu já existentes:

1. **Busca Sequencial** (já existente na base original, mantida).
2. **Validação Modular de Entradas** — função `validarNota` isolada, chamada durante o cadastro; rejeita em loop qualquer nota fora do intervalo `[0.0, 10.0]` antes de aceitar o valor.
3. **Estatísticas Complementares Iterativas** — percentual de aprovação da turma e total de alunos com média acima da média geral (requer que a opção 3 do menu principal já tenha sido executada; caso contrário, o sistema avisa isso ao usuário em vez de assumir um valor arbitrário).
4. **Módulos Recursivos Adicionais** — soma dos dígitos da média de um aluno buscado por nome, e exibição da listagem de alunos em ordem invertida, ambos 100% recursivos (sem laços).

Essas quatro funcionalidades vivem dentro do submenu `[8] Funcionalidades Extras`, que teve suas opções ampliadas de 2 para 5 (`[1]` a `[4]` + `[0]` voltar), sem tocar em nenhuma outra parte do fluxo do menu principal.

## 5. Decisões de implementação por linguagem

- **Sem protótipos/forward declarations**: em C++ (a única das três que exige declaração prévia para chamadas "para frente"), todas as funções foram reordenadas de forma que cada uma é definida **antes** de ser chamada por outra — da função mais "folha" (`validarNota`, `calcMediaAluno`) até `showMenu`, com `main()` por último. Python resolve isso em tempo de execução e Rust não exige ordem alguma dentro do mesmo módulo; a ordem dos arquivos foi mantida igual à original por consistência de leitura.
- **`mediaTurma` como estado persistente**: em C++ e Python essa variável já era global. Em Rust, para manter exatamente o mesmo comportamento (saber se a média da turma já foi calculada, usado pelas Estatísticas Complementares), foi adicionado o campo `media_turma: Option<f64>` à `struct Turma` — a única alteração estrutural do arquivo, necessária para paridade real entre as três linguagens.
- **Comentários**: mantidos no mesmo estilo enxuto do código original — uma linha por função, apenas nos pontos que explicam *por que* (caso-base, redução, papel na regra R04/R05), sem bloco de comentários redundante.

## 6. Equivalência entre as três versões

As três implementações produzem exatamente as mesmas saídas para as mesmas entradas: mesmos textos de menu, mesmas mensagens de erro/confirmação, mesma ordem de exibição em cada opção. A tabela abaixo resume o mapeamento de nomes (estilo `camelCase` em C++/Python, `snake_case` idiomático em Rust única diferença sintática):

| C++   | Python| Rust           |
|-------|---------|
| `validarNota` | `validar_nota` |
| `cadastrosAlunosEnotas` | `cadastros_alunos_e_notas` |
| `estatisticasComplementares` | `estatisticas_complementares` |
| `somaDigitosRecursivo` | `soma_digitos_recursivo` |
| `somaDigitosMediaRecursivo` | `soma_digitos_media_recursivo` |
| `exibirInvertidoRecursivo` / `exibirListagemInvertida` | `exibir_invertido_recursivo` / `exibir_listagem_invertida` |

## 7. Testes de mesa recomendados

Cadastrar 5 alunos com notas conhecidas, conferir manualmente: média individual, média da turma, maior/menor, lista de aprovados, e então validar as duas funções recursivas originais (soma e contagem de aprovados) e as duas extras (soma de dígitos de uma média específica e a listagem invertida) contra o cálculo manual.
