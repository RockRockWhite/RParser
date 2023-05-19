use rparser::{gen_code, ActionTable, Config, DerivationBuilder, Dfa, GrammerBuilder};
use std::{env, error::Error};

fn main() {
    let args = Args::build(env::args().collect()).unwrap_or_else(|err| {
        println!("{}", err);
        std::process::exit(1);
    });

    run(args).unwrap_or_else(|err| {
        println!("{}", err);
        std::process::exit(1);
    });

    println!(
        r#"Done.
Please add the following dependencies to your Cargo.toml:
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0""#
    );
}

fn run(args: Args) -> Result<(), Box<dyn Error>> {
    // 读取配置文件
    let config = Config::parse_config(&args.config_file)?;

    // 生成action_table
    let grammer = build_grammer(&config)?;
    let dfa = Dfa::from(&grammer);
    let action_table = ActionTable::from_dfa(&dfa);

    // 生成代码
    let code = gen_code(
        &config.declarations,
        &config.variables,
        &action_table,
        &config.rules,
    );

    // 写入文件
    std::fs::write(&args.output_file, code)?;

    // 执行format
    if let Err(err) = std::process::Command::new("rustfmt")
        .arg(&args.output_file)
        .output()
    {
        return Err(format!("rustfmt error : {}", err).into());
    }

    Ok(())
}

fn build_grammer(config: &rparser::Config) -> Result<rparser::Grammer, Box<dyn Error>> {
    let mut grammer_builder = GrammerBuilder::new();

    // add the symbols
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
    });

    // set state S as the start symbol
    let start_symbol = grammer_builder.get_symbol("S");
    grammer_builder.set_start(&start_symbol);
    let grammer = grammer_builder.build();

    Ok(grammer)
}

struct Args {
    config_file: String,
    output_file: String,
}

impl Args {
    fn build(args: Vec<String>) -> Result<Args, &'static str> {
        if args.len() != 3 {
            return Err("Usage: rparser <config_file> <output_file>");
        }

        Ok(Args {
            config_file: args[1].clone(),
            output_file: args[2].clone(),
        })
    }
}
