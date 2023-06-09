# RParser

## Get Start
Usage: rparser <config_file> <output_file>

```rparser
// sample.rparser
%{
    pub struct Foo {
        x: i32,
        y: i32,
    }
%}

%%
    {S : E} -> |datas| {
        println!("reduce: S -> E");
        datas[0].clone()
    } ;;

    {E : T + E} -> |datas| {
        println!("reduce: E -> T + E");
        let left = datas[0].parse::<i64>().unwrap();
        let right = datas[2].parse::<i64>().unwrap();
        (left + right).to_string()
    } ;;
    {E : T} -> |datas| {
        println!("reduce: E -> T");
        datas[0].clone()
    } ;;

    {T : int * T} -> |datas| {
        println!("reduce: T -> int * T");
        let left = datas[0].parse::<i64>().unwrap();
        let right = datas[2].parse::<i64>().unwrap();
        (left * right).to_string()
    } ;;
    {T : int} -> |datas| {
        println!("reduce: T -> int");
        datas[0].clone()
    } ;;
    {T : ( E )} -> |datas| {
        println!("reduce: T -> int");
        datas[0].clone()
    } ;;
%%
    pub a: i32,
    pub b: i64, 
```

output info:

>
> # Generated Info
>
> ## Base Info
> - config_file: sample.rparser
> - output_file: sample.out
> - time: 2023-05-19 15:41:03.419453 +08:00
>
> ---
>
> ## DFA Graph
> ```mermaid
> graph LR
> 0["T -> •int * T\nT -> •int\nE -> •T\nS -> •E\nT -> •( E )\nE -> •T + E\n__DUMMY_START__ -> •S\n"]
> 1["E -> T•\nE -> T •+ E\n"]
> 2["T -> •int\nE -> •T\nE -> T + •E\nE -> •T + E\nT -> •( E )\nT -> •int * T\n"]
> 3["E -> T + E•\n"]
> 4["T -> int•\nT -> int •* T\n"]
> 5["T -> •int\nT -> •int * T\nT -> •( E )\nT -> int * •T\n"]
> 6["T -> ( •E )\nT -> •int * T\nT -> •int\nE -> •T\nE -> •T + E\nT -> •( E )\n"]
> 7["T -> ( E •)\n"]
> 8["T -> ( E )•\n"]
> 9["T -> int * T•\n"]
> 10["__DUMMY_START__ -> S•\n"]
> 11["S -> E•\n"]
> 
> 2--"T"-->1
> 2--"E"-->3
> 6--"int"-->4
> 6--"T"-->1
> 6--"("-->6
> 7--")"-->8
> 6--"E"-->7
> 5--"("-->6
> 5--"int"-->4
> 5--"T"-->9
> 4--"*"-->5
> 2--"int"-->4
> 2--"("-->6
> 1--"+"-->2
> 0--"T"-->1
> 0--"S"-->10
> 0--"int"-->4
> 0--"E"-->11
> 0--"("-->6
> 
> ```
>
> ---
> ## Action Table
> ```txt
> State 0:
> T: Shift(1)
> E: Shift(11)
> (: Shift(6)
> __$__: Accept
> int: Shift(4)
> S: Shift(10)
> ===================
> State 1:
> +: Shift(2)
> __$__: Reduce(ReduceDerivation { left: "E", right: ["T"] })
> ): Reduce(ReduceDerivation { left: "E", right: ["T"] })
> ===================
> State 2:
> E: Shift(3)
> int: Shift(4)
> (: Shift(6)
> T: Shift(1)
> ===================
> State 3:
> ): Reduce(ReduceDerivation { left: "E", right: ["T", "+", "E"] })
> __$__: Reduce(ReduceDerivation { left: "E", right: ["T", "+", "E"] })
> ===================
> State 4:
> __$__: Reduce(ReduceDerivation { left: "T", right: ["int"] })
> ): Reduce(ReduceDerivation { left: "T", right: ["int"] })
> +: Reduce(ReduceDerivation { left: "T", right: ["int"] })
> *: Shift(5)
> ===================
> State 5:
> T: Shift(9)
> (: Shift(6)
> int: Shift(4)
> ===================
> State 6:
> int: Shift(4)
> T: Shift(1)
> E: Shift(7)
> (: Shift(6)
> ===================
> State 7:
> ): Shift(8)
> ===================
> State 8:
> ): Reduce(ReduceDerivation { left: "T", right: ["(", "E", ")"] })
> __$__: Reduce(ReduceDerivation { left: "T", right: ["(", "E", ")"] })
> +: Reduce(ReduceDerivation { left: "T", right: ["(", "E", ")"] })
> ===================
> State 9:
> __$__: Reduce(ReduceDerivation { left: "T", right: ["int", "*", "T"] })
> ): Reduce(ReduceDerivation { left: "T", right: ["int", "*", "T"] })
> +: Reduce(ReduceDerivation { left: "T", right: ["int", "*", "T"] })
> ===================
> State 10:
> __$__: Reduce(ReduceDerivation { left: "__DUMMY_START__", right: ["S"] })
> ===================
> State 11:
> __$__: Reduce(ReduceDerivation { left: "S", right: ["E"] })
> ===================
> 
> ```
> ---
> generated by rparser
> RockRockWhite 2023
