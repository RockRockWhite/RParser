use rparser::{gen_code, DerivationBuilder, Dfa, GrammerBuilder};

fn main() {
    let config = rparser::parse_config("sample.rparser").unwrap();

    let mut grammer_builder = GrammerBuilder::new();

    // add the symbols
    let mut handlers = Vec::new();
    config.rules.iter().for_each(|rule| {
        // get the symbols
        let left_symbol = grammer_builder.get_symbol(&rule.left);
        let right_symbols = rule
            .right
            .iter()
            .map(|symbol| grammer_builder.get_symbol(symbol))
            .collect::<Vec<_>>();

        // build the derivation
        let mut derivation_builder = DerivationBuilder::new();
        right_symbols.iter().for_each(|symbol| {
            derivation_builder.add_symbol(symbol);
        });

        grammer_builder.add_derivation(&left_symbol, derivation_builder.build());
        handlers.push(rule.handler.clone());
    });

    // set state S as the start symbol
    let start_symbol = grammer_builder.get_symbol("S");
    grammer_builder.set_start(&start_symbol);

    let grammer = grammer_builder.build();

    let dfa = Dfa::from(&grammer);
    println!("{}", rparser::mermaid::parse_dfa(&dfa));

    let action_table = rparser::ActionTable::from_dfa(&dfa);

    // 生成代码
    let code = gen_code(
        &config.declarations,
        &config.variables,
        &action_table,
        &config.rules,
    );

    // 写入文件
    std::fs::write("sample.out", code);
    // print the action
}
