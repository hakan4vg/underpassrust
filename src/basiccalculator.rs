pub fn add (a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract (a: i32, b: i32) -> i32 {
    a - b
}

pub fn multiply (a: i32, b: i32) -> i32 {
    a * b
}

pub fn divide (a: i32, b: i32) -> f32 {
    a as f32/ b as f32
}

pub fn modulo (a: i32, b: i32) -> i32 {
    a % b
}

pub fn power (a: i32, b: i32) -> i32 {
    a.pow(b as u32)
}

pub fn factorial (a: i32) -> i32 {
    if a == 0 {
        return 1;
    }
    else {
        let mut a_factorial = a;
        for i in 1..a{
            a_factorial *= i;
        }
        //return a * factorial(a - 1);
        return a_factorial
    }
}