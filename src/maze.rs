use bevy::prelude::*;
use simple_matrix::Matrix;
use std::fs::File;
use std::io::{self, BufRead};

pub struct MazePlugin;

impl Plugin for MazePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreStartup, load_maze);
    }
}

#[derive(Resource)]
pub struct Maze {
    pub matrix: Matrix<u8>,
    pub exits: Vec<(usize, usize)>,
    pub a_star_start: Option<(usize, usize)>,
    pub limited_depth_start: Option<(usize, usize)>,
}

pub fn load_maze(
    mut commands: Commands,
) {
    //
    // Abre o arquivo maze.txt
    //

    let path = "maze.txt";
    let file = File::open(path).expect("falha ao ler o arquivo maze.txt");
    let reader = io::BufReader::new(file);

    let mut lines = reader.lines();

    //
    // Obtém quantidade de linhas e colunas
    //

    let first_line = lines.next().unwrap().unwrap();
    let mut first_line_split = first_line.split_whitespace();
    let rows = first_line_split
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let cols = first_line_split
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    //
    // Inicializando atributos do labirinto
    //

    let mut matrix: Matrix<u8> = Matrix::new(rows, cols);
    let mut exits: Vec<(usize, usize)> = Vec::new();
    let mut a_star_start: Option<(usize, usize)> = None;
    let mut limited_depth_start: Option<(usize, usize)> = None;

    //
    // Lê o arquivo
    //

    for i in 0..rows {
        let line: String = lines.next().unwrap().unwrap();
        let mut line_split = line.split_whitespace();

        for j in 0..cols {
            let mut value = line_split
                .next()
                .unwrap()
                .parse::<u8>()
                .unwrap();

            if value == 2 {
                exits.push((i, j))
            } else if value == 3 {
                a_star_start = Some((i, j));
            } else if value == 4 {
                limited_depth_start = Some((i, j));
            } else if value == 5 {
                a_star_start = Some((i, j));
                limited_depth_start = Some((i, j));
            }

            matrix.set(i, j, value);
        }
    }

    commands.insert_resource(Maze {
        matrix,
        exits,
        a_star_start,
        limited_depth_start,
    });
}