fn main() {
    let str_a = "conserve";
    let str_b = "converse";

    assert!(str_a.len() == str_b.len());

    let meta_chars: Vec<_> = str_a.chars().zip(str_b.chars())
                            .filter(|(a, b)| a != b)
                            .collect();

    assert_eq!(meta_chars.len(), 2);
    assert!(
        (meta_chars[0].0 == meta_chars[1].1) &&
        (meta_chars[0].1 == meta_chars[1].0)
    );

    println!("{:?}", meta_chars);
}
