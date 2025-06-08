use macroquad::prelude::*;

#[macroquad::main("Wolfenstein by AI")]
async fn main() {
    println!("Starting Wolfenstein by AI...");
    
    loop {
        // Clear the screen with a dark color
        clear_background(BLACK);
        
        // Draw title text
        let title = "WOLFENSTEIN BY AI";
        let title_size = 60.0;
        let title_width = measure_text(title, None, title_size as u16, 1.0).width;
        draw_text(
            title,
            (screen_width() - title_width) * 0.5,
            screen_height() * 0.3,
            title_size,
            GREEN,
        );
        
        // Draw subtitle
        let subtitle = "Hello World from Rust!";
        let subtitle_size = 40.0;
        let subtitle_width = measure_text(subtitle, None, subtitle_size as u16, 1.0).width;
        draw_text(
            subtitle,
            (screen_width() - subtitle_width) * 0.5,
            screen_height() * 0.5,
            subtitle_size,
            WHITE,
        );
        
        // Draw instructions
        let instructions = "Press ESC to exit";
        let inst_size = 20.0;
        let inst_width = measure_text(instructions, None, inst_size as u16, 1.0).width;
        draw_text(
            instructions,
            (screen_width() - inst_width) * 0.5,
            screen_height() * 0.7,
            inst_size,
            GRAY,
        );
        
        // Draw a simple rectangle for visual effect
        draw_rectangle_lines(
            50.0, 
            50.0, 
            screen_width() - 100.0, 
            screen_height() - 100.0, 
            3.0, 
            GREEN
        );
        
        // Exit on ESC
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        
        // This tells macroquad to wait for the next frame
        next_frame().await;
    }
    
    println!("Wolfenstein by AI shutting down...");
}
