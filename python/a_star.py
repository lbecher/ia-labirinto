import time

class AStarNode:
    def __init__(self, parent=None, position=None):
        self.parent = parent
        self.position = position
        self.g = 0
        self.h = 0
        self.f = 0

    def __eq__(self, other):
        return self.position == other.position

# função de minimo, verifica o f e escolhe o menor 
def min_value(list):
    min_value = list[0]

    for value in list:
        if value.f < min_value.f:
            min_value = value

    return min_value

# No labirinto o A* não pode andar na diagonal, mas da pra andar a diagonal normamente
def get_movement(teste_movimento):
    if teste_movimento:
        movement = [(0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (1, -1), (-1, -1), (-1, 1)]
    else:
        movement = [(0, 1), (0, -1), (1, 0), (-1, 0)]
    return movement

def a_Star_(maze_matrix, end_i, end_j, player2_i, player2_j):
    # posição inicial do player e final da matriz
    player_astar = AStarNode(position=(player2_i, player2_j))
    end_maze = AStarNode(position=(end_i, end_j))

    player_astar.g = 0
    player_astar.h = abs(player_astar.position[0] - end_maze.position[0]) + abs(player_astar.position[1] - end_maze.position[1])
    player_astar.f = player_astar.g + player_astar.h

    # faça uma lista aberta contendo apenas o nó inicial
    open_list = []
    open_list.append(player_astar)

    # faça uma lista fechada vazia
    closed_list = []
    
    # Escolhe forma de percorer percorer os nós adjacentes
    adjacency_choice = False
    movement = get_movement(adjacency_choice)

    # enquanto o nó de destino não for alcançado
    while open_list:
        # chama função de minimo de f 
        current_node = min_value(open_list)
        open_list.remove(current_node)

        closed_list.append(current_node)

        # se este nó é o nó de destino :
        if current_node == end_maze:
            path = []
            while current_node:
                path.append(current_node.position)
                current_node = current_node.parent
                # Terminou com SUCESSO
            return path

        else:
            # Verifica os adjacentes da posição atual
            for adjacency in movement:
                # como já existe a posição do nó inicial, calcula a posição x,y do adjacente na matriz
                adjacent_position = (adjacency[0] + current_node.position[0], adjacency[1] + current_node.position[1])

                # Verifica se esta dentro dos limites da matriz em X e Y, verifica se o adjacente não é um obstaculo
                if 0 <= adjacent_position[0] < len(maze_matrix) and 0 <= adjacent_position[1] < len(maze_matrix[0]) and maze_matrix[adjacent_position[0]][adjacent_position[1]] != 0:
                    adjacent_node = AStarNode(position=adjacent_position)
                    adjacent_node.g = current_node.g + 1
                    adjacent_node.h = abs(adjacent_node.position[0] - end_maze.position[0]) + abs (adjacent_node.position[1] - end_maze.position[1])
                    adjacent_node.f = adjacent_node.g + adjacent_node.h

                    # Verifica se algum nó adjacent_node esta na lista fechada
                    in_closed_list = False
                    for node in closed_list:
                        if node == adjacent_node:
                            in_closed_list = True
                            break

                    # Verifica se algum nó adjacent_node esta na lista aberta
                    in_open_list = False
                    for node in open_list:
                        if node == adjacent_node:
                            in_open_list = True
                            break

                    # Se o nó não esta na lista fechada nem na lista aberta
                    if  (in_closed_list == False) and (in_open_list == False):
                        open_list.append(adjacent_node)
                        adjacent_node.parent = current_node

    return None

# Função que é chamada na game engine
# NÃO ALTERAR A CHAMADA E NEM O RETORNO DA FUNÇÃO EM HIPÓTESE ALGUMA!!!
def calculate_a_star(matrix, rows, cols, exits, a_star_start):
    i,j = exits[0]
    x,y = a_star_start

    inicio = time.time_ns()
    caminho = a_Star_(matrix, i, j, x, y)
    fim = time.time_ns()
    tempo = fim - inicio

    return (tempo, caminho)
    