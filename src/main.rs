struct Rectangulo {
    alto: u32,
    ancho: u32,
}

fn main() {

    let rectangulo = Rectangulo {
        alto : 23,
        ancho : 24,
    };
    
    let area = calculo_area(&rectangulo);

    println!("El area del restangulo es: {}", area);


    
}

fn calculo_area(rectangulo : &Rectangulo) -> u32 {
    rectangulo.alto * rectangulo.ancho
}