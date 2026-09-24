#include <stdio.h>
#include <string.h>

/* Dados paralelos da turma: aluno[i] corresponde a n1[i], n2[i] e n3[i]. */
char aluno[10][15];
int Maximo_de_alunos = 10;
double n1[10], n2[10], n3[10];
int total_aluno;
double media_turma;

/* Prototipos das funcoes de entrada e validacao. */
void limpar_entrada(void);
int ler_inteiro(void);
int ler_quantidade(void);
double ler_nota(void);
double Verificar_nota(double nota);

/* Verifica se existe pelo menos um aluno cadastrado. */
int verificar_alunos(void) {
    if (total_aluno == 0) {
        printf("Nenhum aluno cadastrado.\n");
        return 0;
    }

    return 1;
}

/* Le uma quantidade inteira e trata entradas que nao sao numeros. */
int ler_inteiro(void) {
    int valor;

    while (scanf("%d", &valor) != 1) {
        printf("Entrada invalida. Digite um numero inteiro: ");
        limpar_entrada();
    }

    limpar_entrada();
    return valor;
}

/* Le uma quantidade nao negativa de alunos. */
int ler_quantidade(void) {
    int quantidade;

    do {
        printf("Quantos alunos deseja cadastrar? ");
        quantidade = ler_inteiro();

        if (quantidade < 0) {
            printf("A quantidade nao pode ser negativa.\n");
        }
    } while (quantidade < 0);

    return quantidade;
}

/* Remove os caracteres restantes da linha de entrada. */
void limpar_entrada(void) {
    int caractere;

    while ((caractere = getchar()) != '\n' && caractere != EOF) {
    }
}

/* Le uma nota numerica e garante que ela esteja entre 0 e 10. */
double ler_nota(void) {
    double nota;

    while (scanf("%lf", &nota) != 1) {
        printf("Entrada invalida. Digite uma nota entre 0 e 10: ");
        limpar_entrada();
    }

    limpar_entrada();
    return Verificar_nota(nota);
}

/* Cadastra alunos e mantem nomes e notas no mesmo indice. */
void Cadastrar_aluno_nota(void){
    int qtd = ler_quantidade();

    if (total_aluno + qtd > Maximo_de_alunos){
        qtd = Maximo_de_alunos - total_aluno;
        printf("Limite excedido. Serao cadastrados apenas %d alunos.\n", qtd);
    }

    for (int i = 0; i < qtd; i++) {
        int indice = total_aluno + i;

        printf("Digite o nome do aluno: ");
        scanf("%14s", aluno[indice]);

        printf("Digite a nota 1: ");
        n1[indice] = ler_nota();

        printf("Digite a nota 2: ");
        n2[indice] = ler_nota();

        printf("Digite a nota 3: ");
        n3[indice] = ler_nota();
    }

    total_aluno += qtd;
}

/* Repete a leitura ate receber uma nota no intervalo permitido. */
double Verificar_nota(double nota) {
    while (nota < 0 || nota > 10) {
        printf("Nota invalida. Digite novamente entre 0 e 10: ");

        while (scanf("%lf", &nota) != 1) {
            printf("Entrada invalida. Digite uma nota entre 0 e 10: ");
            limpar_entrada();
        }

        limpar_entrada();
    }

    return nota;
}

/* Exibe cada aluno e suas notas em linhas separadas. */
void exibir_alunos(void){
    if (!verificar_alunos()) {
        return;
    }

    for (int i = 0; i < total_aluno; i++) {
        printf("\n%dº Aluno(a): %s\n", i + 1, aluno[i]);
        printf("  Nota 1: %.2f\n", n1[i]);
        printf("  Nota 2: %.2f\n", n2[i]);
        printf("  Nota 3: %.2f\n", n3[i]);
    }
}

/* Calcula a media geral usando um laco sobre todos os alunos. */
void media_geral(void) {
    double soma = 0;

    if (!verificar_alunos()) {
        return;
    }
    for (int i = 0; i < total_aluno; i++) {
        soma += n1[i] + n2[i] + n3[i];
    }
    media_turma = soma / (3 * total_aluno);
    printf("A media geral da turma e: %.2f\n", media_turma);
}

/* Calcula e exibe a media individual de cada aluno. */
void media_por_aluno(void){
    double media=0;

    if (!verificar_alunos()) {
        return;
    }
    for(int i =0;i<total_aluno;i++){
        media = (n1[i] + n2[i] + n3[i]) / 3;
        printf("A media do aluno %14s, é: %.2f\n",aluno[i], media);
    }
}

/* Encontra a maior e a menor nota com comparacoes iterativas. */
void maior_menor(void) {
    if (!verificar_alunos()) {
        return;
    }
    double maior = n1[0];
    double menor = n1[0];

    for (int i = 0; i < total_aluno; i++) {
        if (n1[i] > maior) maior = n1[i];
        if (n2[i] > maior) maior = n2[i];
        if (n3[i] > maior) maior = n3[i];

        if (n1[i] < menor) menor = n1[i];
        if (n2[i] < menor) menor = n2[i];
        if (n3[i] < menor) menor = n3[i];
    }
    printf("Maior nota: %.2f\n", maior);
    printf("Menor nota: %.2f\n", menor);
}

/* Conta os alunos aprovados usando a media individual. */
void aprovados(void){
    if (!verificar_alunos()) {
        return;
    }
    int aprovados =0;
    double media =0;
    for(int i =0;i<total_aluno;i++){
        media = (n1[i] + n2[i] + n3[i]) / 3;
        if (media >= 7){
            aprovados+=1;
        }
    }
    printf("A quantidade de alunos aprovados foi de %d\n", aprovados);
}

/* Soma recursivamente as tres notas de cada aluno. */
double somatorio_notas_recursivo(int indice) {
    if (indice == total_aluno) {
        return 0;
    }

    return n1[indice] + n2[indice] + n3[indice]
           + somatorio_notas_recursivo(indice + 1);
}

/* Conta recursivamente os alunos com media igual ou superior a 7. */
int contar_aprovados_recursivo(int indice) {
    if (indice == total_aluno) {
        return 0;
    }

    double media = (n1[indice] + n2[indice] + n3[indice]) / 3.0;
    int aprovado = media >= 7.0;

    return aprovado + contar_aprovados_recursivo(indice + 1);
}

/* Exibe as estatisticas que usam as funcoes recursivas. */
void estatisticas_recursivas(void) {
    if (!verificar_alunos()) {
        return;
    }

    double soma = somatorio_notas_recursivo(0);
    double media = soma / (3.0 * total_aluno);
    int quantidade_aprovados = contar_aprovados_recursivo(0);

    printf("Media geral da turma: %.2f\n", media);
    printf("Quantidade de alunos aprovados: %d\n", quantidade_aprovados);
}

/* Busca sequencialmente um aluno pelo nome. */
void buscar_aluno_por_nome(void) {
    char nome_buscado[15];
    int encontrou = 0;

    if (!verificar_alunos()) {
        return;
    }

    printf("Digite o nome do aluno: ");
    scanf("%14s", nome_buscado);

    for (int i = 0; i < total_aluno; i++) {
        if (strcmp(aluno[i], nome_buscado) == 0) {
            printf("Aluno encontrado na posicao %d.\n", i + 1);
            printf("Nota 1: %.2f\n", n1[i]);
            printf("Nota 2: %.2f\n", n2[i]);
            printf("Nota 3: %.2f\n", n3[i]);
            encontrou = 1;
            break;
        }
    }

    if (!encontrou) {
        printf("Aluno nao encontrado.\n");
    }
}

/* Calcula o percentual de aprovacao da turma. */
void estatisticas_complementares(void) {
    if (!verificar_alunos()) {
        return;
    }

    int quantidade_aprovados = contar_aprovados_recursivo(0);
    double percentual_aprovacao =
        (quantidade_aprovados * 100.0) / total_aluno;

    printf("Percentual de aprovacao: %.2f%%\n", percentual_aprovacao);
}

/* Exibe os alunos do ultimo indice ate o primeiro por recursao. */
void listar_invertido_recursivo(int indice) {
    if (indice < 0) {
        return;
    }

    printf("%dº Aluno(a): %s\n", indice + 1, aluno[indice]);
    printf("Nota 1: %.2f | Nota 2: %.2f | Nota 3: %.2f\n",
           n1[indice], n2[indice], n3[indice]);

    listar_invertido_recursivo(indice - 1);
}

/* Inicia a listagem invertida depois de validar a turma. */
void exibir_listagem_invertida(void) {
    if (!verificar_alunos()) {
        return;
    }

    listar_invertido_recursivo(total_aluno - 1);
}

/* Informa que o submenu sera encerrado. */
void voltar_menu_principal(void) {
    printf("Voltando ao menu principal...\n");
}

/* Controla as funcionalidades extras do sistema. */
void sub_menu(void) {
    int opcao;

    do {
        printf("\n--- SUBMENU: Funcionalidades Extras ---\n");
        printf("[1] Busca sequencial por nome\n");
        printf("[2] Estatisticas complementares\n");
        printf("[3] Exibir listagem invertida\n");
        printf("[0] Voltar ao menu principal\n\n");
        printf("Digite a opcao desejada: ");
        opcao = ler_inteiro();

        switch (opcao) {
        case 1:
            buscar_aluno_por_nome();
            break;
        case 2:
            estatisticas_complementares();
            break;
        case 3:
            exibir_listagem_invertida();
            break;
        case 0:
            voltar_menu_principal();
            break;
        default:
            printf("Opcao invalida.\n");
        }
    } while (opcao != 0);
}

/* Controla o fluxo principal do sistema. */
void exibir_menu(void) {
    int contador = 1;
    do {
        printf("\n==========MENU==========\n");
        printf("1. Cadastrar alunos e notas \n");
        printf("2. Exibir listagem de alunos \n");
        printf("3. Calcular e exibir média geral da turma \n");
        printf("4. exibir listagem de média por aluno \n");
        printf("5. Encontrar a maior e menor nota registrada \n");
        printf("6. Contar a lista de alunos aprovados (Média = 7.0) \n");
        printf("7. emitir estatísticas \n");
        printf("8. Submenu \n");
        printf("0. Encerrar aplicação \n");
        printf("----------------------------------------\n");

        int opcao;
        printf("Digite a opção desejada: \n");
        opcao = ler_inteiro();
        switch (opcao) {
        case 1:
            Cadastrar_aluno_nota();
            break;
        case 2:
            exibir_alunos();
            break;
        case 3:
            media_geral();
            break;
        case 4:
            media_por_aluno();
            break;
        case 5:
            maior_menor();
            break;
        case 6:
            aprovados();
            break;
        case 7:
            estatisticas_recursivas();
            break;
        case 8:
            sub_menu();
            break;
        case 0:
            printf("digite 0 para sair");
            contador = contador - 1;
            break;
        }
    } while (contador == 1);
}

int main (void){
    exibir_menu();
}
