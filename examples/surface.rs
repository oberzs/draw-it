// Oliver Berzs
// https://github.com/oberzs/duku

// example that draws with a custom ray-marching shader

use duku::glsl::Metadata;
use duku::Duku;
use duku::Result;

fn main() -> Result<()> {
    let (mut duku, window) = Duku::windowed(500, 500)?;

    // read custom shader
    let path = "examples/shaders/raymarch.glsl";
    let mut shader = duku.create_shader_glsl(path)?;
    let mut metadata = Metadata::new(path)?;

    window.while_open(move |_| {
        // hot-reload shader
        if metadata.is_modified() {
            match duku.create_shader_glsl(path) {
                Ok(s) => {
                    println!("* recompiled shader");
                    shader = s;
                }
                Err(err) => println!("{}", err),
            }
        }

        duku.draw(None, |target| {
            target.set_shader(&shader);
            target.draw_surface();
        });
    });

    Ok(())
}
