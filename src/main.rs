// Copyright (c) 2022 Scott Stone <scott.allan.stone@gmail.com>
use std::{process, thread, time::Duration};

fn main() {
    /* 
    Very simple program that kills explorer.exe and restarts it.
    Only supports windows, obviously
     */  
    if cfg!(target_os = "windows") {
        println!("Running on Windows");
    } else {
        println!("Not running on Windows, exiting..");
        return;
    }

    // Kill explorer.exe
    process::Command::new("taskkill.exe")
        .arg("/f")
        .arg("/im")
        .arg("explorer.exe")
        .spawn()
        .expect("Could not kill explorer.exe");

    println!("Killed explorer.exe, restarting..");
    thread::sleep(Duration::from_secs(1));

    // Start explorer.exe
    process::Command::new("explorer.exe")
        .spawn()
        .expect("Could not start explorer.exe");

    println!("Started explorer.exe, exiting..");
    thread::sleep(Duration::from_secs(1));
}