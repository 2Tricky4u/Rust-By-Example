use std::fmt::{write, Formatter};

pub(crate) fn hello() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // Positional arguments can be used. Specifying an integer inside `{}`
    // determines which additional argument will be replaced. Arguments start
    // at 0 immediately after the format string.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!("{subject} {verb} {object}",
             object = "the lazy dog",
             subject = "the quick brown fox",
             verb = "jumps over");

    // Different formatting can be invoked by specifying the format character
    // after a `:`.
    println!("Base 10:               {}", 69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c

    // You can right-justify text with a specified width. This will
    // output "    1". (Four white spaces and a "1", for a total width of 5.)
    println!("{number:>5}", number = 1);

    // You can pad numbers with extra zeroes,
    println!("{number:0>5}", number = 1); // 00001
    // and left-adjust by flipping the sign. This will output "10000".
    println!("{number:0<5}", number = 1); // 10000

    // You can use named arguments in the format specifier by appending a `$`.
    println!("{number:0>width$}", number = 1, width = 5);

    // Rust even checks to make sure the correct number of arguments are used.
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // FIXME ^ Add the missing argument: "James"

    // Only types that implement fmt::Display can be formatted with `{}`. User-
    // defined types do not implement fmt::Display by default.

    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct Structure(i32);

    // This will not compile because `Structure` does not implement
    // fmt::Display.
    //println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line

    // For Rust 1.58 and above, you can directly capture the argument from a
    // surrounding variable. Just like the above, this will output
    // "    1", 4 white spaces and a "1".
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    // Print pi with precision of 3

    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi);
}

pub(crate) fn hello_debug() {
    // Derive the `fmt::Debug` implementation for `Structure`. `Structure`
    // is a structure which contains a single `i32`.
    #[derive(Debug)]
    struct Structure(i32);

    // Put a `Structure` inside of the structure `Deep`. Make it printable
    // also.
    #[derive(Debug)]
    struct Deep(Structure);

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor = "actor's");

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));

    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);
}

pub(crate) fn hello_display() {
    use std::fmt; // Import `fmt`

    // A structure holding two numbers. `Debug` will be derived so the results can
    // be contrasted with `Display`.
    #[derive(Debug)]
    struct MinMax(i64, i64);

    // Implement `Display` for `MinMax`.
    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Use `self.number` to refer to each positional data point.
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    // Define a structure where the fields are nameable for comparison.
    #[derive(Debug)]
    struct Point2D {
        x: f64,
        y: f64,
    }

    // Similarly, implement `Display` for `Point2D`.
    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Customize so only `x` and `y` are denoted.
            write!(f, "x: {}, y: {}", self.x, self.y)
        }
    }

    impl fmt::Binary for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let string = format!("{}/{}", self.x, self.y);
            f.pad_integral(true, "", &string)
        }
    }

    #[derive(Debug)]
    struct Complex {
        real: f32,
        imaginary: f32,
    }

    impl fmt::Display for Complex {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Customize so only `x` and `y` are denoted.
            write!(f, "real: {:#}, imag: {:#}", self.real, self.imaginary)
        }
    }

    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // Error. Both `Debug` and `Display` were implemented, but `{:b}`
    // requires `fmt::Binary` to be implemented. This will not work.
    println!("What does Point2D look like in binary: {:b}?", point);

    let compl = Complex { real: 1.0, imaginary: 0.2 };

    println!("Compare complex:");
    println!("Display: {}", compl);
    println!("Debug: {:?}", compl);
}

pub(crate) fn hello_list() {
    use std::fmt; // Import the `fmt` module.

    // Define a structure named `List` containing a `Vec`.
    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Extract the value using tuple indexing,
            // and create a reference to `vec`.
            let vec = &self.0;

            write!(f, "[")?;

            // Iterate over `v` in `vec` while enumerating the iteration
            // count in `count`.
            for (count, v) in vec.iter().enumerate() {
                // For every element except the first, add a comma.
                // Use the ? operator to return on errors.
                if count != 0 { write!(f, ", ")?; }
                write!(f, "{}: {}", count, v)?;
            }

            // Close the opened bracket and return a fmt::Result value.
            write!(f, "]")
        }
    }

    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}

pub(crate) fn hello_formating() {
    use std::fmt::{self, Formatter, Display};

    struct City {
        name: &'static str,
        // Latitude
        lat: f32,
        // Longitude
        lon: f32,
    }

    impl Display for City {
        // `f` is a buffer, and this method must write the formatted string into it.
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
            let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

            // `write!` is like `format!`, but it will write the formatted string
            // into a buffer (the first argument).
            write!(f, "{}: {:.3}°{} {:.3}°{}",
                   self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
        }
    }

    #[derive(Debug)]
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }

    impl Display for Color {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let r:u32 = (self.red as u32) << 16;
            let g:u32 = (self.green as u32) << 8;
            let b: u32 = self.blue as u32;
            let rgb :u32= r | g | b;

            write!(f, "RGB ({:3}, {:3}, {:3}) 0x{:0>6X}", self.red, self.green, self.blue, rgb)
        }
    }

    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ] {
        println!("{}", city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ] {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("{}", color);
    }
}