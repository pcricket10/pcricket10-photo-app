#![allow(dead_code)]
#![allow(unused_imports)]

use std::str::FromStr;
use std::{fmt, fs, io, path::Path};

use crate::image::Image;
use filter::Filter;
mod filter;
mod image;

#[derive(Debug)]
enum EditOptions {
    AddNoise,
    Rotate,
    Resize,
    Crop,
    AddFilter,
}
#[derive(Debug)]
enum Filters {
    Blur,
    EmphasizeBlue,
    Greenify,
    Warm,
}

fn greet_and_display_photos() -> Vec<String> {
    println!("Welcome to the photo editor!");
    println!("Here are the photos available to edit:");
    let mut photos = Vec::new();

    for entry in std::fs::read_dir("images").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            continue;
        }
        let filename: String = match path.file_name() {
            Some(filename) => filename.to_string_lossy().to_string(),
            None => continue,
        };
        photos.push(filename);
    }
    for (i, photo) in photos.iter().enumerate() {
        println!("{}: {}", i + 1, photo);
        println!("============================");
    }
    photos
}

fn main() {
    greet_and_display_photos();
}
