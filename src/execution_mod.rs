use std::{thread};

pub fn desktop_exit() -> i32{
    thread::spawn(|| {
        std::process::Command::new("vlc")
        .arg("/usr/share/sounds/Kopete_Sent.ogg")
        .arg("--play-and-exit")
        .arg("-Irc")
        .status()
        .expect("Something Failed!!!");

        std::process::Command::new("ulauncher")
        .arg("--no-window")
        .spawn()
        .expect("something went wrong starting ulauncher");
    });
    return 1;
}

pub fn open_all_main_program_exit() -> i32 {
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
        .arg("--working-directory")
        .arg("~")
        .spawn()
        .expect("something went wrong starting alacritty");

        std::process::Command::new("ulauncher")
        .arg("--no-window")
        .spawn()
        .expect("something went wrong starting ulauncher");
    });
    return 2;
}

pub fn discord_browser_exit() -> i32 {
    thread::spawn(|| {
        std::process::Command::new("vlc")
        .arg("/usr/share/sounds/Kopete_Sent.ogg")
        .arg("--play-and-exit")
        .arg("-Irc")
        .status()
        .expect("Something Failed!!!");

        std::process::Command::new("com.google.ChromeDev")
        .spawn()
        .expect("something went wrong starting chrome");

        std::process::Command::new("ulauncher")
        .arg("--no-window")
        .spawn()
        .expect("something went wrong starting ulauncher");

        std::process::Command::new("flatpak")
        .arg("run")
        .arg("com.discordapp.Discord")
        .spawn()
        .expect("something went wrong starting discord");
    });
    return 3;
}

pub fn discord_exit() -> i32 {
    thread::spawn(|| {
        std::process::Command::new("vlc")
        .arg("/usr/share/sounds/Kopete_Sent.ogg")
        .arg("--play-and-exit")
        .arg("-Irc")
        .status()
        .expect("Something Failed!!!");

        std::process::Command::new("ulauncher")
        .arg("--no-window")
        .spawn()
        .expect("something went wrong starting ulauncher");

        std::process::Command::new("flatpak")
        .arg("run")
        .arg("com.discordapp.Discord")
        .spawn()
        .expect("something went wrong starting discord");
    });
    return 4;
}

pub fn spotify_exit() -> i32 {
    thread::spawn(|| {
        std::process::Command::new("vlc")
        .arg("/usr/share/sounds/Kopete_Sent.ogg")
        .arg("--play-and-exit")
        .arg("-Irc")
        .status()
        .expect("Something Failed!!!");

        std::process::Command::new("ulauncher")
        .arg("--no-window")
        .spawn()
        .expect("something went wrong starting ulauncher");

        //com.spotify.Client
        std::process::Command::new("flatpak")
        .arg("run")
        .arg("com.spotify.Client")
        .spawn()
        .expect("something went wrong starting spotify");
    });
    return 5;
}

pub fn alacritty_exit() -> i32 {
    thread::spawn(|| {
        std::process::Command::new("vlc")
        .arg("/usr/share/sounds/Kopete_Sent.ogg")
        .arg("--play-and-exit")
        .arg("-Irc")
        .status()
        .expect("Something Failed!!!");

        std::process::Command::new("ulauncher")
        .arg("--no-window")
        .spawn()
        .expect("something went wrong starting ulauncher");

        std::process::Command::new("alacritty")
        .arg("--working-directory")
        .arg("~")
        .spawn()
        .expect("something went wrong starting alacritty");
    });
    return 6;
}

pub fn browser_exit() -> i32 {
    thread::spawn(|| {
        std::process::Command::new("vlc")
        .arg("/usr/share/sounds/Kopete_Sent.ogg")
        .arg("--play-and-exit")
        .arg("-Irc")
        .status()
        .expect("Something Failed!!!");

        std::process::Command::new("ulauncher")
        .arg("--no-window")
        .spawn()
        .expect("something went wrong starting ulauncher");

        std::process::Command::new("com.google.ChromeDev")
        .spawn()
        .expect("something went wrong starting chrome");
    });
    return 7;
}

pub fn steam_exit() -> i32 {
    thread::spawn(|| {
        std::process::Command::new("vlc")
        .arg("/usr/share/sounds/Kopete_Sent.ogg")
        .arg("--play-and-exit")
        .arg("-Irc")
        .status()
        .expect("Something Failed!!!");

        std::process::Command::new("ulauncher")
        .arg("--no-window")
        .spawn()
        .expect("something went wrong starting ulauncher");

        //flatpak run com.valvesoftware.Steam
        std::process::Command::new("flatpak")
        .arg("run")
        .arg("com.valvesoftware.Steam")
        .spawn()
    });
    return  8;
}