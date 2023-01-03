use ggez::conf::WindowMode;
use ggez::{Context, ContextBuilder, GameResult, timer};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};

struct Chunk { 
    pos: (i32, i32),
    size: (i32, i32),
    field: Vec<u8>
}

impl Chunk {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Chunk {
        let mut field: Vec<u8> = Vec::with_capacity((w * h) as usize);
        for y in (0..h) {
            for x in (0..w) {
                field.push(0);
            }
        }
        
        Chunk {
            pos: (x, y),
            size: (w, h),
            field: field,
        }
    }

    pub fn update(&mut self, ctx: &mut Context) {
        for y in (0..self.size.1) { 
            for x in 0..self.size.0 {
                println!("X: {}, Y: {}", x, y);

                let cur_index: usize =  (x + (y * self.size.0)) as usize;
                let cell: u8 = self.field[cur_index];
                
                if cell > 0 {
                    if y < (self.size.1-1) {
                        // not last row so moving down allowed!
                        let below_index: usize = (x + ((y+1) * self.size.0)) as usize;
                        let cell_below: u8 = self.field[below_index];
                        
                        if cell_below == 0 {
                            // Move below
                            self.field.swap(cur_index, below_index);
                            continue;
                        }

                        if x > 0 {
                            // Can move left 
                            let below_l_index: usize = ((x-1) + ((y+1) * self.size.0)) as usize;
                            let cell_l_below: u8 = self.field[below_l_index];

                            if cell_l_below == 0 {
                                // Move left below

                                self.field.swap(cur_index, below_l_index);
                                continue;
                            }
                        }

                        if x < (self.size.0 - 1) {
                            // Can move right
                            let below_r_index: usize = ((x+1) + ((y+1) * self.size.0)) as usize;
                            let cell_r_below: u8 = self.field[below_r_index];

                            if cell_r_below == 1 {
                                // Move right below 

                                self.field.swap(cur_index, below_r_index);
                                continue;
                            }
                        }
                    }
                }

                if cell == 0 && x == 50 && y == 0 {
                    if(timer::check_update_time(ctx, 60)) {
                        self.field[cur_index] = 1;
                    }
                }
            }
        }
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<graphics::Mesh> {
        let mb = &mut graphics::MeshBuilder::new();

        for y in 0..self.size.1 { 
            for x in 0..self.size.0 {
                let pos = (self.pos.0 + x, self.pos.1 + y);
                
                let cur_index: usize =  (x + (y * self.size.0)) as usize;
                let cell: u8 = self.field[cur_index];

                if cell > 0 {
                    mb.rectangle(
                        graphics::DrawMode::fill(),
                        graphics::Rect::new_i32(pos.0, pos.1, 1, 1),
                        graphics::Color::new(1.0, 0.0, 0.0, 1.0),
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
        let mut chunk: Chunk = Chunk::new(0, 0, 100, 100);
        chunks.push(chunk);

        GGSand {
            chunks: chunks
        }
    }
}

impl EventHandler for GGSand {
   fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Update code here...

        for chunk in self.chunks.iter_mut() {
            chunk.update(ctx);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        for chunk in self.chunks.iter() {
            let chunk_mesh: graphics::Mesh = chunk.draw(ctx)?;
            canvas.draw(
                &chunk_mesh,
                graphics::DrawParam::new()
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