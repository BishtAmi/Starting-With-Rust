use std::io;
fn main()
{
    println !("Enter Your weight(kg) :");
    let mut input = String::new ();
    // take weight as input(String by Default)
    io::stdin().read_line(&mut input).unwrap();
    // parse the string to required data type
    let weight : f32 = input.trim().parse().unwrap();
    let mars_weight : f32 = cal_weight_on_mars(weight);
    println !("Weight on Mars: {}kg", mars_weight);
}

// Function to calculate weight on mars
fn cal_weight_on_mars(weight : f32)->f32
{
    return (weight / 9.81) * 3.711;
}