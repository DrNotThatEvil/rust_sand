use std::f32::consts::PI;
use std::ops::Index;
use std::slice::SliceIndex;

use ggez::{Context, GameResult};
use ggez::graphics;
use oorandom::Rand32;

const DEFAULT_TEMP: i16 = 10;
const DEFAULT_PRESSURE: f32 = 0.0;
const GRID_SIZE: i32 = 6;

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
pub struct Cell {
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

                let mut air_cell_clone = air_cell.clone();
                
                if _x > 30 && _x < 50 && _y > 30 && _y < 50 {
                    air_cell_clone.pressure = 3.0;
                }

                cells.push(air_cell_clone);
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

    pub fn get_neibour_indexes(&self, x: i32, y: i32) -> Vec<usize> {
        let mut cell_indexes: Vec<usize> = Vec::new();

        if x > 0 {
            cell_indexes.push(self.cords_to_index(x-1, y));
        }

        if x < (self.size.0-1) {
            cell_indexes.push(self.cords_to_index(x+1, y));
        }

        if y > 0  {
            cell_indexes.push(self.cords_to_index(x, y-1));
            
            if x > 0 {
                cell_indexes.push(self.cords_to_index(x-1, y-1));
            }
        }

        if y < (self.size.1-1) {
            cell_indexes.push(self.cords_to_index(x, y+1));
            
            if x < (self.size.0-1) {
                cell_indexes.push(self.cords_to_index(x+1, y+1));
            }
        }

        return cell_indexes;
    }

    pub fn get_neibours(&mut self, x: i32, y: i32) -> Vec<&Cell> {
        let indexes = self.get_neibour_indexes(x, y);

        let mut cells: Vec<&Cell> = Vec::new();
        for index in indexes.iter() {
            cells.push(&self.cells[*index]);
        }
        
        return cells;
    }

    pub fn get_mut_neibours(&mut self, x: i32, y: i32) -> Vec<&mut Cell> {
        let indexes = self.get_neibour_indexes(x, y);

        let mut cells: Vec<&mut Cell> = Vec::new();
        for index in indexes.iter() {
            cells.push(&mut self.cells[*index]);
        }
        
        return cells;
    }

  
    pub fn update_pressure(&mut self, ctx: &mut Context) {
        let seconds = 1.0 / 60.0 as f32;
        
        for y in (0..self.size.1).rev() {
            for x in 0..self.size.0 {
                let index = self.cords_to_index(x, y);
                let cell = self.cells.get(index);
                
                if cell.is_none() {
                    continue;
                }

                let cell_pressure = cell.unwrap().pressure;
                let neibours = self.get_neibours(x, y).iter()
                    .filter(|index| self.cells[**index].pressure < cell_pressure);

            }
        }
    }

    pub fn update_pressure_old(&mut self, ctx: &mut Context) {
        let seconds = 1.0 / 60.0 as f32;
        let mut count = 0;

        for y in (0..self.size.1).rev() {
            for x in 0..self.size.0 {
                let index = self.cords_to_index(x, y);
                let cell = self.cells.get(index);
                if cell.is_none() {
                    continue;
                }


                let cell_pressure = cell.unwrap().pressure;
                if cell_pressure.abs() < 0.01 {
                    continue;
                }

                //println!("Less: {}", cell_pressure.abs());
                let cell_pressure_rounded = (cell_pressure * 100.0).round() / 100.0;

                let neibourhood = self.get_neibours(x, y);
                //neibourhood.iter().for_each(|index| println!("{}", index));

                let lower_pressure_neibours: Vec<usize> = neibourhood.iter()
                    .filter(|index| ((self.cells[**index].pressure * 100.0).round() / 100.0) < cell_pressure_rounded)
                    .map(|&x| x)
                    .collect::<Vec::<usize>>();

                let neibourhood_size = lower_pressure_neibours.len() as f32;
                
                if neibourhood_size > 0.0 {
                    count += 1;
                    // lower_pressure_neibours.iter().fold(0.0, |acc, index| acc + self.cells[*index].pressure);
                    // Cell needs to equalize
                    let div_pressure = cell_pressure_rounded / (1.0 + neibourhood_size) * (seconds * 2.0);
                    //if cell_pressure < 0.0 {
                        println!("cell_pressure: {}, Divide pressure: {}, neibourhood_size: {}", cell_pressure, div_pressure, neibourhood_size);
                    //}

                    for neibour in lower_pressure_neibours {
                        let ncell = self.cells.get_mut(neibour);
                        if ncell.is_some() {
                            self.cells[neibour].pressure = self.cells[neibour].pressure - div_pressure;
                        }
                    }

                    self.cells[index].pressure = cell_pressure_rounded - (div_pressure * neibourhood_size);
                    //let cell = self.cells.get_mut(index);
                    //self.cells[index].pressure -= div_pressure * neibourhood_size;
                }
            }
        }

        println!("Total cells updated: {}", count);
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<graphics::Mesh> {
        let mb = &mut graphics::MeshBuilder::new();

        mb.rectangle(
            graphics::DrawMode::stroke(1.0),
            graphics::Rect::new_i32(
                self.pos.0,
                self.pos.1,
                self.size.0 * GRID_SIZE,
                self.size.1 * GRID_SIZE
            ),
            graphics::Color::new(
                1.0,
                1.0,
                1.0,
                0.25
            )
        )?;

        /* 
        let m_old = Vec2::new(self.mouse.last.0, self.mouse.last.1);
        let m_new  = Vec2::new(self.mouse.cur.0, self.mouse.cur.1);
        let text_dest = (m_old + ((m_old - m_new) * seconds));
        
       
        mb.rectangle(
            graphics::DrawMode::fill(),
            graphics::Rect::new(text_dest.x, text_dest.y, 6.5, 6.5),
            graphics::Color::new(1.0, 1.0, 1.0, 1.0)
        )?; 

        */
        
        for y in 0..self.size.1 { 
            for x in 0..self.size.0 {
                let pos = (self.pos.0 + (x * GRID_SIZE), self.pos.1 + (y * GRID_SIZE));

                let bounds = graphics::Rect::new_i32(
                    self.pos.0 + (x * GRID_SIZE),
                    self.pos.1 + (y * GRID_SIZE),
                    GRID_SIZE,
                    GRID_SIZE
                );
                
                let cell: &Cell = &self.cells[self.cords_to_index(x, y)];
                let mut color = cell.color.clone();
                
                if cell.id > 0 {
                    mb.rectangle(
                        graphics::DrawMode::fill(),
                        bounds,
                        color
                    )?;  
                } else {
                    let press_color = (cell.pressure.clamp(-5.0, 5.0) * (PI * 0.1)).sin();

                    mb.rectangle(
                        graphics::DrawMode::fill(),
                        bounds,
                        graphics::Color::new(press_color, 0.0, -press_color, 0.75)
                    )?; 
                }
            }
        }

        Ok(graphics::Mesh::from_data(ctx, mb.build()))
    }
}