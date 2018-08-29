fn main() {
    let str_a = "conserve";
    let str_b = "converse";

    let zippified_a: Vec<_> = str_a.chars().collect();
    let zippified_b: Vec<_> = str_b.chars().collect();

    let zippified: Vec<_> = zippified_a.iter().
                            zip(
                                zippified_b.iter()
                                ).
                            filter(
                                |(a, b)| a != b
                            ).collect();

    
    // let zippified = str_a.chars().zip(str_b.chars().iter());
    // let mappified = zippified.take_while(
    //     |some_char_tuple| some_char_tuple.0 != some_char_tuple.1
    // ).collect();
//    str_a.chars().collect()
    println!("{:?}, {:?}", zippified_a, zippified_b);
    println!("{:?}", zippified);
}
