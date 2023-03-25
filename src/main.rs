// MIT License

// Copyright (c) 2023 O'Brien

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

// This is the main file for the MASS project

use raylib::{prelude::*};
use std::process::exit;

use std::time::Duration;
use std::thread::sleep;
use chrono::{Local};

mod execution_mod;


fn main() {
    let version = "1.0.0";
    let mut changeable_draw_items_id: i32 = 0;
    let (mut rl, thread) = raylib::init()
        .size(1920, 1080)
        .title("Hello, World")
        .build();

    rl.toggle_fullscreen();
    rl.set_target_fps(60);

    let mass_logo = rl
    .load_texture(&thread, "/home/messycode/codef/rust/mass/src/images/massLogo.png")
    .expect("Failed to load texture");

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        // PADING 10

        d.clear_background(Color::from_hex("181818").unwrap());
        d.draw_text("What would you like to do?", 12, 12, 20, Color::WHITE);

        d.draw_rectangle(10, 40, 500, 200, Color::RAYWHITE);
        d.draw_text("Do nothing\nGo to Desktop", 21, 41, 40, Color::BLACK);

        d.draw_rectangle(520, 40, 500, 200, Color::RAYWHITE);
        d.draw_text("Start main programs\nGo to desktop", 531, 41, 40, Color::BLACK);

        d.draw_rectangle(1030, 40, 500, 200, Color::RAYWHITE);
        d.draw_text("Browser/Discord\nGo to desktop", 1041, 41, 40, Color::BLACK);

        d.draw_text("Just open programs then start desktop", 12, 250, 20, Color::WHITE);

        d.draw_rectangle(10, 290, 500, 200, Color::from_hex("5865F2").unwrap());
        d.draw_text("Discord", 21, 290, 60, Color::WHITE);

        d.draw_rectangle(520, 290, 500, 200, Color::from_hex("1CCA5A").unwrap());
        d.draw_text("Spotify", 531, 290, 60, Color::BLACK);

        d.draw_rectangle(1030, 290, 500, 200, Color::from_hex("DC8623").unwrap());
        d.draw_text("Terminal", 1041, 290, 60, Color::BLACK);

        d.draw_rectangle(10, 500, 500, 200, Color::from_hex("F2C94C").unwrap());
        d.draw_text("Browser", 21, 500, 60, Color::BLACK);

        d.draw_rectangle(520, 500, 500, 200, Color::from_hex("144376").unwrap());
        d.draw_text("Steam", 531, 500, 60, Color::WHITE);



        // d.draw_texture(&massLogo, 1920 - 300, 10, Color::WHITE);
        d.draw_texture_ex(
            &mass_logo,
            Vector2::new(1586.0, 40.0),
            0.0,
            1.5,
            Color::WHITE,
        );
        // time under the picture

        let formatted_time = Local::now().format("%H:%M:%S").to_string();
        d.draw_text(
            &format!("Time: {}", formatted_time),
            1586,
            500 - 40,
            40,
            Color::WHITE,
        );

        d.draw_text(
            &format!("Version:  {}", version),
            1586,
            500 - 40 + 40,
            40,
            Color::WHITE,
        );
        
        d = draw_changeable_items(changeable_draw_items_id, d);

        if d.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            /*  btn cords help
                1st number: X
                2nd number: X + width
                3rd number: Y
                4th number: Y + hight
             */
            let btn1_cords: [i32; 4] = [10, 510, 40, 240];
            if d.get_mouse_x() >= btn1_cords[0] && 
              d.get_mouse_x() <= btn1_cords[1]  &&
              d.get_mouse_y() >= btn1_cords[2]  && 
              d.get_mouse_y() <= btn1_cords[3] {
                println!("\nPressed BTN1:  x:{}  y:{}\n",d.get_mouse_x(), d.get_mouse_y());

                changeable_draw_items_id = execution_mod::desktop_exit();
                draw_changeable_items(changeable_draw_items_id, d);

                sleep(Duration::from_secs(2));
                exit(0);
            }

            let btn1_cords: [i32; 4] = [520, 1020, 40, 240];
            if d.get_mouse_x() >= btn1_cords[0] && 
              d.get_mouse_x() <= btn1_cords[1]  &&
              d.get_mouse_y() >= btn1_cords[2]  && 
              d.get_mouse_y() <= btn1_cords[3] {
                println!("\nPressed BTN2:  x:{}  y:{}\n",d.get_mouse_x(), d.get_mouse_y());
                
                changeable_draw_items_id = execution_mod::open_all_main_program_exit();
                draw_changeable_items(changeable_draw_items_id, d);

                sleep(Duration::from_secs(5));
                exit(0);
            }
        
            let btn1_cords: [i32; 4] = [1030, 1530, 40, 240];
            if d.get_mouse_x() >= btn1_cords[0] && 
              d.get_mouse_x() <= btn1_cords[1]  &&
              d.get_mouse_y() >= btn1_cords[2]  && 
              d.get_mouse_y() <= btn1_cords[3] {
                println!("\nPressed BTN2:  x:{}  y:{}\n",d.get_mouse_x(), d.get_mouse_y());

                changeable_draw_items_id = execution_mod::discord_browser_exit();
                draw_changeable_items(changeable_draw_items_id, d);

                sleep(Duration::from_secs(5));
                exit(0);
            }
            
            let btn1_cords: [i32; 4] = [10, 510, 290, 490];
            if d.get_mouse_x() >= btn1_cords[0] && 
              d.get_mouse_x() <= btn1_cords[1]  &&
              d.get_mouse_y() >= btn1_cords[2]  && 
              d.get_mouse_y() <= btn1_cords[3] {
                println!("\nPressed BTN2:  x:{}  y:{}\n",d.get_mouse_x(), d.get_mouse_y());

                changeable_draw_items_id = execution_mod::discord_exit();
                draw_changeable_items(changeable_draw_items_id, d);

                sleep(Duration::from_secs(3));
                exit(0);
            }

            let btn1_cords: [i32; 4] = [520, 1020, 290, 490];
            if d.get_mouse_x() >= btn1_cords[0] && 
              d.get_mouse_x() <= btn1_cords[1]  &&
              d.get_mouse_y() >= btn1_cords[2]  && 
              d.get_mouse_y() <= btn1_cords[3] {
                println!("\nPressed BTN2:  x:{}  y:{}\n",d.get_mouse_x(), d.get_mouse_y());


                changeable_draw_items_id = execution_mod::spotify_exit();
                draw_changeable_items(changeable_draw_items_id, d);

                sleep(Duration::from_secs(3));
                exit(0);
            }

            let btn1_cords: [i32; 4] = [1030, 1530, 290, 490];
            if d.get_mouse_x() >= btn1_cords[0] && 
              d.get_mouse_x() <= btn1_cords[1]  &&
              d.get_mouse_y() >= btn1_cords[2]  && 
              d.get_mouse_y() <= btn1_cords[3] {
                println!("\nPressed BTN2:  x:{}  y:{}\n",d.get_mouse_x(), d.get_mouse_y());


                changeable_draw_items_id = execution_mod::alacritty_exit();
                draw_changeable_items(changeable_draw_items_id, d);

                sleep(Duration::from_secs(3));
                exit(0);
            }

            let btn1_cords: [i32; 4] = [10, 510, 500, 700];
            if d.get_mouse_x() >= btn1_cords[0] && 
              d.get_mouse_x() <= btn1_cords[1]  &&
              d.get_mouse_y() >= btn1_cords[2]  && 
              d.get_mouse_y() <= btn1_cords[3] {
                println!("\nPressed BTN2:  x:{}  y:{}\n",d.get_mouse_x(), d.get_mouse_y());


                changeable_draw_items_id = execution_mod::browser_exit();
                draw_changeable_items(changeable_draw_items_id, d);

                sleep(Duration::from_secs(3));
                exit(0);
            }

            let btn1_cords: [i32; 4] = [520, 1020, 500, 700];
            if d.get_mouse_x() >= btn1_cords[0] && 
              d.get_mouse_x() <= btn1_cords[1]  &&
              d.get_mouse_y() >= btn1_cords[2]  && 
              d.get_mouse_y() <= btn1_cords[3] {
                println!("\nPressed BTN2:  x:{}  y:{}\n",d.get_mouse_x(), d.get_mouse_y());


                changeable_draw_items_id = execution_mod::steam_exit();
                draw_changeable_items(changeable_draw_items_id, d);

                sleep(Duration::from_secs(3));
                exit(0);
            }
        }

    }
}

fn draw_changeable_items(id_level: i32, mut d: RaylibDrawHandle) -> RaylibDrawHandle {
    if id_level == 1 {
        d.draw_text("Exiting to desktop...                   good day", 10, 1050, 20, Color::WHITE);
    }
    if id_level == 2 {
        d.draw_text("All programs started. Will now exit in 5 sec...", 10, 1050, 20, Color::WHITE);
    }
    if id_level == 3 {
        d.draw_text("Chrome and Discord is now starting, exiting MASS.", 10, 1050, 20, Color::WHITE);
    }
    if id_level == 4 {
        d.draw_text("Starting discord.... good day", 10, 1050, 20, Color::WHITE);
    }
    if id_level == 5 {
        d.draw_text("Starting spotify.... good day", 10, 1050, 20, Color::WHITE);
    }
    if id_level == 6 {
        d.draw_text("Starting alacritty.... good day", 10, 1050, 20, Color::WHITE);
    }
    if id_level == 7 {
        d.draw_text("Starting chrome.... good day", 10, 1050, 20, Color::WHITE);
    }
    if id_level == 8 {
        d.draw_text("Starting steam.... good day                --NOTE: steam takes awhile to load", 10, 1050, 20, Color::WHITE);
    }

    return d;
}