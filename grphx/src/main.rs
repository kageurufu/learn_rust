extern crate sdl2;

use sdl2::video;
use sdl2::render;
use sdl2::pixels;
use sdl2::event;
use sdl2::keycode::KeyCode;

fn main() {
    sdl2::init(sdl2::INIT_VIDEO);

    let window = match video::Window::new("rust-sdl2 test", video::WindowPos::PosCentered, video::WindowPos::PosCentered, 800, 600, video::OPENGL) {
        Ok(window)  => window,
        Err(err)    => panic!(format!("Failed to create window: {}", err))
    };

    let renderer = match render::Renderer::from_window(window, render::RenderDriverIndex::Auto, render::ACCELERATED | render::PRESENTVSYNC) {
        Ok(renderer)    => renderer,
        Err(err)        => panic!(format!("Failed to create renderer: {}", err))
    };
    
    let mut r = 255;
    let mut g = 255;
    let mut b = 255;
    let mut up = true;

    'main : loop {
        'event : loop {
            match event::poll_event() {
                event::Event::Quit(_) => break 'main,
                event::Event::KeyDown(_, _, key, _, _, _) => {
                    if key == KeyCode::Escape {
                        break 'main
                    }
                },
                event::Event::None => break 'event,
                _ => {}
            }
        }
        
        if up == true {
            if r > 0 { r -= 1 }
            else if g > 0 { g -= 1 }
            else if b > 0 { b -= 1 }
            else { up = false }
        } else {
            if b < 255 { b+= 1 }
            else if g < 255 { g += 1 }
            else if r < 255 { r += 1 }
            else { up = true }
        }


        let _ = renderer.set_draw_color(pixels::Color::RGB(r, g, b));
        let _ = renderer.clear();
        renderer.present();
    }

    sdl2::quit();
}
