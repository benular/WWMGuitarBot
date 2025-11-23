


use scrap::{Capturer, Display};
use image::{RgbaImage, Rgba};
use enigo::{Enigo, Key, Keyboard, Settings, Direction};
use std::time::{Duration, Instant};
use std::io::{self, Write};

struct Lane {
    x: u32,           
    color: Rgba<u8>,  
    key: Key,         
}

struct ChingeBot {
    capturer: Capturer,
    display: Display,
    lanes: Vec<Lane>,
    hitzone_y: u32,   
    scan_height: u32, 
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Chinge Bot - rhythm game automation");
    
    let (display, display_index) = select_display()?;
    let capturer = Capturer::new(display)?;
    let display_for_struct = get_display_by_index(display_index)?;
    
    let bot = ChingeBot {
        capturer,
        display: display_for_struct,
        lanes: generate_lanes(),
        hitzone_y: (1225 + 1269) / 2,  // Center of yTop and yBottom from ReadMe
        scan_height: 50,
    };
    
    println!("Bot configured. Press Ctrl+C to stop.");
    std::thread::sleep(Duration::from_secs(3));
    
    let mut bot = bot;
    bot.detect_and_play()
}

impl ChingeBot {
    fn detect_and_play(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut enigo = Enigo::new(&Settings::default())?;
        
        loop {
            let start = Instant::now();
            
            // Erfasse den gesamten Bildschirm
            let (frame, width, height) = match self.capturer.frame() {
                Ok(frame) => {
                    let w = self.display.width() as u32;
                    let h = self.display.height() as u32;
                    (frame, w, h)
                },
                Err(_) => {
                    std::thread::sleep(Duration::from_millis(1));
                    continue;
                }
            };
            
            // Konvertiere zu RgbaImage
            let image = Self::frame_to_image(&frame, width, height)?;
            
            // Prüfe jede Lane
            for lane in &self.lanes {
                if self.note_in_hitzone(&image, lane) {
                    enigo.key(lane.key.clone(), Direction::Click)?;
                }
            }
            
            // Ziel: ~120 Hz Polling
            let elapsed = start.elapsed();
            if elapsed < Duration::from_micros(8333) {
                std::thread::sleep(Duration::from_micros(8333) - elapsed);
            }
        }
    }
    
    fn frame_to_image(frame: &[u8], width: u32, height: u32) -> Result<RgbaImage, Box<dyn std::error::Error>> {
        
        // Scrap liefert BGRA-Format, konvertiere zu RGBA
        let mut rgba_data = Vec::with_capacity(frame.len());
        for chunk in frame.chunks_exact(4) {
            rgba_data.push(chunk[2]); // R
            rgba_data.push(chunk[1]); // G  
            rgba_data.push(chunk[0]); // B
            rgba_data.push(chunk[3]); // A
        }
        
        RgbaImage::from_raw(width, height, rgba_data)
            .ok_or("Failed to create image from frame".into())
    }
    
    fn note_in_hitzone(&self, image: &RgbaImage, lane: &Lane) -> bool {
        // Prüfe direkt an der Lane-Position
        let scan_start_y = self.hitzone_y.saturating_sub(self.scan_height);
        let scan_end_y = self.hitzone_y + 10;
        
        for y in scan_start_y..scan_end_y.min(image.height()) {
            if lane.x < image.width() {
                let area_size = 5; // 5x5 pixel area for average color
                let avg_color = self.get_avg_color(image, 
                    lane.x.saturating_sub(area_size/2), 
                    y.saturating_sub(area_size/2), 
                    area_size, 
                    area_size);
                if self.color_matches(&avg_color, &lane.color) {
                    return true;
                }
            }
        }
        false
    }
    
    fn color_matches(&self, pixel: &Rgba<u8>, target: &Rgba<u8>) -> bool {
        let threshold = 30; // Toleranz
        
        // Check for brightness above #a1a2a1 (161, 162, 161)
        let brightness_threshold = Rgba([161, 162, 161, 255]);
        if pixel[0] > brightness_threshold[0] && 
           pixel[1] > brightness_threshold[1] && 
           pixel[2] > brightness_threshold[2] {
            return true;
        }
        
        // Original color matching
        (pixel[0] as i32 - target[0] as i32).abs() < threshold &&
        (pixel[1] as i32 - target[1] as i32).abs() < threshold &&
        (pixel[2] as i32 - target[2] as i32).abs() < threshold
    }

    fn get_avg_color(&self, image: &RgbaImage, x: u32, y: u32, width: u32, height: u32) -> Rgba<u8> {
        let mut r_sum: u64 = 0;
        let mut g_sum: u64 = 0; 
        let mut b_sum: u64 = 0;
        let mut pixel_count: u64 = 0;

        let end_x = (x + width).min(image.width());
        let end_y = (y + height).min(image.height());

        for py in y..end_y {
            for px in x..end_x {
                let pixel = image.get_pixel(px, py);
                r_sum += pixel[0] as u64;
                g_sum += pixel[1] as u64;
                b_sum += pixel[2] as u64;
                pixel_count += 1;
            }
        }

        if pixel_count == 0 {
            return Rgba([0, 0, 0, 255]);
        }

        Rgba([
            (r_sum / pixel_count) as u8,
            (g_sum / pixel_count) as u8, 
            (b_sum / pixel_count) as u8,
            255
        ])
    }
}

fn select_display() -> Result<(Display, usize), Box<dyn std::error::Error>> {
    let displays = Display::all()?;
    
    if displays.is_empty() {
        return Err("No displays found".into());
    }
    
    if displays.len() == 1 {
        let display = displays.into_iter().next().unwrap();
        return Ok((display, 0));
    }
    
    println!("\nAvailable Displays:");
    println!("==================");
    
    for (index, display) in displays.iter().enumerate() {
        println!("{}. Display {} - {}x{}", 
            index + 1, 
            index,
            display.width(), 
            display.height()
        );
    }
    
    loop {
        print!("\nSelect display (1-{}): ", displays.len());
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        match input.trim().parse::<usize>() {
            Ok(choice) if choice >= 1 && choice <= displays.len() => {
                let selected_idx = choice - 1;
                let selected = displays.into_iter().nth(selected_idx).unwrap();
                println!("Selected: Display {} ({}x{})\n", 
                    selected_idx, selected.width(), selected.height());
                return Ok((selected, selected_idx));
            }
            _ => {
                println!("Invalid selection. Please enter a number between 1 and {}.", displays.len());
            }
        }
    }
}

fn get_display_by_index(index: usize) -> Result<Display, Box<dyn std::error::Error>> {
    let displays = Display::all()?;
    displays.into_iter().nth(index)
        .ok_or_else(|| format!("Display index {} not found", index).into())
}

fn generate_lanes() -> Vec<Lane> {
    vec![
        Lane { 
            x: (400 + 488) / 2,  // Lt - center of xLeft and xRight
            color: Rgba([0, 255, 0, 255]), 
            key: Key::Unicode('q') 
        },
        Lane { 
            x: (623 + 737) / 2,  // Lb
            color: Rgba([255, 0, 0, 255]), 
            key: Key::Unicode('w') 
        },
        Lane { 
            x: (950 + 1045) / 2, // DPadUp
            color: Rgba([255, 255, 0, 255]), 
            key: Key::Unicode('e') 
        },
        Lane { 
            x: (1516 + 1608) / 2, // Y
            color: Rgba([0, 0, 255, 255]), 
            key: Key::Unicode('r') 
        },
        Lane { 
            x: (1789 + 1888) / 2, // Rb
            color: Rgba([255, 165, 0, 255]), 
            key: Key::Unicode('t') 
        },
        Lane { 
            x: (2070 + 2161) / 2, // Rt
            color: Rgba([128, 0, 128, 255]), 
            key: Key::Unicode('y') 
        },
    ]
}

// TODO: Soll den jeweiligen Compositor erkennen und dementsprechend das Skript anpassen
// Prio 4
fn get_compositor() {
    let _wayland = std::env::var("WAYLAND_DISPLAY");
    let _x11 = std::env::var("DISPLAY");    


}
