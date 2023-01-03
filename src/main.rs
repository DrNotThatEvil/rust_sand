use ggez::conf::WindowMode;
use ggez::{Context, ContextBuilder, GameResult};
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
        
        Chunk {
            pos: (x, y),
            size: (w, h),
            field: field,
        }
    }

    pub fn update(&mut self) {
        for y in (0..self.size.1).rev() { 
            println!("y: {}", y);
        }
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
            chunk.update();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        // Draw code here...
        canvas.finish(ctx)
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