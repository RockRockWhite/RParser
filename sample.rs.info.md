
# Generated Info

## Base Info
- config_file: sample.rparser
- output_file: sample.rs
- time: 2023-05-20 11:12:57.384800 +08:00

---

## DFA Graph
```mermaid
graph LR
0["S -> •program\n__DUMMY_START__ -> •S\nblock -> •left_curly_brace decls stmts right_curly_brace\nprogram -> •block\n"]
1["S -> program•\n"]
2["program -> block•\n"]
3["decls -> •\nblock -> left_curly_brace •decls stmts right_curly_brace\ndecls -> •decls decl\n"]
4["decl -> •type id ;\nblock -> left_curly_brace decls •stmts right_curly_brace\nstmts -> •stmts stmt\ndecls -> decls •decl\ntype -> •basic\nstmts -> •\ntype -> •type [ num ]\n"]
5["type -> type •[ num ]\ndecl -> type •id ;\n"]
6["decl -> type id •;\n"]
7["decl -> type id ;•\n"]
8["type -> type [ •num ]\n"]
9["type -> type [ num •]\n"]
10["type -> type [ num ]•\n"]
11["decls -> decls decl•\n"]
12["type -> basic•\n"]
13["stmt -> •block\nstmt -> •do stmt while ( bool ) ;\nblock -> •left_curly_brace decls stmts right_curly_brace\nstmt -> •break ;\nstmt -> •while ( bool ) stmt\nstmt -> •if ( bool ) stmt else stmt\nblock -> left_curly_brace decls stmts •right_curly_brace\nstmt -> •loc = bool ;\nstmts -> stmts •stmt\nloc -> •id\nloc -> •loc [ num ]\nstmt -> •if ( bool ) stmt\n"]
14["stmts -> stmts stmt•\n"]
15["block -> left_curly_brace decls stmts right_curly_brace•\n"]
16["loc -> id•\n"]
17["stmt -> while •( bool ) stmt\n"]
18["rel -> •expr > expr\nfactor -> •real\nrel -> •expr\nloc -> •loc [ num ]\nfactor -> •( bool )\nequality -> •equality != rel\nunary -> •- unary\nfactor -> •false\nrel -> •expr >= expr\nrel -> •expr < expr\nstmt -> while ( •bool ) stmt\nequality -> •equality == rel\nfactor -> •loc\nequality -> •rel\nunary -> •! unary\nfactor -> •num\nbool -> •bool || join\nloc -> •id\nunary -> •factor\njoin -> •equality\nexpr -> •expr - term\nbool -> •join\nexpr -> •term\nexpr -> •expr + term\nterm -> •unary\nfactor -> •true\nterm -> •term / unary\njoin -> •join && equality\nrel -> •expr <= expr\nterm -> •term * unary\n"]
19["factor -> real•\n"]
20["equality -> equality •!= rel\nequality -> equality •== rel\njoin -> equality•\n"]
21["rel -> •expr < expr\nterm -> •term / unary\nexpr -> •expr + term\nfactor -> •loc\nequality -> equality != •rel\nfactor -> •true\nunary -> •factor\nexpr -> •expr - term\nterm -> •unary\nfactor -> •false\nfactor -> •real\nterm -> •term * unary\nloc -> •loc [ num ]\nloc -> •id\nexpr -> •term\nrel -> •expr > expr\nunary -> •- unary\nfactor -> •( bool )\nfactor -> •num\nrel -> •expr >= expr\nunary -> •! unary\nrel -> •expr <= expr\nrel -> •expr\n"]
22["unary -> ! •unary\nfactor -> •true\nfactor -> •real\nfactor -> •( bool )\nfactor -> •loc\nloc -> •id\nfactor -> •num\nunary -> •! unary\nunary -> •- unary\nloc -> •loc [ num ]\nfactor -> •false\nunary -> •factor\n"]
23["loc -> loc •[ num ]\nfactor -> loc•\n"]
24["loc -> loc [ •num ]\n"]
25["loc -> loc [ num •]\n"]
26["loc -> loc [ num ]•\n"]
27["factor -> num•\n"]
28["factor -> false•\n"]
29["loc -> •id\nfactor -> •true\nfactor -> ( •bool )\nfactor -> •num\nunary -> •factor\nequality -> •equality == rel\nbool -> •join\nexpr -> •expr + term\nloc -> •loc [ num ]\nfactor -> •( bool )\nexpr -> •expr - term\nrel -> •expr\nfactor -> •loc\nexpr -> •term\nterm -> •term * unary\njoin -> •equality\nfactor -> •real\nequality -> •equality != rel\nunary -> •- unary\njoin -> •join && equality\nfactor -> •false\nrel -> •expr >= expr\nterm -> •term / unary\nrel -> •expr <= expr\nequality -> •rel\nrel -> •expr > expr\nunary -> •! unary\nbool -> •bool || join\nrel -> •expr < expr\nterm -> •unary\n"]
30["equality -> rel•\n"]
31["rel -> expr•\nexpr -> expr •+ term\nrel -> expr •<= expr\nexpr -> expr •- term\nrel -> expr •>= expr\nrel -> expr •> expr\nrel -> expr •< expr\n"]
32["term -> •unary\nunary -> •factor\nloc -> •loc [ num ]\nloc -> •id\nfactor -> •real\nterm -> •term / unary\nunary -> •! unary\nterm -> •term * unary\nfactor -> •( bool )\nfactor -> •true\nunary -> •- unary\nfactor -> •false\nexpr -> expr - •term\nfactor -> •loc\nfactor -> •num\n"]
33["term -> unary•\n"]
34["loc -> •loc [ num ]\nunary -> •factor\nfactor -> •num\nfactor -> •( bool )\nloc -> •id\nunary -> •- unary\nunary -> •! unary\nfactor -> •true\nfactor -> •loc\nfactor -> •real\nfactor -> •false\nunary -> - •unary\n"]
35["unary -> - unary•\n"]
36["factor -> true•\n"]
37["unary -> factor•\n"]
38["term -> term •/ unary\nterm -> term •* unary\nexpr -> expr - term•\n"]
39["unary -> •factor\nfactor -> •num\nfactor -> •false\nfactor -> •true\nfactor -> •loc\nfactor -> •( bool )\nloc -> •loc [ num ]\nloc -> •id\nunary -> •! unary\nterm -> term * •unary\nfactor -> •real\nunary -> •- unary\n"]
40["term -> term * unary•\n"]
41["factor -> •num\nunary -> •! unary\nfactor -> •false\nfactor -> •real\nfactor -> •true\nfactor -> •loc\nunary -> •- unary\nunary -> •factor\nterm -> term / •unary\nloc -> •loc [ num ]\nloc -> •id\nfactor -> •( bool )\n"]
42["term -> term / unary•\n"]
43["term -> •term * unary\nfactor -> •true\nrel -> expr >= •expr\nterm -> •unary\nfactor -> •loc\nunary -> •factor\nloc -> •id\nfactor -> •( bool )\nexpr -> •term\nunary -> •- unary\nfactor -> •false\nloc -> •loc [ num ]\nunary -> •! unary\nterm -> •term / unary\nfactor -> •real\nfactor -> •num\nexpr -> •expr - term\nexpr -> •expr + term\n"]
44["rel -> expr >= expr•\nexpr -> expr •- term\nexpr -> expr •+ term\n"]
45["factor -> •num\nloc -> •loc [ num ]\nloc -> •id\nfactor -> •real\nfactor -> •( bool )\nterm -> •term / unary\nfactor -> •true\nunary -> •factor\nfactor -> •loc\nexpr -> expr + •term\nunary -> •! unary\nterm -> •unary\nunary -> •- unary\nterm -> •term * unary\nfactor -> •false\n"]
46["expr -> expr + term•\nterm -> term •* unary\nterm -> term •/ unary\n"]
47["term -> term •/ unary\nterm -> term •* unary\nexpr -> term•\n"]
48["loc -> •id\nterm -> •term / unary\nexpr -> •term\nterm -> •unary\nfactor -> •real\nterm -> •term * unary\nunary -> •! unary\nexpr -> •expr + term\nfactor -> •true\nloc -> •loc [ num ]\nrel -> expr > •expr\nfactor -> •( bool )\nexpr -> •expr - term\nfactor -> •loc\nfactor -> •num\nfactor -> •false\nunary -> •- unary\nunary -> •factor\n"]
49["expr -> expr •- term\nexpr -> expr •+ term\nrel -> expr > expr•\n"]
50["term -> •term * unary\nexpr -> •expr + term\nfactor -> •real\nfactor -> •num\nfactor -> •true\nfactor -> •( bool )\nloc -> •loc [ num ]\nunary -> •! unary\nexpr -> •expr - term\nfactor -> •loc\nunary -> •factor\nrel -> expr < •expr\nloc -> •id\nterm -> •unary\nunary -> •- unary\nexpr -> •term\nfactor -> •false\nterm -> •term / unary\n"]
51["expr -> expr •- term\nrel -> expr < expr•\nexpr -> expr •+ term\n"]
52["unary -> •- unary\nfactor -> •real\nexpr -> •term\nexpr -> •expr + term\nfactor -> •true\nunary -> •factor\nfactor -> •( bool )\nterm -> •unary\nterm -> •term * unary\nterm -> •term / unary\nfactor -> •false\nrel -> expr <= •expr\nloc -> •loc [ num ]\nunary -> •! unary\nfactor -> •num\nfactor -> •loc\nexpr -> •expr - term\nloc -> •id\n"]
53["rel -> expr <= expr•\nexpr -> expr •+ term\nexpr -> expr •- term\n"]
54["factor -> ( bool •)\nbool -> bool •|| join\n"]
55["factor -> ( bool )•\n"]
56["unary -> •factor\nloc -> •id\nexpr -> •term\nfactor -> •num\nrel -> •expr < expr\nequality -> •rel\nfactor -> •false\nrel -> •expr >= expr\nbool -> bool || •join\nrel -> •expr\nunary -> •! unary\nloc -> •loc [ num ]\nexpr -> •expr - term\nfactor -> •true\nterm -> •term / unary\nterm -> •unary\nequality -> •equality == rel\njoin -> •join && equality\nequality -> •equality != rel\nfactor -> •real\nrel -> •expr <= expr\nunary -> •- unary\nterm -> •term * unary\nfactor -> •loc\nfactor -> •( bool )\nexpr -> •expr + term\nrel -> •expr > expr\njoin -> •equality\n"]
57["join -> join •&& equality\nbool -> bool || join•\n"]
58["rel -> •expr > expr\nexpr -> •expr - term\nfactor -> •( bool )\nrel -> •expr <= expr\nequality -> •equality == rel\nrel -> •expr\nterm -> •unary\nrel -> •expr >= expr\nexpr -> •expr + term\njoin -> join && •equality\nfactor -> •num\nfactor -> •real\nexpr -> •term\nequality -> •equality != rel\nloc -> •id\nfactor -> •false\nfactor -> •loc\nunary -> •factor\nrel -> •expr < expr\nterm -> •term / unary\nequality -> •rel\nunary -> •- unary\nunary -> •! unary\nfactor -> •true\nloc -> •loc [ num ]\nterm -> •term * unary\n"]
59["equality -> equality •!= rel\njoin -> join && equality•\nequality -> equality •== rel\n"]
60["equality -> equality == •rel\nfactor -> •real\nrel -> •expr >= expr\nterm -> •term * unary\nexpr -> •expr + term\nfactor -> •true\nrel -> •expr\nloc -> •loc [ num ]\nunary -> •! unary\nfactor -> •false\nloc -> •id\nfactor -> •num\nrel -> •expr <= expr\nrel -> •expr > expr\nfactor -> •loc\nexpr -> •term\nexpr -> •expr - term\nterm -> •term / unary\nfactor -> •( bool )\nunary -> •- unary\nunary -> •factor\nrel -> •expr < expr\nterm -> •unary\n"]
61["equality -> equality == rel•\n"]
62["bool -> join•\njoin -> join •&& equality\n"]
63["unary -> ! unary•\n"]
64["equality -> equality != rel•\n"]
65["bool -> bool •|| join\nstmt -> while ( bool •) stmt\n"]
66["block -> •left_curly_brace decls stmts right_curly_brace\nstmt -> •do stmt while ( bool ) ;\nstmt -> •loc = bool ;\nstmt -> •if ( bool ) stmt\nstmt -> •break ;\nstmt -> while ( bool ) •stmt\nloc -> •loc [ num ]\nloc -> •id\nstmt -> •if ( bool ) stmt else stmt\nstmt -> •while ( bool ) stmt\nstmt -> •block\n"]
67["stmt -> loc •= bool ;\nloc -> loc •[ num ]\n"]
68["factor -> •real\nrel -> •expr >= expr\nrel -> •expr < expr\nrel -> •expr\nterm -> •unary\nfactor -> •false\nequality -> •equality == rel\nfactor -> •loc\nloc -> •loc [ num ]\nfactor -> •num\nunary -> •- unary\nexpr -> •expr - term\nequality -> •rel\nexpr -> •expr + term\nequality -> •equality != rel\njoin -> •equality\nunary -> •! unary\nfactor -> •true\nexpr -> •term\nbool -> •bool || join\nunary -> •factor\nbool -> •join\nstmt -> loc = •bool ;\nterm -> •term / unary\nfactor -> •( bool )\nrel -> •expr > expr\nterm -> •term * unary\nrel -> •expr <= expr\nloc -> •id\njoin -> •join && equality\n"]
69["stmt -> loc = bool •;\nbool -> bool •|| join\n"]
70["stmt -> loc = bool ;•\n"]
71["stmt -> block•\n"]
72["stmt -> if •( bool ) stmt\nstmt -> if •( bool ) stmt else stmt\n"]
73["loc -> •loc [ num ]\nstmt -> if ( •bool ) stmt\nfactor -> •real\nrel -> •expr <= expr\nloc -> •id\nrel -> •expr > expr\nfactor -> •num\nequality -> •equality != rel\njoin -> •join && equality\nfactor -> •true\nterm -> •term / unary\nunary -> •- unary\nrel -> •expr\nfactor -> •( bool )\nbool -> •bool || join\nfactor -> •loc\nunary -> •! unary\nrel -> •expr >= expr\nequality -> •rel\nfactor -> •false\nunary -> •factor\nterm -> •unary\nstmt -> if ( •bool ) stmt else stmt\nexpr -> •expr + term\njoin -> •equality\nexpr -> •expr - term\nterm -> •term * unary\nbool -> •join\nexpr -> •term\nrel -> •expr < expr\nequality -> •equality == rel\n"]
74["stmt -> if ( bool •) stmt else stmt\nbool -> bool •|| join\nstmt -> if ( bool •) stmt\n"]
75["loc -> •id\nstmt -> if ( bool ) •stmt else stmt\nstmt -> •block\nblock -> •left_curly_brace decls stmts right_curly_brace\nstmt -> •do stmt while ( bool ) ;\nstmt -> •while ( bool ) stmt\nloc -> •loc [ num ]\nstmt -> •if ( bool ) stmt\nstmt -> •if ( bool ) stmt else stmt\nstmt -> if ( bool ) •stmt\nstmt -> •break ;\nstmt -> •loc = bool ;\n"]
76["stmt -> break •;\n"]
77["stmt -> break ;•\n"]
78["stmt -> •if ( bool ) stmt else stmt\nloc -> •loc [ num ]\nloc -> •id\nstmt -> •while ( bool ) stmt\nstmt -> •do stmt while ( bool ) ;\nstmt -> •block\nblock -> •left_curly_brace decls stmts right_curly_brace\nstmt -> •loc = bool ;\nstmt -> do •stmt while ( bool ) ;\nstmt -> •break ;\nstmt -> •if ( bool ) stmt\n"]
79["stmt -> do stmt •while ( bool ) ;\n"]
80["stmt -> do stmt while •( bool ) ;\n"]
81["factor -> •( bool )\nbool -> •join\nrel -> •expr >= expr\nequality -> •rel\nterm -> •term * unary\nunary -> •! unary\nequality -> •equality == rel\nstmt -> do stmt while ( •bool ) ;\nbool -> •bool || join\nrel -> •expr > expr\nrel -> •expr\nfactor -> •loc\nfactor -> •real\nloc -> •loc [ num ]\nterm -> •term / unary\nexpr -> •term\nfactor -> •false\nrel -> •expr <= expr\nexpr -> •expr - term\nunary -> •- unary\nterm -> •unary\nunary -> •factor\nfactor -> •true\nloc -> •id\nexpr -> •expr + term\njoin -> •join && equality\njoin -> •equality\nequality -> •equality != rel\nrel -> •expr < expr\nfactor -> •num\n"]
82["stmt -> do stmt while ( bool •) ;\nbool -> bool •|| join\n"]
83["stmt -> do stmt while ( bool ) •;\n"]
84["stmt -> do stmt while ( bool ) ;•\n"]
85["stmt -> if ( bool ) stmt•\nstmt -> if ( bool ) stmt •else stmt\n"]
86["loc -> •id\nstmt -> •loc = bool ;\nstmt -> •block\nblock -> •left_curly_brace decls stmts right_curly_brace\nstmt -> •break ;\nstmt -> •if ( bool ) stmt\nstmt -> •if ( bool ) stmt else stmt\nstmt -> •while ( bool ) stmt\nstmt -> •do stmt while ( bool ) ;\nstmt -> if ( bool ) stmt else •stmt\nloc -> •loc [ num ]\n"]
87["stmt -> if ( bool ) stmt else stmt•\n"]
88["stmt -> while ( bool ) stmt•\n"]
89["__DUMMY_START__ -> S•\n"]

0--"program"-->1
0--"block"-->2
6--";"-->7
5--"id"-->6
9--"]"-->10
8--"num"-->9
5--"["-->8
4--"type"-->5
4--"decl"-->11
4--"basic"-->12
13--"stmt"-->14
13--"left_curly_brace"-->3
13--"right_curly_brace"-->15
13--"id"-->16
18--"real"-->19
25--"]"-->26
24--"num"-->25
23--"["-->24
22--"loc"-->23
22--"id"-->16
22--"num"-->27
22--"false"-->28
22--"!"-->22
22--"real"-->19
29--"rel"-->30
29--"id"-->16
29--"num"-->27
29--"false"-->28
32--"real"-->19
32--"!"-->22
32--"unary"-->33
34--"id"-->16
34--"loc"-->23
34--"!"-->22
34--"real"-->19
34--"false"-->28
34--"("-->29
34--"unary"-->35
34--"true"-->36
34--"factor"-->37
34--"-"-->34
34--"num"-->27
32--"-"-->34
39--"loc"-->23
39--"!"-->22
39--"true"-->36
39--"unary"-->40
39--"-"-->34
39--"real"-->19
39--"false"-->28
39--"num"-->27
39--"factor"-->37
39--"("-->29
39--"id"-->16
38--"*"-->39
41--"factor"-->37
41--"unary"-->42
41--"("-->29
41--"-"-->34
41--"true"-->36
41--"false"-->28
41--"id"-->16
41--"loc"-->23
41--"num"-->27
41--"real"-->19
41--"!"-->22
38--"/"-->41
32--"term"-->38
32--"true"-->36
32--"factor"-->37
32--"false"-->28
32--"num"-->27
32--"loc"-->23
32--"id"-->16
32--"("-->29
31--"-"-->32
43--"false"-->28
43--"unary"-->33
43--"real"-->19
43--"num"-->27
43--"!"-->22
43--"true"-->36
43--"("-->29
43--"-"-->34
44--"-"-->32
45--"factor"-->37
45--"id"-->16
45--"real"-->19
45--"-"-->34
45--"false"-->28
45--"num"-->27
45--"true"-->36
46--"*"-->39
46--"/"-->41
45--"term"-->46
45--"!"-->22
45--"unary"-->33
45--"("-->29
45--"loc"-->23
44--"+"-->45
43--"expr"-->44
47--"*"-->39
47--"/"-->41
43--"term"-->47
43--"loc"-->23
43--"id"-->16
43--"factor"-->37
31--">="-->43
31--"+"-->45
48--"loc"-->23
48--"("-->29
48--"false"-->28
48--"-"-->34
48--"factor"-->37
48--"term"-->47
48--"real"-->19
48--"true"-->36
49--"+"-->45
49--"-"-->32
48--"expr"-->49
48--"unary"-->33
48--"num"-->27
48--"!"-->22
48--"id"-->16
31--">"-->48
50--"real"-->19
50--"num"-->27
50--"("-->29
50--"!"-->22
50--"id"-->16
50--"unary"-->33
50--"factor"-->37
50--"true"-->36
50--"false"-->28
50--"-"-->34
50--"term"-->47
50--"loc"-->23
51--"+"-->45
51--"-"-->32
50--"expr"-->51
31--"<"-->50
52--"real"-->19
52--"factor"-->37
52--"false"-->28
52--"("-->29
52--"true"-->36
53--"-"-->32
53--"+"-->45
52--"expr"-->53
52--"-"-->34
52--"term"-->47
52--"unary"-->33
52--"num"-->27
52--"loc"-->23
52--"id"-->16
52--"!"-->22
31--"<="-->52
29--"expr"-->31
29--"factor"-->37
29--"true"-->36
29--"loc"-->23
29--"term"-->47
29--"unary"-->33
54--")"-->55
56--"id"-->16
56--"("-->29
56--"unary"-->33
56--"rel"-->30
56--"real"-->19
56--"!"-->22
56--"equality"-->20
56--"term"-->47
56--"num"-->27
56--"true"-->36
56--"factor"-->37
56--"false"-->28
56--"-"-->34
56--"loc"-->23
58--"term"-->47
60--"real"-->19
60--"false"-->28
60--"num"-->27
60--"true"-->36
60--"factor"-->37
60--"loc"-->23
60--"("-->29
60--"-"-->34
60--"expr"-->31
60--"id"-->16
60--"unary"-->33
60--"term"-->47
60--"!"-->22
60--"rel"-->61
59--"=="-->60
59--"!="-->21
58--"equality"-->59
58--"!"-->22
58--"("-->29
58--"id"-->16
58--"rel"-->30
58--"-"-->34
58--"unary"-->33
58--"loc"-->23
58--"factor"-->37
58--"true"-->36
58--"real"-->19
58--"num"-->27
58--"expr"-->31
58--"false"-->28
57--"&&"-->58
56--"join"-->57
56--"expr"-->31
54--"||"-->56
29--"bool"-->54
29--"equality"-->20
29--"real"-->19
62--"&&"-->58
29--"join"-->62
29--"("-->29
29--"-"-->34
29--"!"-->22
22--"("-->29
22--"true"-->36
22--"factor"-->37
22--"-"-->34
22--"unary"-->63
21--"!"-->22
21--"rel"-->64
21--"true"-->36
21--"unary"-->33
21--"expr"-->31
21--"false"-->28
21--"real"-->19
21--"loc"-->23
21--"("-->29
21--"term"-->47
21--"num"-->27
21--"factor"-->37
21--"id"-->16
21--"-"-->34
20--"!="-->21
20--"=="-->60
18--"equality"-->20
18--"!"-->22
18--"-"-->34
18--"false"-->28
18--"rel"-->30
18--"term"-->47
18--"expr"-->31
18--"unary"-->33
18--"true"-->36
65--"||"-->56
68--"join"-->62
68--"num"-->27
68--"expr"-->31
68--"loc"-->23
68--"rel"-->30
68--"!"-->22
68--"real"-->19
68--"factor"-->37
68--"true"-->36
69--"||"-->56
69--";"-->70
68--"bool"-->69
68--"id"-->16
68--"("-->29
68--"false"-->28
68--"equality"-->20
68--"term"-->47
68--"-"-->34
68--"unary"-->33
67--"="-->68
67--"["-->24
66--"loc"-->67
66--"while"-->17
66--"block"-->71
66--"id"-->16
73--"!"-->22
75--"id"-->16
75--"while"-->17
76--";"-->77
75--"break"-->76
75--"loc"-->67
78--"break"-->76
78--"do"-->78
78--"block"-->71
78--"loc"-->67
78--"while"-->17
78--"left_curly_brace"-->3
78--"if"-->72
81--"loc"-->23
83--";"-->84
82--")"-->83
82--"||"-->56
81--"bool"-->82
81--"false"-->28
81--"expr"-->31
81--"rel"-->30
81--"factor"-->37
81--"term"-->47
81--"equality"-->20
81--"-"-->34
81--"join"-->62
81--"!"-->22
81--"true"-->36
81--"real"-->19
81--"id"-->16
81--"unary"-->33
81--"num"-->27
81--"("-->29
80--"("-->81
79--"while"-->80
78--"stmt"-->79
78--"id"-->16
75--"do"-->78
75--"block"-->71
75--"left_curly_brace"-->3
86--"stmt"-->87
86--"left_curly_brace"-->3
86--"do"-->78
86--"while"-->17
86--"loc"-->67
86--"break"-->76
86--"id"-->16
86--"if"-->72
86--"block"-->71
85--"else"-->86
75--"stmt"-->85
75--"if"-->72
74--")"-->75
74--"||"-->56
73--"bool"-->74
73--"rel"-->30
73--"num"-->27
73--"expr"-->31
73--"equality"-->20
73--"-"-->34
73--"term"-->47
73--"factor"-->37
73--"unary"-->33
73--"true"-->36
73--"false"-->28
73--"id"-->16
73--"join"-->62
73--"loc"-->23
73--"real"-->19
73--"("-->29
72--"("-->73
66--"if"-->72
66--"do"-->78
66--"break"-->76
66--"left_curly_brace"-->3
66--"stmt"-->88
65--")"-->66
18--"bool"-->65
18--"num"-->27
18--"join"-->62
18--"("-->29
18--"id"-->16
18--"factor"-->37
18--"loc"-->23
17--"("-->18
13--"while"-->17
13--"loc"-->67
13--"block"-->71
13--"do"-->78
13--"break"-->76
13--"if"-->72
4--"stmts"-->13
3--"decls"-->4
0--"left_curly_brace"-->3
0--"S"-->89

```

---

## Follow Set
```txt
!=: ["num", "-", "false", "(", "id", "real", "!", "true"]
*: ["!", "id", "(", "-", "true", "real", "false", "num"]
if: ["("]
&&: ["-", "true", "num", "false", "id", "(", "!", "real"]
true: ["<", "+", "*", "!=", "/", ">=", "&&", ";", "-", ">", "<=", ")", "==", "||"]
=: ["real", "false", "-", "true", "!", "id", "num", "("]
__EPSILON__: ["basic", "if", "while", "right_curly_brace", "id", "left_curly_brace", "do", "break"]
==: ["(", "-", "!", "num", "false", "real", "id", "true"]
stmts: ["do", "break", "left_curly_brace", "while", "right_curly_brace", "id", "if"]
break: [";"]
<: ["false", "true", "real", "id", "(", "num", "-", "!"]
/: ["id", "real", "num", "-", "(", "false", "true", "!"]
num: ["!=", ")", ";", ">=", "+", "==", "<", ">", "]", "-", "&&", "<=", "||", "/", "*"]
block: ["do", "id", "else", "while", "left_curly_brace", "break", "if", "__$__", "right_curly_brace"]
right_curly_brace: ["while", "__$__", "right_curly_brace", "left_curly_brace", "break", "do", "id", "else", "if"]
bool: [")", ";", "||"]
): ["break", "&&", "||", "+", ";", "-", "/", "<", "<=", ">=", "==", "while", "*", "do", ">", "left_curly_brace", "!=", "if", ")", "id"]
>: ["!", "real", "id", "num", "false", "true", "-", "("]
factor: ["+", "||", "==", ")", "-", "&&", "/", "<", "*", ">", "<=", ">=", "!=", ";"]
decl: ["id", "do", "while", "if", "break", "basic", "left_curly_brace"]
do: ["if", "do", "break", "left_curly_brace", "while", "id"]
id: ["[", ">", "*", "/", "&&", "=", ">=", "+", "-", ";", "!=", "<", ")", "<=", "||", "=="]
(: ["!", "real", "num", "-", "false", "true", "id", "("]
expr: ["<", "-", "||", "<=", "&&", ")", "+", ">=", "!=", "==", ";", ">"]
__$__: []
unary: [")", "!=", ";", "<=", "*", "&&", ">=", ">", "<", "+", "/", "||", "==", "-"]
;: ["left_curly_brace", "else", "right_curly_brace", "if", "break", "do", "while", "id", "basic"]
S: ["__$__"]
loc: ["=", "[", ")", ";", ">=", "||", "!=", ">", "<", "&&", "==", "*", "<=", "-", "/", "+"]
||: ["false", "num", "-", "(", "true", "real", "id", "!"]
join: [";", ")", "&&", "||"]
]: ["!=", ";", "-", "<=", "&&", "||", "=", "id", "*", "<", "/", ")", "==", ">=", ">", "[", "+"]
equality: ["!=", "==", ")", "||", ";", "&&"]
-: ["real", "id", "-", "true", "num", "false", "!", "("]
else: ["break", "if", "id", "do", "left_curly_brace", "while"]
basic: ["id", "["]
real: [";", "||", "*", ")", "!=", ">=", "&&", ">", "/", "-", "==", "<", "<=", "+"]
false: [">", ")", ";", "+", ">=", "||", "!=", "/", "<", "==", "&&", "<=", "*", "-"]
term: ["/", "<=", ">=", "==", "+", "-", ")", ";", "||", "!=", "*", "&&", ">", "<"]
program: ["__$__"]
type: ["[", "id"]
[: ["num"]
+: ["false", "(", "!", "id", "-", "real", "true", "num"]
while: ["("]
left_curly_brace: ["basic"]
!: ["true", "real", "id", "false", "(", "num", "-", "!"]
>=: ["-", "id", "!", "(", "num", "false", "true", "real"]
rel: ["||", ";", "!=", ")", "&&", "=="]
<=: ["-", "(", "id", "false", "true", "!", "num", "real"]
stmt: ["left_curly_brace", "id", "if", "else", "break", "while", "do", "right_curly_brace"]
__DUMMY_START__: ["__$__"]
decls: ["id", "do", "while", "if", "left_curly_brace", "break", "basic"]
```

---
## Action Table
```txt
State 0:
__$__: Accept
program: Shift(1)
block: Shift(2)
left_curly_brace: Shift(3)
S: Shift(89)
===================
State 1:
__$__: Reduce(ReduceDerivation { left: "S", right: ["program"] })
===================
State 2:
__$__: Reduce(ReduceDerivation { left: "program", right: ["block"] })
===================
State 3:
left_curly_brace: Reduce(ReduceDerivation { left: "decls", right: [] })
while: Reduce(ReduceDerivation { left: "decls", right: [] })
basic: Reduce(ReduceDerivation { left: "decls", right: [] })
do: Reduce(ReduceDerivation { left: "decls", right: [] })
id: Reduce(ReduceDerivation { left: "decls", right: [] })
if: Reduce(ReduceDerivation { left: "decls", right: [] })
break: Reduce(ReduceDerivation { left: "decls", right: [] })
decls: Shift(4)
===================
State 4:
right_curly_brace: Reduce(ReduceDerivation { left: "stmts", right: [] })
do: Reduce(ReduceDerivation { left: "stmts", right: [] })
basic: Shift(12)
left_curly_brace: Reduce(ReduceDerivation { left: "stmts", right: [] })
if: Reduce(ReduceDerivation { left: "stmts", right: [] })
type: Shift(5)
while: Reduce(ReduceDerivation { left: "stmts", right: [] })
decl: Shift(11)
break: Reduce(ReduceDerivation { left: "stmts", right: [] })
id: Reduce(ReduceDerivation { left: "stmts", right: [] })
stmts: Shift(13)
===================
State 5:
id: Shift(6)
[: Shift(8)
===================
State 6:
;: Shift(7)
===================
State 7:
left_curly_brace: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
id: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
basic: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
while: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
do: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
if: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
break: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
===================
State 8:
num: Shift(9)
===================
State 9:
]: Shift(10)
===================
State 10:
[: Reduce(ReduceDerivation { left: "type", right: ["type", "[", "num", "]"] })
id: Reduce(ReduceDerivation { left: "type", right: ["type", "[", "num", "]"] })
===================
State 11:
do: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
left_curly_brace: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
break: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
if: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
basic: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
while: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
id: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
===================
State 12:
id: Reduce(ReduceDerivation { left: "type", right: ["basic"] })
[: Reduce(ReduceDerivation { left: "type", right: ["basic"] })
===================
State 13:
while: Shift(17)
right_curly_brace: Shift(15)
do: Shift(78)
id: Shift(16)
left_curly_brace: Shift(3)
block: Shift(71)
stmt: Shift(14)
loc: Shift(67)
break: Shift(76)
if: Shift(72)
===================
State 14:
do: Reduce(ReduceDerivation { left: "stmts", right: ["stmts", "stmt"] })
break: Reduce(ReduceDerivation { left: "stmts", right: ["stmts", "stmt"] })
while: Reduce(ReduceDerivation { left: "stmts", right: ["stmts", "stmt"] })
right_curly_brace: Reduce(ReduceDerivation { left: "stmts", right: ["stmts", "stmt"] })
id: Reduce(ReduceDerivation { left: "stmts", right: ["stmts", "stmt"] })
if: Reduce(ReduceDerivation { left: "stmts", right: ["stmts", "stmt"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmts", right: ["stmts", "stmt"] })
===================
State 15:
right_curly_brace: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
while: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
else: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
do: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
__$__: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
id: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
break: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
if: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
left_curly_brace: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
===================
State 16:
-: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
/: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
;: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
||: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
<=: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
=: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
&&: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
<: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
>=: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
>: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
!=: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
[: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
+: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
==: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
): Reduce(ReduceDerivation { left: "loc", right: ["id"] })
*: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
===================
State 17:
(: Shift(18)
===================
State 18:
unary: Shift(33)
factor: Shift(37)
real: Shift(19)
bool: Shift(65)
join: Shift(62)
loc: Shift(23)
rel: Shift(30)
id: Shift(16)
true: Shift(36)
term: Shift(47)
(: Shift(29)
expr: Shift(31)
!: Shift(22)
equality: Shift(20)
-: Shift(34)
false: Shift(28)
num: Shift(27)
===================
State 19:
-: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
+: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
&&: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
*: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
>: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
): Reduce(ReduceDerivation { left: "factor", right: ["real"] })
<=: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
>=: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
!=: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
;: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
<: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
/: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
||: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
==: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
===================
State 20:
||: Reduce(ReduceDerivation { left: "join", right: ["equality"] })
&&: Reduce(ReduceDerivation { left: "join", right: ["equality"] })
): Reduce(ReduceDerivation { left: "join", right: ["equality"] })
!=: Shift(21)
==: Shift(60)
;: Reduce(ReduceDerivation { left: "join", right: ["equality"] })
===================
State 21:
term: Shift(47)
num: Shift(27)
false: Shift(28)
real: Shift(19)
factor: Shift(37)
true: Shift(36)
rel: Shift(64)
expr: Shift(31)
loc: Shift(23)
id: Shift(16)
(: Shift(29)
unary: Shift(33)
!: Shift(22)
-: Shift(34)
===================
State 22:
unary: Shift(63)
!: Shift(22)
(: Shift(29)
num: Shift(27)
id: Shift(16)
real: Shift(19)
true: Shift(36)
factor: Shift(37)
-: Shift(34)
loc: Shift(23)
false: Shift(28)
===================
State 23:
!=: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
==: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
+: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
<: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
[: Shift(24)
*: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
/: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
>=: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
;: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
): Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
<=: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
>: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
||: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
-: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
&&: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
===================
State 24:
num: Shift(25)
===================
State 25:
]: Shift(26)
===================
State 26:
/: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
*: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
[: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
;: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
&&: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
<=: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
=: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
!=: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
>=: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
): Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
==: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
+: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
||: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
>: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
-: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
<: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
===================
State 27:
+: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
<: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
-: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
<=: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
>=: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
!=: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
*: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
;: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
&&: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
==: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
||: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
): Reduce(ReduceDerivation { left: "factor", right: ["num"] })
/: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
>: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
===================
State 28:
-: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
/: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
*: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
!=: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
<: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
>: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
&&: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
): Reduce(ReduceDerivation { left: "factor", right: ["false"] })
>=: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
||: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
+: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
==: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
<=: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
;: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
===================
State 29:
bool: Shift(54)
term: Shift(47)
real: Shift(19)
unary: Shift(33)
-: Shift(34)
equality: Shift(20)
factor: Shift(37)
expr: Shift(31)
true: Shift(36)
false: Shift(28)
rel: Shift(30)
id: Shift(16)
(: Shift(29)
join: Shift(62)
loc: Shift(23)
num: Shift(27)
!: Shift(22)
===================
State 30:
==: Reduce(ReduceDerivation { left: "equality", right: ["rel"] })
&&: Reduce(ReduceDerivation { left: "equality", right: ["rel"] })
;: Reduce(ReduceDerivation { left: "equality", right: ["rel"] })
||: Reduce(ReduceDerivation { left: "equality", right: ["rel"] })
!=: Reduce(ReduceDerivation { left: "equality", right: ["rel"] })
): Reduce(ReduceDerivation { left: "equality", right: ["rel"] })
===================
State 31:
!=: Reduce(ReduceDerivation { left: "rel", right: ["expr"] })
-: Shift(32)
>=: Shift(43)
+: Shift(45)
&&: Reduce(ReduceDerivation { left: "rel", right: ["expr"] })
==: Reduce(ReduceDerivation { left: "rel", right: ["expr"] })
<: Shift(50)
<=: Shift(52)
;: Reduce(ReduceDerivation { left: "rel", right: ["expr"] })
): Reduce(ReduceDerivation { left: "rel", right: ["expr"] })
>: Shift(48)
||: Reduce(ReduceDerivation { left: "rel", right: ["expr"] })
===================
State 32:
(: Shift(29)
-: Shift(34)
true: Shift(36)
real: Shift(19)
num: Shift(27)
term: Shift(38)
id: Shift(16)
false: Shift(28)
factor: Shift(37)
loc: Shift(23)
unary: Shift(33)
!: Shift(22)
===================
State 33:
!=: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
*: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
&&: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
>: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
>=: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
-: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
): Reduce(ReduceDerivation { left: "term", right: ["unary"] })
/: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
<: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
<=: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
+: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
;: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
||: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
==: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
===================
State 34:
id: Shift(16)
real: Shift(19)
num: Shift(27)
false: Shift(28)
unary: Shift(35)
loc: Shift(23)
(: Shift(29)
!: Shift(22)
factor: Shift(37)
true: Shift(36)
-: Shift(34)
===================
State 35:
): Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
!=: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
*: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
/: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
&&: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
<=: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
>=: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
;: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
<: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
||: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
+: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
>: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
==: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
-: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
===================
State 36:
&&: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
;: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
): Reduce(ReduceDerivation { left: "factor", right: ["true"] })
||: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
-: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
<=: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
*: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
>: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
+: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
==: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
<: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
/: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
!=: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
>=: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
===================
State 37:
*: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
<: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
): Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
<=: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
+: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
==: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
&&: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
>: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
>=: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
/: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
!=: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
;: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
||: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
-: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
===================
State 38:
*: Shift(39)
-: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
!=: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
/: Shift(41)
): Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
+: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
<: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
==: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
>: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
>=: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
<=: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
&&: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
;: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
||: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
===================
State 39:
-: Shift(34)
unary: Shift(40)
real: Shift(19)
loc: Shift(23)
factor: Shift(37)
true: Shift(36)
id: Shift(16)
(: Shift(29)
!: Shift(22)
num: Shift(27)
false: Shift(28)
===================
State 40:
||: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
>: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
+: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
): Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
*: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
&&: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
!=: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
==: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
>=: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
/: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
;: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
<: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
-: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
<=: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
===================
State 41:
!: Shift(22)
id: Shift(16)
factor: Shift(37)
loc: Shift(23)
num: Shift(27)
real: Shift(19)
(: Shift(29)
unary: Shift(42)
true: Shift(36)
-: Shift(34)
false: Shift(28)
===================
State 42:
||: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
!=: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
&&: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
>=: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
/: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
>: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
<=: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
;: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
*: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
+: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
-: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
): Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
<: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
==: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
===================
State 43:
id: Shift(16)
term: Shift(47)
num: Shift(27)
expr: Shift(44)
true: Shift(36)
false: Shift(28)
!: Shift(22)
(: Shift(29)
loc: Shift(23)
unary: Shift(33)
real: Shift(19)
-: Shift(34)
factor: Shift(37)
===================
State 44:
;: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">=", "expr"] })
): Reduce(ReduceDerivation { left: "rel", right: ["expr", ">=", "expr"] })
&&: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">=", "expr"] })
||: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">=", "expr"] })
+: Shift(45)
==: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">=", "expr"] })
!=: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">=", "expr"] })
-: Shift(32)
===================
State 45:
true: Shift(36)
real: Shift(19)
num: Shift(27)
unary: Shift(33)
factor: Shift(37)
id: Shift(16)
!: Shift(22)
false: Shift(28)
loc: Shift(23)
(: Shift(29)
term: Shift(46)
-: Shift(34)
===================
State 46:
/: Shift(41)
||: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
&&: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
<=: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
): Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
-: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
+: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
<: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
>=: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
==: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
;: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
!=: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
>: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
*: Shift(39)
===================
State 47:
&&: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
;: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
!=: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
*: Shift(39)
>=: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
>: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
): Reduce(ReduceDerivation { left: "expr", right: ["term"] })
||: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
==: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
-: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
/: Shift(41)
<: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
<=: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
+: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
===================
State 48:
num: Shift(27)
term: Shift(47)
unary: Shift(33)
real: Shift(19)
!: Shift(22)
(: Shift(29)
id: Shift(16)
expr: Shift(49)
false: Shift(28)
-: Shift(34)
factor: Shift(37)
true: Shift(36)
loc: Shift(23)
===================
State 49:
+: Shift(45)
==: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">", "expr"] })
&&: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">", "expr"] })
;: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">", "expr"] })
): Reduce(ReduceDerivation { left: "rel", right: ["expr", ">", "expr"] })
!=: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">", "expr"] })
-: Shift(32)
||: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">", "expr"] })
===================
State 50:
factor: Shift(37)
true: Shift(36)
(: Shift(29)
-: Shift(34)
term: Shift(47)
real: Shift(19)
num: Shift(27)
loc: Shift(23)
id: Shift(16)
false: Shift(28)
unary: Shift(33)
expr: Shift(51)
!: Shift(22)
===================
State 51:
&&: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<", "expr"] })
;: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<", "expr"] })
!=: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<", "expr"] })
||: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<", "expr"] })
+: Shift(45)
): Reduce(ReduceDerivation { left: "rel", right: ["expr", "<", "expr"] })
==: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<", "expr"] })
-: Shift(32)
===================
State 52:
!: Shift(22)
true: Shift(36)
real: Shift(19)
unary: Shift(33)
expr: Shift(53)
factor: Shift(37)
-: Shift(34)
(: Shift(29)
term: Shift(47)
num: Shift(27)
loc: Shift(23)
false: Shift(28)
id: Shift(16)
===================
State 53:
&&: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<=", "expr"] })
+: Shift(45)
||: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<=", "expr"] })
): Reduce(ReduceDerivation { left: "rel", right: ["expr", "<=", "expr"] })
==: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<=", "expr"] })
-: Shift(32)
;: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<=", "expr"] })
!=: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<=", "expr"] })
===================
State 54:
): Shift(55)
||: Shift(56)
===================
State 55:
-: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
<: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
>: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
>=: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
!=: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
/: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
): Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
==: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
&&: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
||: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
*: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
<=: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
;: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
+: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
===================
State 56:
equality: Shift(20)
-: Shift(34)
expr: Shift(31)
join: Shift(57)
false: Shift(28)
loc: Shift(23)
unary: Shift(33)
id: Shift(16)
num: Shift(27)
true: Shift(36)
factor: Shift(37)
term: Shift(47)
!: Shift(22)
real: Shift(19)
(: Shift(29)
rel: Shift(30)
===================
State 57:
&&: Shift(58)
;: Reduce(ReduceDerivation { left: "bool", right: ["bool", "||", "join"] })
||: Reduce(ReduceDerivation { left: "bool", right: ["bool", "||", "join"] })
): Reduce(ReduceDerivation { left: "bool", right: ["bool", "||", "join"] })
===================
State 58:
factor: Shift(37)
-: Shift(34)
(: Shift(29)
expr: Shift(31)
unary: Shift(33)
num: Shift(27)
real: Shift(19)
equality: Shift(59)
false: Shift(28)
!: Shift(22)
id: Shift(16)
rel: Shift(30)
loc: Shift(23)
true: Shift(36)
term: Shift(47)
===================
State 59:
&&: Reduce(ReduceDerivation { left: "join", right: ["join", "&&", "equality"] })
==: Shift(60)
;: Reduce(ReduceDerivation { left: "join", right: ["join", "&&", "equality"] })
!=: Shift(21)
): Reduce(ReduceDerivation { left: "join", right: ["join", "&&", "equality"] })
||: Reduce(ReduceDerivation { left: "join", right: ["join", "&&", "equality"] })
===================
State 60:
(: Shift(29)
true: Shift(36)
factor: Shift(37)
num: Shift(27)
real: Shift(19)
false: Shift(28)
-: Shift(34)
expr: Shift(31)
id: Shift(16)
term: Shift(47)
unary: Shift(33)
loc: Shift(23)
rel: Shift(61)
!: Shift(22)
===================
State 61:
;: Reduce(ReduceDerivation { left: "equality", right: ["equality", "==", "rel"] })
!=: Reduce(ReduceDerivation { left: "equality", right: ["equality", "==", "rel"] })
==: Reduce(ReduceDerivation { left: "equality", right: ["equality", "==", "rel"] })
): Reduce(ReduceDerivation { left: "equality", right: ["equality", "==", "rel"] })
&&: Reduce(ReduceDerivation { left: "equality", right: ["equality", "==", "rel"] })
||: Reduce(ReduceDerivation { left: "equality", right: ["equality", "==", "rel"] })
===================
State 62:
&&: Shift(58)
;: Reduce(ReduceDerivation { left: "bool", right: ["join"] })
||: Reduce(ReduceDerivation { left: "bool", right: ["join"] })
): Reduce(ReduceDerivation { left: "bool", right: ["join"] })
===================
State 63:
!=: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
+: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
/: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
): Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
&&: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
<: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
<=: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
>: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
||: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
==: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
-: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
;: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
>=: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
*: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
===================
State 64:
): Reduce(ReduceDerivation { left: "equality", right: ["equality", "!=", "rel"] })
;: Reduce(ReduceDerivation { left: "equality", right: ["equality", "!=", "rel"] })
!=: Reduce(ReduceDerivation { left: "equality", right: ["equality", "!=", "rel"] })
&&: Reduce(ReduceDerivation { left: "equality", right: ["equality", "!=", "rel"] })
==: Reduce(ReduceDerivation { left: "equality", right: ["equality", "!=", "rel"] })
||: Reduce(ReduceDerivation { left: "equality", right: ["equality", "!=", "rel"] })
===================
State 65:
||: Shift(56)
): Shift(66)
===================
State 66:
while: Shift(17)
loc: Shift(67)
if: Shift(72)
stmt: Shift(88)
block: Shift(71)
id: Shift(16)
break: Shift(76)
left_curly_brace: Shift(3)
do: Shift(78)
===================
State 67:
[: Shift(24)
=: Shift(68)
===================
State 68:
rel: Shift(30)
!: Shift(22)
true: Shift(36)
bool: Shift(69)
loc: Shift(23)
false: Shift(28)
expr: Shift(31)
factor: Shift(37)
-: Shift(34)
equality: Shift(20)
id: Shift(16)
real: Shift(19)
num: Shift(27)
(: Shift(29)
unary: Shift(33)
join: Shift(62)
term: Shift(47)
===================
State 69:
;: Shift(70)
||: Shift(56)
===================
State 70:
else: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
break: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
id: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
do: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
right_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
while: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
if: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
===================
State 71:
while: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
right_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
else: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
id: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
break: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
do: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
if: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
===================
State 72:
(: Shift(73)
===================
State 73:
expr: Shift(31)
factor: Shift(37)
unary: Shift(33)
loc: Shift(23)
num: Shift(27)
true: Shift(36)
id: Shift(16)
!: Shift(22)
bool: Shift(74)
equality: Shift(20)
(: Shift(29)
false: Shift(28)
term: Shift(47)
-: Shift(34)
join: Shift(62)
rel: Shift(30)
real: Shift(19)
===================
State 74:
||: Shift(56)
): Shift(75)
===================
State 75:
left_curly_brace: Shift(3)
id: Shift(16)
loc: Shift(67)
block: Shift(71)
stmt: Shift(85)
break: Shift(76)
if: Shift(72)
while: Shift(17)
do: Shift(78)
===================
State 76:
;: Shift(77)
===================
State 77:
if: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
while: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
else: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
break: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
id: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
do: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
right_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
===================
State 78:
do: Shift(78)
left_curly_brace: Shift(3)
stmt: Shift(79)
break: Shift(76)
block: Shift(71)
while: Shift(17)
id: Shift(16)
if: Shift(72)
loc: Shift(67)
===================
State 79:
while: Shift(80)
===================
State 80:
(: Shift(81)
===================
State 81:
id: Shift(16)
real: Shift(19)
(: Shift(29)
false: Shift(28)
expr: Shift(31)
factor: Shift(37)
!: Shift(22)
bool: Shift(82)
rel: Shift(30)
unary: Shift(33)
true: Shift(36)
term: Shift(47)
-: Shift(34)
loc: Shift(23)
num: Shift(27)
equality: Shift(20)
join: Shift(62)
===================
State 82:
||: Shift(56)
): Shift(83)
===================
State 83:
;: Shift(84)
===================
State 84:
if: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
while: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
id: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
break: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
right_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
else: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
do: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
===================
State 85:
else: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
do: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
break: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
while: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
id: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
if: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
right_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
===================
State 86:
do: Shift(78)
loc: Shift(67)
block: Shift(71)
while: Shift(17)
break: Shift(76)
stmt: Shift(87)
left_curly_brace: Shift(3)
if: Shift(72)
id: Shift(16)
===================
State 87:
if: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
break: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
do: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
right_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
id: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
while: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
else: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
===================
State 88:
while: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
right_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
do: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
if: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
id: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
else: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
break: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
===================
State 89:
__$__: Reduce(ReduceDerivation { left: "__DUMMY_START__", right: ["S"] })
===================

```
---
generated by rparser
RockRockWhite 2023
    