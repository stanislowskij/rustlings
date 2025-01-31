struct ColorRegularStruct {
    // Done: Add the fields that the test `regular_structs` expects.
    // What types should the fields have? What are the minimum and maximum values for RGB colors?
    red: i32,
    green: i32,
    blue: i32
}

/* Done: Add the fields that the test `tuple_structs` expects */
struct ColorTupleStruct(i32, i32, i32);

#[derive(Debug)]
struct UnitStruct;

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        // Done: Instantiate a regular struct.
        let green: ColorRegularStruct = ColorRegularStruct {
            red: 0,
            green: 255,
            blue: 0
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // Done: Instantiate a tuple struct.
        let green: ColorTupleStruct = ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // Done: Instantiate a unit struct.
        let unit_struct: UnitStruct = UnitStruct;
        let message = format!("{unit_struct:?}s are fun!");

        assert_eq!(message, "UnitStructs are fun!");
    }
}
