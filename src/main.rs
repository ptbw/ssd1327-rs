extern crate ssd1327_driver;

use ssd1327_driver::ssd1327::*;

fn main() {
	let mut display = SSD1327::new();
	display.begin().unwrap();
	display.clear();    
	display.update_all().unwrap();
	
	//println!("***draw pixel");
	//display.draw_pixel(64, 64, WHITE).unwrap();    	
   
	//println!("***draw line");
	//display.draw_line( 0,0,127,0, WHITE,1).unwrap();		
	//display.draw_line( 0,127,127,127, WHITE,1).unwrap();
	//display.draw_line( 0,0,0,127, WHITE,1).unwrap();
	//display.draw_line( 127,0,127,127, WHITE,1).unwrap();
	
	//println!("***draw rectangle");
	display.draw_rectangle(20, 20, 110, 30, WHITE, true);
	
	//println!("***draw text");
	display.draw_text(20, 42, "Amy's Dad's", WHITE);
	display.draw_text(12, 56, "SSD1327 driver", WHITE);
	display.draw_text(20, 68, "1.5inch OLED", WHITE);
	display.draw_text(32, 84, "in Rust!!!", WHITE);
	
	display.update_all().unwrap();
	//display.update().unwrap();	
}

