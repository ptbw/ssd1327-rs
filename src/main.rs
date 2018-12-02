extern crate image;
extern crate ssd1327_driver;

use ssd1327_driver::ssd1327::*;
//use image::*;

fn main() {
	let mut display = SSD1327::new("/dev/i2c-9");
	display.begin().unwrap();
	display.clear();    
	display.update_all().unwrap();
	
	//println!("***draw pixel");
	//display.draw_pixel(2, 2, WHITE).unwrap();    	
   
	//println!("***draw line");
	//display.draw_line( 0,0,127,0, WHITE,1).unwrap();		
	//display.draw_line( 0,127,127,127, WHITE,1).unwrap();
	//display.draw_line( 0,0,0,127, WHITE,1).unwrap();
	//display.draw_line( 127,0,127,127, WHITE,1).unwrap();
	
	//println!("***draw rectangle");
	//let mut col = 0;
	//for grey in 0x01..0x0F {
		//display.draw_rectangle(col + 5, 5, col + 10, 40, grey, true).unwrap();
		//col = col + 10;
	//}
		
	//println!("***draw text");
	//display.draw_text(20, 42, "Amy's Dad's", WHITE).unwrap();
	//display.draw_text(12, 52, "SSD1327 driver", LT_GREY).unwrap();
	//display.draw_text(20, 62, "1.5inch OLED", LT_GREY).unwrap();
	//display.draw_text(28, 84, "in Rust!!!", LT_GREY).unwrap();
	
	display.draw_text(4, 4, "Forest Fighters", LT_GREY).unwrap();
	let tiny = image::open("The Canyons of Mars Menu Item.jpg").unwrap();
	
	display.draw_image( 0, 16, tiny ).unwrap();
	display.draw_text(4, 108, "Canyons of Mars", WHITE).unwrap();
	//display.update_all().unwrap();
	display.update().unwrap();	
}

