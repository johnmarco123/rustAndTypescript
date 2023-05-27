main.rs
C:/Users/jmtas/uol/projects/rust_and_typescript/index.ts
mod shapes;

use crate::shapes::{rect::Rect, circle::Circle, area::Area};

fn main() {
    let rect = Rect::default();
    for point in &rect{
        
    }
    println!("{}", rect);
}
