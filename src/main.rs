use rparser::{Derivation, Dfa, Grammer, GrammerBuilder};

enum Token {
    Number(i64),
    Op(char),
    EOF,
}

fn main() {
    let grammer = GrammerBuilder::new()
        .set_start("E")
        .add_derivation("E", Derivation::from(vec!["T".to_string()]))
        .add_derivation(
            "E",
            Derivation::from(vec!["T".to_string(), "+".to_string(), "E".to_string()]),
        )
        .add_derivation(
            "T",
            Derivation::from(vec!["(".to_string(), "E".to_string(), ")".to_string()]),
        )
        .add_derivation(
            "T",
            Derivation::from(vec!["int".to_string(), "*".to_string(), "T".to_string()]),
        )
        .add_derivation("T", Derivation::from(vec!["int".to_string()]))
        .build();

    let dfa = Dfa::from(&grammer);
    let res = rparser::mermaid::parse_dfa(&dfa, &grammer);
    println!("{}", res);
}
