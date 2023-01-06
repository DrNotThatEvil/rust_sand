use ggez::{Context, GameResult};
use ggez::graphics;
use oorandom::Rand32;

const DEFAULT_TEMP: i16 = 10;
const DEFAULT_PRESSURE: f32 = 1.0;

#[derive(Copy, Clone)]
enum CellState {
    UnStable,
    TempratureStable,
    MovementStable,
    Stable
}

fn get_cell_color(id: u16, rng: &mut Rand32) -> graphics::Color {
    let sand_colors: [graphics::Color; 3] = [
        graphics::Color::from_rgba(254, 246, 91, 255),
        graphics::Color::from_rgba(228, 221, 81, 255),
        graphics::Color::from_rgba(254, 249, 156, 255),
    ];

    match id {
        0 => graphics::Color::new(0.0, 0.0, 0.0, 1.0),
        1 => sand_colors[rng.rand_range(0..3) as usize],
        _ => graphics::Color::from_rgba(255, 0, 255, 255),
    }
}

#[derive(Copy, Clone)]
struct Cell {
    id: u16,
    pressure: f32,
    temp: i16,
    color: graphics::Color,
    state: CellState
}

impl Cell {
    pub fn new(id: u16, rng: &mut Rand32) -> Cell {
        Cell {
            id: id,
            pressure: DEFAULT_PRESSURE,
            temp: DEFAULT_TEMP,
            color: get_cell_color(id, rng).clone(),
            state: CellState::UnStable
        }
    }
}

pub struct Grid {
    pos: (i32, i32),
    size: (i32, i32),
    cells: Vec<Cell>,
}

impl Grid {
    pub fn new(x: i32, y: i32, w: i32, h:i32, rng: &mut Rand32) -> Grid {
        let mut cells: Vec<Cell> = Vec::with_capacity((w * h) as usize);
        let air_cell = Cell::new(0, rng);
        
        for _y in 0..h {
            for _x in 0..w {
                if _y == 50 {
                    cells.push(Cell::new(1, rng));
                    continue;
                }

                cells.push(air_cell.clone());
            }
        }

        Grid {
            pos: (x, y),
            size: (w, h),
            cells: cells,
        }
    }

    pub fn cords_to_index(&self, x: i32, y: i32) -> usize {
        return (x + (y * self.size.0)) as usize;   
    }

    pub fn get_neibours(&self, x: i32, y: i32) -> Vec<usize> {
        let mut cell_indexes: Vec<usize> = Vec::new();

        if x > 0 {
            cell_indexes.push(self.cords_to_index(x-1, y));
        }

        if x < self.size.0 {
            cell_indexes.push(self.cords_to_index(x+1, y));
        }

        if y > 0  {
            cell_indexes.push(self.cords_to_index(x, y-1));
            
            if x > 0 {
                cell_indexes.push(self.cords_to_index(x-1, y-1));
            }
        }

        if y < self.size.1 {
            cell_indexes.push(self.cords_to_index(x, y+1));
            
            if x < self.size.0 {
                cell_indexes.push(self.cords_to_index(x+1, y+1));
            }
        }

        return cell_indexes;
    }
    
    pub fn draw(&self, ctx: &mut Context) -> GameResult<graphics::Mesh> {
        let mb = &mut graphics::MeshBuilder::new();

        mb.rectangle(
            graphics::DrawMode::stroke(1.0),
            graphics::Rect::new_i32(
                self.pos.0,
                self.pos.1,
                self.size.0,
                self.size.1
            ),
            graphics::Color::new(
                1.0,
                1.0,
                1.0,
                0.25
            )
        )?;
        
        for y in 0..self.size.1 { 
            for x in 0..self.size.0 {
                let pos = (self.pos.0 + x, self.pos.1 + y);
                
                let cell: &Cell = &self.cells[self.cords_to_index(x, y)];
                let mut color = cell.color.clone();
                
                if cell.id > 0 {
                    mb.rectangle(
                        graphics::DrawMode::fill(),
                        graphics::Rect::new_i32(pos.0, pos.1, 1, 1),
                        color
                    )?;  
                }
            }
        }

        Ok(graphics::Mesh::from_data(ctx, mb.build()))
    }
}