mod pyrsutils;
use pyrsutils::prelude::*;

// struct que representa os vetores paralelos (nomes/notas), declarada
// no estilo C++/Python via a macro class! da pyrsutils
class! {
    struct Turma {
        nomes: Vec<String>,
        n1: Vec<f64>,
        n2: Vec<f64>,
        n3: Vec<f64>,
        medias: Vec<f64>,
        media_turma: Option<f64>
    }
    impl {}
}

// validação modular: garante nota dentro do intervalo 0.0 - 10.0 (extra)
fn validar_nota(prompt: &str) -> f64
{
    loop
    {
        let valor: f64 = input(prompt);
        if valor >= 0.0 && valor <= 10.0
        {
            return valor;
        }
        println!("Nota inválida! Digite um valor entre 0.0 e 10.0.");
    }
}

// cadastro em lote: pede a quantidade e preenche os vetores paralelos
fn cadastros_alunos_e_notas(turma: &mut Turma)
{
    let qtd: usize = input("Quantos alunos deseja cadastrar? ");

    for i in range(qtd as i64)
    {
        let nome: String = input(&format!("Nome do aluno {}: ", i + 1));
        let n1 = validar_nota("Nota N1: ");
        let n2 = validar_nota("Nota N2: ");
        let n3 = validar_nota("Nota N3: ");

        turma.nomes.append_item(nome);
        turma.n1.append_item(n1);
        turma.n2.append_item(n2);
        turma.n3.append_item(n3);
        turma.medias.append_item(calc_media_aluno(n1, n2, n3));
    }

    println!("\n{} aluno(s) cadastrado(s) com sucesso!\n", qtd);
}

fn exibir_listagem_alunos_notas(turma: &Turma)
{
    if turma.nomes.is_empty()
    {
        println!("\nNenhum aluno cadastrado.\n");
        return;
    }

    println!("\n--- Listagem de Alunos e Notas ---");
    for i in range(len(&turma.nomes[..]) as i64)
    {
        let i = i as usize;
        println!("{} | N1: {:.2} | N2: {:.2} | N3: {:.2}", turma.nomes[i], turma.n1[i], turma.n2[i], turma.n3[i]);
    }
    println!();
}

fn exibir_listagem_media_aluno(turma: &Turma)
{
    if turma.nomes.is_empty()
    {
        println!("\nNenhum aluno cadastrado.\n");
        return;
    }

    println!("\n--- Médias por Aluno ---");
    for i in range(len(&turma.nomes[..]) as i64)
    {
        let i = i as usize;
        println!("{} - Média: {:.2}", turma.nomes[i], turma.medias[i]);
    }
    println!();
}

// média individual, usada no momento do cadastro
fn calc_media_aluno(n1: f64, n2: f64, n3: f64) -> f64
{
    (n1 + n2 + n3) / 3.0
}

// média da turma via laço (R04 - iterativo)
fn calc_media_turma(turma: &mut Turma)
{
    if turma.medias.is_empty()
    {
        println!("\nNenhum aluno cadastrado.\n");
        return;
    }

    let mut soma = 0.0;
    for media in &turma.medias
    {
        soma += media;
    }

    let media_turma = soma / len(&turma.medias[..]) as f64;
    turma.media_turma = Some(media_turma);
    println!("\nMédia geral da turma: {:.2}\n", media_turma);
}

// maior/menor via laço (R04 - iterativo)
fn encontrar_maior_menor_notas_registradas(turma: &Turma)
{
    if turma.medias.is_empty()
    {
        println!("\nNenhum aluno cadastrado.\n");
        return;
    }

    let mut maior = turma.medias[0];
    let mut menor = turma.medias[0];

    for &media in &turma.medias
    {
        if media > maior { maior = media; }
        if media < menor { menor = media; }
    }

    println!("\nMaior média registrada: {:.2}", maior);
    println!("Menor média registrada: {:.2}\n", menor);
}

// contagem/listagem de aprovados via laço (R04 - iterativo)
fn contar_listar_aprovados(turma: &Turma)
{
    if turma.medias.is_empty()
    {
        println!("\nNenhum aluno cadastrado.\n");
        return;
    }

    let mut contador = 0;
    println!("\n--- Alunos Aprovados (Média >= 7.0) ---");
    for i in range(len(&turma.nomes[..]) as i64)
    {
        let i = i as usize;
        if turma.medias[i] >= 7.0
        {
            println!("{} - Média: {:.2}", turma.nomes[i], turma.medias[i]);
            contador += 1;
        }
    }
    println!("Total de aprovados: {}\n", contador);
}

// soma recursiva das médias dos aprovados (caso base: indice == tamanho do vetor)
fn soma_medias_aprovados_recursivo(medias: &Vec<f64>, indice: usize) -> f64
{
    if indice == len(&medias[..])
    {
        return 0.0;
    }

    let soma = if medias[indice] >= 7.0 { medias[indice] } else { 0.0 };
    soma + soma_medias_aprovados_recursivo(medias, indice + 1)
}

// lista e conta recursivamente apenas os aprovados (caso base: indice == tamanho do vetor)
fn listar_aprovados_recursivo(turma: &Turma, indice: usize) -> i32
{
    if indice == len(&turma.nomes[..])
    {
        return 0;
    }

    let mut aprovados = 0;
    if turma.medias[indice] >= 7.0
    {
        println!("{} - Média: {:.2}", turma.nomes[indice], turma.medias[indice]);
        aprovados = 1;
    }

    aprovados + listar_aprovados_recursivo(turma, indice + 1)
}

// estatísticas emitidas 100% via recursão, mostrando apenas os aprovados
fn emitir_estatisticas(turma: &Turma)
{
    if turma.nomes.is_empty()
    {
        println!("\nNenhum aluno cadastrado.\n");
        return;
    }

    println!("\n--- Estatísticas (via recursão) ---");
    let total = listar_aprovados_recursivo(turma, 0);

    if total == 0
    {
        println!("Nenhum aluno aprovado.\n");
        return;
    }

    let soma = soma_medias_aprovados_recursivo(&turma.medias, 0);
    println!("Total de aprovados: {}", total);
    println!("Soma das médias dos aprovados: {:.2}", soma);
    println!("Média dos aprovados: {:.2}\n", soma / total as f64);
}

// busca sequencial simples, funcionalidade extra
fn busca_sequencial_aluno(turma: &Turma)
{
    let nome_busca: String = input("Digite o nome do aluno: ");

    for i in range(len(&turma.nomes[..]) as i64)
    {
        let i = i as usize;
        if turma.nomes[i].to_lowercase() == nome_busca.to_lowercase()
        {
            println!("\nAluno encontrado! Média: {:.2}\n", turma.medias[i]);
            return;
        }
    }

    println!("\nAluno não encontrado.\n");
}

// estatísticas complementares iterativas: % de aprovação e total acima da média (extra)
fn estatisticas_complementares(turma: &Turma)
{
    if turma.medias.is_empty()
    {
        println!("\nNenhum aluno cadastrado.\n");
        return;
    }

    let mut aprovados = 0;
    for &media in &turma.medias
    {
        if media >= 7.0 { aprovados += 1; }
    }
    let percentual = (aprovados as f64 / len(&turma.medias[..]) as f64) * 100.0;

    println!("\n--- Estatísticas Complementares ---");
    println!("Percentual de aprovação: {:.2}%", percentual);

    match turma.media_turma
    {
        Some(media_turma) =>
        {
            let mut acima_media = 0;
            for &media in &turma.medias
            {
                if media > media_turma { acima_media += 1; }
            }
            println!("Alunos acima da média da turma: {}\n", acima_media);
        }
        None => println!("Calcule a média da turma (opção 3) para ver quantos estão acima dela.\n"),
    }
}

// soma recursiva dos dígitos de um número (caso base: n == 0) - módulo recursivo extra
fn soma_digitos_recursivo(n: i64) -> i64
{
    if n == 0
    {
        return 0;
    }
    n % 10 + soma_digitos_recursivo(n / 10)
}

// busca o aluno e aciona a soma recursiva dos dígitos da média dele (extra)
fn soma_digitos_media_recursivo(turma: &Turma)
{
    let nome_busca: String = input("Digite o nome do aluno: ");

    for i in range(len(&turma.nomes[..]) as i64)
    {
        let i = i as usize;
        if turma.nomes[i].to_lowercase() == nome_busca.to_lowercase()
        {
            let digitos = turma.medias[i].round() as i64;
            println!("\nSoma dos dígitos da média de {} ({}): {}\n", turma.nomes[i], digitos, soma_digitos_recursivo(digitos));
            return;
        }
    }

    println!("\nAluno não encontrado.\n");
}

// exibição invertida via recursão (caso base: indice < 0) - módulo recursivo extra
fn exibir_invertido_recursivo(turma: &Turma, indice: i32)
{
    if indice < 0
    {
        return;
    }

    println!("{} - Média: {:.2}", turma.nomes[indice as usize], turma.medias[indice as usize]);
    exibir_invertido_recursivo(turma, indice - 1);
}

fn exibir_listagem_invertida(turma: &Turma)
{
    if turma.nomes.is_empty()
    {
        println!("\nNenhum aluno cadastrado.\n");
        return;
    }

    println!("\n--- Listagem Invertida (via recursão) ---");
    exibir_invertido_recursivo(turma, len(&turma.nomes[..]) as i32 - 1);
    println!();
}

fn sub_menu(turma: &Turma)
{
    println!("\n--- SUBMENU: Funcionalidades Extras ---");
    println!("[1] Busca sequencial por nome");
    println!("[2] Estatísticas complementares (aprovação % e acima da média)");
    println!("[3] Soma dos dígitos da média de um aluno (recursivo)");
    println!("[4] Exibir listagem invertida (recursivo)");
    println!("[0] Voltar ao menu principal\n");

    let opcao: i32 = input("Escolha uma opção: ");

    match opcao
    {
        1 => busca_sequencial_aluno(turma),
        2 => estatisticas_complementares(turma),
        3 => soma_digitos_media_recursivo(turma),
        4 => exibir_listagem_invertida(turma),
        _ => {}
    }
}

fn sair() -> bool
{
    let opcao: String = input("Deseja mesmo sair? S/n ");
    opcao == "S" || opcao == "s"
}

// Função responsável pelo menu principal (laço contínuo - R01)
fn show_menu()
{
    let mut turma = Turma::new(Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), None);
    let mut continuar = true;

    while continuar
    {
        println!("\n========BEM VINDO AO SISTEMA RUST DE NOTAS=======");
        println!("[1] Cadastras alunos e notas");
        println!("[2] Exibir listagem de alunos");
        println!("[3] Calcular e exibir média geral da turma");
        println!("[4] Exibir listagem de médias por aluno");
        println!("[5] Encontrar a maior e menor nota registrada");
        println!("[6] Contar e listar alunos aprovados (Média >= 7.0)");
        println!("[7] Emitir estatísticas");
        println!("[8] Funcionalidades Extras (Submenu)");
        println!("[0] Encerrar aplicação\n");
        println!("{}", "*".repeat(50));

        let opcao: i32 = input("Digite o número da opção desejada: ");

        match opcao
        {
            1 => cadastros_alunos_e_notas(&mut turma),
            2 => exibir_listagem_alunos_notas(&turma),
            3 => calc_media_turma(&mut turma),
            4 => exibir_listagem_media_aluno(&turma),
            5 => encontrar_maior_menor_notas_registradas(&turma),
            6 => contar_listar_aprovados(&turma),
            7 => emitir_estatisticas(&turma),
            8 => sub_menu(&turma),
            0 => { if sair() { continuar = false; } },
            _ => println!("Opção inválida.\n"),
        }
    }
}

// Ponto de entrada do programa
fn main()
{
    show_menu();
}
