# ia-labirinto

Trabalho de algoritmos de busca da disciplica de Inteligência Artificial.

Alunos:
* Luiz Fernando Becher de Araujo
* Matheus Lucas Ferreira Jacinto
* Vinicius Sendoski de Andrade

## Execução

Na pasta raiz do projeto (que contém o arquivo `Cargo.toml`), execute o comando:

```
cargo run
```

## Algoritmos de busca

A implementação dos algoritmos de busca encontram-se no subdiretório `python`, tanto o A* quanto o busca em profundidade limitada.

A heurística do A* é definida no código em Python (`a_star.py`), na linha 51.

O limite do algoritmo de busca em pronfundidade limitada é definido no código em Python (`limited_depth.py`), na linha 70.

## Personalização do labirinto

O labirinto é lido do arquivo `maze.txt`.

Exemplos prontos de labirinto podem ser encontrados em `maze_options.txt`.

O labirinto, representado por uma matriz de inteiros, pode ter qualquer tamanho, desde que respeite a representação inteira utilizada pelo Python (ou seja, 2147483648 por 2147483648). Além disso, o tamanho do labirinto deve respeitar a capacidade de memória da sua máquina. O tamanho do labirinto deve ser declarado na primeira linha do arquivo, no formato linha coluna.

A matriz do labirinto é baseada na representação matricial de grafos, onde as paredes do labirinto (nodos desconexos) são representadas por 0, enquanto um caminho possível é representado por 1. Extendendo essa representação para adicionar algumas informações extras, temos:

* 2: representa uma saída do labirinto;
* 3: representa a posição inicial do player Amelia (A*);
* 4: representa a posição inicial do player Bob (profundidade limitada);
* 5: representa a mesma posição inicial tanto para Amelia quanto para Bob.

Vale resaltar que, no código dos algoritmos de busca, as posições inicias dos players também são consideradas como nós válidos (equivalente a 1).

### Exemplo 1

```
3 4
0 2 0 0
0 1 1 4
0 0 3 0
```

### Exemplo 2
```
10 10
0 0 2 0 0 0 0 0 0 0
0 0 1 0 0 0 0 1 0 0
0 0 1 1 0 0 0 1 0 0
0 0 0 1 1 1 1 1 1 0
0 0 0 1 0 1 0 0 0 0 
0 0 1 1 0 0 0 0 0 0
0 0 1 0 0 0 0 1 1 2
0 0 1 1 0 0 0 1 0 0
0 0 0 1 1 1 1 1 1 0
0 0 0 3 0 4 0 0 0 0
```

## Instalação de dependências para Debian/Ubuntu/Linux Mint

Na pasta raiz do projeto (que contém o arquivo `install-debian-dependencies.sh`), execute os comandos:

```
chmod +x install-debian-dependencies.sh
```

```
./install-debian-dependencies.sh
```

![Texto Alternativo](https://github.com/lbecher/ia-labirinto/blob/master/Trabalho_IA.png)
