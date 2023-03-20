use raylib::{prelude::*};
use std::process::exit;
use std::{thread};

use std::time::Duration;
use std::thread::sleep;


fn main() {
    let mut changeable_draw_items_id: i32 = 0;
    let (mut rl, thread) = raylib::init()
        .size(1920, 1080)
        .title("Hello, World")
        .build();

    rl.toggle_fullscreen();
    rl.set_target_fps(60);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::from_hex("181818").unwrap());
        d.draw_text("What would you like to do?", 12, 12, 20, Color::WHITE);

        d.draw_rectangle(10, 40, 500, 200, Color::RAYWHITE);
        d.draw_text("Go to Desktop", 21, 41, 40, Color::BLACK);

        d.draw_rectangle(520, 40, 500, 200, Color::RAYWHITE);
        d.draw_text("Start main programs\nthen go to desktop", 531, 41, 40, Color::BLACK);

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
                thread::spawn(|| {
                    std::process::Command::new("vlc")
                    .arg("/usr/share/sounds/Kopete_Sent.ogg")
                    .arg("--play-and-exit")
                    .arg("-Irc")
                    .status()
                    .expect("Something Failed!!!");
                });
                changeable_draw_items_id = 1;
                draw_changeable_items(changeable_draw_items_id, d);

                sleep(Duration::from_secs(1));
                exit(0);
            }

            let btn1_cords: [i32; 4] = [520, 1020, 40, 240];
            if d.get_mouse_x() >= btn1_cords[0] && 
              d.get_mouse_x() <= btn1_cords[1]  &&
              d.get_mouse_y() >= btn1_cords[2]  && 
              d.get_mouse_y() <= btn1_cords[3] {
                println!("\nPressed BTN2:  x:{}  y:{}\n",d.get_mouse_x(), d.get_mouse_y());
                thread::spawn(|| {
                    std::process::Command::new("vlc")
                    .arg("/usr/share/sounds/Kopete_Sent.ogg")
                    .arg("--play-and-exit")
                    .arg("-Irc")
                    .status()
                    .expect("Something Failed!!!");
                    
                    // Start opening flatpak programs
                    // flatpak run com.discordapp.Discord
                    std::process::Command::new("flatpak")
                    .arg("run")
                    .arg("com.discordapp.Discord")
                    .spawn()
                    .expect("something went wrong starting discord");
                    
                    //com.spotify.Client
                    std::process::Command::new("flatpak")
                    .arg("run")
                    .arg("com.spotify.Client")
                    .spawn()
                    .expect("something went wrong starting spotify");

                    std::process::Command::new("com.google.ChromeDev")
                    .spawn()
                    .expect("something went wrong starting chrome");

                    std::process::Command::new("alacritty")
                    .spawn()
                    .expect("something went wrong starting alacritty");
                });
                // start here
                changeable_draw_items_id = 2;
                draw_changeable_items(changeable_draw_items_id, d);

                sleep(Duration::from_secs(5));
                exit(0);
            }
        }

    }
}

fn draw_changeable_items(id_level: i32, mut d: RaylibDrawHandle) -> RaylibDrawHandle{
    if id_level == 1 {
        d.draw_text("Exiting to desktop...                   good day", 10, 1050, 20, Color::WHITE);
    }

    if id_level == 2 {
        d.draw_text("All programs started. Will now exit in 5 sec...", 10, 1050, 20, Color::WHITE);
    }

    return d;
}
