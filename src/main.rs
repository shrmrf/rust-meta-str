fn main() {
    let str_a = "conserve";
    let str_b = "converse";

    assert!(str_a.len() == str_b.len());

    let zippified: Vec<_> = str_a.chars().zip(str_b.chars())
                            .filter(|(a, b)| a != b)
                            .collect();

    assert_eq!(zippified.len(), 2);
    assert!(
        (zippified[0].0 == zippified[1].1) &&
        (zippified[0].1 == zippified[1].0)
    );

    println!("{:?}", zippified);
}
