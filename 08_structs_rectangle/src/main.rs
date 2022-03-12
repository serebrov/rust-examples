#[derive(Debug)]
struct Rect {
    x0: u32,
    y0: u32,
    x1: u32,
    y1: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        (self.x1 - self.x0)*(self.y1-self.y0)
    }

    // Methods not necessary have `self` parameter.
    fn square(size: u32) -> Rect {
        Rect { x0: 0, y0: 0, x1: size, y1: size }
    }
}

fn main() {
    let r1 = Rect{ x0: 10, y0: 10, x1: 100, y1: 100 };
    println!("area {}", r1.area());

    let s1 = Rect::square(10);
    println!("area {}", s1.area());
}
