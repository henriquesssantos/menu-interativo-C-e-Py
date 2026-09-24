# Documentacao do Sistema de Gerenciamento de Notas em C

## 1. Objetivo

O programa implementa um sistema interativo para cadastro de alunos e tres notas por aluno. O sistema calcula medias, identifica a maior e a menor nota, conta aprovados e oferece funcionalidades extras em um submenu.

Os dados sao armazenados em vetores paralelos:

- `aluno[i]`: nome do aluno;
- `n1[i]`, `n2[i]` e `n3[i]`: notas do mesmo aluno;
- `total_aluno`: quantidade de alunos cadastrados.

O indice `i` mantem o nome e as notas sincronizados. Por exemplo, `aluno[2]` corresponde a `n1[2]`, `n2[2]` e `n3[2]`.

## 2. Atendimento aos requisitos do trabalho

### Menu e controle de fluxo

O menu principal esta implementado na funcao `exibir_menu()` com:

- laco `do-while`, conforme exigido para C;
- selecao de operacoes com `switch`;
- opcao de encerramento pelo valor `0`.

O submenu tambem usa `do-while` e permite retornar ao menu principal pela opcao `0`.

### Vetores paralelos e cadastro

O cadastro e feito pela funcao `Cadastrar_aluno_nota()`. O indice dos novos alunos e calculado a partir de `total_aluno`, evitando sobrescrever registros existentes.

A funcao limita o cadastro a dez alunos e armazena cada nome e suas tres notas no mesmo indice.

As entradas sao validadas por funcoes auxiliares:

- quantidades negativas sao rejeitadas;
- entradas que nao sao numeros sao rejeitadas;
- notas fora do intervalo de `0` a `10` sao rejeitadas.

### Funcoes e modularizacao

O programa possui 23 funcoes definidas, contando `main()`:

- 22 funcoes de apoio e operacao do sistema;
- 1 funcao principal, `main()`.

Esse numero supera o minimo de seis funcoes solicitado. Cada funcao possui uma responsabilidade especifica, como cadastro, validacao, calculo, busca ou controle de menu.

### Calculos iterativos

Os calculos tradicionais usam lacos `for`:

- media geral da turma;
- media individual por aluno;
- maior e menor nota;
- quantidade de aprovados.

A aprovacao considera media maior ou igual a `7.0`.

### Recursividade

O programa possui tres funcoes recursivas:

1. `somatorio_notas_recursivo()` soma as tres notas de todos os alunos;
2. `contar_aprovados_recursivo()` conta os alunos cuja media e maior ou igual a `7.0`;
3. `listar_invertido_recursivo()` exibe os alunos do ultimo indice ate o primeiro.

As duas primeiras sao usadas nas estatisticas da opcao 7, sem substituir a recursao por um laco. A terceira e acionada pela opcao 3 do submenu.

### Organizacao e legibilidade

O codigo possui comentarios por grupos de responsabilidade:

- dados paralelos;
- entrada e validacao;
- cadastro;
- calculos iterativos;
- recursividade;
- busca e estatisticas complementares;
- submenu e menu principal.

A exibicao dos alunos usa uma linha separada para cada nota, facilitando a leitura.

## 3. Funcoes utilizadas

### Entrada, validacao e cadastro

| Funcao | Responsabilidade |
|---|---|
| `verificar_alunos()` | Verifica se existe pelo menos um aluno cadastrado antes de executar operacoes de consulta. |
| `ler_inteiro()` | Le um numero inteiro e trata entradas que nao sao numericas. |
| `ler_quantidade()` | Le a quantidade de alunos e rejeita valores negativos. |
| `limpar_entrada()` | Remove caracteres restantes do teclado depois de uma leitura. |
| `ler_nota()` | Le uma nota numerica e encaminha o valor para validacao de intervalo. |
| `Verificar_nota()` | Garante que a nota esteja entre `0` e `10`, repetindo a leitura quando necessario. |
| `Cadastrar_aluno_nota()` | Cadastra nomes e tres notas, mantendo os vetores paralelos sincronizados. |

### Listagens e calculos

| Funcao | Responsabilidade |
|---|---|
| `exibir_alunos()` | Exibe todos os alunos e suas respectivas notas. |
| `media_geral()` | Calcula a media de todas as notas da turma usando um laco. |
| `media_por_aluno()` | Calcula e exibe a media individual de cada aluno. |
| `maior_menor()` | Encontra a maior e a menor nota cadastrada. |
| `aprovados()` | Conta os alunos aprovados usando medias calculadas em um laco. |

### Recursividade e estatisticas

| Funcao | Responsabilidade |
|---|---|
| `somatorio_notas_recursivo()` | Soma recursivamente as notas de todos os alunos. |
| `contar_aprovados_recursivo()` | Conta recursivamente os alunos aprovados. |
| `estatisticas_recursivas()` | Usa as duas funcoes recursivas anteriores para exibir media geral e aprovados. |
| `estatisticas_complementares()` | Calcula o percentual de aprovacao da turma. |
| `listar_invertido_recursivo()` | Exibe recursivamente os alunos em ordem inversa. |
| `exibir_listagem_invertida()` | Valida a existencia de alunos e inicia a listagem recursiva. |

### Busca e menus

| Funcao | Responsabilidade |
|---|---|
| `buscar_aluno_por_nome()` | Faz uma busca sequencial pelo nome e exibe as notas encontradas. |
| `voltar_menu_principal()` | Exibe a mensagem de retorno ao menu principal. |
| `sub_menu()` | Controla as funcionalidades extras do sistema. |
| `exibir_menu()` | Controla o menu principal, o `switch` e o encerramento do programa. |
| `main()` | Inicia a execucao chamando `exibir_menu()`. |

## 4. Organizacao das opcoes do menu

### Menu principal

| Opcao | Funcao chamada |
|---|---|
| `1` | `Cadastrar_aluno_nota()` |
| `2` | `exibir_alunos()` |
| `3` | `media_geral()` |
| `4` | `media_por_aluno()` |
| `5` | `maior_menor()` |
| `6` | `aprovados()` |
| `7` | `estatisticas_recursivas()` |
| `8` | `sub_menu()` |
| `0` | Encerra o menu principal |

### Submenu

| Opcao | Funcao chamada |
|---|---|
| `1` | `buscar_aluno_por_nome()` |
| `2` | `estatisticas_complementares()` |
| `3` | `exibir_listagem_invertida()` |
| `0` | `voltar_menu_principal()` e retorno ao menu principal |


## 5. Compilacao e testes

Comando utilizado para compilar com avisos rigorosos:

```sh
gcc -std=c11 -Wall -Wextra -Wpedantic -g3 c-henrique.c -o output/c-henrique
```

Foram testados:

- cadastro de alunos e notas;
- consultas sem alunos cadastrados;
- medias, maior e menor nota;
- contagem de aprovados;
- estatisticas recursivas;
- busca encontrada e nao encontrada;
- estatisticas complementares;
- listagem invertida;
- retorno do submenu;
- opcao invalida;
- quantidade nao numerica;
- quantidade negativa;
- nota nao numerica;
- nota fora do intervalo permitido.

O codigo C foi compilado sem erros ou avisos com o comando acima.

## 6. Versao equivalente em Python

O arquivo `python-henrique.py` possui uma implementacao equivalente ao
programa em C. Ele reproduz:

- cadastro e validacao de alunos e notas;
- medias, maior e menor nota e contagem de aprovados;
- somatorio e contagem recursivos;
- busca sequencial por nome;
- percentual de aprovacao;
- listagem invertida recursiva;
- menu principal e submenu.

As diferencas de sintaxe seguem as caracteristicas de cada linguagem: o C
usa `do-while` e `switch`, enquanto o Python usa `while` e `if-elif`.

O Python foi executado com um cenario de dois alunos e apresentou os mesmos
resultados principais do C.
