#[derive(Debug)]
struct Rectangle {
    width: usize,
    height: usize,
}

impl Rectangle {
    fn area(&self) -> usize {
        return self.width * self.height
    }
}

fn main() {
    let rectangle = Rectangle{
        width: 1,
        height: 2
    };

    println!("The area of a rectangle is {}", rectangle.area())
}