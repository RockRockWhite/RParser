
# Generated Info

## Base Info
- config_file: sample.rparser
- output_file: sample.rs
- time: 2023-05-20 13:57:23.463923 +08:00

---

## DFA Graph
```mermaid
graph LR
0["S -> •program\nprogram -> •block\n__DUMMY_START__ -> •S\nblock -> •left_curly_brace decls stmts right_curly_brace\n"]
1["S -> program•\n"]
2["program -> block•\n"]
3["__DUMMY_START__ -> S•\n"]
4["block -> left_curly_brace •decls stmts right_curly_brace\ndecls -> •\ndecls -> •decls decl\n"]
5["type -> •type [ num ]\ndecls -> decls •decl\nblock -> left_curly_brace decls •stmts right_curly_brace\ndecl -> •type id ;\nstmts -> •stmts stmt\ntype -> •basic\nstmts -> •\n"]
6["decls -> decls decl•\n"]
7["type -> basic•\n"]
8["type -> type •[ num ]\ndecl -> type •id ;\n"]
9["decl -> type id •;\n"]
10["decl -> type id ;•\n"]
11["type -> type [ •num ]\n"]
12["type -> type [ num •]\n"]
13["type -> type [ num ]•\n"]
14["stmt -> •block\nstmt -> •break ;\nstmt -> •while ( bool ) stmt\nstmt -> •loc = bool ;\nstmts -> stmts •stmt\nstmt -> •if ( bool ) stmt else stmt\nblock -> left_curly_brace decls stmts •right_curly_brace\nstmt -> •if ( bool ) stmt\nblock -> •left_curly_brace decls stmts right_curly_brace\nstmt -> •do stmt while ( bool ) ;\nloc -> •loc [ num ]\nloc -> •id\n"]
15["stmt -> break •;\n"]
16["stmt -> break ;•\n"]
17["stmt -> if •( bool ) stmt\nstmt -> if •( bool ) stmt else stmt\n"]
18["rel -> •expr > expr\nfactor -> •loc\nstmt -> if ( •bool ) stmt\nfactor -> •( bool )\nterm -> •term * unary\nrel -> •expr < expr\nunary -> •- unary\nfactor -> •num\nexpr -> •expr - term\njoin -> •join && equality\nfactor -> •true\nrel -> •expr <= expr\nloc -> •id\nunary -> •! unary\nunary -> •factor\nequality -> •rel\nterm -> •term / unary\njoin -> •equality\nfactor -> •real\nbool -> •bool || join\nexpr -> •term\nrel -> •expr\nstmt -> if ( •bool ) stmt else stmt\nfactor -> •false\nequality -> •equality != rel\nequality -> •equality == rel\nloc -> •loc [ num ]\nbool -> •join\nrel -> •expr >= expr\nexpr -> •expr + term\nterm -> •unary\n"]
19["stmt -> if ( bool •) stmt\nstmt -> if ( bool •) stmt else stmt\nbool -> bool •|| join\n"]
20["rel -> •expr >= expr\nterm -> •term / unary\nexpr -> •term\nunary -> •- unary\nequality -> •equality != rel\nfactor -> •( bool )\nfactor -> •num\nequality -> •equality == rel\nunary -> •factor\nterm -> •term * unary\nexpr -> •expr + term\nrel -> •expr\njoin -> •equality\nexpr -> •expr - term\nfactor -> •loc\nfactor -> •true\nrel -> •expr < expr\nunary -> •! unary\njoin -> •join && equality\nloc -> •loc [ num ]\nbool -> bool || •join\nloc -> •id\nfactor -> •false\nterm -> •unary\nrel -> •expr <= expr\nequality -> •rel\nfactor -> •real\nrel -> •expr > expr\n"]
21["factor -> •( bool )\nfactor -> •loc\nunary -> •factor\nunary -> •! unary\nfactor -> •true\nloc -> •id\nfactor -> •false\nfactor -> •num\nunary -> •- unary\nunary -> - •unary\nfactor -> •real\nloc -> •loc [ num ]\n"]
22["factor -> true•\n"]
23["equality -> •equality != rel\njoin -> •join && equality\nfactor -> •loc\nloc -> •id\nbool -> •bool || join\nrel -> •expr <= expr\nterm -> •term / unary\nfactor -> •real\nunary -> •factor\nexpr -> •expr + term\nfactor -> •num\nfactor -> •false\nterm -> •term * unary\nrel -> •expr >= expr\nunary -> •! unary\njoin -> •equality\nexpr -> •term\nequality -> •rel\nrel -> •expr\nfactor -> •true\nbool -> •join\nequality -> •equality == rel\nrel -> •expr > expr\nunary -> •- unary\nloc -> •loc [ num ]\nfactor -> •( bool )\nexpr -> •expr - term\nfactor -> ( •bool )\nrel -> •expr < expr\nterm -> •unary\n"]
24["expr -> expr •+ term\nrel -> expr •>= expr\nrel -> expr•\nrel -> expr •> expr\nrel -> expr •< expr\nexpr -> expr •- term\nrel -> expr •<= expr\n"]
25["term -> •unary\nloc -> •id\nfactor -> •true\nunary -> •factor\nloc -> •loc [ num ]\nfactor -> •num\nterm -> •term / unary\nfactor -> •false\nterm -> •term * unary\nfactor -> •loc\nfactor -> •( bool )\nunary -> •! unary\nunary -> •- unary\nexpr -> expr - •term\nfactor -> •real\n"]
26["factor -> false•\n"]
27["factor -> real•\n"]
28["factor -> num•\n"]
29["unary -> •- unary\nloc -> •loc [ num ]\nfactor -> •real\nunary -> •! unary\nfactor -> •loc\nfactor -> •( bool )\nfactor -> •num\nunary -> ! •unary\nfactor -> •false\nfactor -> •true\nunary -> •factor\nloc -> •id\n"]
30["unary -> ! unary•\n"]
31["loc -> id•\n"]
32["loc -> loc •[ num ]\nfactor -> loc•\n"]
33["loc -> loc [ •num ]\n"]
34["loc -> loc [ num •]\n"]
35["loc -> loc [ num ]•\n"]
36["unary -> factor•\n"]
37["term -> term •/ unary\nexpr -> expr - term•\nterm -> term •* unary\n"]
38["factor -> •( bool )\nfactor -> •true\nfactor -> •num\nunary -> •! unary\nloc -> •loc [ num ]\nfactor -> •false\nterm -> term / •unary\nunary -> •- unary\nfactor -> •real\nunary -> •factor\nfactor -> •loc\nloc -> •id\n"]
39["term -> term / unary•\n"]
40["loc -> •id\nfactor -> •true\nfactor -> •num\nfactor -> •false\nfactor -> •( bool )\nunary -> •factor\nloc -> •loc [ num ]\nterm -> term * •unary\nunary -> •! unary\nfactor -> •loc\nfactor -> •real\nunary -> •- unary\n"]
41["term -> term * unary•\n"]
42["term -> unary•\n"]
43["factor -> •num\nunary -> •factor\nunary -> •- unary\nunary -> •! unary\nfactor -> •real\nterm -> •term * unary\nexpr -> •expr - term\nterm -> •term / unary\nrel -> expr < •expr\nloc -> •loc [ num ]\nfactor -> •false\nfactor -> •loc\nterm -> •unary\nexpr -> •term\nloc -> •id\nfactor -> •true\nexpr -> •expr + term\nfactor -> •( bool )\n"]
44["rel -> expr < expr•\nexpr -> expr •- term\nexpr -> expr •+ term\n"]
45["factor -> •false\nloc -> •loc [ num ]\nfactor -> •true\nfactor -> •real\nterm -> •term * unary\nexpr -> expr + •term\nterm -> •unary\nloc -> •id\nfactor -> •( bool )\nfactor -> •loc\nunary -> •factor\nterm -> •term / unary\nunary -> •- unary\nfactor -> •num\nunary -> •! unary\n"]
46["term -> term •* unary\nexpr -> expr + term•\nterm -> term •/ unary\n"]
47["expr -> term•\nterm -> term •* unary\nterm -> term •/ unary\n"]
48["term -> •term / unary\nfactor -> •loc\nloc -> •loc [ num ]\nexpr -> •term\nfactor -> •real\nloc -> •id\nunary -> •factor\nrel -> expr > •expr\nterm -> •term * unary\nfactor -> •num\nterm -> •unary\nunary -> •! unary\nfactor -> •true\nexpr -> •expr - term\nfactor -> •false\nexpr -> •expr + term\nunary -> •- unary\nfactor -> •( bool )\n"]
49["expr -> expr •- term\nexpr -> expr •+ term\nrel -> expr > expr•\n"]
50["loc -> •loc [ num ]\nterm -> •unary\nfactor -> •true\nfactor -> •num\nunary -> •- unary\nfactor -> •false\nfactor -> •real\nfactor -> •loc\nunary -> •! unary\nfactor -> •( bool )\nunary -> •factor\nexpr -> •term\nexpr -> •expr + term\nterm -> •term * unary\nexpr -> •expr - term\nterm -> •term / unary\nrel -> expr <= •expr\nloc -> •id\n"]
51["rel -> expr <= expr•\nexpr -> expr •+ term\nexpr -> expr •- term\n"]
52["factor -> •loc\nexpr -> •term\nfactor -> •real\nunary -> •! unary\nfactor -> •false\nloc -> •loc [ num ]\nterm -> •term * unary\nterm -> •term / unary\nfactor -> •true\nfactor -> •num\nterm -> •unary\nfactor -> •( bool )\nexpr -> •expr + term\nexpr -> •expr - term\nunary -> •- unary\nrel -> expr >= •expr\nunary -> •factor\nloc -> •id\n"]
53["expr -> expr •- term\nexpr -> expr •+ term\nrel -> expr >= expr•\n"]
54["bool -> bool •|| join\nfactor -> ( bool •)\n"]
55["factor -> ( bool )•\n"]
56["join -> join •&& equality\nbool -> join•\n"]
57["equality -> •rel\nrel -> •expr > expr\nrel -> •expr\nexpr -> •term\nfactor -> •( bool )\nloc -> •id\nunary -> •! unary\nrel -> •expr <= expr\nfactor -> •num\nrel -> •expr < expr\nfactor -> •true\nloc -> •loc [ num ]\nterm -> •term / unary\nterm -> •unary\nunary -> •- unary\nunary -> •factor\nequality -> •equality == rel\nexpr -> •expr + term\nterm -> •term * unary\nexpr -> •expr - term\nfactor -> •loc\nequality -> •equality != rel\nfactor -> •real\njoin -> join && •equality\nfactor -> •false\nrel -> •expr >= expr\n"]
58["equality -> equality •== rel\nequality -> equality •!= rel\njoin -> join && equality•\n"]
59["unary -> •! unary\nrel -> •expr >= expr\nfactor -> •num\nunary -> •- unary\nloc -> •id\nfactor -> •loc\nfactor -> •false\nterm -> •term * unary\nfactor -> •real\nexpr -> •expr - term\nunary -> •factor\nterm -> •term / unary\nfactor -> •( bool )\nloc -> •loc [ num ]\nrel -> •expr > expr\nrel -> •expr\nrel -> •expr <= expr\nrel -> •expr < expr\nexpr -> •expr + term\nequality -> equality != •rel\nfactor -> •true\nexpr -> •term\nterm -> •unary\n"]
60["equality -> equality != rel•\n"]
61["factor -> •num\nexpr -> •expr + term\nfactor -> •true\nterm -> •unary\nexpr -> •expr - term\nfactor -> •real\nunary -> •- unary\nrel -> •expr < expr\nrel -> •expr > expr\nunary -> •! unary\nrel -> •expr <= expr\nunary -> •factor\nequality -> equality == •rel\nloc -> •id\nrel -> •expr >= expr\nfactor -> •loc\nterm -> •term * unary\nloc -> •loc [ num ]\nterm -> •term / unary\nfactor -> •( bool )\nfactor -> •false\nexpr -> •term\nrel -> •expr\n"]
62["equality -> equality == rel•\n"]
63["equality -> rel•\n"]
64["equality -> equality •!= rel\nequality -> equality •== rel\njoin -> equality•\n"]
65["unary -> - unary•\n"]
66["join -> join •&& equality\nbool -> bool || join•\n"]
67["block -> •left_curly_brace decls stmts right_curly_brace\nstmt -> •while ( bool ) stmt\nstmt -> •block\nstmt -> •break ;\nstmt -> if ( bool ) •stmt else stmt\nstmt -> if ( bool ) •stmt\nloc -> •id\nstmt -> •if ( bool ) stmt\nloc -> •loc [ num ]\nstmt -> •loc = bool ;\nstmt -> •if ( bool ) stmt else stmt\nstmt -> •do stmt while ( bool ) ;\n"]
68["stmt -> block•\n"]
69["stmt -> if ( bool ) stmt •else stmt\nstmt -> if ( bool ) stmt•\n"]
70["stmt -> if ( bool ) stmt else •stmt\nstmt -> •break ;\nstmt -> •if ( bool ) stmt else stmt\nloc -> •id\nblock -> •left_curly_brace decls stmts right_curly_brace\nstmt -> •do stmt while ( bool ) ;\nstmt -> •block\nloc -> •loc [ num ]\nstmt -> •while ( bool ) stmt\nstmt -> •loc = bool ;\nstmt -> •if ( bool ) stmt\n"]
71["stmt -> if ( bool ) stmt else stmt•\n"]
72["stmt -> •do stmt while ( bool ) ;\nstmt -> •break ;\nstmt -> do •stmt while ( bool ) ;\nloc -> •id\nblock -> •left_curly_brace decls stmts right_curly_brace\nstmt -> •if ( bool ) stmt\nstmt -> •while ( bool ) stmt\nstmt -> •block\nstmt -> •loc = bool ;\nloc -> •loc [ num ]\nstmt -> •if ( bool ) stmt else stmt\n"]
73["stmt -> do stmt •while ( bool ) ;\n"]
74["stmt -> do stmt while •( bool ) ;\n"]
75["expr -> •expr + term\nbool -> •bool || join\njoin -> •join && equality\nfactor -> •false\nbool -> •join\nexpr -> •term\nfactor -> •true\nfactor -> •loc\nrel -> •expr >= expr\nexpr -> •expr - term\nunary -> •! unary\nterm -> •term / unary\nrel -> •expr > expr\njoin -> •equality\nunary -> •- unary\nunary -> •factor\nterm -> •term * unary\nequality -> •equality != rel\nfactor -> •( bool )\nterm -> •unary\nrel -> •expr < expr\nequality -> •equality == rel\nfactor -> •num\nfactor -> •real\nstmt -> do stmt while ( •bool ) ;\nequality -> •rel\nloc -> •id\nrel -> •expr\nrel -> •expr <= expr\nloc -> •loc [ num ]\n"]
76["bool -> bool •|| join\nstmt -> do stmt while ( bool •) ;\n"]
77["stmt -> do stmt while ( bool ) •;\n"]
78["stmt -> do stmt while ( bool ) ;•\n"]
79["stmt -> while •( bool ) stmt\n"]
80["term -> •unary\nrel -> •expr >= expr\nunary -> •factor\nunary -> •! unary\nterm -> •term / unary\nbool -> •join\nunary -> •- unary\njoin -> •join && equality\njoin -> •equality\nfactor -> •real\nbool -> •bool || join\nrel -> •expr <= expr\nrel -> •expr\nfactor -> •true\nfactor -> •( bool )\nexpr -> •expr - term\nexpr -> •term\nstmt -> while ( •bool ) stmt\nloc -> •id\nterm -> •term * unary\nequality -> •equality == rel\nfactor -> •loc\nequality -> •rel\nfactor -> •false\nequality -> •equality != rel\nrel -> •expr < expr\nfactor -> •num\nloc -> •loc [ num ]\nexpr -> •expr + term\nrel -> •expr > expr\n"]
81["stmt -> while ( bool •) stmt\nbool -> bool •|| join\n"]
82["stmt -> •break ;\nloc -> •loc [ num ]\nstmt -> •if ( bool ) stmt else stmt\nstmt -> •loc = bool ;\nstmt -> •block\nstmt -> •while ( bool ) stmt\nstmt -> while ( bool ) •stmt\nstmt -> •if ( bool ) stmt\nstmt -> •do stmt while ( bool ) ;\nloc -> •id\nblock -> •left_curly_brace decls stmts right_curly_brace\n"]
83["stmt -> while ( bool ) stmt•\n"]
84["stmt -> loc •= bool ;\nloc -> loc •[ num ]\n"]
85["join -> •equality\nrel -> •expr < expr\nunary -> •- unary\nterm -> •term * unary\nequality -> •equality == rel\nequality -> •equality != rel\nfactor -> •real\nrel -> •expr <= expr\nstmt -> loc = •bool ;\nfactor -> •num\nexpr -> •term\nunary -> •! unary\nloc -> •id\nexpr -> •expr - term\nrel -> •expr\nfactor -> •loc\nbool -> •bool || join\nunary -> •factor\nrel -> •expr >= expr\nfactor -> •false\nrel -> •expr > expr\nexpr -> •expr + term\nterm -> •term / unary\njoin -> •join && equality\nfactor -> •true\nterm -> •unary\nfactor -> •( bool )\nbool -> •join\nequality -> •rel\nloc -> •loc [ num ]\n"]
86["stmt -> loc = bool •;\nbool -> bool •|| join\n"]
87["stmt -> loc = bool ;•\n"]
88["stmts -> stmts stmt•\n"]
89["block -> left_curly_brace decls stmts right_curly_brace•\n"]

0--"program"-->1
0--"block"-->2
0--"S"-->3
5--"decl"-->6
5--"basic"-->7
9--";"-->10
8--"id"-->9
12--"]"-->13
11--"num"-->12
8--"["-->11
5--"type"-->8
15--";"-->16
14--"break"-->15
21--"true"-->22
25--"false"-->26
25--"real"-->27
25--"num"-->28
29--"!"-->29
29--"-"-->21
29--"num"-->28
29--"unary"-->30
29--"id"-->31
29--"false"-->26
34--"]"-->35
33--"num"-->34
32--"["-->33
29--"loc"-->32
29--"real"-->27
29--"factor"-->36
29--"true"-->22
29--"("-->23
25--"!"-->29
25--"-"-->21
25--"("-->23
38--"("-->23
38--"real"-->27
38--"true"-->22
38--"id"-->31
38--"factor"-->36
38--"num"-->28
38--"false"-->26
38--"!"-->29
38--"unary"-->39
38--"-"-->21
38--"loc"-->32
37--"/"-->38
40--"!"-->29
40--"real"-->27
40--"false"-->26
40--"id"-->31
40--"-"-->21
40--"factor"-->36
40--"num"-->28
40--"unary"-->41
40--"("-->23
40--"loc"-->32
40--"true"-->22
37--"*"-->40
25--"term"-->37
25--"unary"-->42
25--"true"-->22
25--"factor"-->36
25--"id"-->31
25--"loc"-->32
24--"-"-->25
43--"id"-->31
43--"!"-->29
43--"real"-->27
43--"loc"-->32
43--"unary"-->42
43--"-"-->21
44--"-"-->25
45--"num"-->28
45--"!"-->29
45--"id"-->31
45--"unary"-->42
45--"false"-->26
45--"-"-->21
46--"/"-->38
46--"*"-->40
45--"term"-->46
45--"("-->23
45--"loc"-->32
45--"real"-->27
45--"factor"-->36
45--"true"-->22
44--"+"-->45
43--"expr"-->44
47--"*"-->40
47--"/"-->38
43--"term"-->47
43--"true"-->22
43--"num"-->28
43--"("-->23
43--"false"-->26
43--"factor"-->36
24--"<"-->43
48--"num"-->28
48--"id"-->31
48--"unary"-->42
48--"true"-->22
48--"real"-->27
48--"!"-->29
48--"-"-->21
48--"factor"-->36
48--"term"-->47
48--"loc"-->32
49--"-"-->25
49--"+"-->45
48--"expr"-->49
48--"("-->23
48--"false"-->26
24--">"-->48
50--"-"-->21
50--"real"-->27
50--"unary"-->42
50--"("-->23
51--"-"-->25
51--"+"-->45
50--"expr"-->51
50--"factor"-->36
50--"!"-->29
50--"id"-->31
50--"loc"-->32
50--"num"-->28
50--"true"-->22
50--"term"-->47
50--"false"-->26
24--"<="-->50
52--"!"-->29
52--"unary"-->42
52--"("-->23
52--"-"-->21
52--"id"-->31
52--"false"-->26
52--"factor"-->36
53--"+"-->45
53--"-"-->25
52--"expr"-->53
52--"num"-->28
52--"loc"-->32
52--"real"-->27
52--"true"-->22
52--"term"-->47
24--">="-->52
24--"+"-->45
23--"expr"-->24
54--"||"-->20
54--")"-->55
23--"bool"-->54
57--"("-->23
59--"true"-->22
59--"unary"-->42
59--"rel"-->60
59--"false"-->26
59--"loc"-->32
59--"!"-->29
59--"num"-->28
59--"term"-->47
59--"real"-->27
59--"factor"-->36
59--"expr"-->24
59--"id"-->31
59--"-"-->21
59--"("-->23
58--"!="-->59
61--"-"-->21
61--"unary"-->42
61--"!"-->29
61--"true"-->22
61--"("-->23
61--"real"-->27
61--"expr"-->24
61--"id"-->31
61--"term"-->47
61--"false"-->26
61--"factor"-->36
61--"num"-->28
61--"rel"-->62
61--"loc"-->32
58--"=="-->61
57--"equality"-->58
57--"!"-->29
57--"loc"-->32
57--"rel"-->63
57--"false"-->26
57--"term"-->47
57--"num"-->28
57--"true"-->22
57--"real"-->27
57--"expr"-->24
57--"id"-->31
57--"factor"-->36
57--"-"-->21
57--"unary"-->42
56--"&&"-->57
23--"join"-->56
23--"loc"-->32
23--"factor"-->36
23--"false"-->26
23--"rel"-->63
23--"id"-->31
23--"true"-->22
23--"real"-->27
23--"!"-->29
23--"num"-->28
23--"-"-->21
23--"unary"-->42
64--"=="-->61
64--"!="-->59
23--"equality"-->64
23--"("-->23
23--"term"-->47
21--"("-->23
21--"loc"-->32
21--"-"-->21
21--"unary"-->65
21--"id"-->31
21--"factor"-->36
21--"real"-->27
21--"false"-->26
21--"num"-->28
21--"!"-->29
20--"-"-->21
20--"false"-->26
20--"rel"-->63
20--"equality"-->64
20--"id"-->31
20--"unary"-->42
20--"true"-->22
20--"real"-->27
20--"num"-->28
20--"!"-->29
66--"&&"-->57
20--"join"-->66
20--"("-->23
20--"term"-->47
20--"loc"-->32
20--"expr"-->24
20--"factor"-->36
19--"||"-->20
67--"block"-->68
67--"break"-->15
70--"if"-->17
70--"stmt"-->71
72--"do"-->72
72--"left_curly_brace"-->4
75--"real"-->27
75--"id"-->31
76--"||"-->20
77--";"-->78
76--")"-->77
75--"bool"-->76
75--"term"-->47
75--"expr"-->24
75--"!"-->29
75--"equality"-->64
75--"rel"-->63
75--"unary"-->42
75--"("-->23
75--"false"-->26
75--"true"-->22
75--"factor"-->36
75--"join"-->56
75--"num"-->28
75--"-"-->21
75--"loc"-->32
74--"("-->75
73--"while"-->74
72--"stmt"-->73
72--"if"-->17
80--"true"-->22
80--"unary"-->42
80--"("-->23
80--"false"-->26
80--"expr"-->24
80--"join"-->56
80--"equality"-->64
80--"!"-->29
80--"num"-->28
80--"loc"-->32
80--"-"-->21
80--"factor"-->36
80--"real"-->27
80--"id"-->31
80--"term"-->47
80--"rel"-->63
81--"||"-->20
82--"block"-->68
82--"break"-->15
82--"id"-->31
82--"left_curly_brace"-->4
82--"do"-->72
82--"if"-->17
82--"stmt"-->83
82--"while"-->79
84--"["-->33
85--"-"-->21
85--"!"-->29
85--"unary"-->42
85--"term"-->47
85--"true"-->22
85--"("-->23
85--"rel"-->63
85--"id"-->31
85--"expr"-->24
85--"real"-->27
85--"false"-->26
85--"num"-->28
85--"loc"-->32
86--";"-->87
86--"||"-->20
85--"bool"-->86
85--"factor"-->36
85--"join"-->56
85--"equality"-->64
84--"="-->85
82--"loc"-->84
81--")"-->82
80--"bool"-->81
79--"("-->80
72--"while"-->79
72--"break"-->15
72--"block"-->68
72--"loc"-->84
72--"id"-->31
70--"do"-->72
70--"loc"-->84
70--"while"-->79
70--"left_curly_brace"-->4
70--"break"-->15
70--"block"-->68
70--"id"-->31
69--"else"-->70
67--"stmt"-->69
67--"id"-->31
67--"do"-->72
67--"left_curly_brace"-->4
67--"if"-->17
67--"while"-->79
67--"loc"-->84
19--")"-->67
18--"bool"-->19
18--"expr"-->24
18--"num"-->28
18--"true"-->22
18--"id"-->31
18--"!"-->29
18--"false"-->26
18--"unary"-->42
18--"rel"-->63
18--"real"-->27
18--"term"-->47
18--"factor"-->36
18--"equality"-->64
18--"loc"-->32
18--"join"-->56
18--"-"-->21
18--"("-->23
17--"("-->18
14--"if"-->17
14--"block"-->68
14--"while"-->79
14--"stmt"-->88
14--"right_curly_brace"-->89
14--"left_curly_brace"-->4
14--"id"-->31
14--"loc"-->84
14--"do"-->72
5--"stmts"-->14
4--"decls"-->5
0--"left_curly_brace"-->4

```

---

## Follow Set
```txt
bool: ["||", ")", ";"]
(: ["true", "(", "num", "false", "-", "real", "id", "!"]
basic: ["[", "id"]
term: ["/", "+", "||", "<", "!=", "&&", "-", ">", ">=", "<=", "*", ")", ";", "=="]
stmts: ["left_curly_brace", "if", "do", "break", "id", "while", "right_curly_brace"]
__DUMMY_START__: ["__$__"]
+: ["!", "real", "false", "true", "num", "id", "-", "("]
[: ["num"]
while: ["("]
factor: ["+", "-", "!=", "<=", "||", ")", ">=", "/", "*", "<", ">", "==", "&&", ";"]
S: ["__$__"]
decl: ["left_curly_brace", "break", "right_curly_brace", "do", "if", "while", "id", "basic"]
;: ["if", "id", "else", "right_curly_brace", "do", "break", "basic", "left_curly_brace", "while"]
do: ["break", "left_curly_brace", "do", "id", "if", "while"]
break: [";"]
!: ["num", "id", "false", "real", "(", "true", "-", "!"]
type: ["id", "["]
join: [")", "&&", "||", ";"]
): ["-", ";", "<=", "*", "<", "/", "id", "break", ">", "&&", "||", "==", "!=", ">=", ")", "do", "if", "while", "+", "left_curly_brace"]
if: ["("]
equality: ["!=", "==", "&&", ")", ";", "||"]
!=: ["real", "-", "num", "!", "true", "(", "false", "id"]
rel: ["!=", "||", ")", ";", "==", "&&"]
expr: ["+", "!=", ">=", ")", ";", "==", "||", "<", "&&", "<=", "-", ">"]
loc: ["&&", "-", "||", ")", ">=", "*", ";", "==", "=", "+", "<", "/", "[", ">", "!=", "<="]
>: ["id", "!", "-", "(", "true", "num", "real", "false"]
__EPSILON__: ["if", "break", "id", "while", "right_curly_brace", "do", "left_curly_brace", "basic"]
unary: ["*", ")", "!=", "+", "||", ";", "/", "<", "-", ">", "<=", ">=", "==", "&&"]
else: ["id", "left_curly_brace", "if", "break", "do", "while"]
num: ["]", ">", "&&", "*", "/", "==", "||", ">=", ";", "-", "<=", "+", "!=", "<", ")"]
<=: ["real", "true", "!", "false", "-", "id", "num", "("]
false: ["-", "+", "||", ";", "<", ">", "*", "==", "&&", ">=", "!=", ")", "<=", "/"]
right_curly_brace: ["id", "left_curly_brace", "__$__", "if", "else", "right_curly_brace", "break", "do", "while"]
program: ["__$__"]
<: ["false", "real", "id", "true", "!", "-", "(", "num"]
>=: ["!", "true", "false", "id", "-", "(", "real", "num"]
stmt: ["break", "right_curly_brace", "left_curly_brace", "else", "while", "if", "id", "do"]
block: ["__$__", "break", "while", "right_curly_brace", "left_curly_brace", "id", "if", "do", "else"]
=: ["!", "-", "false", "(", "id", "num", "true", "real"]
/: ["num", "false", "id", "-", "(", "true", "!", "real"]
real: ["<=", "<", ";", "*", "||", "/", "!=", "&&", "==", ">=", "-", "+", ")", ">"]
-: ["real", "false", "!", "id", "-", "(", "true", "num"]
true: ["/", ")", "==", "!=", ">", ">=", "*", "<", ";", "||", "-", "<=", "+", "&&"]
id: ["[", ";", "!=", "+", ">", "==", "*", "<", "-", "&&", ")", ">=", "/", "<=", "||", "="]
||: ["!", "num", "-", "false", "id", "true", "(", "real"]
&&: ["true", "(", "id", "false", "!", "num", "-", "real"]
*: ["real", "!", "true", "-", "(", "id", "false", "num"]
__$__: []
]: ["id", "+", "<", "=", ";", "||", "&&", "/", ">", "<=", ">=", "[", "*", ")", "-", "==", "!="]
left_curly_brace: ["break", "left_curly_brace", "right_curly_brace", "while", "do", "id", "if", "basic"]
decls: ["do", "if", "break", "left_curly_brace", "id", "basic", "right_curly_brace", "while"]
==: ["id", "false", "num", "real", "!", "(", "-", "true"]
```

---
## Action Table
```txt
State 0:
program: Shift(1)
block: Shift(2)
S: Shift(3)
__$__: Accept
left_curly_brace: Shift(4)
===================
State 1:
__$__: Reduce(ReduceDerivation { left: "S", right: ["program"] })
===================
State 2:
__$__: Reduce(ReduceDerivation { left: "program", right: ["block"] })
===================
State 3:
__$__: Reduce(ReduceDerivation { left: "__DUMMY_START__", right: ["S"] })
===================
State 4:
while: Reduce(ReduceDerivation { left: "decls", right: [] })
decls: Shift(5)
basic: Reduce(ReduceDerivation { left: "decls", right: [] })
right_curly_brace: Reduce(ReduceDerivation { left: "decls", right: [] })
if: Reduce(ReduceDerivation { left: "decls", right: [] })
break: Reduce(ReduceDerivation { left: "decls", right: [] })
left_curly_brace: Reduce(ReduceDerivation { left: "decls", right: [] })
id: Reduce(ReduceDerivation { left: "decls", right: [] })
do: Reduce(ReduceDerivation { left: "decls", right: [] })
===================
State 5:
do: Reduce(ReduceDerivation { left: "stmts", right: [] })
basic: Shift(7)
type: Shift(8)
break: Reduce(ReduceDerivation { left: "stmts", right: [] })
id: Reduce(ReduceDerivation { left: "stmts", right: [] })
while: Reduce(ReduceDerivation { left: "stmts", right: [] })
right_curly_brace: Reduce(ReduceDerivation { left: "stmts", right: [] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmts", right: [] })
stmts: Shift(14)
decl: Shift(6)
if: Reduce(ReduceDerivation { left: "stmts", right: [] })
===================
State 6:
left_curly_brace: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
right_curly_brace: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
break: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
id: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
while: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
if: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
do: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
basic: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
===================
State 7:
id: Reduce(ReduceDerivation { left: "type", right: ["basic"] })
[: Reduce(ReduceDerivation { left: "type", right: ["basic"] })
===================
State 8:
id: Shift(9)
[: Shift(11)
===================
State 9:
;: Shift(10)
===================
State 10:
while: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
do: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
break: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
left_curly_brace: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
right_curly_brace: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
id: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
if: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
basic: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
===================
State 11:
num: Shift(12)
===================
State 12:
]: Shift(13)
===================
State 13:
id: Reduce(ReduceDerivation { left: "type", right: ["type", "[", "num", "]"] })
[: Reduce(ReduceDerivation { left: "type", right: ["type", "[", "num", "]"] })
===================
State 14:
break: Shift(15)
while: Shift(79)
right_curly_brace: Shift(89)
left_curly_brace: Shift(4)
if: Shift(17)
loc: Shift(84)
do: Shift(72)
id: Shift(31)
block: Shift(68)
stmt: Shift(88)
===================
State 15:
;: Shift(16)
===================
State 16:
else: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
while: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
id: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
right_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
if: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
break: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
do: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
===================
State 17:
(: Shift(18)
===================
State 18:
num: Shift(28)
term: Shift(47)
factor: Shift(36)
unary: Shift(42)
real: Shift(27)
bool: Shift(19)
id: Shift(31)
-: Shift(21)
!: Shift(29)
loc: Shift(32)
join: Shift(56)
equality: Shift(64)
(: Shift(23)
true: Shift(22)
expr: Shift(24)
false: Shift(26)
rel: Shift(63)
===================
State 19:
||: Shift(20)
): Shift(67)
===================
State 20:
id: Shift(31)
unary: Shift(42)
loc: Shift(32)
factor: Shift(36)
false: Shift(26)
real: Shift(27)
num: Shift(28)
!: Shift(29)
-: Shift(21)
expr: Shift(24)
true: Shift(22)
equality: Shift(64)
rel: Shift(63)
term: Shift(47)
join: Shift(66)
(: Shift(23)
===================
State 21:
!: Shift(29)
num: Shift(28)
unary: Shift(65)
factor: Shift(36)
(: Shift(23)
false: Shift(26)
true: Shift(22)
loc: Shift(32)
-: Shift(21)
real: Shift(27)
id: Shift(31)
===================
State 22:
>: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
;: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
/: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
<: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
-: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
<=: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
): Reduce(ReduceDerivation { left: "factor", right: ["true"] })
&&: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
+: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
*: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
>=: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
||: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
==: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
!=: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
===================
State 23:
bool: Shift(54)
term: Shift(47)
loc: Shift(32)
unary: Shift(42)
join: Shift(56)
id: Shift(31)
factor: Shift(36)
false: Shift(26)
rel: Shift(63)
-: Shift(21)
!: Shift(29)
real: Shift(27)
(: Shift(23)
num: Shift(28)
true: Shift(22)
equality: Shift(64)
expr: Shift(24)
===================
State 24:
;: Reduce(ReduceDerivation { left: "rel", right: ["expr"] })
>: Shift(48)
>=: Shift(52)
-: Shift(25)
+: Shift(45)
==: Reduce(ReduceDerivation { left: "rel", right: ["expr"] })
!=: Reduce(ReduceDerivation { left: "rel", right: ["expr"] })
<: Shift(43)
&&: Reduce(ReduceDerivation { left: "rel", right: ["expr"] })
<=: Shift(50)
||: Reduce(ReduceDerivation { left: "rel", right: ["expr"] })
): Reduce(ReduceDerivation { left: "rel", right: ["expr"] })
===================
State 25:
-: Shift(21)
id: Shift(31)
real: Shift(27)
loc: Shift(32)
false: Shift(26)
(: Shift(23)
unary: Shift(42)
factor: Shift(36)
num: Shift(28)
!: Shift(29)
true: Shift(22)
term: Shift(37)
===================
State 26:
*: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
==: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
;: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
<: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
<=: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
-: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
||: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
>=: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
/: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
): Reduce(ReduceDerivation { left: "factor", right: ["false"] })
>: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
+: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
&&: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
!=: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
===================
State 27:
<=: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
&&: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
): Reduce(ReduceDerivation { left: "factor", right: ["real"] })
<: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
>=: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
*: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
>: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
+: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
==: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
-: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
!=: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
;: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
||: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
/: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
===================
State 28:
<=: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
||: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
+: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
-: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
/: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
==: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
&&: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
<: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
>=: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
*: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
>: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
;: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
): Reduce(ReduceDerivation { left: "factor", right: ["num"] })
!=: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
===================
State 29:
unary: Shift(30)
-: Shift(21)
false: Shift(26)
factor: Shift(36)
!: Shift(29)
true: Shift(22)
(: Shift(23)
loc: Shift(32)
num: Shift(28)
id: Shift(31)
real: Shift(27)
===================
State 30:
<: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
<=: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
||: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
>=: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
!=: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
-: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
>: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
): Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
+: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
/: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
;: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
==: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
*: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
&&: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
===================
State 31:
==: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
/: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
<=: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
||: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
;: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
-: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
*: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
!=: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
>: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
=: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
&&: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
+: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
<: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
): Reduce(ReduceDerivation { left: "loc", right: ["id"] })
[: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
>=: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
===================
State 32:
>: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
||: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
): Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
<=: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
*: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
+: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
==: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
<: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
-: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
!=: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
;: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
/: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
&&: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
[: Shift(33)
>=: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
===================
State 33:
num: Shift(34)
===================
State 34:
]: Shift(35)
===================
State 35:
<: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
!=: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
<=: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
[: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
>=: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
/: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
;: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
+: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
): Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
==: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
-: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
||: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
*: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
>: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
=: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
&&: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
===================
State 36:
&&: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
*: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
+: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
>: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
>=: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
/: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
;: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
||: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
): Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
-: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
!=: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
<: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
<=: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
==: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
===================
State 37:
!=: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
): Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
*: Shift(40)
>=: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
==: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
||: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
/: Shift(38)
&&: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
<=: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
-: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
>: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
+: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
<: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
;: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
===================
State 38:
true: Shift(22)
!: Shift(29)
unary: Shift(39)
-: Shift(21)
loc: Shift(32)
id: Shift(31)
real: Shift(27)
factor: Shift(36)
num: Shift(28)
(: Shift(23)
false: Shift(26)
===================
State 39:
>: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
/: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
<=: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
;: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
): Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
||: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
-: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
*: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
==: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
>=: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
<: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
!=: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
&&: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
+: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
===================
State 40:
real: Shift(27)
loc: Shift(32)
num: Shift(28)
false: Shift(26)
-: Shift(21)
!: Shift(29)
factor: Shift(36)
unary: Shift(41)
(: Shift(23)
id: Shift(31)
true: Shift(22)
===================
State 41:
*: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
): Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
;: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
==: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
>=: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
/: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
!=: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
<: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
||: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
&&: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
-: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
>: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
<=: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
+: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
===================
State 42:
*: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
>=: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
!=: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
||: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
/: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
&&: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
>: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
<: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
-: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
<=: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
;: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
+: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
): Reduce(ReduceDerivation { left: "term", right: ["unary"] })
==: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
===================
State 43:
unary: Shift(42)
id: Shift(31)
expr: Shift(44)
loc: Shift(32)
term: Shift(47)
-: Shift(21)
(: Shift(23)
!: Shift(29)
false: Shift(26)
real: Shift(27)
true: Shift(22)
num: Shift(28)
factor: Shift(36)
===================
State 44:
+: Shift(45)
;: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<", "expr"] })
&&: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<", "expr"] })
-: Shift(25)
==: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<", "expr"] })
): Reduce(ReduceDerivation { left: "rel", right: ["expr", "<", "expr"] })
!=: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<", "expr"] })
||: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<", "expr"] })
===================
State 45:
!: Shift(29)
real: Shift(27)
unary: Shift(42)
loc: Shift(32)
id: Shift(31)
num: Shift(28)
true: Shift(22)
(: Shift(23)
factor: Shift(36)
term: Shift(46)
false: Shift(26)
-: Shift(21)
===================
State 46:
==: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
-: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
;: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
&&: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
+: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
/: Shift(38)
<: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
>=: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
!=: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
*: Shift(40)
>: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
||: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
<=: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
): Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
===================
State 47:
;: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
<: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
&&: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
): Reduce(ReduceDerivation { left: "expr", right: ["term"] })
/: Shift(38)
||: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
-: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
<=: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
==: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
+: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
!=: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
>: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
>=: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
*: Shift(40)
===================
State 48:
loc: Shift(32)
expr: Shift(49)
(: Shift(23)
num: Shift(28)
true: Shift(22)
!: Shift(29)
false: Shift(26)
unary: Shift(42)
id: Shift(31)
factor: Shift(36)
term: Shift(47)
-: Shift(21)
real: Shift(27)
===================
State 49:
+: Shift(45)
): Reduce(ReduceDerivation { left: "rel", right: ["expr", ">", "expr"] })
==: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">", "expr"] })
;: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">", "expr"] })
&&: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">", "expr"] })
!=: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">", "expr"] })
-: Shift(25)
||: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">", "expr"] })
===================
State 50:
factor: Shift(36)
(: Shift(23)
-: Shift(21)
false: Shift(26)
term: Shift(47)
unary: Shift(42)
id: Shift(31)
true: Shift(22)
real: Shift(27)
num: Shift(28)
!: Shift(29)
loc: Shift(32)
expr: Shift(51)
===================
State 51:
&&: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<=", "expr"] })
||: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<=", "expr"] })
+: Shift(45)
==: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<=", "expr"] })
!=: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<=", "expr"] })
;: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<=", "expr"] })
-: Shift(25)
): Reduce(ReduceDerivation { left: "rel", right: ["expr", "<=", "expr"] })
===================
State 52:
!: Shift(29)
expr: Shift(53)
-: Shift(21)
(: Shift(23)
id: Shift(31)
unary: Shift(42)
num: Shift(28)
factor: Shift(36)
real: Shift(27)
false: Shift(26)
true: Shift(22)
loc: Shift(32)
term: Shift(47)
===================
State 53:
;: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">=", "expr"] })
!=: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">=", "expr"] })
+: Shift(45)
-: Shift(25)
==: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">=", "expr"] })
&&: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">=", "expr"] })
): Reduce(ReduceDerivation { left: "rel", right: ["expr", ">=", "expr"] })
||: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">=", "expr"] })
===================
State 54:
||: Shift(20)
): Shift(55)
===================
State 55:
||: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
&&: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
+: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
>=: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
!=: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
/: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
>: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
<: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
;: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
-: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
): Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
<=: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
*: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
==: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
===================
State 56:
;: Reduce(ReduceDerivation { left: "bool", right: ["join"] })
||: Reduce(ReduceDerivation { left: "bool", right: ["join"] })
): Reduce(ReduceDerivation { left: "bool", right: ["join"] })
&&: Shift(57)
===================
State 57:
(: Shift(23)
expr: Shift(24)
equality: Shift(58)
unary: Shift(42)
num: Shift(28)
!: Shift(29)
false: Shift(26)
term: Shift(47)
true: Shift(22)
real: Shift(27)
id: Shift(31)
-: Shift(21)
factor: Shift(36)
loc: Shift(32)
rel: Shift(63)
===================
State 58:
==: Shift(61)
||: Reduce(ReduceDerivation { left: "join", right: ["join", "&&", "equality"] })
): Reduce(ReduceDerivation { left: "join", right: ["join", "&&", "equality"] })
&&: Reduce(ReduceDerivation { left: "join", right: ["join", "&&", "equality"] })
;: Reduce(ReduceDerivation { left: "join", right: ["join", "&&", "equality"] })
!=: Shift(59)
===================
State 59:
term: Shift(47)
id: Shift(31)
unary: Shift(42)
num: Shift(28)
real: Shift(27)
expr: Shift(24)
(: Shift(23)
true: Shift(22)
rel: Shift(60)
false: Shift(26)
factor: Shift(36)
-: Shift(21)
loc: Shift(32)
!: Shift(29)
===================
State 60:
!=: Reduce(ReduceDerivation { left: "equality", right: ["equality", "!=", "rel"] })
;: Reduce(ReduceDerivation { left: "equality", right: ["equality", "!=", "rel"] })
&&: Reduce(ReduceDerivation { left: "equality", right: ["equality", "!=", "rel"] })
||: Reduce(ReduceDerivation { left: "equality", right: ["equality", "!=", "rel"] })
==: Reduce(ReduceDerivation { left: "equality", right: ["equality", "!=", "rel"] })
): Reduce(ReduceDerivation { left: "equality", right: ["equality", "!=", "rel"] })
===================
State 61:
false: Shift(26)
num: Shift(28)
true: Shift(22)
factor: Shift(36)
unary: Shift(42)
expr: Shift(24)
(: Shift(23)
real: Shift(27)
id: Shift(31)
-: Shift(21)
term: Shift(47)
rel: Shift(62)
loc: Shift(32)
!: Shift(29)
===================
State 62:
||: Reduce(ReduceDerivation { left: "equality", right: ["equality", "==", "rel"] })
==: Reduce(ReduceDerivation { left: "equality", right: ["equality", "==", "rel"] })
): Reduce(ReduceDerivation { left: "equality", right: ["equality", "==", "rel"] })
!=: Reduce(ReduceDerivation { left: "equality", right: ["equality", "==", "rel"] })
;: Reduce(ReduceDerivation { left: "equality", right: ["equality", "==", "rel"] })
&&: Reduce(ReduceDerivation { left: "equality", right: ["equality", "==", "rel"] })
===================
State 63:
;: Reduce(ReduceDerivation { left: "equality", right: ["rel"] })
!=: Reduce(ReduceDerivation { left: "equality", right: ["rel"] })
||: Reduce(ReduceDerivation { left: "equality", right: ["rel"] })
==: Reduce(ReduceDerivation { left: "equality", right: ["rel"] })
): Reduce(ReduceDerivation { left: "equality", right: ["rel"] })
&&: Reduce(ReduceDerivation { left: "equality", right: ["rel"] })
===================
State 64:
==: Shift(61)
||: Reduce(ReduceDerivation { left: "join", right: ["equality"] })
;: Reduce(ReduceDerivation { left: "join", right: ["equality"] })
): Reduce(ReduceDerivation { left: "join", right: ["equality"] })
&&: Reduce(ReduceDerivation { left: "join", right: ["equality"] })
!=: Shift(59)
===================
State 65:
+: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
;: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
<: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
-: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
>: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
<=: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
/: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
>=: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
): Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
&&: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
||: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
*: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
==: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
!=: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
===================
State 66:
&&: Shift(57)
): Reduce(ReduceDerivation { left: "bool", right: ["bool", "||", "join"] })
;: Reduce(ReduceDerivation { left: "bool", right: ["bool", "||", "join"] })
||: Reduce(ReduceDerivation { left: "bool", right: ["bool", "||", "join"] })
===================
State 67:
block: Shift(68)
stmt: Shift(69)
if: Shift(17)
do: Shift(72)
id: Shift(31)
break: Shift(15)
while: Shift(79)
loc: Shift(84)
left_curly_brace: Shift(4)
===================
State 68:
else: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
break: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
right_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
do: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
if: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
while: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
id: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
===================
State 69:
id: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
break: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
if: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
right_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
do: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
else: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
while: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
===================
State 70:
block: Shift(68)
stmt: Shift(71)
loc: Shift(84)
id: Shift(31)
do: Shift(72)
break: Shift(15)
if: Shift(17)
left_curly_brace: Shift(4)
while: Shift(79)
===================
State 71:
right_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
else: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
break: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
if: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
id: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
while: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
do: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
===================
State 72:
stmt: Shift(73)
break: Shift(15)
do: Shift(72)
left_curly_brace: Shift(4)
block: Shift(68)
loc: Shift(84)
id: Shift(31)
while: Shift(79)
if: Shift(17)
===================
State 73:
while: Shift(74)
===================
State 74:
(: Shift(75)
===================
State 75:
join: Shift(56)
rel: Shift(63)
num: Shift(28)
!: Shift(29)
true: Shift(22)
real: Shift(27)
unary: Shift(42)
loc: Shift(32)
term: Shift(47)
bool: Shift(76)
-: Shift(21)
false: Shift(26)
factor: Shift(36)
id: Shift(31)
(: Shift(23)
equality: Shift(64)
expr: Shift(24)
===================
State 76:
): Shift(77)
||: Shift(20)
===================
State 77:
;: Shift(78)
===================
State 78:
break: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
if: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
else: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
id: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
do: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
while: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
right_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
===================
State 79:
(: Shift(80)
===================
State 80:
loc: Shift(32)
rel: Shift(63)
real: Shift(27)
unary: Shift(42)
num: Shift(28)
join: Shift(56)
equality: Shift(64)
expr: Shift(24)
true: Shift(22)
!: Shift(29)
-: Shift(21)
factor: Shift(36)
id: Shift(31)
term: Shift(47)
false: Shift(26)
bool: Shift(81)
(: Shift(23)
===================
State 81:
): Shift(82)
||: Shift(20)
===================
State 82:
left_curly_brace: Shift(4)
block: Shift(68)
break: Shift(15)
if: Shift(17)
stmt: Shift(83)
loc: Shift(84)
id: Shift(31)
do: Shift(72)
while: Shift(79)
===================
State 83:
right_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
if: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
do: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
else: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
while: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
break: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
id: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
===================
State 84:
=: Shift(85)
[: Shift(33)
===================
State 85:
unary: Shift(42)
false: Shift(26)
bool: Shift(86)
id: Shift(31)
num: Shift(28)
term: Shift(47)
(: Shift(23)
factor: Shift(36)
equality: Shift(64)
loc: Shift(32)
expr: Shift(24)
rel: Shift(63)
join: Shift(56)
real: Shift(27)
-: Shift(21)
!: Shift(29)
true: Shift(22)
===================
State 86:
;: Shift(87)
||: Shift(20)
===================
State 87:
break: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
else: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
while: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
id: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
if: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
right_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
do: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
===================
State 88:
do: Reduce(ReduceDerivation { left: "stmts", right: ["stmts", "stmt"] })
break: Reduce(ReduceDerivation { left: "stmts", right: ["stmts", "stmt"] })
if: Reduce(ReduceDerivation { left: "stmts", right: ["stmts", "stmt"] })
id: Reduce(ReduceDerivation { left: "stmts", right: ["stmts", "stmt"] })
while: Reduce(ReduceDerivation { left: "stmts", right: ["stmts", "stmt"] })
right_curly_brace: Reduce(ReduceDerivation { left: "stmts", right: ["stmts", "stmt"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmts", right: ["stmts", "stmt"] })
===================
State 89:
else: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
while: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
right_curly_brace: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
left_curly_brace: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
id: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
__$__: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
break: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
if: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
do: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
===================

```
---
generated by rparser
RockRockWhite 2023
    