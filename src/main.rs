fn main() {
    let str_a = "conserve";
    let str_b = "converse";

    let zippified = str_a.chars().zip(str_b.chars());
//    str_a.chars().collect()
    println!("{:?}", zippified);
}
