use regex::Regex;
use std::{collections::HashMap, error::Error, fs::File, io::Read};

const CONFIG_FILE_SAMPLE: &str = r#"
%{
    pub struct Foo {
        x: i32,
        y: i32,
    }
%}

%%
    {E : T + E} -> |datas| {
        println!("reduce: E -> T + E");
        let left = datas[0].parse::<i64>().unwrap();
        let right = datas[2].parse::<i64>().unwrap();
        (left + right).to_string()
    } ;;
    {E : T} -> |datas| {
        println!("reduce: E -> T");
        datas[0].clone()
    }

    {T : int * T} -> |datas| {
        println!("reduce: T -> int * T");
        let left = datas[0].parse::<i64>().unwrap();
        let right = datas[2].parse::<i64>().unwrap();
        (left * right).to_string()
    } ;;
    {T : int} -> |datas| {
        println!("reduce: T -> int");
        datas[0].clone()
    }
    {T : ( E )} -> |datas| {
        println!("reduce: T -> int");
        datas[0].clone()
    }
%%
    pub a: i32,
    pub b: i64, 
"#;

pub struct Rule {
    pub left: String,
    pub right: Vec<String>,
    pub handler: String,
}
pub struct Config {
    pub declarations: String,
    pub rules: Vec<Rule>,
    pub variables: String,
}

pub fn parse_config(path: &str) -> Result<Config, Box<dyn Error>> {
    // 读取文件
    let (declarations, definitions, rules, variables) = read_config_file(path)?;
    // 将definitions中的变量提取出来
    let definitions = parse_definations(&definitions)?;
    // 将rules中的变量提取出来
    let rules = parse_rules(&rules)?;

    Ok(Config {
        declarations,
        rules,
        variables,
    })
}

fn read_config_file(path: &str) -> Result<(String, String, String, String), Box<dyn Error>> {
    let mut f = File::open(path)?;

    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    // 读取各项配置
    if let Some(captures) = Regex::new(r"(?s)%\{(.+?)%\}(.*?)%%(.*?)%%(.+)")
        .unwrap()
        .captures(&buf)
    {
        let declarations = captures[1].to_string();
        let definitions = captures[2].to_string();
        let rules = captures[3].to_string();
        let variables = captures[4].to_string();

        Ok((declarations, definitions, rules, variables))
    } else {
        Err(format!(
            "parsing config error: config file format error\n*.flex file sample:\n{}\n",
            CONFIG_FILE_SAMPLE
        )
        .into())
    }
}

fn parse_definations(definitions: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut raw_definations: Vec<(String, String)> = Vec::new();

    // parse definitions to hash_map
    for captures in Regex::new(r#"([\s\S]*?)=([\s\S]*?)\n"#)
        .unwrap()
        .captures_iter(&definitions)
    {
        let key = captures[1].trim().to_string();
        let value = captures[2].trim().to_string();
        raw_definations.push((key, value));
    }

    // replace all variables
    // let old_difinations = res.clone();
    let mut res: HashMap<String, String> = HashMap::new();
    for (key, value) in &mut raw_definations {
        res.insert(key.clone(), replace_regex_variables(value, &res)?.clone());
    }

    Ok(res)
}

fn parse_rules(rules: &str) -> Result<Vec<Rule>, Box<dyn Error>> {
    let mut res: Vec<Rule> = Vec::new();
    for captures in Regex::new(r#"([\s\S]*?)->([\s\S]*?);;"#)
        .unwrap()
        .captures_iter(&rules)
    {
        let key = captures.get(1).unwrap().as_str().trim().to_string();
        // 除去了开头结尾的{}，剩下的就是变量
        let key = key.get(1..key.len() - 1).unwrap().to_string();

        // A : B C D
        // 其中 A为left，[B C D]为right
        // 根据：分割出left和right
        let mut key = key.split(":");
        let left = key.next().unwrap().trim().to_string();
        let right = key.next().unwrap().trim().to_string();

        // 根据空格分割出right中的每一项
        let right = right
            .split(" ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let handler = captures.get(2).unwrap().as_str().trim().to_string();
        res.push(Rule {
            left,
            right,
            handler,
        });
    }

    Ok(res)
}

fn replace_regex_variables(
    s: &str,
    definitions: &HashMap<String, String>,
) -> Result<String, Box<dyn Error>> {
    let mut res = s.to_string();
    let mut variables = Vec::new();

    // find all variables
    Regex::new(r"\{\w+\}")
        .unwrap()
        .captures_iter(&res)
        .for_each(|captures| {
            variables.push(captures[0].to_string());
        });

    // replace all variables
    for variable in variables {
        let variable_no_blanket = variable.replace("{", "").replace("}", "");
        if let Some(val) = definitions.get(&variable_no_blanket) {
            res = res.replace(&variable, format!("({})", val).as_str());
        } else {
            return Err(format!(
                "parsing config error: variable \"{}\" not defined",
                variable_no_blanket
            )
            .into());
        }
    }
    Ok(res)
}
