
# Generated Info

## Base Info
- config_file: sample.rparser
- output_file: sample.rs
- time: 2023-05-19 20:39:20.266747 +08:00

---

## DFA Graph
```mermaid
graph LR
0["program -> •block\n__DUMMY_START__ -> •S\nblock -> •left_curly_brace decls stmts right_curly_brace\nS -> •program\n"]
1["program -> block•\n"]
2["__DUMMY_START__ -> S•\n"]
3["S -> program•\n"]
4["decl -> •type id ;\nstmt -> •block\nloc -> •loc [ num ]\nstmt -> •if ( bool ) stmt else stmt\nstmt -> •if ( bool ) stmt\ndecls -> •decls decl\nstmt -> •do stmt while ( bool ) ;\nloc -> •id\nblock -> left_curly_brace •decls stmts right_curly_brace\nstmt -> •loc = bool ;\ntype -> •basic\nstmts -> •stmts stmt\ntype -> •type [ num ]\nstmt -> •while ( bool ) stmt\nblock -> •left_curly_brace decls stmts right_curly_brace\nstmt -> •break ;\n"]
5["stmt -> block•\n"]
6["stmt -> if •( bool ) stmt else stmt\nstmt -> if •( bool ) stmt\n"]
7["factor -> •loc\nexpr -> •expr + term\nterm -> •unary\nstmt -> if ( •bool ) stmt\nrel -> •expr <= expr\nfactor -> •real\nunary -> •- unary\nfactor -> •( bool )\nunary -> •! unary\nequality -> •rel\nbool -> •join\nrel -> •expr >= expr\nequality -> •equality == rel\nrel -> •expr\nexpr -> •term\nfactor -> •num\nterm -> •term / unary\njoin -> •join && equality\nterm -> •term * unary\nloc -> •id\nfactor -> •false\nexpr -> •expr - term\nrel -> •expr < expr\nfactor -> •true\nunary -> •factor\nequality -> •equality != rel\njoin -> •equality\nrel -> •expr > expr\nbool -> •bool || join\nstmt -> if ( •bool ) stmt else stmt\nloc -> •loc [ num ]\n"]
8["loc -> id•\n"]
9["factor -> •loc\nunary -> •- unary\nunary -> ! •unary\nfactor -> •real\nfactor -> •( bool )\nfactor -> •false\nfactor -> •true\nunary -> •factor\nloc -> •loc [ num ]\nunary -> •! unary\nfactor -> •num\nloc -> •id\n"]
10["factor -> num•\n"]
11["factor -> true•\n"]
12["unary -> •factor\nunary -> •- unary\nunary -> - •unary\nunary -> •! unary\nfactor -> •true\nfactor -> •real\nfactor -> •loc\nfactor -> •num\nfactor -> •false\nfactor -> •( bool )\nloc -> •loc [ num ]\nloc -> •id\n"]
13["factor -> false•\n"]
14["rel -> •expr <= expr\nfactor -> •( bool )\njoin -> •join && equality\nrel -> •expr >= expr\nterm -> •term / unary\nloc -> •id\nfactor -> •loc\nexpr -> •expr + term\nloc -> •loc [ num ]\nfactor -> •num\nunary -> •! unary\nrel -> •expr\nrel -> •expr > expr\nfactor -> •false\nterm -> •unary\nfactor -> •real\nrel -> •expr < expr\nunary -> •factor\nequality -> •rel\nbool -> •join\nterm -> •term * unary\nexpr -> •expr - term\njoin -> •equality\nfactor -> ( •bool )\nbool -> •bool || join\nfactor -> •true\nunary -> •- unary\nequality -> •equality != rel\nexpr -> •term\nequality -> •equality == rel\n"]
15["factor -> loc•\nloc -> loc •[ num ]\n"]
16["loc -> loc [ •num ]\n"]
17["loc -> loc [ num •]\n"]
18["loc -> loc [ num ]•\n"]
19["bool -> bool •|| join\nfactor -> ( bool •)\n"]
20["term -> •term / unary\nterm -> •unary\nrel -> •expr < expr\nfactor -> •loc\nexpr -> •expr + term\nequality -> •equality == rel\nfactor -> •real\nfactor -> •false\nfactor -> •true\nrel -> •expr\nunary -> •! unary\nloc -> •id\nfactor -> •( bool )\nexpr -> •expr - term\nequality -> •equality != rel\nrel -> •expr > expr\nexpr -> •term\nunary -> •- unary\nterm -> •term * unary\nrel -> •expr <= expr\nrel -> •expr >= expr\njoin -> •join && equality\nloc -> •loc [ num ]\njoin -> •equality\nequality -> •rel\nunary -> •factor\nbool -> bool || •join\nfactor -> •num\n"]
21["unary -> factor•\n"]
22["join -> equality•\nequality -> equality •== rel\nequality -> equality •!= rel\n"]
23["expr -> •term\nunary -> •factor\nloc -> •loc [ num ]\nrel -> •expr > expr\nunary -> •! unary\nfactor -> •loc\nfactor -> •true\nrel -> •expr < expr\nexpr -> •expr - term\nrel -> •expr >= expr\nfactor -> •( bool )\nfactor -> •real\nexpr -> •expr + term\nterm -> •unary\nterm -> •term / unary\nterm -> •term * unary\nrel -> •expr\nfactor -> •num\nunary -> •- unary\nequality -> equality == •rel\nfactor -> •false\nrel -> •expr <= expr\nloc -> •id\n"]
24["equality -> equality == rel•\n"]
25["rel -> expr •> expr\nrel -> expr •< expr\nexpr -> expr •- term\nrel -> expr •<= expr\nrel -> expr •>= expr\nexpr -> expr •+ term\nrel -> expr•\n"]
26["factor -> •false\nexpr -> expr + •term\nterm -> •term / unary\nunary -> •factor\nunary -> •- unary\nfactor -> •true\nfactor -> •( bool )\nfactor -> •real\nfactor -> •loc\nloc -> •loc [ num ]\nloc -> •id\nfactor -> •num\nterm -> •term * unary\nunary -> •! unary\nterm -> •unary\n"]
27["term -> unary•\n"]
28["term -> term •* unary\nexpr -> expr + term•\nterm -> term •/ unary\n"]
29["factor -> •loc\nloc -> •loc [ num ]\nloc -> •id\nfactor -> •( bool )\nunary -> •- unary\nfactor -> •false\nunary -> •factor\nfactor -> •real\nfactor -> •num\nterm -> term / •unary\nunary -> •! unary\nfactor -> •true\n"]
30["factor -> real•\n"]
31["term -> term / unary•\n"]
32["term -> term * •unary\nfactor -> •num\nfactor -> •( bool )\nunary -> •factor\nloc -> •id\nfactor -> •real\nunary -> •! unary\nunary -> •- unary\nloc -> •loc [ num ]\nfactor -> •false\nfactor -> •loc\nfactor -> •true\n"]
33["term -> term * unary•\n"]
34["factor -> •true\nfactor -> •num\nterm -> •unary\nfactor -> •real\nunary -> •! unary\nterm -> •term * unary\nfactor -> •( bool )\nloc -> •id\nunary -> •factor\nterm -> •term / unary\nexpr -> expr - •term\nunary -> •- unary\nloc -> •loc [ num ]\nfactor -> •false\nfactor -> •loc\n"]
35["term -> term •/ unary\nexpr -> expr - term•\nterm -> term •* unary\n"]
36["expr -> •expr - term\nfactor -> •false\nterm -> •term / unary\nfactor -> •real\nfactor -> •true\nfactor -> •( bool )\nexpr -> •expr + term\nloc -> •loc [ num ]\nterm -> •unary\nexpr -> •term\nunary -> •! unary\nfactor -> •loc\nrel -> expr < •expr\nunary -> •- unary\nunary -> •factor\nloc -> •id\nfactor -> •num\nterm -> •term * unary\n"]
37["expr -> term•\nterm -> term •/ unary\nterm -> term •* unary\n"]
38["expr -> expr •- term\nrel -> expr < expr•\nexpr -> expr •+ term\n"]
39["term -> •term / unary\nloc -> •id\nfactor -> •real\nfactor -> •true\nexpr -> •expr + term\nexpr -> •term\nrel -> expr <= •expr\nterm -> •term * unary\nunary -> •! unary\nunary -> •factor\nexpr -> •expr - term\nfactor -> •false\nterm -> •unary\nunary -> •- unary\nfactor -> •( bool )\nfactor -> •loc\nfactor -> •num\nloc -> •loc [ num ]\n"]
40["expr -> expr •+ term\nrel -> expr <= expr•\nexpr -> expr •- term\n"]
41["expr -> •expr + term\nunary -> •factor\nexpr -> •term\nfactor -> •loc\nfactor -> •true\nterm -> •term / unary\nfactor -> •real\nterm -> •unary\nrel -> expr >= •expr\nunary -> •- unary\nfactor -> •num\nterm -> •term * unary\nunary -> •! unary\nloc -> •loc [ num ]\nfactor -> •false\nexpr -> •expr - term\nloc -> •id\nfactor -> •( bool )\n"]
42["rel -> expr >= expr•\nexpr -> expr •- term\nexpr -> expr •+ term\n"]
43["expr -> •expr - term\nunary -> •factor\nexpr -> •term\nfactor -> •loc\nterm -> •unary\nfactor -> •num\nfactor -> •( bool )\nexpr -> •expr + term\nfactor -> •real\nterm -> •term * unary\nterm -> •term / unary\nunary -> •! unary\nrel -> expr > •expr\nloc -> •id\nfactor -> •false\nfactor -> •true\nunary -> •- unary\nloc -> •loc [ num ]\n"]
44["expr -> expr •- term\nexpr -> expr •+ term\nrel -> expr > expr•\n"]
45["factor -> •false\nunary -> •factor\nrel -> •expr\nfactor -> •( bool )\nfactor -> •true\nloc -> •loc [ num ]\nexpr -> •expr + term\nloc -> •id\nfactor -> •real\nrel -> •expr <= expr\nrel -> •expr > expr\nfactor -> •num\nunary -> •- unary\nequality -> equality != •rel\nterm -> •term / unary\nfactor -> •loc\nexpr -> •term\nrel -> •expr >= expr\nterm -> •unary\nunary -> •! unary\nexpr -> •expr - term\nrel -> •expr < expr\nterm -> •term * unary\n"]
46["equality -> equality != rel•\n"]
47["join -> join •&& equality\nbool -> bool || join•\n"]
48["equality -> •equality == rel\nfactor -> •real\nequality -> •equality != rel\njoin -> join && •equality\nrel -> •expr <= expr\nterm -> •unary\nunary -> •! unary\nunary -> •factor\nunary -> •- unary\nfactor -> •num\nexpr -> •expr + term\nrel -> •expr\nexpr -> •expr - term\nfactor -> •loc\nloc -> •id\nfactor -> •( bool )\nfactor -> •false\nexpr -> •term\nterm -> •term / unary\nrel -> •expr > expr\nloc -> •loc [ num ]\nterm -> •term * unary\nequality -> •rel\nfactor -> •true\nrel -> •expr < expr\nrel -> •expr >= expr\n"]
49["equality -> equality •== rel\nequality -> equality •!= rel\njoin -> join && equality•\n"]
50["equality -> rel•\n"]
51["factor -> ( bool )•\n"]
52["join -> join •&& equality\nbool -> join•\n"]
53["unary -> - unary•\n"]
54["unary -> ! unary•\n"]
55["bool -> bool •|| join\nstmt -> if ( bool •) stmt else stmt\nstmt -> if ( bool •) stmt\n"]
56["block -> •left_curly_brace decls stmts right_curly_brace\nstmt -> •do stmt while ( bool ) ;\nstmt -> •if ( bool ) stmt else stmt\nloc -> •loc [ num ]\nstmt -> if ( bool ) •stmt else stmt\nstmt -> •break ;\nstmt -> •block\nstmt -> •if ( bool ) stmt\nstmt -> •loc = bool ;\nstmt -> •while ( bool ) stmt\nloc -> •id\nstmt -> if ( bool ) •stmt\n"]
57["stmt -> break •;\n"]
58["stmt -> break ;•\n"]
59["stmt -> while •( bool ) stmt\n"]
60["term -> •term * unary\nbool -> •join\njoin -> •join && equality\nequality -> •equality != rel\nunary -> •factor\nexpr -> •expr + term\nrel -> •expr > expr\nterm -> •unary\nstmt -> while ( •bool ) stmt\nfactor -> •num\nunary -> •- unary\nunary -> •! unary\nfactor -> •( bool )\nrel -> •expr <= expr\nexpr -> •expr - term\nloc -> •id\nfactor -> •real\nfactor -> •false\nrel -> •expr >= expr\njoin -> •equality\nloc -> •loc [ num ]\nbool -> •bool || join\nexpr -> •term\nequality -> •rel\nrel -> •expr\nequality -> •equality == rel\nfactor -> •loc\nterm -> •term / unary\nfactor -> •true\nrel -> •expr < expr\n"]
61["stmt -> while ( bool •) stmt\nbool -> bool •|| join\n"]
62["stmt -> •break ;\nblock -> •left_curly_brace decls stmts right_curly_brace\nstmt -> •do stmt while ( bool ) ;\nloc -> •id\nstmt -> while ( bool ) •stmt\nstmt -> •while ( bool ) stmt\nstmt -> •block\nstmt -> •loc = bool ;\nstmt -> •if ( bool ) stmt\nloc -> •loc [ num ]\nstmt -> •if ( bool ) stmt else stmt\n"]
63["stmt -> •while ( bool ) stmt\nstmt -> •loc = bool ;\nstmt -> •if ( bool ) stmt\nblock -> •left_curly_brace decls stmts right_curly_brace\nloc -> •loc [ num ]\nstmt -> •if ( bool ) stmt else stmt\nstmt -> •break ;\nstmt -> do •stmt while ( bool ) ;\nloc -> •id\nstmt -> •do stmt while ( bool ) ;\nstmt -> •block\n"]
64["loc -> loc •[ num ]\nstmt -> loc •= bool ;\n"]
65["expr -> •expr + term\nfactor -> •loc\nexpr -> •expr - term\nfactor -> •( bool )\nterm -> •term * unary\nterm -> •term / unary\njoin -> •join && equality\nterm -> •unary\nunary -> •! unary\njoin -> •equality\nbool -> •bool || join\nstmt -> loc = •bool ;\nunary -> •factor\nfactor -> •num\nrel -> •expr < expr\nequality -> •equality == rel\nfactor -> •false\nrel -> •expr >= expr\nfactor -> •real\nbool -> •join\nrel -> •expr > expr\nrel -> •expr\nequality -> •equality != rel\nloc -> •id\nunary -> •- unary\nloc -> •loc [ num ]\nexpr -> •term\nfactor -> •true\nrel -> •expr <= expr\nequality -> •rel\n"]
66["bool -> bool •|| join\nstmt -> loc = bool •;\n"]
67["stmt -> loc = bool ;•\n"]
68["stmt -> do stmt •while ( bool ) ;\n"]
69["stmt -> do stmt while •( bool ) ;\n"]
70["factor -> •false\nunary -> •factor\nequality -> •equality == rel\nfactor -> •true\njoin -> •equality\nunary -> •! unary\nexpr -> •expr - term\nrel -> •expr >= expr\nbool -> •join\nbool -> •bool || join\nfactor -> •loc\nfactor -> •real\nterm -> •unary\nloc -> •loc [ num ]\nrel -> •expr < expr\nrel -> •expr\nrel -> •expr <= expr\nterm -> •term * unary\nequality -> •equality != rel\nequality -> •rel\nexpr -> •term\nfactor -> •( bool )\nfactor -> •num\njoin -> •join && equality\nrel -> •expr > expr\nstmt -> do stmt while ( •bool ) ;\nexpr -> •expr + term\nterm -> •term / unary\nloc -> •id\nunary -> •- unary\n"]
71["bool -> bool •|| join\nstmt -> do stmt while ( bool •) ;\n"]
72["stmt -> do stmt while ( bool ) •;\n"]
73["stmt -> do stmt while ( bool ) ;•\n"]
74["stmt -> while ( bool ) stmt•\n"]
75["stmt -> if ( bool ) stmt •else stmt\nstmt -> if ( bool ) stmt•\n"]
76["stmt -> •if ( bool ) stmt\nstmt -> if ( bool ) stmt else •stmt\nstmt -> •do stmt while ( bool ) ;\nstmt -> •loc = bool ;\nloc -> •id\nstmt -> •break ;\nstmt -> •if ( bool ) stmt else stmt\nblock -> •left_curly_brace decls stmts right_curly_brace\nloc -> •loc [ num ]\nstmt -> •while ( bool ) stmt\nstmt -> •block\n"]
77["stmt -> if ( bool ) stmt else stmt•\n"]
78["type -> basic•\n"]
79["block -> •left_curly_brace decls stmts right_curly_brace\nstmts -> stmts •stmt\nstmt -> •while ( bool ) stmt\nstmt -> •block\nstmt -> •if ( bool ) stmt\nstmt -> •loc = bool ;\nstmt -> •if ( bool ) stmt else stmt\nstmt -> •do stmt while ( bool ) ;\nloc -> •loc [ num ]\nloc -> •id\nstmt -> •break ;\n"]
80["stmts -> stmts stmt•\n"]
81["stmt -> •while ( bool ) stmt\nblock -> left_curly_brace decls •stmts right_curly_brace\ntype -> •basic\nstmt -> •if ( bool ) stmt\nloc -> •loc [ num ]\nstmt -> •break ;\nstmt -> •loc = bool ;\nblock -> •left_curly_brace decls stmts right_curly_brace\nloc -> •id\nstmt -> •if ( bool ) stmt else stmt\nstmts -> •stmts stmt\nstmt -> •do stmt while ( bool ) ;\ndecls -> decls •decl\ndecl -> •type id ;\ntype -> •type [ num ]\nstmt -> •block\n"]
82["block -> left_curly_brace decls stmts •right_curly_brace\nstmt -> •do stmt while ( bool ) ;\nloc -> •loc [ num ]\nstmt -> •break ;\nstmts -> stmts •stmt\nstmt -> •loc = bool ;\nstmt -> •while ( bool ) stmt\nloc -> •id\nstmt -> •if ( bool ) stmt else stmt\nstmt -> •block\nstmt -> •if ( bool ) stmt\nblock -> •left_curly_brace decls stmts right_curly_brace\n"]
83["block -> left_curly_brace decls stmts right_curly_brace•\n"]
84["decls -> decls decl•\n"]
85["decl -> type •id ;\ntype -> type •[ num ]\n"]
86["decl -> type id •;\n"]
87["decl -> type id ;•\n"]
88["type -> type [ •num ]\n"]
89["type -> type [ num •]\n"]
90["type -> type [ num ]•\n"]

0--"block"-->1
0--"S"-->2
0--"program"-->3
4--"block"-->5
7--"id"-->8
9--"num"-->10
9--"true"-->11
12--"false"-->13
12--"!"-->9
12--"true"-->11
12--"-"-->12
17--"]"-->18
16--"num"-->17
15--"["-->16
14--"loc"-->15
20--"id"-->8
20--"num"-->10
20--"factor"-->21
23--"rel"-->24
23--"("-->14
23--"id"-->8
23--"false"-->13
23--"loc"-->15
26--"unary"-->27
26--"!"-->9
26--"-"-->12
26--"false"-->13
29--"factor"-->21
29--"num"-->10
29--"("-->14
29--"-"-->12
29--"real"-->30
29--"false"-->13
29--"unary"-->31
29--"loc"-->15
29--"id"-->8
29--"!"-->9
29--"true"-->11
28--"/"-->29
32--"("-->14
32--"-"-->12
32--"false"-->13
32--"factor"-->21
32--"real"-->30
32--"num"-->10
32--"id"-->8
32--"!"-->9
32--"loc"-->15
32--"true"-->11
32--"unary"-->33
28--"*"-->32
26--"term"-->28
26--"factor"-->21
26--"num"-->10
26--"("-->14
26--"true"-->11
26--"real"-->30
26--"loc"-->15
26--"id"-->8
25--"+"-->26
34--"id"-->8
34--"!"-->9
34--"-"-->12
34--"unary"-->27
34--"loc"-->15
34--"real"-->30
34--"false"-->13
34--"("-->14
34--"factor"-->21
34--"true"-->11
35--"/"-->29
35--"*"-->32
34--"term"-->35
34--"num"-->10
25--"-"-->34
36--"-"-->12
36--"("-->14
36--"true"-->11
36--"id"-->8
36--"false"-->13
36--"real"-->30
36--"factor"-->21
37--"/"-->29
37--"*"-->32
36--"term"-->37
36--"num"-->10
36--"!"-->9
38--"+"-->26
38--"-"-->34
36--"expr"-->38
36--"unary"-->27
36--"loc"-->15
25--"<"-->36
39--"loc"-->15
39--"unary"-->27
39--"("-->14
39--"id"-->8
39--"true"-->11
39--"real"-->30
39--"term"-->37
39--"factor"-->21
39--"false"-->13
40--"+"-->26
40--"-"-->34
39--"expr"-->40
39--"!"-->9
39--"-"-->12
39--"num"-->10
25--"<="-->39
42--"-"-->34
42--"+"-->26
41--"expr"-->42
41--"real"-->30
41--"unary"-->27
41--"loc"-->15
41--"true"-->11
41--"term"-->37
41--"!"-->9
41--"false"-->13
41--"id"-->8
41--"num"-->10
41--"("-->14
41--"factor"-->21
41--"-"-->12
25--">="-->41
43--"id"-->8
43--"!"-->9
43--"false"-->13
43--"("-->14
43--"term"-->37
43--"true"-->11
44--"-"-->34
44--"+"-->26
43--"expr"-->44
43--"factor"-->21
43--"loc"-->15
43--"unary"-->27
43--"real"-->30
43--"-"-->12
43--"num"-->10
25--">"-->43
23--"expr"-->25
23--"num"-->10
23--"true"-->11
23--"!"-->9
23--"factor"-->21
23--"term"-->37
23--"real"-->30
23--"unary"-->27
23--"-"-->12
22--"=="-->23
45--"factor"-->21
45--"!"-->9
45--"("-->14
45--"id"-->8
45--"-"-->12
45--"real"-->30
45--"num"-->10
45--"rel"-->46
45--"unary"-->27
45--"loc"-->15
45--"expr"-->25
45--"term"-->37
45--"true"-->11
45--"false"-->13
22--"!="-->45
20--"equality"-->22
20--"expr"-->25
20--"unary"-->27
48--"("-->14
48--"false"-->13
48--"expr"-->25
48--"term"-->37
48--"!"-->9
49--"=="-->23
49--"!="-->45
48--"equality"-->49
48--"-"-->12
48--"rel"-->50
48--"factor"-->21
48--"id"-->8
48--"real"-->30
48--"unary"-->27
48--"loc"-->15
48--"num"-->10
48--"true"-->11
47--"&&"-->48
20--"join"-->47
20--"loc"-->15
20--"!"-->9
20--"real"-->30
20--"true"-->11
20--"-"-->12
20--"term"-->37
20--"false"-->13
20--"rel"-->50
20--"("-->14
19--"||"-->20
19--")"-->51
14--"bool"-->19
14--"term"-->37
14--"rel"-->50
52--"&&"-->48
14--"join"-->52
14--"unary"-->27
14--"equality"-->22
14--"id"-->8
14--"!"-->9
14--"expr"-->25
14--"num"-->10
14--"true"-->11
14--"("-->14
14--"-"-->12
14--"real"-->30
14--"factor"-->21
14--"false"-->13
12--"("-->14
12--"num"-->10
12--"loc"-->15
12--"unary"-->53
12--"id"-->8
12--"real"-->30
12--"factor"-->21
9--"-"-->12
9--"real"-->30
9--"unary"-->54
9--"("-->14
9--"!"-->9
9--"loc"-->15
9--"false"-->13
9--"factor"-->21
9--"id"-->8
7--"!"-->9
7--"-"-->12
7--"false"-->13
7--"rel"-->50
7--"("-->14
7--"expr"-->25
7--"join"-->52
7--"equality"-->22
7--"term"-->37
7--"loc"-->15
7--"real"-->30
7--"num"-->10
7--"true"-->11
7--"factor"-->21
55--"||"-->20
57--";"-->58
56--"break"-->57
60--"equality"-->22
60--"num"-->10
60--"term"-->37
60--"id"-->8
60--"loc"-->15
60--"-"-->12
60--"unary"-->27
60--"!"-->9
62--"while"-->59
62--"left_curly_brace"-->4
64--"["-->16
65--"unary"-->27
65--"num"-->10
65--"join"-->52
65--"id"-->8
65--"!"-->9
65--"-"-->12
65--"("-->14
65--"rel"-->50
65--"true"-->11
65--"expr"-->25
65--"real"-->30
65--"loc"-->15
65--"factor"-->21
65--"equality"-->22
65--"term"-->37
66--";"-->67
66--"||"-->20
65--"bool"-->66
65--"false"-->13
64--"="-->65
63--"loc"-->64
63--"break"-->57
70--"!"-->9
70--"-"-->12
70--"term"-->37
72--";"-->73
71--")"-->72
71--"||"-->20
70--"bool"-->71
70--"equality"-->22
70--"false"-->13
70--"expr"-->25
70--"join"-->52
70--"loc"-->15
70--"real"-->30
70--"id"-->8
70--"true"-->11
70--"factor"-->21
70--"num"-->10
70--"("-->14
70--"unary"-->27
70--"rel"-->50
69--"("-->70
68--"while"-->69
63--"stmt"-->68
63--"while"-->59
63--"block"-->5
63--"id"-->8
63--"if"-->6
63--"left_curly_brace"-->4
63--"do"-->63
62--"do"-->63
62--"id"-->8
62--"loc"-->64
62--"stmt"-->74
62--"break"-->57
62--"if"-->6
62--"block"-->5
61--")"-->62
61--"||"-->20
60--"bool"-->61
60--"real"-->30
60--"("-->14
60--"rel"-->50
60--"true"-->11
60--"factor"-->21
60--"expr"-->25
60--"false"-->13
60--"join"-->52
59--"("-->60
56--"while"-->59
56--"block"-->5
76--"left_curly_brace"-->4
76--"break"-->57
76--"stmt"-->77
76--"do"-->63
76--"block"-->5
76--"id"-->8
76--"loc"-->64
76--"while"-->59
76--"if"-->6
75--"else"-->76
56--"stmt"-->75
56--"loc"-->64
56--"left_curly_brace"-->4
56--"if"-->6
56--"do"-->63
56--"id"-->8
55--")"-->56
7--"bool"-->55
7--"unary"-->27
6--"("-->7
4--"if"-->6
4--"id"-->8
4--"basic"-->78
79--"id"-->8
79--"if"-->6
79--"stmt"-->80
79--"while"-->59
79--"block"-->5
79--"loc"-->64
79--"do"-->63
79--"break"-->57
79--"left_curly_brace"-->4
4--"stmts"-->79
4--"while"-->59
4--"break"-->57
4--"loc"-->64
81--"break"-->57
81--"id"-->8
81--"do"-->63
81--"if"-->6
81--"loc"-->64
81--"while"-->59
81--"left_curly_brace"-->4
82--"do"-->63
82--"left_curly_brace"-->4
82--"stmt"-->80
82--"block"-->5
82--"right_curly_brace"-->83
82--"if"-->6
82--"loc"-->64
82--"while"-->59
82--"break"-->57
82--"id"-->8
81--"stmts"-->82
81--"decl"-->84
81--"basic"-->78
86--";"-->87
85--"id"-->86
89--"]"-->90
88--"num"-->89
85--"["-->88
81--"type"-->85
81--"block"-->5
4--"decls"-->81
4--"type"-->85
4--"do"-->63
4--"left_curly_brace"-->4
0--"left_curly_brace"-->4

```

---
## Action Table
```txt
State 0:
program: Shift(3)
S: Shift(2)
block: Shift(1)
left_curly_brace: Shift(4)
__$__: Accept
===================
State 1:
__$__: Reduce(ReduceDerivation { left: "program", right: ["block"] })
===================
State 2:
__$__: Reduce(ReduceDerivation { left: "__DUMMY_START__", right: ["S"] })
===================
State 3:
__$__: Reduce(ReduceDerivation { left: "S", right: ["program"] })
===================
State 4:
do: Shift(63)
block: Shift(5)
break: Shift(57)
while: Shift(59)
decls: Shift(81)
if: Shift(6)
left_curly_brace: Shift(4)
basic: Shift(78)
loc: Shift(64)
id: Shift(8)
stmts: Shift(79)
type: Shift(85)
===================
State 5:
id: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
do: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
if: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
right_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
break: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
while: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
else: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
===================
State 6:
(: Shift(7)
===================
State 7:
factor: Shift(21)
rel: Shift(50)
real: Shift(30)
unary: Shift(27)
id: Shift(8)
false: Shift(13)
equality: Shift(22)
bool: Shift(55)
term: Shift(37)
!: Shift(9)
true: Shift(11)
-: Shift(12)
(: Shift(14)
join: Shift(52)
num: Shift(10)
loc: Shift(15)
expr: Shift(25)
===================
State 8:
): Reduce(ReduceDerivation { left: "loc", right: ["id"] })
/: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
;: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
!=: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
<: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
-: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
+: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
[: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
*: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
>: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
>=: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
==: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
&&: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
<=: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
=: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
||: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
===================
State 9:
(: Shift(14)
!: Shift(9)
num: Shift(10)
loc: Shift(15)
false: Shift(13)
factor: Shift(21)
id: Shift(8)
real: Shift(30)
unary: Shift(54)
true: Shift(11)
-: Shift(12)
===================
State 10:
;: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
*: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
-: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
>: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
==: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
+: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
!=: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
>=: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
<: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
&&: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
): Reduce(ReduceDerivation { left: "factor", right: ["num"] })
<=: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
/: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
||: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
===================
State 11:
||: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
>: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
-: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
*: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
<: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
&&: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
): Reduce(ReduceDerivation { left: "factor", right: ["true"] })
>=: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
!=: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
/: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
==: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
<=: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
+: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
;: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
===================
State 12:
unary: Shift(53)
(: Shift(14)
num: Shift(10)
real: Shift(30)
!: Shift(9)
loc: Shift(15)
id: Shift(8)
factor: Shift(21)
false: Shift(13)
true: Shift(11)
-: Shift(12)
===================
State 13:
*: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
-: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
==: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
;: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
>=: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
!=: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
&&: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
<: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
/: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
<=: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
+: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
): Reduce(ReduceDerivation { left: "factor", right: ["false"] })
||: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
>: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
===================
State 14:
(: Shift(14)
true: Shift(11)
equality: Shift(22)
real: Shift(30)
loc: Shift(15)
unary: Shift(27)
!: Shift(9)
-: Shift(12)
join: Shift(52)
expr: Shift(25)
id: Shift(8)
rel: Shift(50)
num: Shift(10)
false: Shift(13)
term: Shift(37)
factor: Shift(21)
bool: Shift(19)
===================
State 15:
+: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
<=: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
<: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
>: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
-: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
!=: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
[: Shift(16)
||: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
>=: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
): Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
/: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
;: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
&&: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
*: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
==: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
===================
State 16:
num: Shift(17)
===================
State 17:
]: Shift(18)
===================
State 18:
-: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
<: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
): Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
;: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
<=: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
||: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
!=: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
/: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
=: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
>: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
>=: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
[: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
*: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
&&: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
==: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
+: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
===================
State 19:
): Shift(51)
||: Shift(20)
===================
State 20:
term: Shift(37)
unary: Shift(27)
id: Shift(8)
factor: Shift(21)
-: Shift(12)
join: Shift(47)
false: Shift(13)
equality: Shift(22)
(: Shift(14)
loc: Shift(15)
true: Shift(11)
num: Shift(10)
!: Shift(9)
real: Shift(30)
expr: Shift(25)
rel: Shift(50)
===================
State 21:
&&: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
==: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
>: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
+: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
/: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
<: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
||: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
>=: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
-: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
*: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
<=: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
;: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
): Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
!=: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
===================
State 22:
&&: Reduce(ReduceDerivation { left: "join", right: ["equality"] })
): Reduce(ReduceDerivation { left: "join", right: ["equality"] })
||: Reduce(ReduceDerivation { left: "join", right: ["equality"] })
;: Reduce(ReduceDerivation { left: "join", right: ["equality"] })
==: Shift(23)
!=: Shift(45)
===================
State 23:
term: Shift(37)
unary: Shift(27)
-: Shift(12)
num: Shift(10)
true: Shift(11)
real: Shift(30)
rel: Shift(24)
loc: Shift(15)
id: Shift(8)
(: Shift(14)
!: Shift(9)
false: Shift(13)
factor: Shift(21)
expr: Shift(25)
===================
State 24:
||: Reduce(ReduceDerivation { left: "equality", right: ["equality", "==", "rel"] })
!=: Reduce(ReduceDerivation { left: "equality", right: ["equality", "==", "rel"] })
==: Reduce(ReduceDerivation { left: "equality", right: ["equality", "==", "rel"] })
): Reduce(ReduceDerivation { left: "equality", right: ["equality", "==", "rel"] })
&&: Reduce(ReduceDerivation { left: "equality", right: ["equality", "==", "rel"] })
;: Reduce(ReduceDerivation { left: "equality", right: ["equality", "==", "rel"] })
===================
State 25:
>: Shift(43)
;: Reduce(ReduceDerivation { left: "rel", right: ["expr"] })
<: Shift(36)
!=: Reduce(ReduceDerivation { left: "rel", right: ["expr"] })
&&: Reduce(ReduceDerivation { left: "rel", right: ["expr"] })
==: Reduce(ReduceDerivation { left: "rel", right: ["expr"] })
-: Shift(34)
): Reduce(ReduceDerivation { left: "rel", right: ["expr"] })
<=: Shift(39)
+: Shift(26)
||: Reduce(ReduceDerivation { left: "rel", right: ["expr"] })
>=: Shift(41)
===================
State 26:
num: Shift(10)
term: Shift(28)
factor: Shift(21)
!: Shift(9)
-: Shift(12)
(: Shift(14)
loc: Shift(15)
true: Shift(11)
id: Shift(8)
false: Shift(13)
unary: Shift(27)
real: Shift(30)
===================
State 27:
;: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
||: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
*: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
>=: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
<: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
&&: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
+: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
!=: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
<=: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
/: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
==: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
>: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
-: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
): Reduce(ReduceDerivation { left: "term", right: ["unary"] })
===================
State 28:
*: Shift(32)
<=: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
+: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
!=: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
>=: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
==: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
-: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
<: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
>: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
&&: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
): Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
;: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
/: Shift(29)
||: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
===================
State 29:
num: Shift(10)
real: Shift(30)
true: Shift(11)
false: Shift(13)
-: Shift(12)
!: Shift(9)
unary: Shift(31)
(: Shift(14)
factor: Shift(21)
id: Shift(8)
loc: Shift(15)
===================
State 30:
<=: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
==: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
+: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
>=: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
/: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
>: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
-: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
||: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
*: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
): Reduce(ReduceDerivation { left: "factor", right: ["real"] })
;: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
<: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
!=: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
&&: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
===================
State 31:
-: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
+: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
*: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
<=: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
>=: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
/: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
<: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
;: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
): Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
&&: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
>: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
==: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
!=: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
||: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
===================
State 32:
(: Shift(14)
loc: Shift(15)
true: Shift(11)
num: Shift(10)
factor: Shift(21)
!: Shift(9)
false: Shift(13)
id: Shift(8)
unary: Shift(33)
real: Shift(30)
-: Shift(12)
===================
State 33:
+: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
||: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
<=: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
>=: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
-: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
/: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
;: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
): Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
*: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
!=: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
<: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
>: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
==: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
&&: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
===================
State 34:
-: Shift(12)
unary: Shift(27)
(: Shift(14)
true: Shift(11)
loc: Shift(15)
false: Shift(13)
num: Shift(10)
factor: Shift(21)
real: Shift(30)
term: Shift(35)
id: Shift(8)
!: Shift(9)
===================
State 35:
>=: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
||: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
;: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
): Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
+: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
==: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
*: Shift(32)
&&: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
<=: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
<: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
!=: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
>: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
/: Shift(29)
-: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
===================
State 36:
loc: Shift(15)
term: Shift(37)
expr: Shift(38)
id: Shift(8)
factor: Shift(21)
!: Shift(9)
true: Shift(11)
(: Shift(14)
num: Shift(10)
false: Shift(13)
unary: Shift(27)
real: Shift(30)
-: Shift(12)
===================
State 37:
>=: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
;: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
*: Shift(32)
+: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
>: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
/: Shift(29)
||: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
<: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
&&: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
==: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
): Reduce(ReduceDerivation { left: "expr", right: ["term"] })
<=: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
-: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
!=: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
===================
State 38:
+: Shift(26)
||: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<", "expr"] })
;: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<", "expr"] })
&&: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<", "expr"] })
!=: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<", "expr"] })
): Reduce(ReduceDerivation { left: "rel", right: ["expr", "<", "expr"] })
==: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<", "expr"] })
-: Shift(34)
===================
State 39:
true: Shift(11)
false: Shift(13)
!: Shift(9)
id: Shift(8)
term: Shift(37)
factor: Shift(21)
real: Shift(30)
(: Shift(14)
expr: Shift(40)
num: Shift(10)
-: Shift(12)
unary: Shift(27)
loc: Shift(15)
===================
State 40:
-: Shift(34)
||: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<=", "expr"] })
!=: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<=", "expr"] })
;: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<=", "expr"] })
==: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<=", "expr"] })
): Reduce(ReduceDerivation { left: "rel", right: ["expr", "<=", "expr"] })
&&: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<=", "expr"] })
+: Shift(26)
===================
State 41:
id: Shift(8)
(: Shift(14)
loc: Shift(15)
expr: Shift(42)
true: Shift(11)
real: Shift(30)
factor: Shift(21)
num: Shift(10)
unary: Shift(27)
false: Shift(13)
-: Shift(12)
!: Shift(9)
term: Shift(37)
===================
State 42:
-: Shift(34)
==: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">=", "expr"] })
&&: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">=", "expr"] })
;: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">=", "expr"] })
+: Shift(26)
||: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">=", "expr"] })
!=: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">=", "expr"] })
): Reduce(ReduceDerivation { left: "rel", right: ["expr", ">=", "expr"] })
===================
State 43:
loc: Shift(15)
true: Shift(11)
-: Shift(12)
num: Shift(10)
(: Shift(14)
real: Shift(30)
false: Shift(13)
id: Shift(8)
term: Shift(37)
factor: Shift(21)
unary: Shift(27)
!: Shift(9)
expr: Shift(44)
===================
State 44:
-: Shift(34)
&&: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">", "expr"] })
): Reduce(ReduceDerivation { left: "rel", right: ["expr", ">", "expr"] })
;: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">", "expr"] })
+: Shift(26)
!=: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">", "expr"] })
||: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">", "expr"] })
==: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">", "expr"] })
===================
State 45:
false: Shift(13)
factor: Shift(21)
rel: Shift(46)
-: Shift(12)
num: Shift(10)
term: Shift(37)
id: Shift(8)
(: Shift(14)
!: Shift(9)
unary: Shift(27)
real: Shift(30)
loc: Shift(15)
expr: Shift(25)
true: Shift(11)
===================
State 46:
||: Reduce(ReduceDerivation { left: "equality", right: ["equality", "!=", "rel"] })
==: Reduce(ReduceDerivation { left: "equality", right: ["equality", "!=", "rel"] })
&&: Reduce(ReduceDerivation { left: "equality", right: ["equality", "!=", "rel"] })
!=: Reduce(ReduceDerivation { left: "equality", right: ["equality", "!=", "rel"] })
;: Reduce(ReduceDerivation { left: "equality", right: ["equality", "!=", "rel"] })
): Reduce(ReduceDerivation { left: "equality", right: ["equality", "!=", "rel"] })
===================
State 47:
||: Reduce(ReduceDerivation { left: "bool", right: ["bool", "||", "join"] })
&&: Shift(48)
;: Reduce(ReduceDerivation { left: "bool", right: ["bool", "||", "join"] })
): Reduce(ReduceDerivation { left: "bool", right: ["bool", "||", "join"] })
===================
State 48:
loc: Shift(15)
expr: Shift(25)
!: Shift(9)
true: Shift(11)
term: Shift(37)
num: Shift(10)
factor: Shift(21)
(: Shift(14)
false: Shift(13)
-: Shift(12)
real: Shift(30)
unary: Shift(27)
rel: Shift(50)
id: Shift(8)
equality: Shift(49)
===================
State 49:
&&: Reduce(ReduceDerivation { left: "join", right: ["join", "&&", "equality"] })
==: Shift(23)
): Reduce(ReduceDerivation { left: "join", right: ["join", "&&", "equality"] })
||: Reduce(ReduceDerivation { left: "join", right: ["join", "&&", "equality"] })
!=: Shift(45)
;: Reduce(ReduceDerivation { left: "join", right: ["join", "&&", "equality"] })
===================
State 50:
&&: Reduce(ReduceDerivation { left: "equality", right: ["rel"] })
!=: Reduce(ReduceDerivation { left: "equality", right: ["rel"] })
==: Reduce(ReduceDerivation { left: "equality", right: ["rel"] })
): Reduce(ReduceDerivation { left: "equality", right: ["rel"] })
;: Reduce(ReduceDerivation { left: "equality", right: ["rel"] })
||: Reduce(ReduceDerivation { left: "equality", right: ["rel"] })
===================
State 51:
): Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
-: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
/: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
*: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
!=: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
&&: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
>: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
+: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
||: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
>=: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
<: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
==: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
<=: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
;: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
===================
State 52:
;: Reduce(ReduceDerivation { left: "bool", right: ["join"] })
&&: Shift(48)
||: Reduce(ReduceDerivation { left: "bool", right: ["join"] })
): Reduce(ReduceDerivation { left: "bool", right: ["join"] })
===================
State 53:
+: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
): Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
==: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
||: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
>=: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
!=: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
&&: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
-: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
*: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
<=: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
;: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
>: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
/: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
<: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
===================
State 54:
<=: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
!=: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
||: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
;: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
/: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
==: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
+: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
>: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
>=: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
&&: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
-: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
<: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
*: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
): Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
===================
State 55:
||: Shift(20)
): Shift(56)
===================
State 56:
break: Shift(57)
stmt: Shift(75)
left_curly_brace: Shift(4)
do: Shift(63)
while: Shift(59)
if: Shift(6)
block: Shift(5)
id: Shift(8)
loc: Shift(64)
===================
State 57:
;: Shift(58)
===================
State 58:
if: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
do: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
id: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
while: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
else: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
break: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
right_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
===================
State 59:
(: Shift(60)
===================
State 60:
loc: Shift(15)
rel: Shift(50)
id: Shift(8)
real: Shift(30)
equality: Shift(22)
term: Shift(37)
!: Shift(9)
true: Shift(11)
expr: Shift(25)
(: Shift(14)
false: Shift(13)
factor: Shift(21)
bool: Shift(61)
-: Shift(12)
num: Shift(10)
join: Shift(52)
unary: Shift(27)
===================
State 61:
): Shift(62)
||: Shift(20)
===================
State 62:
do: Shift(63)
id: Shift(8)
if: Shift(6)
left_curly_brace: Shift(4)
loc: Shift(64)
stmt: Shift(74)
block: Shift(5)
break: Shift(57)
while: Shift(59)
===================
State 63:
break: Shift(57)
stmt: Shift(68)
do: Shift(63)
while: Shift(59)
block: Shift(5)
if: Shift(6)
loc: Shift(64)
left_curly_brace: Shift(4)
id: Shift(8)
===================
State 64:
=: Shift(65)
[: Shift(16)
===================
State 65:
real: Shift(30)
-: Shift(12)
term: Shift(37)
unary: Shift(27)
id: Shift(8)
expr: Shift(25)
equality: Shift(22)
loc: Shift(15)
bool: Shift(66)
num: Shift(10)
(: Shift(14)
rel: Shift(50)
true: Shift(11)
factor: Shift(21)
!: Shift(9)
false: Shift(13)
join: Shift(52)
===================
State 66:
;: Shift(67)
||: Shift(20)
===================
State 67:
else: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
while: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
if: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
id: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
right_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
do: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
break: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
===================
State 68:
while: Shift(69)
===================
State 69:
(: Shift(70)
===================
State 70:
-: Shift(12)
real: Shift(30)
term: Shift(37)
factor: Shift(21)
equality: Shift(22)
loc: Shift(15)
true: Shift(11)
unary: Shift(27)
num: Shift(10)
join: Shift(52)
!: Shift(9)
false: Shift(13)
bool: Shift(71)
expr: Shift(25)
id: Shift(8)
(: Shift(14)
rel: Shift(50)
===================
State 71:
||: Shift(20)
): Shift(72)
===================
State 72:
;: Shift(73)
===================
State 73:
left_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
break: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
if: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
right_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
else: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
do: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
id: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
while: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
===================
State 74:
left_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
else: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
while: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
id: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
do: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
if: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
right_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
break: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
===================
State 75:
id: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
do: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
else: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
right_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
while: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
if: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
break: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
===================
State 76:
left_curly_brace: Shift(4)
block: Shift(5)
break: Shift(57)
do: Shift(63)
id: Shift(8)
stmt: Shift(77)
while: Shift(59)
loc: Shift(64)
if: Shift(6)
===================
State 77:
while: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
id: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
else: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
break: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
right_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
do: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
if: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
===================
State 78:
[: Reduce(ReduceDerivation { left: "type", right: ["basic"] })
id: Reduce(ReduceDerivation { left: "type", right: ["basic"] })
===================
State 79:
while: Shift(59)
break: Shift(57)
stmt: Shift(80)
do: Shift(63)
left_curly_brace: Shift(4)
block: Shift(5)
loc: Shift(64)
if: Shift(6)
id: Shift(8)
===================
State 80:
if: Reduce(ReduceDerivation { left: "stmts", right: ["stmts", "stmt"] })
break: Reduce(ReduceDerivation { left: "stmts", right: ["stmts", "stmt"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmts", right: ["stmts", "stmt"] })
id: Reduce(ReduceDerivation { left: "stmts", right: ["stmts", "stmt"] })
while: Reduce(ReduceDerivation { left: "stmts", right: ["stmts", "stmt"] })
right_curly_brace: Reduce(ReduceDerivation { left: "stmts", right: ["stmts", "stmt"] })
do: Reduce(ReduceDerivation { left: "stmts", right: ["stmts", "stmt"] })
===================
State 81:
decl: Shift(84)
break: Shift(57)
type: Shift(85)
while: Shift(59)
if: Shift(6)
id: Shift(8)
do: Shift(63)
left_curly_brace: Shift(4)
loc: Shift(64)
stmts: Shift(82)
basic: Shift(78)
block: Shift(5)
===================
State 82:
do: Shift(63)
while: Shift(59)
if: Shift(6)
loc: Shift(64)
break: Shift(57)
id: Shift(8)
stmt: Shift(80)
left_curly_brace: Shift(4)
block: Shift(5)
right_curly_brace: Shift(83)
===================
State 83:
right_curly_brace: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
left_curly_brace: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
id: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
else: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
break: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
__$__: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
do: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
if: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
while: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
===================
State 84:
break: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
if: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
id: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
basic: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
while: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
left_curly_brace: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
do: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
===================
State 85:
id: Shift(86)
[: Shift(88)
===================
State 86:
;: Shift(87)
===================
State 87:
left_curly_brace: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
do: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
while: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
id: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
if: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
break: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
basic: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
===================
State 88:
num: Shift(89)
===================
State 89:
]: Shift(90)
===================
State 90:
[: Reduce(ReduceDerivation { left: "type", right: ["type", "[", "num", "]"] })
id: Reduce(ReduceDerivation { left: "type", right: ["type", "[", "num", "]"] })
===================

```
---
generated by rparser
RockRockWhite 2023
    