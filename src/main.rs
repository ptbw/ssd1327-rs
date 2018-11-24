extern crate ssd1327_driver;

use ssd1327_driver::ssd1327::*;

fn main() {
    let mut display = SSD1327::new();
    display.begin().unwrap();
    display.clear();    
    display.update().unwrap();
    
    println!("***draw pixel");
    display.draw_pixel(64, 64, WHITE).unwrap();    
   
    println!("***draw line");
	display.draw_line( 0,0,126,0, WHITE,1).unwrap();	
	display.draw_line( 126,0,126,126, WHITE,1).unwrap();
	display.draw_line( 126,126,0,126, WHITE,1).unwrap();
	display.draw_line( 0,126,0,0, WHITE,1).unwrap();
	
	println!("***draw rectangle");
	display.draw_rectangle(8, 10, 110, 20, WHITE, true);
	
	println!("***draw text");
	display.draw_text(20, 22, "Amy's Dad's", WHITE);
	display.draw_text(12, 36, "SSD1327 driver", WHITE);
	display.draw_text(20, 48, "1.5inch OLED", WHITE);
	display.draw_text(32, 64, "in Rust!!!", WHITE);
	
	display.update().unwrap();
}

