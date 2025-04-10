// You can bring module paths into scopes and provide new names for them with
// the `use` and `as` keywords.

// Comment: I found this exercise pretty confusing, it took me a bit of reading
// to figure out what was actually going on with the use statements here without
// just relying on the compiler.
// Need to study Rust file structure more.
// `pub use` seems to essentially allow aliasing names of things in modules

mod delicious_snacks {
    // DONE: Add the following two `use` statements after fixing them.
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;

    mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie,
    );
}
