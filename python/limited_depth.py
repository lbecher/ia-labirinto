import time

def busca_profundidade_limitada(labirinto, posicao_atual, limite, caminhos=[], caminho_atual=[], visitadas=set()):
    lin, col = posicao_atual

    if limite < 0:
        return caminhos  # Limite atingido, retorna a lista de caminhos atualizada

    if labirinto[lin][col] == 2:
        # Encontrou a saída, adiciona o caminho atual à lista de caminhos
        caminho_atual.append(posicao_atual)
        caminhos.append(caminho_atual.copy())
        return caminhos

    if labirinto[lin][col] == 0 or posicao_atual in visitadas:
        return caminhos  # Parede ou posição já visitada

    visitadas.add(posicao_atual)
    caminho_atual.append(posicao_atual)

    # Passa o labirinto como parâmetro para a função obter_vizinhos
    vizinhos = obter_vizinhos(labirinto, posicao_atual)
    for vizinho in vizinhos:
        if labirinto[vizinho[0]][vizinho[1]] == 0 or vizinho in visitadas:
            continue  # Ignora paredes e posições já visitadas

        resultado = busca_profundidade_limitada(labirinto, vizinho, limite - 1, caminhos, caminho_atual, visitadas)

        # Se encontrou um caminho, retorna os caminhos
        if resultado:
            return resultado

    # Se nenhum caminho foi encontrado, remove a posição atual das visitadas e do caminho
    visitadas.remove(posicao_atual)
    caminho_atual.pop()

    return caminhos


# Restante do código...
def obter_vizinhos(labirinto, posicao):
    lin, col = posicao
    vizinhos = []

    # Verifica para cima
    if lin > 0:
        vizinhos.append((lin - 1, col))
    # Verifica para baixo
    if lin < len(labirinto) - 1:
        vizinhos.append((lin + 1, col))
    # Verifica à esquerda
    if col > 0:
        vizinhos.append((lin, col - 1))
    # Verifica à direita
    if col < len(labirinto[0]) - 1:
        vizinhos.append((lin, col + 1))

    return vizinhos


# Função que é chamada na game engine
# NÃO ALTERAR A CHAMADA E NEM O RETORNO DA FUNÇÃO EM HIPÓTESE ALGUMA!!!
def calculate_limited_depth(matrix, rows, cols, exits, limited_depth_start):
    labirinto = matrix
    linhas = rows
    colunas = cols
    saidas = exits
    posicao_inicial = limited_depth_start

    limite = 400

    inicio = time.time_ns()
    caminho = busca_profundidade_limitada(labirinto, posicao_inicial, limite)
    fim = time.time_ns()
    tempo = fim - inicio

    return (tempo, caminho[0])


# Função de teste utilizada para validar o algoritmo
# No terminal, estando no subdiretório "python", execute:
#   $ python3
#   >>> from limited_depth import teste
#   >>> teste()
#   >>> exit()
def teste():
    labirinto = [
        [0, 0, 3, 0, 0],
        [0, 1, 1, 1, 0],
        [0, 1, 0, 0, 0],
        [0, 1, 1, 0, 0],
        [0, 0, 2, 0, 0],
    ]
    saidas = [(2, 4)]
    posicao_do_jogador = (0, 2)
    caminho = calculate_limited_depth(labirinto, 5, 5, saidas, posicao_do_jogador)
    print(caminho)
