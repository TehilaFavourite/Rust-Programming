fn main() {

    let number = 1_337;
    let number_16 : i16 = number as i16;
    let number_float: f32 = 1333.333333;
    // Print out the number with 3 digits of precision.
    println!("{:.3}", number_float);

    let with_milk = true;
    let with_sugar = false;
    let is_my_type_of_coffee = with_milk && with_sugar;
    let is_acceptable_coffee = with_milk || with_sugar;

    let array: [i8; 4] = [1, 2, 3, 4];
    dbg!(array);

    let another_array: [i8; 4] = [5, 6, 7, 8];
    println!("{:#?}", another_array);

    let tuple = (number, number_float, is_my_type_of_coffee, array, another_array);
    dbg!(tuple);

    let (a, b, c, d, e) = tuple;
    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
    println!("{:#?}", d);
    
    /*
Declare an i32 variable assigned to 1337.
Use the underscore character to add a visual
separator between the numbers.
 
Cast the i32 to an i16 integer and assign the result
to a separate variable.
 
Declare a floating-point value of your choosing.
Print out the number with 3 digits of precision.
 
Declare a 'with_milk' variable set to a Boolean.
Declare a 'with_sugar` variable set to a Boolean.
 
Declare a 'is_my_type_of_coffee` variable. It should
be set to true if the coffee has both milk and sugar.
 
Declare an `is_acceptable_coffee` variable. It should
be set to true if the coffee has either milk or
sugar.
 
Declare an array with four i8 integers of your choosing
Print out the array in its Debug representation.
 
Declare a tuple consisting of the integer, float,
a Boolean, and the array that you previously declared.
Print out the tuple in its Debug representation.
*/
}
