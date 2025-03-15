pub fn execute_rectangle_sample() {
    sample_using_variables();
    sample_using_tuples();
    sample_using_structures();
}

fn sample_using_variables() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels",
        area(width1, height1)
    );

    fn area(width: u32, height: u32) -> u32 {
        width * height
    }
}

fn sample_using_tuples() {
    let rect1 = (30, 50);

    println!("The area of the rectangle is {} square pixels", area(rect1));

    fn area(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }
}

fn sample_using_structures() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn width(&self) -> bool {
            self.width > 0
        }

        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let square = Rectangle::square(30);

    println!("{}", square.height);

    rect1.area();

    rect1.can_hold(&rect2);

    println!(
        "The area of the rectangle is {} square pixels",
        area(&rect1)
    );
    println!("rect1 is {:?}", rect1);

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }
}

fn sample_mult_impl_blocks() {
    let str = String::from("test");

    // Definindo a trait
    trait StringExtensions {
        fn square(size: u32) -> Self;
    }

    impl StringExtensions for String {
        fn square(size: u32) -> Self {
            // Aqui você pode definir o comportamento que deseja
            String::from("AWE")
        }
    }
}

fn sample_trait(){
    trait Greet {
        fn greet(&self);
    }

    struct Pessoa;

    impl Greet for Pessoa {
        fn greet(&self) {
            println!("Olá, pessoa!");
        }
    }

    struct Carro;

    impl Greet for Carro {
        fn greet(&self) {
            println!("Olá, carro!");
        }
    }

    fn saudar(g: &dyn Greet) {
        g.greet();
    }

    let pessoa = Pessoa;
    let carro = Carro;


    saudar(&pessoa); // Saída: Olá, pessoa!
    saudar(&carro);  // Saída: Olá, carro!

}
