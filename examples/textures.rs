// Oliver Berzs
// https://github.com/oberzs/duku

// example that draws textures

use duku::Duku;
use duku::Result;

fn main() -> Result<()> {
    let (mut duku, window) = Duku::windowed(800, 400)?;

    let texture_1 = duku.create_texture_png("examples/textures/prototype/green.png", None)?;

    let texture_2 = duku.create_texture_jpeg("examples/textures/cat.jpg", None)?;

    // save imported jpeg as a png
    texture_2.save("cat.png")?;

    window.while_open(move |_| {
        duku.draw(None, |target| {
            target.transform.move_down(200.0);
            target.transform.move_left(400.0);
            target.draw_texture(&texture_1, [400.0, 400.0]);
            target.transform.move_right(400.0);
            target.draw_texture(&texture_2, [400.0, 400.0]);
        });
    });

    Ok(())
}
