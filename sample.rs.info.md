
# Generated Info

## Base Info
- config_file: sample.rparser
- output_file: sample.rs
- time: 2023-05-19 21:14:29.282888 +08:00

---

## DFA Graph
```mermaid
graph LR
0["program -> •block\n__DUMMY_START__ -> •S\nblock -> •left_curly_brace decls stmts right_curly_brace\nS -> •program\n"]
1["stmt -> •block\ntype -> •basic\nblock -> •left_curly_brace decls stmts right_curly_brace\nloc -> •loc [ num ]\nstmt -> •if ( bool ) stmt\nstmt -> •do stmt while ( bool ) ;\nstmt -> •loc = bool ;\nblock -> left_curly_brace •decls stmts right_curly_brace\nstmts -> •stmts stmt\ndecls -> •decls decl\ntype -> •type [ num ]\nstmt -> •while ( bool ) stmt\nstmt -> •if ( bool ) stmt else stmt\nloc -> •id\ndecl -> •type id ;\nstmt -> •break ;\n"]
2["stmt -> •if ( bool ) stmt else stmt\nblock -> •left_curly_brace decls stmts right_curly_brace\ntype -> •type [ num ]\nloc -> •loc [ num ]\nloc -> •id\ndecl -> •type id ;\nstmt -> •if ( bool ) stmt\nstmt -> •block\nstmt -> •break ;\nstmt -> •do stmt while ( bool ) ;\nstmt -> •while ( bool ) stmt\nblock -> left_curly_brace decls •stmts right_curly_brace\ndecls -> decls •decl\nstmts -> •stmts stmt\ntype -> •basic\nstmt -> •loc = bool ;\n"]
3["stmt -> •loc = bool ;\nstmt -> •do stmt while ( bool ) ;\nstmt -> •block\nstmt -> •break ;\nloc -> •id\nstmt -> •while ( bool ) stmt\nstmt -> •if ( bool ) stmt else stmt\nloc -> •loc [ num ]\nblock -> •left_curly_brace decls stmts right_curly_brace\nstmt -> •if ( bool ) stmt\nstmts -> stmts •stmt\nblock -> left_curly_brace decls stmts •right_curly_brace\n"]
4["stmt -> if •( bool ) stmt\nstmt -> if •( bool ) stmt else stmt\n"]
5["rel -> •expr < expr\nbool -> •join\nunary -> •! unary\nloc -> •id\nequality -> •equality == rel\nfactor -> •( bool )\nequality -> •rel\nfactor -> •num\nstmt -> if ( •bool ) stmt else stmt\nterm -> •term / unary\nunary -> •factor\njoin -> •join && equality\nrel -> •expr\nterm -> •unary\nfactor -> •real\nequality -> •equality != rel\nstmt -> if ( •bool ) stmt\njoin -> •equality\nfactor -> •true\nrel -> •expr <= expr\nexpr -> •expr - term\nterm -> •term * unary\nfactor -> •false\nbool -> •bool || join\nrel -> •expr > expr\nrel -> •expr >= expr\nloc -> •loc [ num ]\nunary -> •- unary\nfactor -> •loc\nexpr -> •expr + term\nexpr -> •term\n"]
6["factor -> real•\n"]
7["unary -> •- unary\nfactor -> •num\nunary -> - •unary\nfactor -> •( bool )\nfactor -> •real\nunary -> •! unary\nfactor -> •true\nloc -> •loc [ num ]\nunary -> •factor\nfactor -> •false\nloc -> •id\nfactor -> •loc\n"]
8["unary -> - unary•\n"]
9["bool -> •bool || join\nrel -> •expr <= expr\nequality -> •equality == rel\nexpr -> •expr + term\nfactor -> •loc\nunary -> •factor\nbool -> •join\nloc -> •id\nfactor -> •( bool )\nfactor -> ( •bool )\nequality -> •rel\njoin -> •join && equality\nunary -> •! unary\nexpr -> •term\nrel -> •expr >= expr\nloc -> •loc [ num ]\nterm -> •unary\nfactor -> •num\nfactor -> •true\nterm -> •term / unary\njoin -> •equality\nfactor -> •real\nrel -> •expr < expr\nrel -> •expr > expr\nunary -> •- unary\nfactor -> •false\nexpr -> •expr - term\nterm -> •term * unary\nrel -> •expr\nequality -> •equality != rel\n"]
10["factor -> num•\n"]
11["factor -> true•\n"]
12["loc -> id•\n"]
13["equality -> rel•\n"]
14["expr -> term•\nterm -> term •* unary\nterm -> term •/ unary\n"]
15["loc -> •loc [ num ]\nunary -> •! unary\nfactor -> •num\nfactor -> •( bool )\nunary -> •- unary\nfactor -> •false\nfactor -> •loc\nfactor -> •true\nunary -> •factor\nterm -> term / •unary\nfactor -> •real\nloc -> •id\n"]
16["unary -> factor•\n"]
17["term -> term / unary•\n"]
18["factor -> loc•\nloc -> loc •[ num ]\n"]
19["loc -> loc [ •num ]\n"]
20["loc -> loc [ num •]\n"]
21["loc -> loc [ num ]•\n"]
22["factor -> false•\n"]
23["unary -> •- unary\nfactor -> •( bool )\nfactor -> •real\nunary -> •factor\nloc -> •id\nunary -> •! unary\nfactor -> •num\nunary -> ! •unary\nloc -> •loc [ num ]\nfactor -> •loc\nfactor -> •true\nfactor -> •false\n"]
24["unary -> ! unary•\n"]
25["factor -> •loc\nloc -> •id\nfactor -> •false\nloc -> •loc [ num ]\nfactor -> •num\nunary -> •! unary\nfactor -> •( bool )\nfactor -> •real\nunary -> •- unary\nfactor -> •true\nunary -> •factor\nterm -> term * •unary\n"]
26["term -> term * unary•\n"]
27["factor -> ( bool •)\nbool -> bool •|| join\n"]
28["factor -> ( bool )•\n"]
29["unary -> •factor\nexpr -> •term\nexpr -> •expr - term\nunary -> •- unary\njoin -> •equality\nfactor -> •( bool )\nequality -> •equality == rel\nfactor -> •loc\nbool -> bool || •join\nrel -> •expr <= expr\nrel -> •expr\nexpr -> •expr + term\nrel -> •expr >= expr\nfactor -> •true\nloc -> •id\nfactor -> •num\njoin -> •join && equality\nunary -> •! unary\nfactor -> •real\nrel -> •expr > expr\nloc -> •loc [ num ]\nfactor -> •false\nequality -> •equality != rel\nrel -> •expr < expr\nterm -> •term / unary\nterm -> •unary\nterm -> •term * unary\nequality -> •rel\n"]
30["term -> unary•\n"]
31["bool -> bool || join•\njoin -> join •&& equality\n"]
32["loc -> •id\nunary -> •- unary\nrel -> •expr < expr\nexpr -> •expr - term\nfactor -> •num\nrel -> •expr <= expr\nfactor -> •true\nfactor -> •false\nfactor -> •real\nexpr -> •term\nunary -> •! unary\nterm -> •term / unary\nexpr -> •expr + term\nequality -> •equality == rel\nrel -> •expr\nrel -> •expr > expr\njoin -> join && •equality\nterm -> •unary\nequality -> •equality != rel\nunary -> •factor\nloc -> •loc [ num ]\nrel -> •expr >= expr\nfactor -> •loc\nequality -> •rel\nterm -> •term * unary\nfactor -> •( bool )\n"]
33["rel -> expr •<= expr\nexpr -> expr •- term\nexpr -> expr •+ term\nrel -> expr•\nrel -> expr •> expr\nrel -> expr •< expr\nrel -> expr •>= expr\n"]
34["factor -> •num\nfactor -> •loc\nfactor -> •true\nterm -> •unary\nfactor -> •real\nunary -> •- unary\nexpr -> •term\nexpr -> •expr - term\nrel -> expr < •expr\nunary -> •factor\nterm -> •term * unary\nfactor -> •false\nloc -> •id\nunary -> •! unary\nfactor -> •( bool )\nterm -> •term / unary\nexpr -> •expr + term\nloc -> •loc [ num ]\n"]
35["expr -> expr •- term\nrel -> expr < expr•\nexpr -> expr •+ term\n"]
36["unary -> •factor\nterm -> •term / unary\nterm -> •term * unary\nloc -> •id\nexpr -> expr - •term\nunary -> •- unary\nfactor -> •( bool )\nfactor -> •real\nunary -> •! unary\nfactor -> •num\nfactor -> •true\nfactor -> •loc\nterm -> •unary\nloc -> •loc [ num ]\nfactor -> •false\n"]
37["term -> term •/ unary\nterm -> term •* unary\nexpr -> expr - term•\n"]
38["factor -> •false\nexpr -> expr + •term\nunary -> •factor\nloc -> •loc [ num ]\nloc -> •id\nfactor -> •( bool )\nfactor -> •num\nunary -> •! unary\nfactor -> •loc\nterm -> •term * unary\nunary -> •- unary\nfactor -> •true\nterm -> •unary\nterm -> •term / unary\nfactor -> •real\n"]
39["term -> term •* unary\nterm -> term •/ unary\nexpr -> expr + term•\n"]
40["unary -> •- unary\nunary -> •! unary\nexpr -> •expr + term\nterm -> •term * unary\nexpr -> •term\nterm -> •unary\nfactor -> •real\nrel -> expr <= •expr\nexpr -> •expr - term\nloc -> •id\nfactor -> •false\nfactor -> •loc\nunary -> •factor\nfactor -> •true\nloc -> •loc [ num ]\nfactor -> •( bool )\nterm -> •term / unary\nfactor -> •num\n"]
41["expr -> expr •+ term\nrel -> expr <= expr•\nexpr -> expr •- term\n"]
42["factor -> •num\nloc -> •loc [ num ]\nterm -> •unary\nunary -> •- unary\nterm -> •term * unary\nunary -> •! unary\nloc -> •id\nexpr -> •expr - term\nexpr -> •expr + term\nfactor -> •false\nunary -> •factor\nfactor -> •loc\nfactor -> •true\nexpr -> •term\nrel -> expr > •expr\nterm -> •term / unary\nfactor -> •( bool )\nfactor -> •real\n"]
43["expr -> expr •+ term\nrel -> expr > expr•\nexpr -> expr •- term\n"]
44["factor -> •real\nloc -> •loc [ num ]\nunary -> •factor\nunary -> •! unary\nfactor -> •false\nunary -> •- unary\nterm -> •term * unary\nfactor -> •( bool )\nfactor -> •true\nrel -> expr >= •expr\nexpr -> •term\nloc -> •id\nfactor -> •loc\nterm -> •term / unary\nexpr -> •expr + term\nfactor -> •num\nexpr -> •expr - term\nterm -> •unary\n"]
45["rel -> expr >= expr•\nexpr -> expr •+ term\nexpr -> expr •- term\n"]
46["equality -> equality •!= rel\nequality -> equality •== rel\njoin -> join && equality•\n"]
47["term -> •term / unary\nfactor -> •real\nexpr -> •term\nfactor -> •false\nrel -> •expr < expr\nunary -> •! unary\nexpr -> •expr - term\nequality -> equality != •rel\nunary -> •- unary\nterm -> •term * unary\nrel -> •expr >= expr\nrel -> •expr <= expr\nterm -> •unary\nfactor -> •num\nloc -> •id\nunary -> •factor\nrel -> •expr\nfactor -> •true\nfactor -> •( bool )\nrel -> •expr > expr\nloc -> •loc [ num ]\nfactor -> •loc\nexpr -> •expr + term\n"]
48["equality -> equality != rel•\n"]
49["factor -> •false\nrel -> •expr >= expr\nterm -> •term / unary\nexpr -> •expr - term\nterm -> •unary\nexpr -> •expr + term\nrel -> •expr\nloc -> •id\nterm -> •term * unary\nrel -> •expr < expr\nunary -> •! unary\nexpr -> •term\nfactor -> •num\nfactor -> •real\nfactor -> •loc\nrel -> •expr > expr\nfactor -> •true\nloc -> •loc [ num ]\nequality -> equality == •rel\nrel -> •expr <= expr\nfactor -> •( bool )\nunary -> •factor\nunary -> •- unary\n"]
50["equality -> equality == rel•\n"]
51["equality -> equality •== rel\nequality -> equality •!= rel\njoin -> equality•\n"]
52["bool -> join•\njoin -> join •&& equality\n"]
53["bool -> bool •|| join\nstmt -> if ( bool •) stmt\nstmt -> if ( bool •) stmt else stmt\n"]
54["stmt -> if ( bool ) •stmt\nloc -> •id\nstmt -> •do stmt while ( bool ) ;\nstmt -> •loc = bool ;\nloc -> •loc [ num ]\nstmt -> •if ( bool ) stmt else stmt\nstmt -> •break ;\nstmt -> •while ( bool ) stmt\nstmt -> •block\nblock -> •left_curly_brace decls stmts right_curly_brace\nstmt -> if ( bool ) •stmt else stmt\nstmt -> •if ( bool ) stmt\n"]
55["stmt -> if ( bool ) stmt•\nstmt -> if ( bool ) stmt •else stmt\n"]
56["stmt -> •if ( bool ) stmt\nloc -> •id\nstmt -> •do stmt while ( bool ) ;\nstmt -> if ( bool ) stmt else •stmt\nstmt -> •if ( bool ) stmt else stmt\nloc -> •loc [ num ]\nstmt -> •break ;\nstmt -> •loc = bool ;\nstmt -> •block\nstmt -> •while ( bool ) stmt\nblock -> •left_curly_brace decls stmts right_curly_brace\n"]
57["stmt -> if ( bool ) stmt else stmt•\n"]
58["stmt -> break •;\n"]
59["stmt -> break ;•\n"]
60["stmt -> block•\n"]
61["stmt -> •while ( bool ) stmt\nstmt -> do •stmt while ( bool ) ;\nstmt -> •if ( bool ) stmt else stmt\nblock -> •left_curly_brace decls stmts right_curly_brace\nstmt -> •do stmt while ( bool ) ;\nstmt -> •block\nstmt -> •break ;\nloc -> •id\nstmt -> •if ( bool ) stmt\nloc -> •loc [ num ]\nstmt -> •loc = bool ;\n"]
62["loc -> loc •[ num ]\nstmt -> loc •= bool ;\n"]
63["factor -> •real\nunary -> •- unary\nrel -> •expr >= expr\nexpr -> •expr + term\nequality -> •rel\nunary -> •! unary\nfactor -> •true\nloc -> •id\nterm -> •term / unary\nunary -> •factor\nfactor -> •loc\nrel -> •expr < expr\nrel -> •expr\njoin -> •join && equality\njoin -> •equality\nterm -> •term * unary\nexpr -> •term\nexpr -> •expr - term\nequality -> •equality == rel\nrel -> •expr <= expr\nbool -> •bool || join\nfactor -> •( bool )\nloc -> •loc [ num ]\nbool -> •join\nterm -> •unary\nfactor -> •false\nfactor -> •num\nequality -> •equality != rel\nrel -> •expr > expr\nstmt -> loc = •bool ;\n"]
64["bool -> bool •|| join\nstmt -> loc = bool •;\n"]
65["stmt -> loc = bool ;•\n"]
66["stmt -> while •( bool ) stmt\n"]
67["factor -> •false\nequality -> •rel\nfactor -> •true\nrel -> •expr\nfactor -> •num\nterm -> •term / unary\nrel -> •expr < expr\nexpr -> •term\nterm -> •term * unary\nbool -> •bool || join\nequality -> •equality != rel\nfactor -> •loc\nfactor -> •( bool )\nexpr -> •expr + term\nunary -> •- unary\njoin -> •join && equality\nrel -> •expr <= expr\nstmt -> while ( •bool ) stmt\nunary -> •factor\nterm -> •unary\nrel -> •expr >= expr\nexpr -> •expr - term\nbool -> •join\njoin -> •equality\nrel -> •expr > expr\nloc -> •loc [ num ]\nfactor -> •real\nequality -> •equality == rel\nunary -> •! unary\nloc -> •id\n"]
68["stmt -> while ( bool •) stmt\nbool -> bool •|| join\n"]
69["block -> •left_curly_brace decls stmts right_curly_brace\nstmt -> •break ;\nloc -> •id\nstmt -> while ( bool ) •stmt\nstmt -> •if ( bool ) stmt\nstmt -> •while ( bool ) stmt\nloc -> •loc [ num ]\nstmt -> •loc = bool ;\nstmt -> •block\nstmt -> •if ( bool ) stmt else stmt\nstmt -> •do stmt while ( bool ) ;\n"]
70["stmt -> while ( bool ) stmt•\n"]
71["stmt -> do stmt •while ( bool ) ;\n"]
72["stmt -> do stmt while •( bool ) ;\n"]
73["term -> •unary\nrel -> •expr <= expr\nequality -> •rel\nloc -> •loc [ num ]\nrel -> •expr < expr\nfactor -> •loc\nfactor -> •false\nunary -> •- unary\nrel -> •expr >= expr\nrel -> •expr\nunary -> •factor\nfactor -> •( bool )\nbool -> •join\njoin -> •join && equality\nexpr -> •term\nequality -> •equality == rel\nexpr -> •expr - term\nfactor -> •real\nbool -> •bool || join\nfactor -> •true\nterm -> •term * unary\nexpr -> •expr + term\nfactor -> •num\nloc -> •id\nequality -> •equality != rel\njoin -> •equality\nunary -> •! unary\nrel -> •expr > expr\nstmt -> do stmt while ( •bool ) ;\nterm -> •term / unary\n"]
74["bool -> bool •|| join\nstmt -> do stmt while ( bool •) ;\n"]
75["stmt -> do stmt while ( bool ) •;\n"]
76["stmt -> do stmt while ( bool ) ;•\n"]
77["stmts -> stmts stmt•\n"]
78["block -> left_curly_brace decls stmts right_curly_brace•\n"]
79["type -> basic•\n"]
80["type -> type •[ num ]\ndecl -> type •id ;\n"]
81["decl -> type id •;\n"]
82["decl -> type id ;•\n"]
83["type -> type [ •num ]\n"]
84["type -> type [ num •]\n"]
85["type -> type [ num ]•\n"]
86["decls -> decls decl•\n"]
87["stmt -> •block\nstmt -> •break ;\nstmt -> •do stmt while ( bool ) ;\nstmt -> •if ( bool ) stmt else stmt\nstmt -> •while ( bool ) stmt\nstmts -> stmts •stmt\nstmt -> •if ( bool ) stmt\nloc -> •loc [ num ]\nloc -> •id\nblock -> •left_curly_brace decls stmts right_curly_brace\nstmt -> •loc = bool ;\n"]
88["program -> block•\n"]
89["__DUMMY_START__ -> S•\n"]
90["S -> program•\n"]

5--"real"-->6
7--"unary"-->8
9--"num"-->10
9--"true"-->11
9--"id"-->12
9--"rel"-->13
15--"factor"-->16
15--"unary"-->17
15--"id"-->12
20--"]"-->21
19--"num"-->20
18--"["-->19
15--"loc"-->18
15--"true"-->11
15--"num"-->10
15--"false"-->22
15--"real"-->6
15--"-"-->7
15--"("-->9
23--"("-->9
23--"real"-->6
23--"-"-->7
23--"!"-->23
23--"loc"-->18
23--"unary"-->24
23--"num"-->10
23--"id"-->12
23--"factor"-->16
23--"true"-->11
23--"false"-->22
15--"!"-->23
14--"/"-->15
25--"false"-->22
25--"true"-->11
25--"real"-->6
25--"unary"-->26
25--"id"-->12
25--"factor"-->16
25--"!"-->23
25--"loc"-->18
25--"num"-->10
25--"("-->9
25--"-"-->7
14--"*"-->25
9--"term"-->14
27--")"-->28
29--"loc"-->18
29--"num"-->10
29--"!"-->23
29--"factor"-->16
29--"unary"-->30
29--"real"-->6
29--"id"-->12
29--"term"-->14
34--"true"-->11
34--"id"-->12
34--"real"-->6
34--"!"-->23
34--"loc"-->18
34--"("-->9
34--"term"-->14
36--"num"-->10
36--"id"-->12
36--"-"-->7
36--"!"-->23
36--"real"-->6
36--"true"-->11
36--"factor"-->16
36--"loc"-->18
36--"unary"-->30
37--"/"-->15
37--"*"-->25
36--"term"-->37
36--"("-->9
36--"false"-->22
35--"-"-->36
38--"id"-->12
38--"false"-->22
38--"loc"-->18
38--"factor"-->16
38--"("-->9
38--"num"-->10
39--"/"-->15
39--"*"-->25
38--"term"-->39
38--"!"-->23
38--"true"-->11
38--"unary"-->30
38--"real"-->6
38--"-"-->7
35--"+"-->38
34--"expr"-->35
34--"false"-->22
34--"factor"-->16
34--"-"-->7
34--"num"-->10
34--"unary"-->30
33--"<"-->34
40--"("-->9
40--"loc"-->18
40--"num"-->10
40--"term"-->14
40--"factor"-->16
40--"!"-->23
41--"-"-->36
41--"+"-->38
40--"expr"-->41
40--"real"-->6
40--"id"-->12
40--"false"-->22
40--"true"-->11
40--"unary"-->30
40--"-"-->7
33--"<="-->40
33--"-"-->36
33--"+"-->38
42--"("-->9
42--"real"-->6
42--"factor"-->16
42--"-"-->7
42--"num"-->10
42--"loc"-->18
42--"unary"-->30
42--"false"-->22
42--"term"-->14
42--"true"-->11
42--"!"-->23
43--"-"-->36
43--"+"-->38
42--"expr"-->43
42--"id"-->12
33--">"-->42
44--"factor"-->16
44--"false"-->22
44--"true"-->11
44--"loc"-->18
44--"!"-->23
45--"-"-->36
45--"+"-->38
44--"expr"-->45
44--"id"-->12
44--"num"-->10
44--"-"-->7
44--"unary"-->30
44--"real"-->6
44--"term"-->14
44--"("-->9
33--">="-->44
32--"expr"-->33
32--"rel"-->13
32--"!"-->23
32--"-"-->7
32--"real"-->6
32--"num"-->10
32--"("-->9
32--"factor"-->16
32--"term"-->14
32--"unary"-->30
32--"true"-->11
32--"false"-->22
32--"id"-->12
47--"num"-->10
47--"term"-->14
47--"expr"-->33
47--"!"-->23
47--"true"-->11
47--"unary"-->30
47--"("-->9
47--"real"-->6
47--"false"-->22
47--"rel"-->48
47--"-"-->7
47--"id"-->12
47--"factor"-->16
47--"loc"-->18
46--"!="-->47
49--"!"-->23
49--"factor"-->16
49--"num"-->10
49--"-"-->7
49--"real"-->6
49--"expr"-->33
49--"term"-->14
49--"false"-->22
49--"id"-->12
49--"unary"-->30
49--"loc"-->18
49--"true"-->11
49--"rel"-->50
49--"("-->9
46--"=="-->49
32--"equality"-->46
32--"loc"-->18
31--"&&"-->32
29--"join"-->31
29--"true"-->11
51--"=="-->49
51--"!="-->47
29--"equality"-->51
29--"-"-->7
29--"expr"-->33
29--"false"-->22
29--"("-->9
29--"rel"-->13
27--"||"-->29
9--"bool"-->27
9--"equality"-->51
9--"("-->9
9--"real"-->6
9--"-"-->7
9--"unary"-->30
9--"loc"-->18
9--"expr"-->33
9--"factor"-->16
52--"&&"-->32
9--"join"-->52
9--"!"-->23
9--"false"-->22
7--"("-->9
7--"num"-->10
7--"real"-->6
7--"false"-->22
7--"id"-->12
7--"-"-->7
7--"loc"-->18
7--"factor"-->16
7--"!"-->23
7--"true"-->11
5--"-"-->7
5--"term"-->14
5--"rel"-->13
5--"join"-->52
5--"factor"-->16
5--"unary"-->30
5--"equality"-->51
5--"num"-->10
5--"id"-->12
5--"false"-->22
5--"!"-->23
5--"expr"-->33
5--"("-->9
5--"true"-->11
5--"loc"-->18
56--"stmt"-->57
56--"left_curly_brace"-->1
56--"if"-->4
58--";"-->59
56--"break"-->58
56--"block"-->60
61--"break"-->58
61--"if"-->4
61--"id"-->12
62--"["-->19
63--"real"-->6
63--"expr"-->33
63--"rel"-->13
63--"factor"-->16
63--"unary"-->30
63--"join"-->52
63--"num"-->10
63--"loc"-->18
63--"-"-->7
63--"!"-->23
64--"||"-->29
64--";"-->65
63--"bool"-->64
63--"("-->9
63--"true"-->11
63--"false"-->22
63--"id"-->12
63--"equality"-->51
63--"term"-->14
62--"="-->63
61--"loc"-->62
67--"equality"-->51
67--"expr"-->33
68--"||"-->29
69--"if"-->4
69--"left_curly_brace"-->1
69--"break"-->58
69--"while"-->66
69--"block"-->60
69--"do"-->61
69--"stmt"-->70
69--"id"-->12
69--"loc"-->62
68--")"-->69
67--"bool"-->68
67--"("-->9
67--"real"-->6
67--"false"-->22
67--"rel"-->13
67--"true"-->11
67--"term"-->14
67--"!"-->23
67--"-"-->7
67--"unary"-->30
67--"factor"-->16
67--"id"-->12
67--"join"-->52
67--"num"-->10
67--"loc"-->18
66--"("-->67
61--"while"-->66
61--"do"-->61
61--"left_curly_brace"-->1
73--"-"-->7
73--"join"-->52
73--"num"-->10
73--"unary"-->30
73--"expr"-->33
73--"factor"-->16
75--";"-->76
74--")"-->75
74--"||"-->29
73--"bool"-->74
73--"rel"-->13
73--"!"-->23
73--"term"-->14
73--"equality"-->51
73--"id"-->12
73--"false"-->22
73--"true"-->11
73--"loc"-->18
73--"("-->9
73--"real"-->6
72--"("-->73
71--"while"-->72
61--"stmt"-->71
61--"block"-->60
56--"do"-->61
56--"id"-->12
56--"loc"-->62
56--"while"-->66
55--"else"-->56
54--"stmt"-->55
54--"id"-->12
54--"break"-->58
54--"while"-->66
54--"do"-->61
54--"loc"-->62
54--"if"-->4
54--"block"-->60
54--"left_curly_brace"-->1
53--")"-->54
53--"||"-->29
5--"bool"-->53
4--"("-->5
3--"if"-->4
3--"break"-->58
3--"loc"-->62
3--"do"-->61
3--"left_curly_brace"-->1
3--"stmt"-->77
3--"right_curly_brace"-->78
3--"block"-->60
3--"while"-->66
3--"id"-->12
2--"stmts"-->3
2--"basic"-->79
2--"if"-->4
2--"id"-->12
81--";"-->82
80--"id"-->81
84--"]"-->85
83--"num"-->84
80--"["-->83
2--"type"-->80
2--"left_curly_brace"-->1
2--"do"-->61
2--"decl"-->86
2--"break"-->58
2--"loc"-->62
2--"block"-->60
2--"while"-->66
1--"decls"-->2
87--"loc"-->62
87--"do"-->61
87--"block"-->60
87--"break"-->58
87--"if"-->4
87--"left_curly_brace"-->1
87--"while"-->66
87--"id"-->12
87--"stmt"-->77
1--"stmts"-->87
1--"do"-->61
1--"loc"-->62
1--"block"-->60
1--"basic"-->79
1--"type"-->80
1--"while"-->66
1--"left_curly_brace"-->1
1--"if"-->4
1--"id"-->12
1--"break"-->58
0--"left_curly_brace"-->1
0--"block"-->88
0--"S"-->89
0--"program"-->90

```

---
## Action Table
```txt
State 0:
left_curly_brace: Shift(1)
program: Shift(90)
__$__: Accept
block: Shift(88)
S: Shift(89)
===================
State 1:
stmts: Shift(87)
do: Shift(61)
decls: Shift(2)
id: Shift(12)
if: Shift(4)
break: Shift(58)
left_curly_brace: Shift(1)
block: Shift(60)
loc: Shift(62)
type: Shift(80)
while: Shift(66)
basic: Shift(79)
===================
State 2:
stmts: Shift(3)
basic: Shift(79)
left_curly_brace: Shift(1)
break: Shift(58)
block: Shift(60)
id: Shift(12)
type: Shift(80)
if: Shift(4)
do: Shift(61)
loc: Shift(62)
while: Shift(66)
decl: Shift(86)
===================
State 3:
left_curly_brace: Shift(1)
break: Shift(58)
loc: Shift(62)
if: Shift(4)
do: Shift(61)
while: Shift(66)
id: Shift(12)
block: Shift(60)
stmt: Shift(77)
right_curly_brace: Shift(78)
===================
State 4:
(: Shift(5)
===================
State 5:
bool: Shift(53)
factor: Shift(16)
term: Shift(14)
loc: Shift(18)
real: Shift(6)
-: Shift(7)
!: Shift(23)
expr: Shift(33)
unary: Shift(30)
(: Shift(9)
rel: Shift(13)
join: Shift(52)
num: Shift(10)
id: Shift(12)
false: Shift(22)
true: Shift(11)
equality: Shift(51)
===================
State 6:
<: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
/: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
||: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
==: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
): Reduce(ReduceDerivation { left: "factor", right: ["real"] })
>=: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
&&: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
-: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
!=: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
*: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
>: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
;: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
+: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
<=: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
===================
State 7:
!: Shift(23)
factor: Shift(16)
unary: Shift(8)
false: Shift(22)
id: Shift(12)
true: Shift(11)
(: Shift(9)
num: Shift(10)
real: Shift(6)
loc: Shift(18)
-: Shift(7)
===================
State 8:
*: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
||: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
<=: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
): Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
!=: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
>: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
<: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
==: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
/: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
+: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
&&: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
>=: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
;: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
-: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
===================
State 9:
false: Shift(22)
num: Shift(10)
factor: Shift(16)
expr: Shift(33)
bool: Shift(27)
unary: Shift(30)
rel: Shift(13)
(: Shift(9)
term: Shift(14)
real: Shift(6)
id: Shift(12)
equality: Shift(51)
join: Shift(52)
true: Shift(11)
!: Shift(23)
-: Shift(7)
loc: Shift(18)
===================
State 10:
!=: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
<: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
/: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
>: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
&&: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
*: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
==: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
;: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
||: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
-: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
+: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
): Reduce(ReduceDerivation { left: "factor", right: ["num"] })
<=: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
>=: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
===================
State 11:
!=: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
>: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
+: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
&&: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
<=: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
>=: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
): Reduce(ReduceDerivation { left: "factor", right: ["true"] })
;: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
-: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
<: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
==: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
/: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
||: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
*: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
===================
State 12:
/: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
||: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
[: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
): Reduce(ReduceDerivation { left: "loc", right: ["id"] })
-: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
<: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
;: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
==: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
*: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
&&: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
>=: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
=: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
!=: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
+: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
<=: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
>: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
===================
State 13:
): Reduce(ReduceDerivation { left: "equality", right: ["rel"] })
||: Reduce(ReduceDerivation { left: "equality", right: ["rel"] })
==: Reduce(ReduceDerivation { left: "equality", right: ["rel"] })
!=: Reduce(ReduceDerivation { left: "equality", right: ["rel"] })
&&: Reduce(ReduceDerivation { left: "equality", right: ["rel"] })
;: Reduce(ReduceDerivation { left: "equality", right: ["rel"] })
===================
State 14:
==: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
): Reduce(ReduceDerivation { left: "expr", right: ["term"] })
+: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
*: Shift(25)
<=: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
>=: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
-: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
!=: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
/: Shift(15)
||: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
>: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
<: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
&&: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
;: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
===================
State 15:
factor: Shift(16)
unary: Shift(17)
real: Shift(6)
!: Shift(23)
id: Shift(12)
true: Shift(11)
loc: Shift(18)
-: Shift(7)
(: Shift(9)
num: Shift(10)
false: Shift(22)
===================
State 16:
>=: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
>: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
): Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
!=: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
+: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
<=: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
&&: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
;: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
/: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
==: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
<: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
*: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
-: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
||: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
===================
State 17:
>: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
/: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
*: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
): Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
-: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
<: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
&&: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
||: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
!=: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
;: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
<=: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
+: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
>=: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
==: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
===================
State 18:
[: Shift(19)
>=: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
>: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
): Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
==: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
<=: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
!=: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
*: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
&&: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
-: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
+: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
<: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
||: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
;: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
/: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
===================
State 19:
num: Shift(20)
===================
State 20:
]: Shift(21)
===================
State 21:
>: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
;: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
*: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
!=: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
&&: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
[: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
<: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
>=: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
||: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
<=: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
+: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
=: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
-: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
/: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
): Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
==: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
===================
State 22:
): Reduce(ReduceDerivation { left: "factor", right: ["false"] })
&&: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
<=: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
;: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
<: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
>: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
/: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
||: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
-: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
==: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
!=: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
>=: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
*: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
+: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
===================
State 23:
!: Shift(23)
-: Shift(7)
false: Shift(22)
loc: Shift(18)
true: Shift(11)
id: Shift(12)
(: Shift(9)
real: Shift(6)
factor: Shift(16)
num: Shift(10)
unary: Shift(24)
===================
State 24:
||: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
+: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
<=: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
): Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
>=: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
&&: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
!=: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
;: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
>: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
*: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
<: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
-: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
/: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
==: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
===================
State 25:
factor: Shift(16)
!: Shift(23)
(: Shift(9)
true: Shift(11)
unary: Shift(26)
real: Shift(6)
loc: Shift(18)
id: Shift(12)
false: Shift(22)
num: Shift(10)
-: Shift(7)
===================
State 26:
!=: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
||: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
<: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
): Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
>=: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
==: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
/: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
<=: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
&&: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
*: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
+: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
;: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
>: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
-: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
===================
State 27:
): Shift(28)
||: Shift(29)
===================
State 28:
;: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
/: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
!=: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
==: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
<=: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
<: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
&&: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
): Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
>: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
+: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
-: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
*: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
||: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
>=: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
===================
State 29:
join: Shift(31)
term: Shift(14)
-: Shift(7)
loc: Shift(18)
true: Shift(11)
rel: Shift(13)
id: Shift(12)
(: Shift(9)
equality: Shift(51)
false: Shift(22)
expr: Shift(33)
!: Shift(23)
num: Shift(10)
unary: Shift(30)
factor: Shift(16)
real: Shift(6)
===================
State 30:
>=: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
): Reduce(ReduceDerivation { left: "term", right: ["unary"] })
/: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
*: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
;: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
<=: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
==: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
>: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
<: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
!=: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
-: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
||: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
+: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
&&: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
===================
State 31:
): Reduce(ReduceDerivation { left: "bool", right: ["bool", "||", "join"] })
&&: Shift(32)
;: Reduce(ReduceDerivation { left: "bool", right: ["bool", "||", "join"] })
||: Reduce(ReduceDerivation { left: "bool", right: ["bool", "||", "join"] })
===================
State 32:
rel: Shift(13)
true: Shift(11)
term: Shift(14)
real: Shift(6)
false: Shift(22)
unary: Shift(30)
!: Shift(23)
num: Shift(10)
loc: Shift(18)
equality: Shift(46)
-: Shift(7)
(: Shift(9)
id: Shift(12)
expr: Shift(33)
factor: Shift(16)
===================
State 33:
+: Shift(38)
||: Reduce(ReduceDerivation { left: "rel", right: ["expr"] })
>: Shift(42)
<=: Shift(40)
<: Shift(34)
==: Reduce(ReduceDerivation { left: "rel", right: ["expr"] })
!=: Reduce(ReduceDerivation { left: "rel", right: ["expr"] })
): Reduce(ReduceDerivation { left: "rel", right: ["expr"] })
&&: Reduce(ReduceDerivation { left: "rel", right: ["expr"] })
;: Reduce(ReduceDerivation { left: "rel", right: ["expr"] })
>=: Shift(44)
-: Shift(36)
===================
State 34:
real: Shift(6)
true: Shift(11)
!: Shift(23)
term: Shift(14)
expr: Shift(35)
loc: Shift(18)
(: Shift(9)
id: Shift(12)
factor: Shift(16)
num: Shift(10)
false: Shift(22)
unary: Shift(30)
-: Shift(7)
===================
State 35:
!=: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<", "expr"] })
-: Shift(36)
;: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<", "expr"] })
+: Shift(38)
==: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<", "expr"] })
): Reduce(ReduceDerivation { left: "rel", right: ["expr", "<", "expr"] })
&&: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<", "expr"] })
||: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<", "expr"] })
===================
State 36:
!: Shift(23)
false: Shift(22)
term: Shift(37)
(: Shift(9)
factor: Shift(16)
unary: Shift(30)
-: Shift(7)
num: Shift(10)
true: Shift(11)
id: Shift(12)
loc: Shift(18)
real: Shift(6)
===================
State 37:
&&: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
;: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
+: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
>: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
*: Shift(25)
/: Shift(15)
): Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
<=: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
!=: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
==: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
-: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
<: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
>=: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
||: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
===================
State 38:
id: Shift(12)
loc: Shift(18)
term: Shift(39)
factor: Shift(16)
num: Shift(10)
unary: Shift(30)
-: Shift(7)
true: Shift(11)
(: Shift(9)
real: Shift(6)
false: Shift(22)
!: Shift(23)
===================
State 39:
==: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
<: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
>=: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
): Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
*: Shift(25)
&&: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
;: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
/: Shift(15)
+: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
>: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
||: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
<=: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
-: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
!=: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
===================
State 40:
num: Shift(10)
id: Shift(12)
true: Shift(11)
factor: Shift(16)
(: Shift(9)
!: Shift(23)
false: Shift(22)
term: Shift(14)
expr: Shift(41)
real: Shift(6)
loc: Shift(18)
unary: Shift(30)
-: Shift(7)
===================
State 41:
-: Shift(36)
+: Shift(38)
): Reduce(ReduceDerivation { left: "rel", right: ["expr", "<=", "expr"] })
;: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<=", "expr"] })
==: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<=", "expr"] })
||: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<=", "expr"] })
!=: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<=", "expr"] })
&&: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<=", "expr"] })
===================
State 42:
expr: Shift(43)
(: Shift(9)
real: Shift(6)
num: Shift(10)
factor: Shift(16)
id: Shift(12)
loc: Shift(18)
unary: Shift(30)
false: Shift(22)
true: Shift(11)
-: Shift(7)
term: Shift(14)
!: Shift(23)
===================
State 43:
): Reduce(ReduceDerivation { left: "rel", right: ["expr", ">", "expr"] })
+: Shift(38)
!=: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">", "expr"] })
||: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">", "expr"] })
-: Shift(36)
;: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">", "expr"] })
&&: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">", "expr"] })
==: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">", "expr"] })
===================
State 44:
real: Shift(6)
-: Shift(7)
factor: Shift(16)
unary: Shift(30)
term: Shift(14)
(: Shift(9)
true: Shift(11)
expr: Shift(45)
id: Shift(12)
false: Shift(22)
loc: Shift(18)
!: Shift(23)
num: Shift(10)
===================
State 45:
;: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">=", "expr"] })
!=: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">=", "expr"] })
-: Shift(36)
&&: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">=", "expr"] })
||: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">=", "expr"] })
): Reduce(ReduceDerivation { left: "rel", right: ["expr", ">=", "expr"] })
==: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">=", "expr"] })
+: Shift(38)
===================
State 46:
&&: Reduce(ReduceDerivation { left: "join", right: ["join", "&&", "equality"] })
!=: Shift(47)
): Reduce(ReduceDerivation { left: "join", right: ["join", "&&", "equality"] })
==: Shift(49)
;: Reduce(ReduceDerivation { left: "join", right: ["join", "&&", "equality"] })
||: Reduce(ReduceDerivation { left: "join", right: ["join", "&&", "equality"] })
===================
State 47:
expr: Shift(33)
true: Shift(11)
(: Shift(9)
real: Shift(6)
-: Shift(7)
factor: Shift(16)
id: Shift(12)
num: Shift(10)
term: Shift(14)
!: Shift(23)
loc: Shift(18)
rel: Shift(48)
unary: Shift(30)
false: Shift(22)
===================
State 48:
||: Reduce(ReduceDerivation { left: "equality", right: ["equality", "!=", "rel"] })
&&: Reduce(ReduceDerivation { left: "equality", right: ["equality", "!=", "rel"] })
==: Reduce(ReduceDerivation { left: "equality", right: ["equality", "!=", "rel"] })
;: Reduce(ReduceDerivation { left: "equality", right: ["equality", "!=", "rel"] })
!=: Reduce(ReduceDerivation { left: "equality", right: ["equality", "!=", "rel"] })
): Reduce(ReduceDerivation { left: "equality", right: ["equality", "!=", "rel"] })
===================
State 49:
num: Shift(10)
term: Shift(14)
!: Shift(23)
factor: Shift(16)
real: Shift(6)
id: Shift(12)
false: Shift(22)
unary: Shift(30)
loc: Shift(18)
expr: Shift(33)
true: Shift(11)
rel: Shift(50)
(: Shift(9)
-: Shift(7)
===================
State 50:
!=: Reduce(ReduceDerivation { left: "equality", right: ["equality", "==", "rel"] })
&&: Reduce(ReduceDerivation { left: "equality", right: ["equality", "==", "rel"] })
==: Reduce(ReduceDerivation { left: "equality", right: ["equality", "==", "rel"] })
||: Reduce(ReduceDerivation { left: "equality", right: ["equality", "==", "rel"] })
;: Reduce(ReduceDerivation { left: "equality", right: ["equality", "==", "rel"] })
): Reduce(ReduceDerivation { left: "equality", right: ["equality", "==", "rel"] })
===================
State 51:
==: Shift(49)
!=: Shift(47)
&&: Reduce(ReduceDerivation { left: "join", right: ["equality"] })
): Reduce(ReduceDerivation { left: "join", right: ["equality"] })
;: Reduce(ReduceDerivation { left: "join", right: ["equality"] })
||: Reduce(ReduceDerivation { left: "join", right: ["equality"] })
===================
State 52:
||: Reduce(ReduceDerivation { left: "bool", right: ["join"] })
): Reduce(ReduceDerivation { left: "bool", right: ["join"] })
&&: Shift(32)
;: Reduce(ReduceDerivation { left: "bool", right: ["join"] })
===================
State 53:
): Shift(54)
||: Shift(29)
===================
State 54:
break: Shift(58)
id: Shift(12)
while: Shift(66)
do: Shift(61)
block: Shift(60)
if: Shift(4)
stmt: Shift(55)
loc: Shift(62)
left_curly_brace: Shift(1)
===================
State 55:
right_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
if: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
id: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
else: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
break: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
while: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
do: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
===================
State 56:
left_curly_brace: Shift(1)
block: Shift(60)
if: Shift(4)
while: Shift(66)
break: Shift(58)
loc: Shift(62)
do: Shift(61)
stmt: Shift(57)
id: Shift(12)
===================
State 57:
right_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
if: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
else: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
while: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
do: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
id: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
break: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
===================
State 58:
;: Shift(59)
===================
State 59:
if: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
do: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
break: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
right_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
id: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
else: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
while: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
===================
State 60:
right_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
else: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
do: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
break: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
if: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
while: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
id: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
===================
State 61:
break: Shift(58)
loc: Shift(62)
do: Shift(61)
if: Shift(4)
left_curly_brace: Shift(1)
stmt: Shift(71)
block: Shift(60)
id: Shift(12)
while: Shift(66)
===================
State 62:
[: Shift(19)
=: Shift(63)
===================
State 63:
!: Shift(23)
loc: Shift(18)
rel: Shift(13)
factor: Shift(16)
expr: Shift(33)
join: Shift(52)
id: Shift(12)
-: Shift(7)
equality: Shift(51)
false: Shift(22)
unary: Shift(30)
true: Shift(11)
bool: Shift(64)
num: Shift(10)
real: Shift(6)
term: Shift(14)
(: Shift(9)
===================
State 64:
;: Shift(65)
||: Shift(29)
===================
State 65:
if: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
right_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
while: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
break: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
id: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
do: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
else: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
===================
State 66:
(: Shift(67)
===================
State 67:
loc: Shift(18)
expr: Shift(33)
real: Shift(6)
unary: Shift(30)
join: Shift(52)
id: Shift(12)
rel: Shift(13)
equality: Shift(51)
true: Shift(11)
-: Shift(7)
factor: Shift(16)
num: Shift(10)
term: Shift(14)
false: Shift(22)
!: Shift(23)
(: Shift(9)
bool: Shift(68)
===================
State 68:
||: Shift(29)
): Shift(69)
===================
State 69:
if: Shift(4)
left_curly_brace: Shift(1)
while: Shift(66)
break: Shift(58)
do: Shift(61)
block: Shift(60)
id: Shift(12)
stmt: Shift(70)
loc: Shift(62)
===================
State 70:
right_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
else: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
id: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
while: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
break: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
do: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
if: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
===================
State 71:
while: Shift(72)
===================
State 72:
(: Shift(73)
===================
State 73:
id: Shift(12)
term: Shift(14)
join: Shift(52)
num: Shift(10)
!: Shift(23)
factor: Shift(16)
unary: Shift(30)
expr: Shift(33)
loc: Shift(18)
real: Shift(6)
-: Shift(7)
false: Shift(22)
true: Shift(11)
equality: Shift(51)
rel: Shift(13)
bool: Shift(74)
(: Shift(9)
===================
State 74:
): Shift(75)
||: Shift(29)
===================
State 75:
;: Shift(76)
===================
State 76:
left_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
right_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
if: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
else: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
while: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
do: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
id: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
break: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
===================
State 77:
if: Reduce(ReduceDerivation { left: "stmts", right: ["stmts", "stmt"] })
break: Reduce(ReduceDerivation { left: "stmts", right: ["stmts", "stmt"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmts", right: ["stmts", "stmt"] })
right_curly_brace: Reduce(ReduceDerivation { left: "stmts", right: ["stmts", "stmt"] })
while: Reduce(ReduceDerivation { left: "stmts", right: ["stmts", "stmt"] })
do: Reduce(ReduceDerivation { left: "stmts", right: ["stmts", "stmt"] })
id: Reduce(ReduceDerivation { left: "stmts", right: ["stmts", "stmt"] })
===================
State 78:
left_curly_brace: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
else: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
while: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
__$__: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
if: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
right_curly_brace: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
break: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
id: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
do: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
===================
State 79:
id: Reduce(ReduceDerivation { left: "type", right: ["basic"] })
[: Reduce(ReduceDerivation { left: "type", right: ["basic"] })
===================
State 80:
[: Shift(83)
id: Shift(81)
===================
State 81:
;: Shift(82)
===================
State 82:
left_curly_brace: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
do: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
id: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
while: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
if: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
break: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
basic: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
===================
State 83:
num: Shift(84)
===================
State 84:
]: Shift(85)
===================
State 85:
id: Reduce(ReduceDerivation { left: "type", right: ["type", "[", "num", "]"] })
[: Reduce(ReduceDerivation { left: "type", right: ["type", "[", "num", "]"] })
===================
State 86:
basic: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
break: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
while: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
left_curly_brace: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
do: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
id: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
if: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
===================
State 87:
break: Shift(58)
id: Shift(12)
stmt: Shift(77)
if: Shift(4)
loc: Shift(62)
block: Shift(60)
do: Shift(61)
while: Shift(66)
left_curly_brace: Shift(1)
===================
State 88:
__$__: Reduce(ReduceDerivation { left: "program", right: ["block"] })
===================
State 89:
__$__: Reduce(ReduceDerivation { left: "__DUMMY_START__", right: ["S"] })
===================
State 90:
__$__: Reduce(ReduceDerivation { left: "S", right: ["program"] })
===================

```
---
generated by rparser
RockRockWhite 2023
    