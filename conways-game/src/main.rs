mod color;
mod framebuffer;

use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use color::Color;
use framebuffer::Framebuffer;

fn count_live_neighbors(framebuffer: &Framebuffer, x: isize, y: isize) -> usize {
    let mut count = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x + dx;
            let ny = y + dy;
            if framebuffer.is_alive(nx, ny) {
                count += 1;
            }
        }
    }
    count
}

fn render(framebuffer: &mut Framebuffer) {
    let mut new_buffer = framebuffer.buffer.clone();

    for y in 0..framebuffer.height as isize {
        for x in 0..framebuffer.width as isize {
            let live_neighbors = count_live_neighbors(framebuffer, x, y);
            if framebuffer.is_alive(x, y) {
                if live_neighbors < 2 || live_neighbors > 3 {
                    new_buffer[(y as usize) * framebuffer.width + (x as usize)] = framebuffer.background_color.clone();
                }
            } else {
                if live_neighbors == 3 {
                    new_buffer[(y as usize) * framebuffer.width + (x as usize)] = framebuffer.current_color.clone();
                }
            }
        }
    }

    framebuffer.buffer = new_buffer;
}

fn patterns(framebuffer: &mut Framebuffer) {
    // Pulsar
    framebuffer.point(48,48);
    framebuffer.point(47,48);
    framebuffer.point(47,47);
    framebuffer.point(48,46);
    framebuffer.point(49,46);
    framebuffer.point(49,47);
    framebuffer.point(51,47);
    framebuffer.point(51,46);
    framebuffer.point(52,46);
    framebuffer.point(53,47);
    framebuffer.point(53,48);
    framebuffer.point(52,48);
    framebuffer.point(52,50);
    framebuffer.point(53,50);
    framebuffer.point(53,51);
    framebuffer.point(52,52);
    framebuffer.point(51,52);
    framebuffer.point(51,51);
    framebuffer.point(49,51);
    framebuffer.point(49,52);
    framebuffer.point(48,52);
    framebuffer.point(47,51);
    framebuffer.point(47,50);
    framebuffer.point(48,50);
    framebuffer.point(43,52);
    framebuffer.point(44,52);
    framebuffer.point(45,52);
    framebuffer.point(45,51);
    framebuffer.point(45,47);
    framebuffer.point(45,46);
    framebuffer.point(44,46);
    framebuffer.point(43,46);
    framebuffer.point(47,42);
    framebuffer.point(47,43);
    framebuffer.point(47,44);
    framebuffer.point(48,44);
    framebuffer.point(52,44);
    framebuffer.point(53,44);
    framebuffer.point(53,43);
    framebuffer.point(53,42);
    framebuffer.point(57,46);
    framebuffer.point(56,46);
    framebuffer.point(55,46);
    framebuffer.point(55,47);
    framebuffer.point(55,51);
    framebuffer.point(55,52);
    framebuffer.point(56,52);
    framebuffer.point(57,52);
    framebuffer.point(53,56);
    framebuffer.point(53,55);
    framebuffer.point(53,54);
    framebuffer.point(52,54);
    framebuffer.point(48,54);
    framebuffer.point(47,54);
    framebuffer.point(47,55);
    framebuffer.point(47,56);

    // Glider
    framebuffer.point(10,80);
    framebuffer.point(11,79);
    framebuffer.point(12,79);
    framebuffer.point(11,78);
    framebuffer.point(10,78);

    // Loaf
    framebuffer.point(45,62);
    framebuffer.point(46,62);
    framebuffer.point(47,63);
    framebuffer.point(47,64);
    framebuffer.point(46,65);
    framebuffer.point(45,64);
    framebuffer.point(44,63);

    // Beacon
    framebuffer.point(49,19);
    framebuffer.point(50,19);
    framebuffer.point(49,20);
    framebuffer.point(50,20);
    framebuffer.point(51,21);
    framebuffer.point(52,21);
    framebuffer.point(51,22);
    framebuffer.point(52,22);

    // MWSS
    framebuffer.point(10,20);
    framebuffer.point(11,20);
    framebuffer.point(12,20);
    framebuffer.point(10,21);
    framebuffer.point(11,21);
    framebuffer.point(12,21);
    framebuffer.point(11,22);
    framebuffer.point(12,22);
    framebuffer.point(13,19);
    framebuffer.point(14,19);
    framebuffer.point(14,20);
    framebuffer.point(14,21);
    framebuffer.point(15,20);
    framebuffer.point(13,21);
    framebuffer.point(13,22);

    // Toad1
    framebuffer.point(58,76);
    framebuffer.point(59,77);
    framebuffer.point(59,78);
    framebuffer.point(57,79);
    framebuffer.point(56,78);
    framebuffer.point(56,77);
    
    // LWSS
    framebuffer.point(97,62);
    framebuffer.point(96,63);
    framebuffer.point(97,63);
    framebuffer.point(98,63);
    framebuffer.point(95,64);
    framebuffer.point(96,64);
    framebuffer.point(98,64);
    framebuffer.point(95,65);
    framebuffer.point(96,65);
    framebuffer.point(97,65);
    framebuffer.point(96,66);
    framebuffer.point(97,66);

    // Pentadecathlon
    framebuffer.point(71,24);
    framebuffer.point(70,25);
    framebuffer.point(71,25);
    framebuffer.point(72,25);
    framebuffer.point(70,28);
    framebuffer.point(71,28);
    framebuffer.point(72,28);
    framebuffer.point(70,30);
    framebuffer.point(70,31);
    framebuffer.point(72,30);
    framebuffer.point(72,31);
    framebuffer.point(70,33);
    framebuffer.point(71,33);
    framebuffer.point(72,33);
    framebuffer.point(70,36);
    framebuffer.point(71,36);
    framebuffer.point(72,36);
    framebuffer.point(71,37);

    // HWSS1
    framebuffer.point(78,76);
    framebuffer.point(77,76);
    framebuffer.point(76,76);
    framebuffer.point(75,76);
    framebuffer.point(74,76);
    framebuffer.point(73,76);
    framebuffer.point(73,77);
    framebuffer.point(73,78);
    framebuffer.point(74,79);
    framebuffer.point(76,80);
    framebuffer.point(77,80);
    framebuffer.point(79,79);
    framebuffer.point(79,77);

    // Boat
    framebuffer.point(46,75);
    framebuffer.point(45,76);
    framebuffer.point(46,77);
    framebuffer.point(47,77);
    framebuffer.point(47,76);

    // HWSS2
    framebuffer.point(15,47);
    framebuffer.point(16,47);
    framebuffer.point(17,47);
    framebuffer.point(18,47);
    framebuffer.point(19,47);
    framebuffer.point(20,47);
    framebuffer.point(20,48);
    framebuffer.point(20,49);
    framebuffer.point(19,50);
    framebuffer.point(17,51);
    framebuffer.point(16,51);
    framebuffer.point(14,50);
    framebuffer.point(14,48);

    // Block
    framebuffer.point(20,65);
    framebuffer.point(21,65);
    framebuffer.point(20,66);
    framebuffer.point(21,66);

    // Bee-hive
    framebuffer.point(67,54);
    framebuffer.point(68,54);
    framebuffer.point(69,55);
    framebuffer.point(68,56);
    framebuffer.point(67,56);
    framebuffer.point(66,55);

    // Tub
    framebuffer.point(76,60);
    framebuffer.point(75,61);
    framebuffer.point(77,61);
    framebuffer.point(76,62);

    // Blinker
    framebuffer.point(26,30);
    framebuffer.point(25,30);
    framebuffer.point(27,30);

    // Toad2
    framebuffer.point(36,5);
    framebuffer.point(37,6);
    framebuffer.point(38,6);
    framebuffer.point(39,4);
    framebuffer.point(38,3);
    framebuffer.point(37,3);

}   

fn main() {
    let window_width = 800;
    let window_height = 600;

    let framebuffer_width = 100;
    let framebuffer_height = 100;

    let frame_delay = Duration::from_millis(50);

    let mut framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Rust Graphics - Render Loop Example",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    framebuffer.set_current_color(Color::from_hex("live", 0xFFBE00));

    patterns(&mut framebuffer);

    while window.is_open() {
        // listen to inputs
        if window.is_key_down(Key::Escape) {
            break;
        }

        // Render
        render(&mut framebuffer);

        // Convert the framebuffer buffer to u32 format
        let u32_buffer = framebuffer.to_u32_buffer();

        // Update the window with the framebuffer contents
        window
            .update_with_buffer(&u32_buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
