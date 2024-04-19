mod cell;
mod direction;

use cell::Cell;
use rand::random;
use sdl2::pixels::Color;

use engine::{Engine, Float, Game, Point};

struct Maze {
    width: u32,
    height: u32,
    path_width: i32,
    num_visited_cells: u32,
    cells_to_visit: Vec<Point>,
    cells: Vec<Cell>,
}

impl Maze {
    fn new(width: u32, height: u32, path_width: u32) -> Maze {
        Maze {
            width,
            height,
            num_visited_cells: 0,
            path_width: path_width as i32,
            cells_to_visit: Vec::new(),
            cells: vec![Cell::new(); (width * height) as usize],
        }
    }

    fn get_cell_xy(&mut self, x: i32, y: i32) -> Option<&mut Cell> {
        if x >= self.width as i32 || y >= self.height as i32 || x < 0 || y < 0 {
            return None;
        }

        Some(self.get_cell(Point::new(x, y)))
    }

    fn get_cell(&mut self, position: Point) -> &mut Cell {
        &mut self.cells[(position.y * (self.width as i32) + position.x) as usize]
    }

    fn get_neighbor_cell_position(&mut self, position: Point, direction: direction::Type) -> Point {
        let Point { x, y } = position;
        match direction {
            direction::NORTH => Point::new(x, y - 1),
            direction::EAST => Point::new(x + 1, y),
            direction::SOUTH => Point::new(x, y + 1),
            direction::WEST => Point::new(x - 1, y),
            _ => panic!("Invalid direction"),
        }
    }

    fn get_neighbor_cell(
        &mut self,
        position: Point,
        direction: direction::Type,
    ) -> Option<&mut Cell> {
        let neighbor_position = self.get_neighbor_cell_position(position, direction);
        self.get_cell_xy(neighbor_position.x, neighbor_position.y)
    }

    fn draw_cell(&self, x: i32, y: i32, cell: Cell, engine: &mut Engine) {
        const CELL_BORDER_OFFSET: i32 = 1;

        // Draw cell
        for py in 0..self.path_width {
            for px in 0..self.path_width {
                if cell.visited {
                    engine.draw_point(
                        x * (self.path_width + 1) + px + CELL_BORDER_OFFSET,
                        y * (self.path_width + 1) + py + CELL_BORDER_OFFSET,
                        Color::WHITE,
                    )
                } else {
                    engine.draw_point(
                        x * (self.path_width + 1) + px + CELL_BORDER_OFFSET,
                        y * (self.path_width + 1) + py + CELL_BORDER_OFFSET,
                        Color::BLUE,
                    )
                }
            }
        }

        // Draw paths
        for p in 0..self.path_width {
            if cell.paths & direction::SOUTH != 0 {
                engine.draw_point(
                    x * (self.path_width + 1) + p + CELL_BORDER_OFFSET,
                    y * (self.path_width + 1) + self.path_width + CELL_BORDER_OFFSET,
                    Color::WHITE,
                )
            }

            if cell.paths & direction::EAST != 0 {
                engine.draw_point(
                    x * (self.path_width + 1) + self.path_width + CELL_BORDER_OFFSET,
                    y * (self.path_width + 1) + p + CELL_BORDER_OFFSET,
                    Color::WHITE,
                )
            }
        }
    }
}

impl Game for Maze {
    fn update(&mut self, _dt: Float, _engine: &Engine) {
        if self.num_visited_cells >= (self.width * self.height) {
            return;
        }

        let mut current_cell: Point;
        let mut neighbors: Vec<direction::Type>;

        loop {
            current_cell = *self.cells_to_visit.last().unwrap();

            neighbors = get_unvisited_neighbours(self, current_cell);
            if neighbors.is_empty() {
                // No neighbours found, backtrack and continue in the next frame
                self.cells_to_visit.pop();
                continue;
            }

            break;
        }

        // Choose a random neighbour to visit
        let next_cell_direction = neighbors[random::<usize>() % neighbors.len()];
        let next_cell_position = self.get_neighbor_cell_position(current_cell, next_cell_direction);
        let next_cell = self.get_cell(next_cell_position);

        // Mark next cell as visited
        next_cell.visited = true;

        // Set paths between current cell and next cell
        next_cell.paths |= direction::opposite(next_cell_direction);
        self.get_cell(current_cell).paths |= next_cell_direction;

        // Add next cell to the visit queue
        self.cells_to_visit.push(next_cell_position);

        self.num_visited_cells += 1;

        return;

        fn get_unvisited_neighbours(maze: &mut Maze, current_cell: Point) -> Vec<direction::Type> {
            let mut neighbors = Vec::new();
            add_unvisited_neighbour(maze, current_cell, &mut neighbors, direction::NORTH);
            add_unvisited_neighbour(maze, current_cell, &mut neighbors, direction::EAST);
            add_unvisited_neighbour(maze, current_cell, &mut neighbors, direction::SOUTH);
            add_unvisited_neighbour(maze, current_cell, &mut neighbors, direction::WEST);
            neighbors
        }

        fn add_unvisited_neighbour(
            maze: &mut Maze,
            current_cell: Point,
            neighbors: &mut Vec<direction::Type>,
            direction: direction::Type,
        ) {
            let neighbor = maze.get_neighbor_cell(current_cell, direction);
            if let Some(cell) = neighbor {
                if !cell.visited {
                    neighbors.push(direction);
                }
            }
        }
    }

    fn render(&mut self, engine: &mut Engine) {
        for x in 0..self.width as i32 {
            for y in 0..self.height as i32 {
                let cell = *self.get_cell_xy(x, y).unwrap();
                self.draw_cell(x, y, cell, engine);
            }
        }
    }
}

fn main() {
    let width: u32 = 40;
    let height: u32 = 20;
    let path_width: u32 = 3;

    let mut maze = Maze::new(width, height, path_width);
    maze.cells[0].visited = true;
    maze.cells_to_visit.push(Point::new(0, 0));
    maze.num_visited_cells = 1;

    // Each cell requires path_width + 1 for path on the side.
    // Whole maze requires 1 extra point for the border cell walls.
    let draw_width = width * (path_width + 1) + 1;
    let draw_height = height * (path_width + 1) + 1;

    engine::create(maze, String::from("Maze"))
        .with_point_dimensions(draw_width, draw_height)
        .start();
}
