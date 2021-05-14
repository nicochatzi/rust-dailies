#[derive(Debug)]
enum Colors {
    Black,
    White,
    Green,
    Other(u32),
}

impl Colors {
    fn print(self) {
        match self {
            Colors::Black => println!("Black"),
            Colors::White => println!("White"),
            Colors::Green => println!("Green"),
            Colors::Other(hex_value) => println!("{:x}", hex_value),
        }
    }

    fn print_if_green(self) {
        if let Colors::Green = self {
            self.print();
        }
    }
}

#[derive(Debug)]
enum Error {
    InvalidColor(Colors),
    Other(u32),
}

// enum Result<T, E> {
//     Ok(T),
//     Err(E)
// }

// enum Result {
//     Ok(Colors),
//     Err(Error)
// }

fn try_get_color() -> Result<Colors, Error> {
    if rand::random() {
        Ok(Colors::Black)
    } else {
        Err(Error::InvalidColor(Colors::Green))
    }
}

// enum Option<T> {
//     Some(T),
//     None
// }

// enum Option {
//     Some(Colors),
//     None
// }

fn try_find_color() -> Option<Colors> {
    if rand::random() {
        Some(Colors::White)
    } else {
        None
    }
}

fn main() {
    Colors::Black.print();
    Colors::Black.print_if_green();
    Colors::Green.print_if_green();
    Colors::Other(0x123ab).print();

    // let e = Error::InvalidColor(Colors::Green);

    // match Error::Other(100) {
    //     Error::InvalidColor(c) => c.print(),
    //     Error::Other(code) => panic!("{}", code),
    // }

    // let color_result = try_get_color();

    // match color_result {
    //     Ok(color) => color.print(),
    //     Err(e) => panic!("{:?}", e),
    // }

    let color = try_get_color().unwrap();
    color.print();

    if let Some(color) = try_find_color() {
        color.print();
    }

    let could_be_a_color = try_find_color();

    // auto * ptr = get_ptr();
    // if (ptr != nullptr) {
    //     //..
    // }

    // if (response.data !== undefined ) {
    //     //..
    // }
}
