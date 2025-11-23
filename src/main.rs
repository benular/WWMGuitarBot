


use captrs::Capturer;
use image::{RgbaImage, Rgba};
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::time::{Duration, Instant};
use std::io::{self, Write};

struct Lane {
    x: u32,           
    color: Rgba<u8>,  
    key: Keycode,         
}

struct ChingeBot {
    capturer: Capturer,
    device_state: DeviceState,
    lanes: Vec<Lane>,
    hitzone_y: u32,   
    scan_height: u32, 
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Chinge Bot - rhythm game automation");
    
    let display_id = choose_display_id()?;
    let capturer = Capturer::new(display_id)?;
    let device_state = DeviceState::new();
    
    let mut bot = ChingeBot {
        capturer,
        device_state,
        lanes: generate_lanes(),
        hitzone_y: (1225 + 1269) / 2,  // Center of yTop and yBottom from ReadMe
        scan_height: 50,
    };
    
    println!("Bot configured. Press Ctrl+C to stop.");
    std::thread::sleep(Duration::from_secs(3));
    
    bot.detect_and_play()
}

impl ChingeBot {
    fn detect_and_play(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let start = Instant::now();
            
            // Capture screenshot
            let screenshot = self.capturer.capture_screen()?
                .ok_or("Failed to capture screen")?;
            let rgba_image = screenshot.to_rgba8();
            
            // Check each lane
            for lane in &self.lanes {
                if self.note_in_hitzone(&rgba_image, lane) {
                    // device_query is read-only, can't send keys
                    // This is a limitation - would need different approach
                    println!("Note detected in lane at x={}, would press {:?}", lane.x, lane.key);
                }
            }
            
            // Target: ~120 Hz polling
            let elapsed = start.elapsed();
            if elapsed < Duration::from_micros(8333) {
                std::thread::sleep(Duration::from_micros(8333) - elapsed);
            }
        }
    }
    
    // Removed frame_to_image - screenshots crate handles this automatically
    
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

fn choose_display_id() -> Result<usize, Box<dyn std::error::Error>> {
    // Try to get display count by attempting to create capturers
    let mut available_displays = Vec::new();
    
    for id in 0..10 { // Check up to 10 displays
        match Capturer::new(id) {
            Ok(_) => available_displays.push(id),
            Err(_) => break, // Stop when we can't create more capturers
        }
    }
    
    if available_displays.is_empty() {
        return Err("No displays found".into());
    }
    
    if available_displays.len() == 1 {
        println!("Using display {}", available_displays[0]);
        return Ok(available_displays[0]);
    }
    
    println!("\nAvailable Displays:");
    println!("==================");
    
    for (index, &display_id) in available_displays.iter().enumerate() {
        println!("{}. Display {}", index + 1, display_id);
    }
    
    loop {
        print!("\nSelect display (1-{}): ", available_displays.len());
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        match input.trim().parse::<usize>() {
            Ok(choice) if choice >= 1 && choice <= available_displays.len() => {
                let selected_id = available_displays[choice - 1];
                println!("Selected: Display {}\n", selected_id);
                return Ok(selected_id);
            }
            _ => {
                println!("Invalid selection. Please enter a number between 1 and {}.", available_displays.len());
            }
        }
    }
}

// Removed get_display_by_index - not needed with screenshots crate

fn generate_lanes() -> Vec<Lane> {
    vec![
        Lane { 
            x: (400 + 488) / 2,  // Lt - center of xLeft and xRight
            color: Rgba([0, 255, 0, 255]), 
            key: Keycode::Q
        },
        Lane { 
            x: (623 + 737) / 2,  // Lb
            color: Rgba([255, 0, 0, 255]), 
            key: Keycode::W
        },
        Lane { 
            x: (950 + 1045) / 2, // DPadUp
            color: Rgba([255, 255, 0, 255]), 
            key: Keycode::E
        },
        Lane { 
            x: (1516 + 1608) / 2, // Y
            color: Rgba([0, 0, 255, 255]), 
            key: Keycode::R
        },
        Lane { 
            x: (1789 + 1888) / 2, // Rb
            color: Rgba([255, 165, 0, 255]), 
            key: Keycode::T
        },
        Lane { 
            x: (2070 + 2161) / 2, // Rt
            color: Rgba([128, 0, 128, 255]), 
            key: Keycode::Y
        },
    ]
}

// TODO: Soll den jeweiligen Compositor erkennen und dementsprechend das Skript anpassen
// Prio 4
fn get_compositor() {
    let _wayland = std::env::var("WAYLAND_DISPLAY");
    let _x11 = std::env::var("DISPLAY");    


}
