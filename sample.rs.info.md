
# Generated Info

## Base Info
- config_file: sample.rparser
- output_file: sample.rs
- time: 2023-05-20 14:04:37.488651 +08:00

---

## DFA Graph
```mermaid
graph LR
0["S -> •program\n__DUMMY_START__ -> •S\nblock -> •left_curly_brace decls stmts right_curly_brace\nprogram -> •block\n"]
1["program -> block•\n"]
2["__DUMMY_START__ -> S•\n"]
3["decls -> •\ndecls -> •decls decl\nblock -> left_curly_brace •decls stmts right_curly_brace\n"]
4["type -> •type [ num ]\ndecl -> •type id ;\nblock -> left_curly_brace decls •stmts right_curly_brace\nstmts -> •stmts stmt\ntype -> •basic\ndecls -> decls •decl\nstmts -> •\n"]
5["stmt -> •if ( bool ) stmt\nstmt -> •block\nstmt -> •break ;\nblock -> •left_curly_brace decls stmts right_curly_brace\nstmt -> •if ( bool ) stmt else stmt\nstmt -> •do stmt while ( bool ) ;\nstmt -> •while ( bool ) stmt\nstmt -> •loc = bool ;\nloc -> •id\nloc -> •loc [ num ]\nstmts -> stmts •stmt\nblock -> left_curly_brace decls stmts •right_curly_brace\n"]
6["block -> left_curly_brace decls stmts right_curly_brace•\n"]
7["stmt -> block•\n"]
8["stmt -> break •;\n"]
9["stmt -> break ;•\n"]
10["stmts -> stmts stmt•\n"]
11["stmt -> if •( bool ) stmt else stmt\nstmt -> if •( bool ) stmt\n"]
12["expr -> •term\nloc -> •loc [ num ]\nfactor -> •true\nstmt -> if ( •bool ) stmt else stmt\nrel -> •expr <= expr\nfactor -> •false\nrel -> •expr >= expr\nbool -> •join\nfactor -> •( bool )\nrel -> •expr\nexpr -> •expr - term\nterm -> •term / unary\nfactor -> •loc\nrel -> •expr > expr\nequality -> •equality == rel\nexpr -> •expr + term\nunary -> •! unary\nstmt -> if ( •bool ) stmt\nequality -> •rel\nterm -> •term * unary\nterm -> •unary\nfactor -> •real\njoin -> •join && equality\nunary -> •- unary\nloc -> •id\nunary -> •factor\nfactor -> •num\nequality -> •equality != rel\nbool -> •bool || join\nrel -> •expr < expr\njoin -> •equality\n"]
13["term -> unary•\n"]
14["equality -> equality •!= rel\nequality -> equality •== rel\njoin -> equality•\n"]
15["factor -> •num\nrel -> •expr < expr\nexpr -> •term\nrel -> •expr <= expr\nrel -> •expr > expr\nfactor -> •real\nterm -> •unary\nrel -> •expr >= expr\nterm -> •term / unary\nunary -> •factor\nexpr -> •expr - term\nexpr -> •expr + term\nloc -> •id\nloc -> •loc [ num ]\nrel -> •expr\nunary -> •- unary\nfactor -> •false\nunary -> •! unary\nequality -> equality == •rel\nterm -> •term * unary\nfactor -> •true\nfactor -> •loc\nfactor -> •( bool )\n"]
16["term -> term •/ unary\nterm -> term •* unary\nexpr -> term•\n"]
17["unary -> •! unary\nfactor -> •loc\nfactor -> •true\nunary -> •- unary\nloc -> •id\nfactor -> •( bool )\nfactor -> •false\nfactor -> •num\nloc -> •loc [ num ]\nterm -> term * •unary\nfactor -> •real\nunary -> •factor\n"]
18["factor -> true•\n"]
19["term -> •term * unary\nrel -> •expr <= expr\nexpr -> •expr + term\nrel -> •expr > expr\nbool -> •bool || join\nloc -> •loc [ num ]\nexpr -> •term\nfactor -> •( bool )\nequality -> •equality == rel\nunary -> •factor\nfactor -> •true\nterm -> •term / unary\nfactor -> •real\njoin -> •equality\nbool -> •join\nrel -> •expr\nexpr -> •expr - term\nfactor -> ( •bool )\nfactor -> •loc\nunary -> •! unary\nequality -> •equality != rel\nfactor -> •false\nfactor -> •num\nterm -> •unary\njoin -> •join && equality\nunary -> •- unary\nequality -> •rel\nrel -> •expr >= expr\nloc -> •id\nrel -> •expr < expr\n"]
20["bool -> bool •|| join\nfactor -> ( bool •)\n"]
21["rel -> •expr <= expr\nbool -> bool || •join\nunary -> •! unary\nloc -> •id\nunary -> •factor\nequality -> •rel\nexpr -> •expr - term\nloc -> •loc [ num ]\njoin -> •join && equality\nfactor -> •num\nfactor -> •( bool )\nfactor -> •real\nequality -> •equality != rel\nrel -> •expr >= expr\nterm -> •term * unary\nfactor -> •false\njoin -> •equality\nrel -> •expr < expr\nterm -> •term / unary\nfactor -> •loc\nexpr -> •term\nrel -> •expr\nequality -> •equality == rel\nexpr -> •expr + term\nrel -> •expr > expr\nfactor -> •true\nterm -> •unary\nunary -> •- unary\n"]
22["equality -> rel•\n"]
23["loc -> loc •[ num ]\nfactor -> loc•\n"]
24["loc -> loc [ •num ]\n"]
25["loc -> loc [ num •]\n"]
26["loc -> loc [ num ]•\n"]
27["loc -> id•\n"]
28["factor -> num•\n"]
29["factor -> real•\n"]
30["factor -> false•\n"]
31["rel -> expr •< expr\nexpr -> expr •+ term\nrel -> expr •<= expr\nrel -> expr•\nrel -> expr •> expr\nexpr -> expr •- term\nrel -> expr •>= expr\n"]
32["term -> •term * unary\nterm -> •term / unary\nunary -> •! unary\nfactor -> •true\nloc -> •id\nrel -> expr < •expr\nfactor -> •false\nloc -> •loc [ num ]\nunary -> •factor\nexpr -> •expr + term\nterm -> •unary\nfactor -> •real\nfactor -> •( bool )\nfactor -> •loc\nfactor -> •num\nexpr -> •term\nexpr -> •expr - term\nunary -> •- unary\n"]
33["loc -> •loc [ num ]\nunary -> •factor\nunary -> ! •unary\nfactor -> •true\nunary -> •! unary\nunary -> •- unary\nfactor -> •( bool )\nfactor -> •false\nloc -> •id\nfactor -> •loc\nfactor -> •real\nfactor -> •num\n"]
34["unary -> factor•\n"]
35["unary -> •! unary\nunary -> - •unary\nloc -> •loc [ num ]\nfactor -> •real\nunary -> •factor\nloc -> •id\nfactor -> •( bool )\nfactor -> •num\nunary -> •- unary\nfactor -> •loc\nfactor -> •false\nfactor -> •true\n"]
36["unary -> - unary•\n"]
37["unary -> ! unary•\n"]
38["expr -> expr •+ term\nrel -> expr < expr•\nexpr -> expr •- term\n"]
39["factor -> •loc\nterm -> •term * unary\nunary -> •- unary\nunary -> •! unary\nfactor -> •num\nunary -> •factor\nloc -> •id\nloc -> •loc [ num ]\nfactor -> •false\nterm -> •unary\nexpr -> expr + •term\nfactor -> •( bool )\nfactor -> •real\nterm -> •term / unary\nfactor -> •true\n"]
40["expr -> expr + term•\nterm -> term •* unary\nterm -> term •/ unary\n"]
41["unary -> •! unary\nloc -> •id\nfactor -> •false\nfactor -> •real\nfactor -> •true\nunary -> •factor\nunary -> •- unary\nfactor -> •num\nfactor -> •( bool )\nterm -> term / •unary\nfactor -> •loc\nloc -> •loc [ num ]\n"]
42["term -> term / unary•\n"]
43["factor -> •real\nexpr -> expr - •term\nfactor -> •num\nfactor -> •loc\nfactor -> •false\nunary -> •- unary\nfactor -> •true\nterm -> •unary\nterm -> •term / unary\nunary -> •! unary\nterm -> •term * unary\nunary -> •factor\nloc -> •id\nfactor -> •( bool )\nloc -> •loc [ num ]\n"]
44["term -> term •* unary\nexpr -> expr - term•\nterm -> term •/ unary\n"]
45["loc -> •id\nterm -> •term * unary\nfactor -> •false\nrel -> expr >= •expr\nterm -> •unary\nloc -> •loc [ num ]\nfactor -> •real\nunary -> •- unary\nexpr -> •expr - term\nfactor -> •loc\nfactor -> •true\nexpr -> •expr + term\nfactor -> •( bool )\nterm -> •term / unary\nunary -> •! unary\nunary -> •factor\nexpr -> •term\nfactor -> •num\n"]
46["expr -> expr •- term\nrel -> expr >= expr•\nexpr -> expr •+ term\n"]
47["rel -> expr <= •expr\nunary -> •factor\nloc -> •id\nfactor -> •( bool )\nloc -> •loc [ num ]\nfactor -> •true\nfactor -> •false\nterm -> •term / unary\nfactor -> •num\nterm -> •term * unary\nunary -> •- unary\nunary -> •! unary\nexpr -> •expr - term\nfactor -> •loc\nexpr -> •expr + term\nexpr -> •term\nfactor -> •real\nterm -> •unary\n"]
48["expr -> expr •+ term\nrel -> expr <= expr•\nexpr -> expr •- term\n"]
49["rel -> expr > •expr\nterm -> •unary\nexpr -> •expr - term\nfactor -> •( bool )\nterm -> •term / unary\nexpr -> •term\nunary -> •- unary\nfactor -> •num\nfactor -> •false\nfactor -> •real\nfactor -> •loc\nloc -> •loc [ num ]\nloc -> •id\nunary -> •! unary\nterm -> •term * unary\nfactor -> •true\nunary -> •factor\nexpr -> •expr + term\n"]
50["rel -> expr > expr•\nexpr -> expr •- term\nexpr -> expr •+ term\n"]
51["join -> join •&& equality\nbool -> bool || join•\n"]
52["unary -> •factor\nfactor -> •true\njoin -> join && •equality\nfactor -> •false\nequality -> •rel\nequality -> •equality == rel\nrel -> •expr >= expr\nfactor -> •real\nrel -> •expr > expr\nunary -> •- unary\nrel -> •expr\nterm -> •term * unary\nexpr -> •expr - term\nterm -> •term / unary\nfactor -> •num\nequality -> •equality != rel\nrel -> •expr <= expr\nunary -> •! unary\nexpr -> •expr + term\nterm -> •unary\nloc -> •id\nfactor -> •loc\nrel -> •expr < expr\nexpr -> •term\nloc -> •loc [ num ]\nfactor -> •( bool )\n"]
53["equality -> equality •!= rel\njoin -> join && equality•\nequality -> equality •== rel\n"]
54["term -> •term * unary\nexpr -> •term\nexpr -> •expr - term\nterm -> •term / unary\nrel -> •expr < expr\nfactor -> •false\nrel -> •expr >= expr\nrel -> •expr <= expr\nequality -> equality != •rel\nrel -> •expr > expr\nunary -> •! unary\nterm -> •unary\nrel -> •expr\nunary -> •- unary\nfactor -> •real\nexpr -> •expr + term\nfactor -> •true\nfactor -> •num\nloc -> •id\nloc -> •loc [ num ]\nfactor -> •loc\nunary -> •factor\nfactor -> •( bool )\n"]
55["equality -> equality != rel•\n"]
56["factor -> ( bool )•\n"]
57["join -> join •&& equality\nbool -> join•\n"]
58["term -> term * unary•\n"]
59["equality -> equality == rel•\n"]
60["stmt -> if ( bool •) stmt\nstmt -> if ( bool •) stmt else stmt\nbool -> bool •|| join\n"]
61["stmt -> •break ;\nstmt -> •loc = bool ;\nstmt -> if ( bool ) •stmt else stmt\nstmt -> •while ( bool ) stmt\nblock -> •left_curly_brace decls stmts right_curly_brace\nloc -> •id\nloc -> •loc [ num ]\nstmt -> •if ( bool ) stmt\nstmt -> if ( bool ) •stmt\nstmt -> •if ( bool ) stmt else stmt\nstmt -> •do stmt while ( bool ) ;\nstmt -> •block\n"]
62["stmt -> while •( bool ) stmt\n"]
63["expr -> •expr + term\nterm -> •unary\nloc -> •id\nfactor -> •num\nrel -> •expr <= expr\nrel -> •expr\nexpr -> •expr - term\nrel -> •expr >= expr\nfactor -> •loc\nunary -> •- unary\nterm -> •term / unary\nequality -> •equality == rel\njoin -> •equality\nbool -> •join\nunary -> •factor\nbool -> •bool || join\nstmt -> while ( •bool ) stmt\nfactor -> •real\nloc -> •loc [ num ]\nrel -> •expr < expr\nterm -> •term * unary\nrel -> •expr > expr\nunary -> •! unary\nexpr -> •term\njoin -> •join && equality\nequality -> •rel\nfactor -> •( bool )\nequality -> •equality != rel\nfactor -> •true\nfactor -> •false\n"]
64["bool -> bool •|| join\nstmt -> while ( bool •) stmt\n"]
65["stmt -> •loc = bool ;\nstmt -> •while ( bool ) stmt\nstmt -> •block\nstmt -> while ( bool ) •stmt\nstmt -> •do stmt while ( bool ) ;\nloc -> •loc [ num ]\nstmt -> •if ( bool ) stmt\nstmt -> •if ( bool ) stmt else stmt\nloc -> •id\nstmt -> •break ;\nblock -> •left_curly_brace decls stmts right_curly_brace\n"]
66["stmt -> loc •= bool ;\nloc -> loc •[ num ]\n"]
67["rel -> •expr < expr\nequality -> •rel\nfactor -> •true\nrel -> •expr > expr\nterm -> •term / unary\njoin -> •equality\nrel -> •expr <= expr\nrel -> •expr\nfactor -> •real\nunary -> •factor\nfactor -> •false\nfactor -> •( bool )\nloc -> •id\nloc -> •loc [ num ]\nequality -> •equality != rel\nbool -> •join\nrel -> •expr >= expr\nfactor -> •num\nbool -> •bool || join\nexpr -> •term\nunary -> •! unary\nequality -> •equality == rel\njoin -> •join && equality\nexpr -> •expr + term\nexpr -> •expr - term\nterm -> •unary\nstmt -> loc = •bool ;\nunary -> •- unary\nfactor -> •loc\nterm -> •term * unary\n"]
68["stmt -> loc = bool •;\nbool -> bool •|| join\n"]
69["stmt -> loc = bool ;•\n"]
70["stmt -> while ( bool ) stmt•\n"]
71["stmt -> •break ;\nstmt -> •loc = bool ;\nstmt -> •block\nstmt -> •do stmt while ( bool ) ;\nloc -> •id\nstmt -> do •stmt while ( bool ) ;\nstmt -> •while ( bool ) stmt\nblock -> •left_curly_brace decls stmts right_curly_brace\nloc -> •loc [ num ]\nstmt -> •if ( bool ) stmt else stmt\nstmt -> •if ( bool ) stmt\n"]
72["stmt -> do stmt •while ( bool ) ;\n"]
73["stmt -> do stmt while •( bool ) ;\n"]
74["term -> •term * unary\nfactor -> •false\nfactor -> •( bool )\nstmt -> do stmt while ( •bool ) ;\nequality -> •equality == rel\nloc -> •id\nterm -> •unary\nbool -> •join\nfactor -> •loc\nrel -> •expr > expr\nunary -> •- unary\nunary -> •factor\nbool -> •bool || join\nexpr -> •expr - term\nunary -> •! unary\nequality -> •rel\nrel -> •expr >= expr\nexpr -> •term\njoin -> •join && equality\nrel -> •expr\nloc -> •loc [ num ]\nexpr -> •expr + term\nfactor -> •num\nfactor -> •real\nrel -> •expr <= expr\nrel -> •expr < expr\nequality -> •equality != rel\njoin -> •equality\nfactor -> •true\nterm -> •term / unary\n"]
75["bool -> bool •|| join\nstmt -> do stmt while ( bool •) ;\n"]
76["stmt -> do stmt while ( bool ) •;\n"]
77["stmt -> do stmt while ( bool ) ;•\n"]
78["stmt -> if ( bool ) stmt •else stmt\nstmt -> if ( bool ) stmt•\n"]
79["stmt -> •while ( bool ) stmt\nstmt -> •do stmt while ( bool ) ;\nloc -> •id\nstmt -> •break ;\nstmt -> •block\nstmt -> •loc = bool ;\nloc -> •loc [ num ]\nstmt -> •if ( bool ) stmt else stmt\nstmt -> if ( bool ) stmt else •stmt\nstmt -> •if ( bool ) stmt\nblock -> •left_curly_brace decls stmts right_curly_brace\n"]
80["stmt -> if ( bool ) stmt else stmt•\n"]
81["type -> type •[ num ]\ndecl -> type •id ;\n"]
82["type -> type [ •num ]\n"]
83["type -> type [ num •]\n"]
84["type -> type [ num ]•\n"]
85["decl -> type id •;\n"]
86["decl -> type id ;•\n"]
87["type -> basic•\n"]
88["decls -> decls decl•\n"]
89["S -> program•\n"]

0--"block"-->1
0--"S"-->2
5--"right_curly_brace"-->6
5--"block"-->7
8--";"-->9
5--"break"-->8
5--"stmt"-->10
12--"unary"-->13
17--"true"-->18
19--"true"-->18
21--"rel"-->22
25--"]"-->26
24--"num"-->25
23--"["-->24
21--"loc"-->23
21--"id"-->27
21--"num"-->28
21--"real"-->29
21--"false"-->30
21--"equality"-->14
21--"unary"-->13
32--"id"-->27
32--"true"-->18
33--"factor"-->34
35--"num"-->28
35--"("-->19
35--"-"-->35
35--"id"-->27
35--"real"-->29
35--"unary"-->36
35--"true"-->18
35--"factor"-->34
35--"loc"-->23
35--"false"-->30
35--"!"-->33
33--"-"-->35
33--"false"-->30
33--"!"-->33
33--"real"-->29
33--"loc"-->23
33--"num"-->28
33--"unary"-->37
33--"("-->19
33--"true"-->18
33--"id"-->27
32--"!"-->33
32--"loc"-->23
32--"real"-->29
32--"term"-->16
32--"factor"-->34
32--"num"-->28
39--"loc"-->23
39--"("-->19
40--"*"-->17
41--"false"-->30
41--"real"-->29
41--"("-->19
41--"loc"-->23
41--"unary"-->42
41--"factor"-->34
41--"!"-->33
41--"-"-->35
41--"true"-->18
41--"num"-->28
41--"id"-->27
40--"/"-->41
39--"term"-->40
39--"false"-->30
39--"true"-->18
39--"num"-->28
39--"!"-->33
39--"-"-->35
39--"real"-->29
39--"unary"-->13
39--"id"-->27
39--"factor"-->34
38--"+"-->39
43--"!"-->33
43--"-"-->35
43--"unary"-->13
43--"num"-->28
43--"loc"-->23
43--"factor"-->34
43--"id"-->27
44--"/"-->41
44--"*"-->17
43--"term"-->44
43--"true"-->18
43--"real"-->29
43--"false"-->30
43--"("-->19
38--"-"-->43
32--"expr"-->38
32--"false"-->30
32--"unary"-->13
32--"("-->19
32--"-"-->35
31--"<"-->32
31--"+"-->39
31--"-"-->43
45--"false"-->30
45--"("-->19
45--"factor"-->34
45--"term"-->16
45--"-"-->35
45--"unary"-->13
45--"num"-->28
45--"true"-->18
45--"!"-->33
46--"+"-->39
46--"-"-->43
45--"expr"-->46
45--"real"-->29
45--"id"-->27
45--"loc"-->23
31--">="-->45
47--"!"-->33
47--"unary"-->13
47--"id"-->27
47--"real"-->29
48--"-"-->43
48--"+"-->39
47--"expr"-->48
47--"num"-->28
47--"("-->19
47--"true"-->18
47--"-"-->35
47--"factor"-->34
47--"loc"-->23
47--"false"-->30
47--"term"-->16
31--"<="-->47
49--"("-->19
49--"false"-->30
49--"true"-->18
49--"factor"-->34
49--"unary"-->13
49--"-"-->35
49--"num"-->28
50--"+"-->39
50--"-"-->43
49--"expr"-->50
49--"!"-->33
49--"id"-->27
49--"term"-->16
49--"real"-->29
49--"loc"-->23
31--">"-->49
21--"expr"-->31
21--"term"-->16
21--"true"-->18
21--"factor"-->34
21--"!"-->33
21--"("-->19
21--"-"-->35
52--"!"-->33
52--"real"-->29
52--"expr"-->31
52--"num"-->28
52--"term"-->16
52--"factor"-->34
52--"true"-->18
52--"false"-->30
52--"unary"-->13
52--"("-->19
52--"id"-->27
53--"=="-->15
54--"!"-->33
54--"unary"-->13
54--"true"-->18
54--"num"-->28
54--"expr"-->31
54--"loc"-->23
54--"term"-->16
54--"factor"-->34
54--"real"-->29
54--"id"-->27
54--"-"-->35
54--"("-->19
54--"rel"-->55
54--"false"-->30
53--"!="-->54
52--"equality"-->53
52--"loc"-->23
52--"rel"-->22
52--"-"-->35
51--"&&"-->52
21--"join"-->51
20--"||"-->21
20--")"-->56
19--"bool"-->20
19--"equality"-->14
19--"("-->19
19--"-"-->35
19--"id"-->27
19--"real"-->29
19--"rel"-->22
19--"unary"-->13
19--"term"-->16
19--"factor"-->34
19--"false"-->30
57--"&&"-->52
19--"join"-->57
19--"!"-->33
19--"num"-->28
19--"loc"-->23
19--"expr"-->31
17--"("-->19
17--"num"-->28
17--"!"-->33
17--"unary"-->58
17--"factor"-->34
17--"loc"-->23
17--"false"-->30
17--"-"-->35
17--"id"-->27
17--"real"-->29
16--"*"-->17
16--"/"-->41
15--"term"-->16
15--"real"-->29
15--"id"-->27
15--"unary"-->13
15--"!"-->33
15--"expr"-->31
15--"rel"-->59
15--"true"-->18
15--"factor"-->34
15--"-"-->35
15--"("-->19
15--"loc"-->23
15--"num"-->28
15--"false"-->30
14--"=="-->15
14--"!="-->54
12--"equality"-->14
12--"join"-->57
12--"id"-->27
12--"!"-->33
12--"-"-->35
12--"loc"-->23
12--"term"-->16
12--"false"-->30
12--"real"-->29
12--"expr"-->31
12--"("-->19
63--"loc"-->23
63--"true"-->18
63--"num"-->28
63--"real"-->29
65--"block"-->7
65--"if"-->11
66--"["-->24
67--"loc"-->23
67--"unary"-->13
67--"id"-->27
67--"!"-->33
67--"-"-->35
67--"num"-->28
67--"term"-->16
68--";"-->69
68--"||"-->21
67--"bool"-->68
67--"equality"-->14
67--"factor"-->34
67--"join"-->57
67--"real"-->29
67--"expr"-->31
67--"false"-->30
67--"("-->19
67--"rel"-->22
67--"true"-->18
66--"="-->67
65--"loc"-->66
65--"stmt"-->70
65--"id"-->27
71--"while"-->62
71--"break"-->8
71--"block"-->7
74--"unary"-->13
74--"false"-->30
74--"real"-->29
74--"-"-->35
74--"!"-->33
74--"true"-->18
74--"term"-->16
74--"("-->19
74--"id"-->27
74--"join"-->57
74--"num"-->28
74--"expr"-->31
74--"equality"-->14
75--"||"-->21
76--";"-->77
75--")"-->76
74--"bool"-->75
74--"rel"-->22
74--"loc"-->23
74--"factor"-->34
73--"("-->74
72--"while"-->73
71--"stmt"-->72
71--"left_curly_brace"-->3
71--"do"-->71
71--"if"-->11
71--"loc"-->66
71--"id"-->27
65--"do"-->71
65--"left_curly_brace"-->3
65--"while"-->62
65--"break"-->8
64--")"-->65
64--"||"-->21
63--"bool"-->64
63--"-"-->35
63--"expr"-->31
63--"join"-->57
63--"id"-->27
63--"term"-->16
63--"factor"-->34
63--"rel"-->22
63--"unary"-->13
63--"false"-->30
63--"!"-->33
63--"equality"-->14
63--"("-->19
62--"("-->63
61--"while"-->62
61--"left_curly_brace"-->3
79--"break"-->8
79--"loc"-->66
79--"while"-->62
79--"block"-->7
79--"stmt"-->80
79--"left_curly_brace"-->3
79--"id"-->27
79--"if"-->11
79--"do"-->71
78--"else"-->79
61--"stmt"-->78
61--"break"-->8
61--"block"-->7
61--"loc"-->66
61--"id"-->27
61--"do"-->71
61--"if"-->11
60--")"-->61
60--"||"-->21
12--"bool"-->60
12--"true"-->18
12--"rel"-->22
12--"factor"-->34
12--"num"-->28
11--"("-->12
5--"if"-->11
5--"left_curly_brace"-->3
5--"do"-->71
5--"while"-->62
5--"loc"-->66
5--"id"-->27
4--"stmts"-->5
83--"]"-->84
82--"num"-->83
81--"["-->82
85--";"-->86
81--"id"-->85
4--"type"-->81
4--"basic"-->87
4--"decl"-->88
3--"decls"-->4
0--"left_curly_brace"-->3
0--"program"-->89

```

---

## Follow Set
```txt
+: ["false", "!", "(", "-", "id", "num", "real", "true"]
basic: ["id", "["]
;: ["if", "basic", "break", "else", "while", "right_curly_brace", "left_curly_brace", "id", "do"]
=: ["(", "!", "id", "real", "true", "num", "-", "false"]
num: ["+", ";", "]", "-", "!=", "/", ")", "*", "<", ">=", ">", "<=", "==", "&&", "||"]
<: ["real", "num", "-", "!", "id", "false", "(", "true"]
do: ["break", "id", "left_curly_brace", "do", "while", "if"]
__EPSILON__: ["do", "break", "right_curly_brace", "while", "basic", "if", "id", "left_curly_brace"]
(: ["id", "!", "-", "real", "true", "(", "num", "false"]
rel: [";", "!=", ")", "&&", "==", "||"]
!=: ["real", "(", "!", "false", "true", "id", "num", "-"]
>: ["(", "id", "true", "num", "-", "real", "false", "!"]
false: ["<=", ">", "||", "-", "+", "/", ">=", "<", "*", "==", ")", ";", "!=", "&&"]
__DUMMY_START__: ["__$__"]
/: ["num", "-", "real", "true", "!", "false", "(", "id"]
true: [">", ")", "+", "*", "||", "/", ">=", ";", "&&", "<=", "!=", "<", "==", "-"]
unary: ["-", ">", "+", ";", "/", "&&", "||", "*", "==", ")", "!=", "<=", "<", ">="]
id: [";", "&&", ">=", "*", "/", "[", ")", "-", "||", "==", ">", "=", "<", "+", "<=", "!="]
while: ["("]
loc: ["==", "<", ")", "/", "+", "-", ">=", "<=", "&&", "*", "!=", "=", "[", ";", "||", ">"]
type: ["[", "id"]
]: ["=", "-", ">=", "*", ";", "||", "<", "<=", ">", "+", "&&", "!=", "/", "==", "[", "id", ")"]
break: [";"]
==: ["(", "num", "id", "true", "-", "!", "real", "false"]
expr: [">", "-", "<", "<=", "||", ">=", "==", "!=", ")", "&&", ";", "+"]
[: ["num"]
program: ["__$__"]
left_curly_brace: ["basic", "break", "left_curly_brace", "do", "if", "while", "right_curly_brace", "id"]
__$__: []
stmt: ["do", "else", "while", "if", "right_curly_brace", "id", "break", "left_curly_brace"]
right_curly_brace: ["do", "if", "else", "left_curly_brace", "id", "right_curly_brace", "while", "break", "__$__"]
if: ["("]
decl: ["break", "while", "basic", "do", "left_curly_brace", "right_curly_brace", "id", "if"]
): ["==", ">=", "+", "while", ">", ")", "*", "do", "&&", "id", "-", "||", "<=", "/", "<", "if", "break", "!=", "left_curly_brace", ";"]
else: ["do", "left_curly_brace", "id", "while", "if", "break"]
||: ["real", "!", "id", "(", "false", "-", "true", "num"]
join: [";", ")", "&&", "||"]
&&: ["(", "false", "num", "!", "-", "id", "real", "true"]
S: ["__$__"]
-: ["false", "(", "num", "-", "!", "id", "true", "real"]
*: ["-", "id", "real", "(", "false", "!", "true", "num"]
decls: ["break", "left_curly_brace", "right_curly_brace", "id", "basic", "do", "while", "if"]
!: ["num", "false", "!", "(", "true", "real", "id", "-"]
>=: ["-", "!", "id", "false", "(", "num", "real", "true"]
equality: ["||", "&&", "!=", "==", ";", ")"]
factor: ["-", "<", ")", "==", "*", "&&", "+", "/", ">=", "<=", ">", "!=", ";", "||"]
term: [">=", ")", "||", ";", "+", "!=", "/", "&&", "==", "-", "<=", ">", "*", "<"]
<=: ["(", "num", "false", "real", "true", "!", "id", "-"]
block: ["else", "break", "if", "do", "left_curly_brace", "id", "__$__", "while", "right_curly_brace"]
real: ["==", "+", "<=", ">=", "*", ")", ";", "||", "-", "/", "<", "!=", "&&", ">"]
bool: [")", "||", ";"]
stmts: ["right_curly_brace", "if", "id", "break", "do", "while", "left_curly_brace"]
```

---
## Action Table
```txt
State 0:
S: Shift(2)
__$__: Accept
block: Shift(1)
left_curly_brace: Shift(3)
program: Shift(89)
===================
State 1:
__$__: Reduce(ReduceDerivation { left: "program", right: ["block"] })
===================
State 2:
__$__: Reduce(ReduceDerivation { left: "__DUMMY_START__", right: ["S"] })
===================
State 3:
while: Reduce(ReduceDerivation { left: "decls", right: [] })
if: Reduce(ReduceDerivation { left: "decls", right: [] })
break: Reduce(ReduceDerivation { left: "decls", right: [] })
decls: Shift(4)
do: Reduce(ReduceDerivation { left: "decls", right: [] })
id: Reduce(ReduceDerivation { left: "decls", right: [] })
basic: Reduce(ReduceDerivation { left: "decls", right: [] })
left_curly_brace: Reduce(ReduceDerivation { left: "decls", right: [] })
right_curly_brace: Reduce(ReduceDerivation { left: "decls", right: [] })
===================
State 4:
id: Reduce(ReduceDerivation { left: "stmts", right: [] })
do: Reduce(ReduceDerivation { left: "stmts", right: [] })
basic: Shift(87)
break: Reduce(ReduceDerivation { left: "stmts", right: [] })
while: Reduce(ReduceDerivation { left: "stmts", right: [] })
right_curly_brace: Reduce(ReduceDerivation { left: "stmts", right: [] })
stmts: Shift(5)
decl: Shift(88)
left_curly_brace: Reduce(ReduceDerivation { left: "stmts", right: [] })
if: Reduce(ReduceDerivation { left: "stmts", right: [] })
type: Shift(81)
===================
State 5:
if: Shift(11)
break: Shift(8)
left_curly_brace: Shift(3)
block: Shift(7)
right_curly_brace: Shift(6)
do: Shift(71)
while: Shift(62)
loc: Shift(66)
stmt: Shift(10)
id: Shift(27)
===================
State 6:
__$__: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
right_curly_brace: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
left_curly_brace: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
else: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
while: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
break: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
do: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
id: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
if: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
===================
State 7:
else: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
right_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
while: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
if: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
id: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
do: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
break: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
===================
State 8:
;: Shift(9)
===================
State 9:
left_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
while: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
id: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
right_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
break: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
else: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
do: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
if: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
===================
State 10:
while: Reduce(ReduceDerivation { left: "stmts", right: ["stmts", "stmt"] })
right_curly_brace: Reduce(ReduceDerivation { left: "stmts", right: ["stmts", "stmt"] })
id: Reduce(ReduceDerivation { left: "stmts", right: ["stmts", "stmt"] })
break: Reduce(ReduceDerivation { left: "stmts", right: ["stmts", "stmt"] })
do: Reduce(ReduceDerivation { left: "stmts", right: ["stmts", "stmt"] })
if: Reduce(ReduceDerivation { left: "stmts", right: ["stmts", "stmt"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmts", right: ["stmts", "stmt"] })
===================
State 11:
(: Shift(12)
===================
State 12:
false: Shift(30)
real: Shift(29)
!: Shift(33)
term: Shift(16)
bool: Shift(60)
rel: Shift(22)
num: Shift(28)
(: Shift(19)
id: Shift(27)
-: Shift(35)
unary: Shift(13)
loc: Shift(23)
join: Shift(57)
true: Shift(18)
factor: Shift(34)
equality: Shift(14)
expr: Shift(31)
===================
State 13:
;: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
+: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
||: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
/: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
==: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
!=: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
<=: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
>: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
>=: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
*: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
&&: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
-: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
<: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
): Reduce(ReduceDerivation { left: "term", right: ["unary"] })
===================
State 14:
&&: Reduce(ReduceDerivation { left: "join", right: ["equality"] })
==: Shift(15)
;: Reduce(ReduceDerivation { left: "join", right: ["equality"] })
||: Reduce(ReduceDerivation { left: "join", right: ["equality"] })
!=: Shift(54)
): Reduce(ReduceDerivation { left: "join", right: ["equality"] })
===================
State 15:
real: Shift(29)
(: Shift(19)
expr: Shift(31)
true: Shift(18)
factor: Shift(34)
!: Shift(33)
false: Shift(30)
rel: Shift(59)
-: Shift(35)
id: Shift(27)
loc: Shift(23)
num: Shift(28)
unary: Shift(13)
term: Shift(16)
===================
State 16:
||: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
<=: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
!=: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
&&: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
==: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
>=: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
+: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
>: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
;: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
): Reduce(ReduceDerivation { left: "expr", right: ["term"] })
/: Shift(41)
-: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
*: Shift(17)
<: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
===================
State 17:
id: Shift(27)
(: Shift(19)
unary: Shift(58)
loc: Shift(23)
factor: Shift(34)
-: Shift(35)
false: Shift(30)
true: Shift(18)
real: Shift(29)
!: Shift(33)
num: Shift(28)
===================
State 18:
;: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
!=: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
*: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
/: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
>: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
||: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
<: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
+: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
==: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
-: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
&&: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
>=: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
<=: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
): Reduce(ReduceDerivation { left: "factor", right: ["true"] })
===================
State 19:
factor: Shift(34)
bool: Shift(20)
true: Shift(18)
term: Shift(16)
false: Shift(30)
-: Shift(35)
!: Shift(33)
id: Shift(27)
num: Shift(28)
rel: Shift(22)
real: Shift(29)
loc: Shift(23)
(: Shift(19)
expr: Shift(31)
join: Shift(57)
equality: Shift(14)
unary: Shift(13)
===================
State 20:
||: Shift(21)
): Shift(56)
===================
State 21:
expr: Shift(31)
loc: Shift(23)
real: Shift(29)
(: Shift(19)
!: Shift(33)
-: Shift(35)
false: Shift(30)
factor: Shift(34)
join: Shift(51)
id: Shift(27)
true: Shift(18)
num: Shift(28)
equality: Shift(14)
unary: Shift(13)
term: Shift(16)
rel: Shift(22)
===================
State 22:
): Reduce(ReduceDerivation { left: "equality", right: ["rel"] })
==: Reduce(ReduceDerivation { left: "equality", right: ["rel"] })
&&: Reduce(ReduceDerivation { left: "equality", right: ["rel"] })
||: Reduce(ReduceDerivation { left: "equality", right: ["rel"] })
!=: Reduce(ReduceDerivation { left: "equality", right: ["rel"] })
;: Reduce(ReduceDerivation { left: "equality", right: ["rel"] })
===================
State 23:
>: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
;: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
>=: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
==: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
&&: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
/: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
<=: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
*: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
-: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
!=: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
): Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
<: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
[: Shift(24)
+: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
||: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
===================
State 24:
num: Shift(25)
===================
State 25:
]: Shift(26)
===================
State 26:
+: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
&&: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
!=: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
=: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
>=: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
||: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
>: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
*: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
[: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
): Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
-: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
==: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
<: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
;: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
/: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
<=: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
===================
State 27:
+: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
<=: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
||: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
=: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
!=: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
<: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
&&: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
/: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
==: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
>=: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
;: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
>: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
[: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
): Reduce(ReduceDerivation { left: "loc", right: ["id"] })
*: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
-: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
===================
State 28:
!=: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
<: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
+: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
/: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
-: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
): Reduce(ReduceDerivation { left: "factor", right: ["num"] })
==: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
>=: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
;: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
||: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
&&: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
*: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
<=: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
>: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
===================
State 29:
&&: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
/: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
<: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
!=: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
>=: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
;: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
||: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
): Reduce(ReduceDerivation { left: "factor", right: ["real"] })
-: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
*: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
+: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
==: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
>: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
<=: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
===================
State 30:
<=: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
||: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
;: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
<: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
): Reduce(ReduceDerivation { left: "factor", right: ["false"] })
==: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
*: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
-: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
>: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
>=: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
&&: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
+: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
!=: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
/: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
===================
State 31:
&&: Reduce(ReduceDerivation { left: "rel", right: ["expr"] })
): Reduce(ReduceDerivation { left: "rel", right: ["expr"] })
<: Shift(32)
>=: Shift(45)
<=: Shift(47)
>: Shift(49)
;: Reduce(ReduceDerivation { left: "rel", right: ["expr"] })
!=: Reduce(ReduceDerivation { left: "rel", right: ["expr"] })
==: Reduce(ReduceDerivation { left: "rel", right: ["expr"] })
||: Reduce(ReduceDerivation { left: "rel", right: ["expr"] })
-: Shift(43)
+: Shift(39)
===================
State 32:
term: Shift(16)
id: Shift(27)
false: Shift(30)
unary: Shift(13)
-: Shift(35)
expr: Shift(38)
(: Shift(19)
true: Shift(18)
loc: Shift(23)
!: Shift(33)
real: Shift(29)
num: Shift(28)
factor: Shift(34)
===================
State 33:
!: Shift(33)
id: Shift(27)
true: Shift(18)
loc: Shift(23)
-: Shift(35)
unary: Shift(37)
factor: Shift(34)
(: Shift(19)
false: Shift(30)
num: Shift(28)
real: Shift(29)
===================
State 34:
<: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
+: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
*: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
): Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
||: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
>: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
&&: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
==: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
!=: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
<=: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
;: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
-: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
>=: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
/: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
===================
State 35:
loc: Shift(23)
(: Shift(19)
false: Shift(30)
true: Shift(18)
!: Shift(33)
unary: Shift(36)
num: Shift(28)
-: Shift(35)
id: Shift(27)
real: Shift(29)
factor: Shift(34)
===================
State 36:
>=: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
>: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
||: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
;: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
/: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
*: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
-: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
==: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
<=: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
<: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
!=: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
&&: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
+: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
): Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
===================
State 37:
+: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
*: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
<: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
>=: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
-: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
/: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
>: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
==: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
;: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
): Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
!=: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
<=: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
&&: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
||: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
===================
State 38:
;: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<", "expr"] })
||: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<", "expr"] })
-: Shift(43)
&&: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<", "expr"] })
+: Shift(39)
!=: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<", "expr"] })
==: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<", "expr"] })
): Reduce(ReduceDerivation { left: "rel", right: ["expr", "<", "expr"] })
===================
State 39:
loc: Shift(23)
-: Shift(35)
term: Shift(40)
true: Shift(18)
unary: Shift(13)
id: Shift(27)
num: Shift(28)
(: Shift(19)
false: Shift(30)
!: Shift(33)
real: Shift(29)
factor: Shift(34)
===================
State 40:
*: Shift(17)
/: Shift(41)
==: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
||: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
!=: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
;: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
<: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
): Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
+: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
&&: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
>: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
>=: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
-: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
<=: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
===================
State 41:
real: Shift(29)
false: Shift(30)
unary: Shift(42)
-: Shift(35)
!: Shift(33)
loc: Shift(23)
true: Shift(18)
num: Shift(28)
id: Shift(27)
factor: Shift(34)
(: Shift(19)
===================
State 42:
==: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
+: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
;: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
/: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
<=: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
!=: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
&&: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
>=: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
>: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
-: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
||: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
<: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
*: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
): Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
===================
State 43:
factor: Shift(34)
id: Shift(27)
false: Shift(30)
unary: Shift(13)
-: Shift(35)
num: Shift(28)
loc: Shift(23)
term: Shift(44)
true: Shift(18)
(: Shift(19)
real: Shift(29)
!: Shift(33)
===================
State 44:
==: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
&&: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
;: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
||: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
<: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
>: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
!=: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
+: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
-: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
>=: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
): Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
<=: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
/: Shift(41)
*: Shift(17)
===================
State 45:
-: Shift(35)
loc: Shift(23)
unary: Shift(13)
!: Shift(33)
false: Shift(30)
num: Shift(28)
true: Shift(18)
factor: Shift(34)
expr: Shift(46)
(: Shift(19)
real: Shift(29)
term: Shift(16)
id: Shift(27)
===================
State 46:
==: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">=", "expr"] })
!=: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">=", "expr"] })
;: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">=", "expr"] })
||: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">=", "expr"] })
+: Shift(39)
-: Shift(43)
): Reduce(ReduceDerivation { left: "rel", right: ["expr", ">=", "expr"] })
&&: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">=", "expr"] })
===================
State 47:
loc: Shift(23)
false: Shift(30)
expr: Shift(48)
term: Shift(16)
num: Shift(28)
factor: Shift(34)
unary: Shift(13)
!: Shift(33)
real: Shift(29)
(: Shift(19)
id: Shift(27)
true: Shift(18)
-: Shift(35)
===================
State 48:
==: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<=", "expr"] })
!=: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<=", "expr"] })
+: Shift(39)
-: Shift(43)
;: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<=", "expr"] })
||: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<=", "expr"] })
&&: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<=", "expr"] })
): Reduce(ReduceDerivation { left: "rel", right: ["expr", "<=", "expr"] })
===================
State 49:
factor: Shift(34)
num: Shift(28)
id: Shift(27)
-: Shift(35)
false: Shift(30)
expr: Shift(50)
unary: Shift(13)
!: Shift(33)
(: Shift(19)
real: Shift(29)
term: Shift(16)
loc: Shift(23)
true: Shift(18)
===================
State 50:
-: Shift(43)
!=: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">", "expr"] })
+: Shift(39)
||: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">", "expr"] })
&&: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">", "expr"] })
==: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">", "expr"] })
): Reduce(ReduceDerivation { left: "rel", right: ["expr", ">", "expr"] })
;: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">", "expr"] })
===================
State 51:
||: Reduce(ReduceDerivation { left: "bool", right: ["bool", "||", "join"] })
;: Reduce(ReduceDerivation { left: "bool", right: ["bool", "||", "join"] })
&&: Shift(52)
): Reduce(ReduceDerivation { left: "bool", right: ["bool", "||", "join"] })
===================
State 52:
id: Shift(27)
true: Shift(18)
equality: Shift(53)
factor: Shift(34)
num: Shift(28)
real: Shift(29)
expr: Shift(31)
false: Shift(30)
(: Shift(19)
loc: Shift(23)
term: Shift(16)
unary: Shift(13)
-: Shift(35)
rel: Shift(22)
!: Shift(33)
===================
State 53:
;: Reduce(ReduceDerivation { left: "join", right: ["join", "&&", "equality"] })
==: Shift(15)
&&: Reduce(ReduceDerivation { left: "join", right: ["join", "&&", "equality"] })
||: Reduce(ReduceDerivation { left: "join", right: ["join", "&&", "equality"] })
): Reduce(ReduceDerivation { left: "join", right: ["join", "&&", "equality"] })
!=: Shift(54)
===================
State 54:
true: Shift(18)
rel: Shift(55)
false: Shift(30)
expr: Shift(31)
factor: Shift(34)
real: Shift(29)
-: Shift(35)
(: Shift(19)
num: Shift(28)
unary: Shift(13)
!: Shift(33)
loc: Shift(23)
term: Shift(16)
id: Shift(27)
===================
State 55:
||: Reduce(ReduceDerivation { left: "equality", right: ["equality", "!=", "rel"] })
): Reduce(ReduceDerivation { left: "equality", right: ["equality", "!=", "rel"] })
!=: Reduce(ReduceDerivation { left: "equality", right: ["equality", "!=", "rel"] })
&&: Reduce(ReduceDerivation { left: "equality", right: ["equality", "!=", "rel"] })
;: Reduce(ReduceDerivation { left: "equality", right: ["equality", "!=", "rel"] })
==: Reduce(ReduceDerivation { left: "equality", right: ["equality", "!=", "rel"] })
===================
State 56:
/: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
<=: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
>: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
;: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
<: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
==: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
>=: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
||: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
*: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
!=: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
-: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
+: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
): Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
&&: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
===================
State 57:
): Reduce(ReduceDerivation { left: "bool", right: ["join"] })
&&: Shift(52)
;: Reduce(ReduceDerivation { left: "bool", right: ["join"] })
||: Reduce(ReduceDerivation { left: "bool", right: ["join"] })
===================
State 58:
+: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
;: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
-: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
||: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
<=: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
*: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
): Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
>: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
&&: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
<: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
!=: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
==: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
/: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
>=: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
===================
State 59:
!=: Reduce(ReduceDerivation { left: "equality", right: ["equality", "==", "rel"] })
&&: Reduce(ReduceDerivation { left: "equality", right: ["equality", "==", "rel"] })
==: Reduce(ReduceDerivation { left: "equality", right: ["equality", "==", "rel"] })
||: Reduce(ReduceDerivation { left: "equality", right: ["equality", "==", "rel"] })
;: Reduce(ReduceDerivation { left: "equality", right: ["equality", "==", "rel"] })
): Reduce(ReduceDerivation { left: "equality", right: ["equality", "==", "rel"] })
===================
State 60:
): Shift(61)
||: Shift(21)
===================
State 61:
break: Shift(8)
id: Shift(27)
while: Shift(62)
stmt: Shift(78)
loc: Shift(66)
do: Shift(71)
if: Shift(11)
block: Shift(7)
left_curly_brace: Shift(3)
===================
State 62:
(: Shift(63)
===================
State 63:
equality: Shift(14)
-: Shift(35)
loc: Shift(23)
join: Shift(57)
id: Shift(27)
expr: Shift(31)
unary: Shift(13)
term: Shift(16)
num: Shift(28)
true: Shift(18)
real: Shift(29)
bool: Shift(64)
factor: Shift(34)
rel: Shift(22)
!: Shift(33)
(: Shift(19)
false: Shift(30)
===================
State 64:
||: Shift(21)
): Shift(65)
===================
State 65:
while: Shift(62)
if: Shift(11)
id: Shift(27)
loc: Shift(66)
stmt: Shift(70)
left_curly_brace: Shift(3)
do: Shift(71)
block: Shift(7)
break: Shift(8)
===================
State 66:
=: Shift(67)
[: Shift(24)
===================
State 67:
unary: Shift(13)
id: Shift(27)
equality: Shift(14)
!: Shift(33)
term: Shift(16)
(: Shift(19)
join: Shift(57)
num: Shift(28)
false: Shift(30)
true: Shift(18)
-: Shift(35)
loc: Shift(23)
bool: Shift(68)
rel: Shift(22)
expr: Shift(31)
factor: Shift(34)
real: Shift(29)
===================
State 68:
;: Shift(69)
||: Shift(21)
===================
State 69:
if: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
id: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
while: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
do: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
break: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
right_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
else: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
===================
State 70:
id: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
while: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
else: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
do: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
break: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
if: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
right_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
===================
State 71:
left_curly_brace: Shift(3)
id: Shift(27)
block: Shift(7)
if: Shift(11)
do: Shift(71)
while: Shift(62)
break: Shift(8)
loc: Shift(66)
stmt: Shift(72)
===================
State 72:
while: Shift(73)
===================
State 73:
(: Shift(74)
===================
State 74:
real: Shift(29)
factor: Shift(34)
false: Shift(30)
num: Shift(28)
unary: Shift(13)
id: Shift(27)
!: Shift(33)
true: Shift(18)
equality: Shift(14)
(: Shift(19)
join: Shift(57)
-: Shift(35)
term: Shift(16)
expr: Shift(31)
rel: Shift(22)
loc: Shift(23)
bool: Shift(75)
===================
State 75:
||: Shift(21)
): Shift(76)
===================
State 76:
;: Shift(77)
===================
State 77:
while: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
right_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
id: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
do: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
break: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
else: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
if: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
===================
State 78:
break: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
else: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
while: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
do: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
right_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
if: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
id: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
===================
State 79:
loc: Shift(66)
block: Shift(7)
break: Shift(8)
if: Shift(11)
do: Shift(71)
id: Shift(27)
while: Shift(62)
left_curly_brace: Shift(3)
stmt: Shift(80)
===================
State 80:
right_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
break: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
else: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
id: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
while: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
do: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
if: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
===================
State 81:
id: Shift(85)
[: Shift(82)
===================
State 82:
num: Shift(83)
===================
State 83:
]: Shift(84)
===================
State 84:
[: Reduce(ReduceDerivation { left: "type", right: ["type", "[", "num", "]"] })
id: Reduce(ReduceDerivation { left: "type", right: ["type", "[", "num", "]"] })
===================
State 85:
;: Shift(86)
===================
State 86:
id: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
if: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
right_curly_brace: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
break: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
do: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
left_curly_brace: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
while: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
basic: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
===================
State 87:
id: Reduce(ReduceDerivation { left: "type", right: ["basic"] })
[: Reduce(ReduceDerivation { left: "type", right: ["basic"] })
===================
State 88:
break: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
left_curly_brace: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
right_curly_brace: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
id: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
do: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
basic: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
while: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
if: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
===================
State 89:
__$__: Reduce(ReduceDerivation { left: "S", right: ["program"] })
===================

```
---
generated by rparser
RockRockWhite 2023
    