fn main() {
    let cat: (&str, f64) = ("Furry McFurson", 3.5);

    // Done: Destructure the `cat` tuple in one statement so that the println works.
    let (name, age) = cat;

    println!("{name} is {age} years old");
}
