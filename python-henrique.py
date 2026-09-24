
MAXIMO_DE_ALUNOS = 10
alunos = []
n1 = []
n2 = []
n3 = []


def verificar_alunos():
	"""Verifica se existe pelo menos um aluno cadastrado."""
	if not alunos:
		print("Nenhum aluno cadastrado.")
		return False
	return True


def ler_inteiro(mensagem):
	"""Le um inteiro e repete a leitura quando a entrada for invalida."""
	while True:
		try:
			return int(input(mensagem))
		except ValueError:
			print("Entrada invalida. Digite um numero inteiro.")


def ler_quantidade():
	"""Le uma quantidade nao negativa de alunos."""
	while True:
		quantidade = ler_inteiro("Quantos alunos deseja cadastrar? ")
		if quantidade >= 0:
			return quantidade
		print("A quantidade nao pode ser negativa.")


def verificar_nota(nota):
	"""Garante que a nota esteja entre 0 e 10."""
	while nota < 0 or nota > 10:
		print("Nota invalida. Digite novamente entre 0 e 10.")
		nota = ler_nota()
	return nota


def ler_nota():
	"""Le uma nota numerica e valida seu intervalo."""
	while True:
		try:
			nota = float(input())
			return verificar_nota(nota)
		except ValueError:
			print("Entrada invalida. Digite uma nota entre 0 e 10:", end=" ")


def cadastrar_aluno_nota():
	"""Cadastra nomes e tres notas, mantendo os indices sincronizados."""
	quantidade = ler_quantidade()
	disponiveis = MAXIMO_DE_ALUNOS - len(alunos)

	if quantidade > disponiveis:
		quantidade = disponiveis
		print(
			"Limite excedido. Serao cadastrados apenas "
			f"{quantidade} alunos."
		)

	for _ in range(quantidade):
		nome = input("Digite o nome do aluno: ").strip()
		while not nome:
			print("O nome nao pode ficar vazio.")
			nome = input("Digite o nome do aluno: ").strip()

		alunos.append(nome)
		print("Digite a nota 1: ", end="")
		n1.append(ler_nota())
		print("Digite a nota 2: ", end="")
		n2.append(ler_nota())
		print("Digite a nota 3: ", end="")
		n3.append(ler_nota())


def exibir_alunos():
	"""Exibe cada aluno e suas notas em linhas separadas."""
	if not verificar_alunos():
		return

	for indice, nome in enumerate(alunos, start=1):
		posicao = indice - 1
		print(f"\n{indice}o Aluno(a): {nome}")
		print(f"  Nota 1: {n1[posicao]:.2f}")
		print(f"  Nota 2: {n2[posicao]:.2f}")
		print(f"  Nota 3: {n3[posicao]:.2f}")


def media_geral():
	"""Calcula a media geral usando um laco."""
	if not verificar_alunos():
		return

	soma = 0.0
	for indice in range(len(alunos)):
		soma += n1[indice] + n2[indice] + n3[indice]

	media = soma / (3 * len(alunos))
	print(f"A media geral da turma e: {media:.2f}")


def media_por_aluno():
	"""Calcula e exibe a media individual de cada aluno."""
	if not verificar_alunos():
		return

	for indice, nome in enumerate(alunos):
		media = (n1[indice] + n2[indice] + n3[indice]) / 3
		print(f"A media do aluno {nome} e: {media:.2f}")


def maior_menor():
	"""Encontra a maior e a menor nota com comparacoes iterativas."""
	if not verificar_alunos():
		return

	maior = n1[0]
	menor = n1[0]
	for indice in range(len(alunos)):
		maior = max(maior, n1[indice], n2[indice], n3[indice])
		menor = min(menor, n1[indice], n2[indice], n3[indice])

	print(f"Maior nota: {maior:.2f}")
	print(f"Menor nota: {menor:.2f}")


def aprovados():
	"""Conta os alunos com media maior ou igual a 7."""
	if not verificar_alunos():
		return

	quantidade_aprovados = 0
	for indice in range(len(alunos)):
		media = (n1[indice] + n2[indice] + n3[indice]) / 3
		if media >= 7:
			quantidade_aprovados += 1

	print(f"A quantidade de alunos aprovados foi de {quantidade_aprovados}")


def somatorio_notas_recursivo(indice):
	"""Soma recursivamente as tres notas de cada aluno."""
	if indice == len(alunos):
		return 0.0

	return (
		n1[indice]
		+ n2[indice]
		+ n3[indice]
		+ somatorio_notas_recursivo(indice + 1)
	)


def contar_aprovados_recursivo(indice):
	"""Conta recursivamente os alunos aprovados."""
	if indice == len(alunos):
		return 0

	media = (n1[indice] + n2[indice] + n3[indice]) / 3
	aprovado = int(media >= 7)
	return aprovado + contar_aprovados_recursivo(indice + 1)


def estatisticas_recursivas():
	"""Exibe estatisticas calculadas pelas funcoes recursivas."""
	if not verificar_alunos():
		return

	soma = somatorio_notas_recursivo(0)
	media = soma / (3 * len(alunos))
	quantidade_aprovados = contar_aprovados_recursivo(0)
	print(f"Media geral da turma: {media:.2f}")
	print(f"Quantidade de alunos aprovados: {quantidade_aprovados}")


def buscar_aluno_por_nome():
	"""Faz uma busca sequencial pelo nome do aluno."""
	if not verificar_alunos():
		return

	nome_buscado = input("Digite o nome do aluno: ").strip()
	for indice, nome in enumerate(alunos):
		if nome == nome_buscado:
			print(f"Aluno encontrado na posicao {indice + 1}.")
			print(f"Nota 1: {n1[indice]:.2f}")
			print(f"Nota 2: {n2[indice]:.2f}")
			print(f"Nota 3: {n3[indice]:.2f}")
			return

	print("Aluno nao encontrado.")


def estatisticas_complementares():
	"""Calcula o percentual de aprovacao da turma."""
	if not verificar_alunos():
		return

	quantidade_aprovados = contar_aprovados_recursivo(0)
	percentual = (quantidade_aprovados * 100) / len(alunos)
	print(f"Percentual de aprovacao: {percentual:.2f}%")


def listar_invertido_recursivo(indice):
	"""Exibe os alunos do ultimo indice ate o primeiro por recursao."""
	if indice < 0:
		return

	print(f"{indice + 1}o Aluno(a): {alunos[indice]}")
	print(
		f"Nota 1: {n1[indice]:.2f} | "
		f"Nota 2: {n2[indice]:.2f} | "
		f"Nota 3: {n3[indice]:.2f}"
	)
	listar_invertido_recursivo(indice - 1)


def exibir_listagem_invertida():
	"""Inicia a listagem invertida depois de validar a turma."""
	if not verificar_alunos():
		return
	listar_invertido_recursivo(len(alunos) - 1)


def sub_menu():
	"""Controla as funcionalidades extras."""
	opcao = -1
	while opcao != 0:
		print("\n--- SUBMENU: Funcionalidades Extras ---")
		print("[1] Busca sequencial por nome")
		print("[2] Estatisticas complementares")
		print("[3] Exibir listagem invertida")
		print("[0] Voltar ao menu principal\n")
		opcao = ler_inteiro("Digite a opcao desejada: ")

		if opcao == 1:
			buscar_aluno_por_nome()
		elif opcao == 2:
			estatisticas_complementares()
		elif opcao == 3:
			exibir_listagem_invertida()
		elif opcao == 0:
			print("Voltando ao menu principal...")
		else:
			print("Opcao invalida.")


def exibir_menu():
	"""Controla o fluxo principal do sistema."""
	opcao = -1
	while opcao != 0:
		print("\n========== MENU PRINCIPAL ==========")
		print("1. Cadastrar alunos e notas")
		print("2. Exibir listagem de alunos")
		print("3. Calcular e exibir media geral da turma")
		print("4. Exibir listagem de media por aluno")
		print("5. Encontrar a maior e menor nota registrada")
		print("6. Contar alunos aprovados (media >= 7.0)")
		print("7. Emitir estatisticas recursivas")
		print("8. Submenu")
		print("0. Encerrar aplicacao")
		opcao = ler_inteiro("Digite a opcao desejada: ")

		if opcao == 1:
			cadastrar_aluno_nota()
		elif opcao == 2:
			exibir_alunos()
		elif opcao == 3:
			media_geral()
		elif opcao == 4:
			media_por_aluno()
		elif opcao == 5:
			maior_menor()
		elif opcao == 6:
			aprovados()
		elif opcao == 7:
			estatisticas_recursivas()
		elif opcao == 8:
			sub_menu()
		elif opcao != 0:
			print("Opcao invalida.")


def main():
	"""Inicia o menu principal."""
	exibir_menu()


if __name__ == "__main__":
	main()
