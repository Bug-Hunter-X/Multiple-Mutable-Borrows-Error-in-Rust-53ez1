fn main() {
    let mut x = 5;
    let y = &mut x; 
    let z = &mut x; // this is the error, multiple mutable borrows

    *y = 10;
    *z = 15; 
}