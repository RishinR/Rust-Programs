fn main() {
    let mut data = [1, 2, 3, 4, 5];
    
    let slice = &mut data[1..4]; // Mutable slice of the array
    
    slice[0] = 10;
    slice[1] = 20;
    slice[2] = 30;

    println!("{:?}", data); // Output: [1, 10, 20, 30, 5]
}
