#regra: ter pelo menos 7 funções e pelo menos 2 sendo recursivas, trabalhar com listas/vetores paralelos (vetor para aluno, vetor para notas)
#regra: a recursividade não pode ser substituída por laços de repetição
#regra: emitir estatistica via função recursiva/ apenas mostrar os aprovados
#Nomes/notas/medias por aluno


nomeAluno = []
notaN1 = []
notaN2 = []
notaN3 = []
mediaFinal = []

#media da turma
mediaTurma = None


def cadastrosAlunosEnotas():
    pass

def exibirListagemAlunos_Notas():
    pass

def exibirListagemMediaAluno():
    pass

def calcMediaAluno():
    pass

def calcMediaTurma():
    pass

def encontrarMaiorMenorNotasRegistradas():
    pass

def contarListarAprovados():
    pass

def subMenu():
    pass

def sair():
    opcao = str(input(f"Deseja mesmo sair? S/n": ))

    if opcao == "S" or opcao == "s":
        exit

    if opcao == "N" or opcao == "n":
        showMenu()
    pass

#Função responsável pelo menu principal
def showMenu():
    print("========BEM VINDO AO SISTEMA PYTHON DE NOTAS=======")
    print("[1] Cadastras alunos e notas")
    print("[2] Exibir listagem de alunos")
    print("[3] Calcular e exibir média geral da turma")
    print("[4] Exibir listagem de médias por aluno")
    print("[5] Encontrar a maior e menor nota registrada")
    print("[6] Contar e listar alunos aprovados (Média >= 7.0)")
    print("[7] Emitir estatísticas")
    print("[0] Encerrar aplicação\n")
    print("*" * 50)

    opcao = int(input(f"Digite o número da opção desejada: "))

    if opcao == 1:
        pass
    if opcao == 2:
        pass
    if opcao == 3:
        pass
    if opcao == 4:
        pass
    if opcao == 5:
        pass
    if opcao == 6:
        pass
    if opcao == 7:
        pass
    if opcao == 0:
        sair()
        pass


#Ponto de entrada do programa
showMenu()