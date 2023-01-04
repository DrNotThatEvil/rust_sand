use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};
use oorandom;

#[derive(Copy, Clone)]
struct Element {
    id: u16,
    color: graphics::Color,
    updated: bool,
    move_cooldown: f32,
    stable: bool
}

struct Chunk { 
    pos: (i32, i32),
    size: (i32, i32),
    field: Vec<Element>,
    ran: bool,
    add_timer: f32,
    cut_timer: f32,
    rand: oorandom::Rand32,
}

impl Chunk {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Chunk {
        let mut field: Vec<Element> = Vec::with_capacity((w * h) as usize);
        let air = Element {
            id: 0,
            color: graphics::Color::new(0.0, 0.0, 0.0, 1.0),
            updated: false,
            move_cooldown: 0.0,
            stable: false
        };

        for _y in 0..h {
            for _x in 0..w {
                field.push(air.clone());
            }
        }
        
        Chunk {
            pos: (x, y),
            size: (w, h),
            field: field,
            ran: false,
            add_timer: 0.0,
            cut_timer: 0.0,
            rand: oorandom::Rand32::new(4)
        }
    }

    pub fn unstable_neibours(&mut self, x: i32, y: i32) { 

    }

    pub fn update_water(&mut self, x: i32, y: i32) { 
        let cur_index: usize =  (x + (y * self.size.0)) as usize;
        let cell: Element = self.field[cur_index];

        if cell.updated {
            // already updated
            return;
        }

        self.update_sand(x, y);
        
        let rand_dir = self.rand.rand_range(0..2);

        if rand_dir == 0 {
            if x > 1 {
                // Can move left 
                let l_index: usize = ((x-1) + (y * self.size.0)) as usize;
                let cell_l: Element = self.field[l_index];

                if cell_l.id == 0 {
                    // Move left below
                    self.field.swap(cur_index, l_index);
                    self.field[l_index].updated = true;
                    self.field[l_index].stable = false;
                    self.field[l_index].move_cooldown = 0.0;
                    return;
                }
            }

            if x < (self.size.0 - 1) {
                // Can move right
                let r_index: usize = ((x+1) + (y * self.size.0)) as usize;
                let cell_r: Element = self.field[r_index];

                if cell_r.id == 0 {
                    // Move right below 
                    self.field.swap(cur_index, r_index);
                    self.field[r_index].updated = true;
                    self.field[r_index].stable = false;
                    self.field[r_index].move_cooldown = 0.0;
                    return;
                }
            }
        } else {
            if x < (self.size.0 - 1) {
                // Can move right
                let r_index: usize = ((x+1) + (y * self.size.0)) as usize;
                let cell_r: Element = self.field[r_index];

                if cell_r.id == 0 {
                    // Move right below 
                    self.field.swap(cur_index, r_index);
                    self.field[r_index].updated = true;
                    self.field[r_index].stable = false;
                    self.field[r_index].move_cooldown = 0.0;
                    return;
                }
            }
 
            if x > 0 {
                // Can move left 
                let l_index: usize = ((x-1) + (y * self.size.0)) as usize;
                let cell_l: Element = self.field[l_index];

                if cell_l.id == 0 {
                    // Move left below
                    self.field.swap(cur_index, l_index);
                    self.field[l_index].updated = true;
                    self.field[l_index].stable = false;
                    self.field[l_index].move_cooldown = 0.0;
                    return;
                }
            }
        }

        if !cell.updated {
            self.field[cur_index].move_cooldown += 0.25;
            if cell.move_cooldown > 5.0 {
                self.field[cur_index].stable = true;
            }
        }
        // Could not update it 
        // Stable system?
    }

    pub fn update_sand(&mut self, x: i32, y: i32) {
        let cur_index: usize =  (x + (y * self.size.0)) as usize;
        let cell: Element = self.field[cur_index];
        
        if cell.updated {
            // already updated
            return;
        }

        if y < (self.size.1-1) {
            // not last row so moving down allowed!
            let below_index: usize = (x + ((y+1) * self.size.0)) as usize;
            let cell_below: Element = self.field[below_index];
            
            if cell_below.id == 0 {
                // Move below
                self.field.swap(cur_index, below_index);
                self.field[below_index].updated = true;
                self.field[below_index].stable = false;
                self.field[below_index].move_cooldown = 0.0;
                return;
            }

            if x > 0 {
                // Can move left 
                let below_l_index: usize = ((x-1) + ((y+1) * self.size.0)) as usize;
                let cell_l_below: Element = self.field[below_l_index];

                if cell_l_below.id == 0 {
                    // Move left below
                    self.field.swap(cur_index, below_l_index);
                    self.field[below_l_index].updated = true;
                    self.field[below_l_index].stable = false;
                    self.field[below_l_index].move_cooldown = 0.0;
                    return;
                }
            }

            if x < (self.size.0-1) {
                // Can move right
                let below_r_index: usize = ((x+1) + ((y+1) * self.size.0)) as usize;
                let cell_r_below: Element = self.field[below_r_index];

                if cell_r_below.id == 0 {
                    // Move right below 
                    self.field.swap(cur_index, below_r_index);
                    self.field[below_r_index].updated = true;
                    self.field[below_r_index].stable = false;
                    self.field[below_r_index].move_cooldown = 0.0;
                    return;
                }
            }
        }

        if !cell.updated {
            self.field[cur_index].move_cooldown += 0.25;
            if cell.move_cooldown > 5.0 {
                self.field[cur_index].stable = true;
            }
        }
    }


    pub fn update(&mut self, ctx: &mut Context) {
        let rand_fr_nr = self.rand.rand_i32();

        for y in (0..self.size.1).rev() { 
            for x in 0..self.size.0 {
                if !self.ran {
                    println!("X: {}, Y: {}, T: {}", x, y, rand_fr_nr);
                    self.ran = y == 0 && x == (self.size.0 - 1);
                }

                let cur_index: usize =  (x + (y * self.size.0)) as usize;
                self.field[cur_index].updated = false;

                let cell: Element = self.field[cur_index];
               
                if cell.id == 1 && !cell.stable {
                    self.update_sand(x, y);
                }

                if cell.id == 2 && !cell.stable {
                    self.update_water(x, y);
                }

                if self.cut_timer < 150.0 {
                    if cell.id == 0 && (x > 45 && x < 55) && y == 0 {
                        self.add_timer += 0.1;

                        if self.add_timer > 0.5 {
                            let water = Element {
                                id: 2,
                                color: graphics::Color::new(0.0, 0.25, 1.0, 1.0),
                                updated: false,
                                move_cooldown: 0.0,
                                stable: false
                            };

                            self.field[cur_index] = water.clone();
                            self.add_timer = 0.0;
                            self.cut_timer += 0.1;
                        }
                    }
                }
            }
        }
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<graphics::Mesh> {
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

        let mut b_x_min: i32 = i32::MAX;
        let mut b_y_min: i32 = i32::MAX;

        let mut b_x_max: i32 = i32::MIN;
        let mut b_y_max: i32 = i32::MIN;

        for y in 0..self.size.1 { 
            for x in 0..self.size.0 {
                let pos = (self.pos.0 + x, self.pos.1 + y);
                
                let cur_index: usize =  (x + (y * self.size.0)) as usize;
                let cell: &Element = &self.field[cur_index];

                let mut color = cell.color.clone();
                //color.g = cell.stable;

                if cell.id > 0 {
                    if cell.stable {
                        b_x_min = b_x_min.min(x);
                        b_y_min = b_y_min.min(y);

                        b_x_max = b_x_max.max(x);
                        b_y_max = b_y_max.max(y);
                    } else {
                        if b_x_min < i32::MAX && b_y_max > i32::MIN {
                            println!("_b_x_min: {}, _b_y_min: {}, _b_x_max: {}, _b_y_max: {}", b_x_min, b_y_min, b_x_max, b_y_max);

                            mb.rectangle(
                                graphics::DrawMode::stroke(1.0),
                                graphics::Rect::new_i32(
                                    b_x_min, b_y_min, (b_x_max-b_x_min).abs(), (b_y_max-b_y_min).abs()
                                ),
                                graphics::Color::new(1.0, 0.0, 1.0, 1.0)
                            )?;

                            b_x_min = i32::MAX;
                            b_y_min = i32::MAX;

                            b_x_max = i32::MIN;
                            b_y_max = i32::MIN;
                        }
                    }

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

struct GGSand {
    chunks: Vec<Chunk>
}

impl GGSand {
    pub fn new(ctx: &mut Context) -> GGSand {
        let mut chunks = Vec::new();
        let chunk: Chunk = Chunk::new(0, 0, 100, 100);
        chunks.push(chunk);

        GGSand {
            chunks: chunks
        }
    }
}

impl EventHandler for GGSand {
   fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Update code here...

        while ctx.time.check_update_time(60) {
            for chunk in self.chunks.iter_mut() {
                chunk.update(ctx);
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        for chunk in self.chunks.iter() {
            let chunk_mesh: graphics::Mesh = chunk.draw(ctx)?;
            canvas.draw(
                &chunk_mesh,
                graphics::DrawParam::default().scale([6.5, 6.5])
            );
        }

        canvas.finish(ctx)?;

        Ok(())
    }
}

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new(
            "GGSand", 
            "DrNotThatEvil"
        )
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 700.0))
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = GGSand::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}