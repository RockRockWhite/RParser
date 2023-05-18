use rparser::{DerivationBuilder, Dfa, GrammerBuilder};

enum Token {
    Number(i64),
    Op(char),
    EOF,
}

fn main() {
    let mut grammer_builder = GrammerBuilder::new();

    let e_symbol = grammer_builder.get_symbol("E");
    let t_symbol = grammer_builder.get_symbol("T");
    let plus_symbol = grammer_builder.get_symbol("+");
    let mul_symbol = grammer_builder.get_symbol("*");
    let lparen_symbol = grammer_builder.get_symbol("(");
    let rparen_symbol = grammer_builder.get_symbol(")");
    let int_symbol = grammer_builder.get_symbol("int");

    let grammer = grammer_builder
        .set_start(&e_symbol)
        .add_derivation(
            &e_symbol,
            DerivationBuilder::new().add_symbol(&t_symbol).build(),
        )
        .add_derivation(
            &e_symbol,
            DerivationBuilder::new()
                .add_symbol(&t_symbol)
                .add_symbol(&plus_symbol)
                .add_symbol(&e_symbol)
                .build(),
        )
        .add_derivation(
            &t_symbol,
            DerivationBuilder::new()
                .add_symbol(&lparen_symbol)
                .add_symbol(&e_symbol)
                .add_symbol(&rparen_symbol)
                .build(),
        )
        .add_derivation(
            &t_symbol,
            DerivationBuilder::new()
                .add_symbol(&int_symbol)
                .add_symbol(&mul_symbol)
                .add_symbol(&t_symbol)
                .build(),
        )
        .add_derivation(
            &t_symbol,
            DerivationBuilder::new().add_symbol(&int_symbol).build(),
        )
        .build();

    for each in grammer.symbols.values().into_iter() {
        // 输出每个符号的first集合
        println!(
            "{}: {:?}",
            each.borrow().name,
            each.borrow()
                .first_set
                .iter()
                .map(|x| x.borrow().name.clone())
                .collect::<Vec<_>>()
        );
    }

    println!("=====================");

    for each in grammer.symbols.values().into_iter() {
        // 输出每个符号的first集合
        println!(
            "{}: {:?}",
            each.borrow().name,
            each.borrow()
                .follow_set
                .iter()
                .map(|x| x.borrow().name.clone())
                .collect::<Vec<_>>()
        );
    }

    let dfa = Dfa::from(&grammer);
    let res = rparser::mermaid::parse_dfa(&dfa);
    println!("{}", res);
}
