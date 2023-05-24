
# Generated Info

## Base Info
- config_file: sample.rparser
- output_file: sample.rs
- time: 2023-05-20 16:54:34.355653 +08:00

---

## DFA Graph
```mermaid
graph LR
0["block -> •left_curly_brace decls stmts right_curly_brace\nS -> •program\n__DUMMY_START__ -> •S\nprogram -> •block\n"]
1["program -> block•\n"]
2["S -> program•\n"]
3["__DUMMY_START__ -> S•\n"]
4["decls -> •decls decl\ndecls -> •\nblock -> left_curly_brace •decls stmts right_curly_brace\n"]
5["stmts -> •\ndecls -> decls •decl\ntype -> •type [ num ]\ntype -> •basic\nblock -> left_curly_brace decls •stmts right_curly_brace\ndecl -> •type id ;\nstmts -> •stmts stmt\n"]
6["decls -> decls decl•\n"]
7["stmt -> •do stmt while ( bool ) ;\nloc -> •id\nstmt -> •loc = bool ;\nstmt -> •if ( bool ) stmt\nstmts -> stmts •stmt\nblock -> left_curly_brace decls stmts •right_curly_brace\nblock -> •left_curly_brace decls stmts right_curly_brace\nloc -> •loc [ num ]\nstmt -> •block\nstmt -> •break ;\nstmt -> •if ( bool ) stmt else stmt\nstmt -> •while ( bool ) stmt\n"]
8["stmt -> while •( bool ) stmt\n"]
9["factor -> •( bool )\nstmt -> while ( •bool ) stmt\nterm -> •term * unary\nfactor -> •false\njoin -> •equality\nequality -> •equality == rel\nrel -> •expr\nrel -> •expr < expr\nfactor -> •true\nrel -> •expr <= expr\nequality -> •equality != rel\nexpr -> •expr + term\nfactor -> •loc\nexpr -> •term\nterm -> •unary\nloc -> •id\njoin -> •join && equality\nterm -> •term / unary\nequality -> •rel\nexpr -> •expr - term\nrel -> •expr > expr\nunary -> •- unary\nunary -> •! unary\nloc -> •loc [ num ]\nfactor -> •real\nfactor -> •num\nunary -> •factor\nbool -> •bool || join\nbool -> •join\nrel -> •expr >= expr\n"]
10["factor -> true•\n"]
11["factor -> num•\n"]
12["factor -> false•\n"]
13["rel -> expr •>= expr\nrel -> expr•\nrel -> expr •< expr\nexpr -> expr •- term\nrel -> expr •<= expr\nexpr -> expr •+ term\nrel -> expr •> expr\n"]
14["unary -> •factor\nfactor -> •real\nfactor -> •true\nfactor -> •( bool )\nunary -> •! unary\nloc -> •loc [ num ]\nterm -> •term / unary\nexpr -> expr + •term\nfactor -> •false\nunary -> •- unary\nterm -> •unary\nfactor -> •num\nterm -> •term * unary\nfactor -> •loc\nloc -> •id\n"]
15["factor -> •false\nfactor -> •real\nunary -> •! unary\nfactor -> •loc\nunary -> •- unary\nfactor -> •true\nunary -> •factor\nloc -> •id\nfactor -> •num\nfactor -> •( bool )\nunary -> ! •unary\nloc -> •loc [ num ]\n"]
16["loc -> id•\n"]
17["loc -> •loc [ num ]\nunary -> - •unary\nloc -> •id\nunary -> •factor\nfactor -> •true\nfactor -> •real\nunary -> •! unary\nfactor -> •( bool )\nunary -> •- unary\nfactor -> •num\nfactor -> •loc\nfactor -> •false\n"]
18["bool -> •bool || join\nunary -> •factor\nrel -> •expr\nrel -> •expr > expr\nfactor -> ( •bool )\nbool -> •join\nloc -> •id\nunary -> •! unary\nrel -> •expr >= expr\nterm -> •unary\nequality -> •equality == rel\nterm -> •term * unary\nequality -> •equality != rel\nunary -> •- unary\nfactor -> •false\nfactor -> •loc\nrel -> •expr <= expr\nterm -> •term / unary\nfactor -> •true\nexpr -> •term\nfactor -> •num\nexpr -> •expr - term\nfactor -> •real\nloc -> •loc [ num ]\nrel -> •expr < expr\nfactor -> •( bool )\njoin -> •equality\nequality -> •rel\nexpr -> •expr + term\njoin -> •join && equality\n"]
19["term -> term •* unary\nexpr -> term•\nterm -> term •/ unary\n"]
20["factor -> •loc\nfactor -> •num\nfactor -> •real\nunary -> •- unary\nfactor -> •true\nfactor -> •false\nunary -> •! unary\nloc -> •id\nfactor -> •( bool )\nterm -> term / •unary\nunary -> •factor\nloc -> •loc [ num ]\n"]
21["factor -> loc•\nloc -> loc •[ num ]\n"]
22["loc -> loc [ •num ]\n"]
23["loc -> loc [ num •]\n"]
24["loc -> loc [ num ]•\n"]
25["unary -> factor•\n"]
26["term -> term / unary•\n"]
27["factor -> real•\n"]
28["factor -> •num\nfactor -> •true\nfactor -> •( bool )\nfactor -> •real\nterm -> term * •unary\nloc -> •loc [ num ]\nloc -> •id\nunary -> •- unary\nfactor -> •false\nfactor -> •loc\nunary -> •factor\nunary -> •! unary\n"]
29["term -> term * unary•\n"]
30["equality -> rel•\n"]
31["bool -> join•\njoin -> join •&& equality\n"]
32["rel -> •expr < expr\nfactor -> •real\nequality -> •equality == rel\njoin -> join && •equality\nunary -> •- unary\nrel -> •expr > expr\nexpr -> •expr - term\nfactor -> •false\nterm -> •term / unary\nfactor -> •( bool )\nequality -> •equality != rel\nterm -> •unary\nfactor -> •loc\nunary -> •factor\nfactor -> •true\nrel -> •expr >= expr\nterm -> •term * unary\nequality -> •rel\nexpr -> •term\nunary -> •! unary\nloc -> •loc [ num ]\nrel -> •expr\nexpr -> •expr + term\nloc -> •id\nrel -> •expr <= expr\nfactor -> •num\n"]
33["term -> unary•\n"]
34["join -> join && equality•\nequality -> equality •== rel\nequality -> equality •!= rel\n"]
35["factor -> •true\nexpr -> •expr - term\nrel -> •expr\nterm -> •term * unary\nterm -> •term / unary\nrel -> •expr >= expr\nrel -> •expr <= expr\nfactor -> •num\nunary -> •! unary\nexpr -> •term\nfactor -> •( bool )\nfactor -> •loc\nunary -> •factor\nloc -> •id\nequality -> equality == •rel\nexpr -> •expr + term\nrel -> •expr < expr\nunary -> •- unary\nfactor -> •real\nfactor -> •false\nterm -> •unary\nrel -> •expr > expr\nloc -> •loc [ num ]\n"]
36["equality -> equality == rel•\n"]
37["rel -> •expr > expr\nequality -> equality != •rel\nexpr -> •term\nfactor -> •( bool )\nfactor -> •real\nexpr -> •expr - term\nfactor -> •num\nrel -> •expr < expr\nunary -> •- unary\nrel -> •expr\nloc -> •id\nfactor -> •loc\nfactor -> •false\nunary -> •! unary\nexpr -> •expr + term\nrel -> •expr <= expr\nterm -> •term * unary\nunary -> •factor\nterm -> •unary\nloc -> •loc [ num ]\nrel -> •expr >= expr\nterm -> •term / unary\nfactor -> •true\n"]
38["equality -> equality != rel•\n"]
39["equality -> equality •== rel\nequality -> equality •!= rel\njoin -> equality•\n"]
40["bool -> bool •|| join\nfactor -> ( bool •)\n"]
41["factor -> ( bool )•\n"]
42["rel -> •expr < expr\nterm -> •term * unary\nfactor -> •real\nfactor -> •( bool )\nloc -> •loc [ num ]\nrel -> •expr\nterm -> •term / unary\nterm -> •unary\nequality -> •equality == rel\njoin -> •equality\nfactor -> •false\nrel -> •expr <= expr\njoin -> •join && equality\nbool -> bool || •join\nequality -> •rel\nfactor -> •num\nunary -> •- unary\nunary -> •factor\nrel -> •expr > expr\nexpr -> •expr + term\nunary -> •! unary\nrel -> •expr >= expr\nexpr -> •term\nfactor -> •true\nequality -> •equality != rel\nexpr -> •expr - term\nfactor -> •loc\nloc -> •id\n"]
43["bool -> bool || join•\njoin -> join •&& equality\n"]
44["unary -> - unary•\n"]
45["unary -> ! unary•\n"]
46["term -> term •/ unary\nexpr -> expr + term•\nterm -> term •* unary\n"]
47["unary -> •factor\nfactor -> •false\nloc -> •loc [ num ]\nfactor -> •num\nunary -> •! unary\nexpr -> •expr + term\nfactor -> •loc\nfactor -> •real\nloc -> •id\nrel -> expr < •expr\nfactor -> •( bool )\nterm -> •unary\nterm -> •term / unary\nfactor -> •true\nexpr -> •expr - term\nterm -> •term * unary\nexpr -> •term\nunary -> •- unary\n"]
48["expr -> expr •+ term\nrel -> expr < expr•\nexpr -> expr •- term\n"]
49["term -> •term * unary\nfactor -> •false\nfactor -> •true\nfactor -> •real\nloc -> •id\nunary -> •- unary\nfactor -> •loc\nloc -> •loc [ num ]\nexpr -> expr - •term\nterm -> •unary\nterm -> •term / unary\nunary -> •! unary\nunary -> •factor\nfactor -> •( bool )\nfactor -> •num\n"]
50["term -> term •* unary\nterm -> term •/ unary\nexpr -> expr - term•\n"]
51["loc -> •loc [ num ]\nfactor -> •loc\nfactor -> •num\nexpr -> •term\nloc -> •id\nfactor -> •false\nunary -> •! unary\nterm -> •unary\nunary -> •factor\nfactor -> •real\nexpr -> •expr - term\nexpr -> •expr + term\nunary -> •- unary\nrel -> expr <= •expr\nfactor -> •( bool )\nterm -> •term / unary\nfactor -> •true\nterm -> •term * unary\n"]
52["expr -> expr •- term\nexpr -> expr •+ term\nrel -> expr <= expr•\n"]
53["factor -> •loc\nfactor -> •false\nrel -> expr >= •expr\nexpr -> •expr - term\nfactor -> •num\nfactor -> •( bool )\nexpr -> •expr + term\nexpr -> •term\nfactor -> •true\nloc -> •loc [ num ]\nfactor -> •real\nunary -> •factor\nunary -> •! unary\nterm -> •term * unary\nloc -> •id\nterm -> •unary\nunary -> •- unary\nterm -> •term / unary\n"]
54["expr -> expr •+ term\nexpr -> expr •- term\nrel -> expr >= expr•\n"]
55["loc -> •loc [ num ]\nfactor -> •real\nexpr -> •expr - term\nrel -> expr > •expr\nterm -> •unary\nexpr -> •expr + term\nfactor -> •false\nterm -> •term / unary\nunary -> •! unary\nterm -> •term * unary\nfactor -> •true\nunary -> •factor\nloc -> •id\nfactor -> •num\nunary -> •- unary\nfactor -> •loc\nexpr -> •term\nfactor -> •( bool )\n"]
56["expr -> expr •- term\nrel -> expr > expr•\nexpr -> expr •+ term\n"]
57["stmt -> while ( bool •) stmt\nbool -> bool •|| join\n"]
58["stmt -> •break ;\nstmt -> •if ( bool ) stmt else stmt\nstmt -> while ( bool ) •stmt\nloc -> •id\nstmt -> •if ( bool ) stmt\nstmt -> •loc = bool ;\nstmt -> •do stmt while ( bool ) ;\nloc -> •loc [ num ]\nstmt -> •while ( bool ) stmt\nblock -> •left_curly_brace decls stmts right_curly_brace\nstmt -> •block\n"]
59["stmt -> if •( bool ) stmt\nstmt -> if •( bool ) stmt else stmt\n"]
60["join -> •equality\nequality -> •equality != rel\nloc -> •loc [ num ]\nfactor -> •num\nstmt -> if ( •bool ) stmt else stmt\nfactor -> •true\nrel -> •expr\nbool -> •bool || join\nexpr -> •expr + term\nfactor -> •false\nrel -> •expr >= expr\nequality -> •equality == rel\nunary -> •! unary\nrel -> •expr <= expr\nunary -> •factor\nterm -> •unary\njoin -> •join && equality\nfactor -> •( bool )\nfactor -> •real\nstmt -> if ( •bool ) stmt\nfactor -> •loc\nexpr -> •term\nrel -> •expr > expr\nexpr -> •expr - term\nterm -> •term / unary\nloc -> •id\nunary -> •- unary\nterm -> •term * unary\nrel -> •expr < expr\nequality -> •rel\nbool -> •join\n"]
61["bool -> bool •|| join\nstmt -> if ( bool •) stmt\nstmt -> if ( bool •) stmt else stmt\n"]
62["stmt -> •loc = bool ;\nloc -> •loc [ num ]\nstmt -> •while ( bool ) stmt\nstmt -> •break ;\nblock -> •left_curly_brace decls stmts right_curly_brace\nloc -> •id\nstmt -> •if ( bool ) stmt\nstmt -> •do stmt while ( bool ) ;\nstmt -> if ( bool ) •stmt\nstmt -> •if ( bool ) stmt else stmt\nstmt -> •block\nstmt -> if ( bool ) •stmt else stmt\n"]
63["stmt -> block•\n"]
64["stmt -> •do stmt while ( bool ) ;\nstmt -> •if ( bool ) stmt\nstmt -> •break ;\nblock -> •left_curly_brace decls stmts right_curly_brace\nloc -> •loc [ num ]\nstmt -> •block\nstmt -> •while ( bool ) stmt\nstmt -> •loc = bool ;\nstmt -> •if ( bool ) stmt else stmt\nstmt -> do •stmt while ( bool ) ;\nloc -> •id\n"]
65["stmt -> break •;\n"]
66["stmt -> break ;•\n"]
67["loc -> loc •[ num ]\nstmt -> loc •= bool ;\n"]
68["factor -> •false\nfactor -> •num\nterm -> •unary\nterm -> •term * unary\nloc -> •loc [ num ]\nrel -> •expr >= expr\nfactor -> •real\nfactor -> •true\nrel -> •expr > expr\nequality -> •rel\nbool -> •join\nexpr -> •term\nfactor -> •loc\nequality -> •equality == rel\nloc -> •id\nexpr -> •expr - term\nequality -> •equality != rel\nterm -> •term / unary\nunary -> •! unary\nunary -> •- unary\njoin -> •equality\nfactor -> •( bool )\nrel -> •expr\nexpr -> •expr + term\njoin -> •join && equality\nstmt -> loc = •bool ;\nunary -> •factor\nbool -> •bool || join\nrel -> •expr <= expr\nrel -> •expr < expr\n"]
69["stmt -> loc = bool •;\nbool -> bool •|| join\n"]
70["stmt -> loc = bool ;•\n"]
71["stmt -> do stmt •while ( bool ) ;\n"]
72["stmt -> do stmt while •( bool ) ;\n"]
73["unary -> •- unary\nfactor -> •false\nequality -> •equality == rel\njoin -> •join && equality\nterm -> •unary\nfactor -> •true\nexpr -> •expr - term\nrel -> •expr >= expr\nrel -> •expr > expr\nstmt -> do stmt while ( •bool ) ;\nfactor -> •( bool )\nequality -> •equality != rel\nexpr -> •term\nbool -> •join\nrel -> •expr\nbool -> •bool || join\nfactor -> •real\nfactor -> •num\nunary -> •factor\nloc -> •loc [ num ]\njoin -> •equality\nfactor -> •loc\nrel -> •expr < expr\nloc -> •id\nterm -> •term / unary\nexpr -> •expr + term\nterm -> •term * unary\nunary -> •! unary\nrel -> •expr <= expr\nequality -> •rel\n"]
74["stmt -> do stmt while ( bool •) ;\nbool -> bool •|| join\n"]
75["stmt -> do stmt while ( bool ) •;\n"]
76["stmt -> do stmt while ( bool ) ;•\n"]
77["stmt -> if ( bool ) stmt•\nstmt -> if ( bool ) stmt •else stmt\n"]
78["stmt -> •while ( bool ) stmt\nstmt -> •if ( bool ) stmt\nblock -> •left_curly_brace decls stmts right_curly_brace\nstmt -> •loc = bool ;\nstmt -> •do stmt while ( bool ) ;\nstmt -> •block\nloc -> •loc [ num ]\nstmt -> •if ( bool ) stmt else stmt\nstmt -> if ( bool ) stmt else •stmt\nstmt -> •break ;\nloc -> •id\n"]
79["stmt -> if ( bool ) stmt else stmt•\n"]
80["stmt -> while ( bool ) stmt•\n"]
81["stmts -> stmts stmt•\n"]
82["block -> left_curly_brace decls stmts right_curly_brace•\n"]
83["type -> basic•\n"]
84["decl -> type •id ;\ntype -> type •[ num ]\n"]
85["type -> type [ •num ]\n"]
86["type -> type [ num •]\n"]
87["type -> type [ num ]•\n"]
88["decl -> type id •;\n"]
89["decl -> type id ;•\n"]

0--"block"-->1
0--"program"-->2
0--"S"-->3
5--"decl"-->6
9--"true"-->10
9--"num"-->11
9--"false"-->12
15--"false"-->12
15--"!"-->15
15--"true"-->10
15--"id"-->16
17--"id"-->16
23--"]"-->24
22--"num"-->23
21--"["-->22
20--"loc"-->21
20--"true"-->10
20--"factor"-->25
20--"num"-->11
20--"false"-->12
20--"("-->18
20--"!"-->15
20--"unary"-->26
20--"id"-->16
20--"real"-->27
20--"-"-->17
19--"/"-->20
28--"factor"-->25
28--"("-->18
28--"id"-->16
28--"unary"-->29
28--"loc"-->21
28--"!"-->15
28--"-"-->17
28--"real"-->27
28--"num"-->11
28--"true"-->10
28--"false"-->12
19--"*"-->28
18--"term"-->19
18--"expr"-->13
18--"false"-->12
18--"-"-->17
18--"loc"-->21
18--"rel"-->30
18--"true"-->10
32--"rel"-->30
32--"!"-->15
32--"factor"-->25
32--"false"-->12
32--"unary"-->33
32--"expr"-->13
32--"real"-->27
32--"term"-->19
32--"("-->18
32--"true"-->10
32--"-"-->17
32--"loc"-->21
32--"id"-->16
32--"num"-->11
35--"real"-->27
35--"unary"-->33
35--"term"-->19
35--"expr"-->13
35--"-"-->17
35--"factor"-->25
35--"false"-->12
35--"true"-->10
35--"num"-->11
35--"("-->18
35--"loc"-->21
35--"rel"-->36
35--"id"-->16
35--"!"-->15
34--"=="-->35
37--"rel"-->38
37--"real"-->27
37--"-"-->17
37--"id"-->16
37--"true"-->10
37--"num"-->11
37--"expr"-->13
37--"term"-->19
37--"("-->18
37--"false"-->12
37--"!"-->15
37--"factor"-->25
37--"unary"-->33
37--"loc"-->21
34--"!="-->37
32--"equality"-->34
31--"&&"-->32
18--"join"-->31
18--"!"-->15
18--"num"-->11
18--"unary"-->33
18--"id"-->16
18--"real"-->27
39--"=="-->35
39--"!="-->37
18--"equality"-->39
18--"factor"-->25
40--")"-->41
42--"id"-->16
42--"("-->18
42--"term"-->19
42--"real"-->27
43--"&&"-->32
42--"join"-->43
42--"-"-->17
42--"loc"-->21
42--"equality"-->39
42--"false"-->12
42--"true"-->10
42--"expr"-->13
42--"rel"-->30
42--"num"-->11
42--"factor"-->25
42--"!"-->15
42--"unary"-->33
40--"||"-->42
18--"bool"-->40
18--"("-->18
17--"("-->18
17--"real"-->27
17--"!"-->15
17--"num"-->11
17--"true"-->10
17--"-"-->17
17--"unary"-->44
17--"false"-->12
17--"factor"-->25
17--"loc"-->21
15--"-"-->17
15--"real"-->27
15--"factor"-->25
15--"loc"-->21
15--"("-->18
15--"unary"-->45
15--"num"-->11
14--"!"-->15
14--"loc"-->21
14--"-"-->17
14--"unary"-->33
14--"num"-->11
14--"real"-->27
14--"false"-->12
14--"factor"-->25
14--"true"-->10
14--"id"-->16
14--"("-->18
46--"/"-->20
46--"*"-->28
14--"term"-->46
13--"+"-->14
48--"+"-->14
49--"loc"-->21
49--"true"-->10
49--"unary"-->33
49--"factor"-->25
50--"/"-->20
50--"*"-->28
49--"term"-->50
49--"-"-->17
49--"real"-->27
49--"!"-->15
49--"id"-->16
49--"("-->18
49--"false"-->12
49--"num"-->11
48--"-"-->49
47--"expr"-->48
47--"unary"-->33
47--"true"-->10
47--"factor"-->25
47--"term"-->19
47--"-"-->17
47--"num"-->11
47--"real"-->27
47--"!"-->15
47--"("-->18
47--"id"-->16
47--"loc"-->21
47--"false"-->12
13--"<"-->47
51--"unary"-->33
51--"("-->18
51--"-"-->17
51--"true"-->10
52--"-"-->49
52--"+"-->14
51--"expr"-->52
51--"num"-->11
51--"!"-->15
51--"term"-->19
51--"real"-->27
51--"loc"-->21
51--"false"-->12
51--"factor"-->25
51--"id"-->16
13--"<="-->51
13--"-"-->49
53--"id"-->16
54--"-"-->49
54--"+"-->14
53--"expr"-->54
53--"num"-->11
53--"true"-->10
53--"loc"-->21
53--"!"-->15
53--"-"-->17
53--"real"-->27
53--"("-->18
53--"factor"-->25
53--"unary"-->33
53--"term"-->19
53--"false"-->12
13--">="-->53
55--"factor"-->25
56--"-"-->49
56--"+"-->14
55--"expr"-->56
55--"id"-->16
55--"-"-->17
55--"!"-->15
55--"false"-->12
55--"unary"-->33
55--"real"-->27
55--"num"-->11
55--"("-->18
55--"term"-->19
55--"loc"-->21
55--"true"-->10
13--">"-->55
9--"expr"-->13
9--"loc"-->21
9--"id"-->16
9--"("-->18
9--"term"-->19
9--"!"-->15
60--"term"-->19
60--"("-->18
60--"false"-->12
60--"loc"-->21
60--"!"-->15
60--"factor"-->25
60--"expr"-->13
60--"join"-->31
60--"rel"-->30
60--"-"-->17
60--"equality"-->39
60--"unary"-->33
60--"true"-->10
60--"real"-->27
60--"num"-->11
61--"||"-->42
62--"block"-->63
64--"do"-->64
64--"block"-->63
64--"while"-->8
64--"id"-->16
65--";"-->66
64--"break"-->65
64--"if"-->59
64--"left_curly_brace"-->4
67--"["-->22
68--"factor"-->25
68--"real"-->27
68--"join"-->31
68--"num"-->11
68--"("-->18
68--"false"-->12
68--"true"-->10
69--";"-->70
69--"||"-->42
68--"bool"-->69
68--"-"-->17
68--"term"-->19
68--"loc"-->21
68--"unary"-->33
68--"id"-->16
68--"!"-->15
68--"rel"-->30
68--"expr"-->13
68--"equality"-->39
67--"="-->68
64--"loc"-->67
73--"true"-->10
73--"join"-->31
73--"real"-->27
73--"expr"-->13
73--"-"-->17
73--"rel"-->30
73--"factor"-->25
73--"term"-->19
73--"false"-->12
75--";"-->76
74--")"-->75
74--"||"-->42
73--"bool"-->74
73--"loc"-->21
73--"id"-->16
73--"("-->18
73--"unary"-->33
73--"num"-->11
73--"!"-->15
73--"equality"-->39
72--"("-->73
71--"while"-->72
64--"stmt"-->71
62--"do"-->64
62--"break"-->65
62--"if"-->59
78--"while"-->8
78--"if"-->59
78--"left_curly_brace"-->4
78--"do"-->64
78--"loc"-->67
78--"stmt"-->79
78--"block"-->63
78--"break"-->65
78--"id"-->16
77--"else"-->78
62--"stmt"-->77
62--"id"-->16
62--"while"-->8
62--"loc"-->67
62--"left_curly_brace"-->4
61--")"-->62
60--"bool"-->61
60--"id"-->16
59--"("-->60
58--"if"-->59
58--"do"-->64
58--"stmt"-->80
58--"loc"-->67
58--"break"-->65
58--"id"-->16
58--"while"-->8
58--"left_curly_brace"-->4
58--"block"-->63
57--")"-->58
57--"||"-->42
9--"bool"-->57
9--"real"-->27
9--"equality"-->39
9--"rel"-->30
9--"-"-->17
9--"factor"-->25
9--"join"-->31
9--"unary"-->33
8--"("-->9
7--"while"-->8
7--"stmt"-->81
7--"break"-->65
7--"do"-->64
7--"if"-->59
7--"right_curly_brace"-->82
7--"left_curly_brace"-->4
7--"id"-->16
7--"loc"-->67
7--"block"-->63
5--"stmts"-->7
5--"basic"-->83
86--"]"-->87
85--"num"-->86
84--"["-->85
88--";"-->89
84--"id"-->88
5--"type"-->84
4--"decls"-->5
0--"left_curly_brace"-->4

```

---

## Follow Set
```txt
!=: ["!", "num", "(", "id", "true", "false", "-", "real"]
term: [">=", "&&", "/", "+", "<", "-", "||", ">", ")", "==", "*", "<=", ";", "!="]
join: [";", "||", "&&", ")"]
&&: ["id", "real", "num", "true", "!", "-", "(", "false"]
if: ["("]
;: ["right_curly_brace", "if", "while", "break", "id", "basic", "do", "left_curly_brace", "else"]
__EPSILON__: ["do", "id", "left_curly_brace", "if", "right_curly_brace", "while", "break", "basic"]
program: ["__$__"]
rel: ["==", "!=", "||", ")", "&&", ";"]
unary: [";", "*", ">=", ")", "<", "||", "&&", "!=", "<=", "/", "==", "+", "-", ">"]
num: ["<", ">", "+", "/", "]", "!=", ")", "&&", "==", "*", "||", "<=", "-", ">=", ";"]
left_curly_brace: ["do", "while", "if", "id", "break", "left_curly_brace", "basic", "right_curly_brace"]
>=: ["(", "real", "num", "false", "true", "-", "id", "!"]
-: ["real", "num", "true", "-", "(", "!", "id", "false"]
while: ["("]
decls: ["right_curly_brace", "id", "while", "do", "if", "left_curly_brace", "break", "basic"]
): ["id", ")", "==", "&&", ";", "<", "/", ">=", ">", "-", "left_curly_brace", "if", "*", "while", "break", "<=", "||", "!=", "do", "+"]
right_curly_brace: ["break", "if", "else", "id", "right_curly_brace", "do", "left_curly_brace", "__$__", "while"]
<=: ["num", "(", "true", "false", "-", "id", "real", "!"]
bool: [";", ")", "||"]
*: ["true", "-", "num", "!", "(", "false", "id", "real"]
=: ["id", "false", "true", "(", "-", "real", "!", "num"]
/: ["id", "false", "(", "!", "real", "true", "-", "num"]
block: ["__$__", "while", "left_curly_brace", "break", "if", "else", "id", "do", "right_curly_brace"]
__$__: []
else: ["do", "left_curly_brace", "id", "break", "while", "if"]
!: ["-", "!", "real", "num", "true", "false", "id", "("]
__DUMMY_START__: ["__$__"]
[: ["num"]
(: ["num", "true", "id", "!", "(", "real", "-", "false"]
decl: ["basic", "id", "if", "break", "do", "left_curly_brace", "right_curly_brace", "while"]
do: ["left_curly_brace", "while", "break", "id", "if", "do"]
==: ["!", "true", "(", "id", "num", "-", "real", "false"]
basic: ["id", "["]
expr: ["!=", "||", "<=", "&&", ";", ">=", "<", "-", ">", "==", "+", ")"]
factor: ["&&", "<", "<=", ")", ">=", "!=", "+", "==", "||", ">", "-", "*", ";", "/"]
equality: ["||", "!=", ";", ")", "&&", "=="]
false: ["<=", ">", "+", "&&", "==", "<", "-", ";", "*", "||", "/", ">=", ")", "!="]
break: [";"]
type: ["[", "id"]
stmts: ["if", "while", "right_curly_brace", "do", "break", "id", "left_curly_brace"]
]: ["[", "<", "&&", "||", "<=", "==", "id", "!=", "*", ">=", ")", "+", "/", ";", "-", ">", "="]
real: ["&&", "!=", "==", "+", "*", "<", ">", "-", ">=", "/", ")", "<=", "||", ";"]
||: ["!", "false", "num", "true", "id", "real", "-", "("]
<: ["!", "(", "real", "true", "-", "false", "num", "id"]
+: ["num", "real", "(", "!", "-", "false", "true", "id"]
loc: ["[", "<", "=", "/", "<=", "-", "!=", "||", ")", "+", ";", "==", "*", "&&", ">=", ">"]
S: ["__$__"]
stmt: ["do", "while", "id", "break", "left_curly_brace", "right_curly_brace", "if", "else"]
>: ["-", "(", "false", "id", "real", "num", "!", "true"]
true: ["/", "+", "<", "<=", "&&", "-", "!=", ")", ";", ">=", "*", "||", ">", "=="]
id: ["!=", "<", "+", "/", "[", ">=", ")", "-", "=", ";", "&&", "||", "<=", "==", ">", "*"]
```

---
## Action Table
```txt
State 0:
program: Shift(2)
left_curly_brace: Shift(4)
S: Shift(3)
block: Shift(1)
__$__: Accept
===================
State 1:
__$__: Reduce(ReduceDerivation { left: "program", right: ["block"] })
===================
State 2:
__$__: Reduce(ReduceDerivation { left: "S", right: ["program"] })
===================
State 3:
__$__: Reduce(ReduceDerivation { left: "__DUMMY_START__", right: ["S"] })
===================
State 4:
while: Reduce(ReduceDerivation { left: "decls", right: [] })
basic: Reduce(ReduceDerivation { left: "decls", right: [] })
decls: Shift(5)
if: Reduce(ReduceDerivation { left: "decls", right: [] })
right_curly_brace: Reduce(ReduceDerivation { left: "decls", right: [] })
do: Reduce(ReduceDerivation { left: "decls", right: [] })
break: Reduce(ReduceDerivation { left: "decls", right: [] })
left_curly_brace: Reduce(ReduceDerivation { left: "decls", right: [] })
id: Reduce(ReduceDerivation { left: "decls", right: [] })
===================
State 5:
decl: Shift(6)
left_curly_brace: Reduce(ReduceDerivation { left: "stmts", right: [] })
basic: Shift(83)
right_curly_brace: Reduce(ReduceDerivation { left: "stmts", right: [] })
break: Reduce(ReduceDerivation { left: "stmts", right: [] })
stmts: Shift(7)
do: Reduce(ReduceDerivation { left: "stmts", right: [] })
if: Reduce(ReduceDerivation { left: "stmts", right: [] })
type: Shift(84)
id: Reduce(ReduceDerivation { left: "stmts", right: [] })
while: Reduce(ReduceDerivation { left: "stmts", right: [] })
===================
State 6:
right_curly_brace: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
while: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
do: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
basic: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
left_curly_brace: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
id: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
if: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
break: Reduce(ReduceDerivation { left: "decls", right: ["decls", "decl"] })
===================
State 7:
stmt: Shift(81)
break: Shift(65)
while: Shift(8)
do: Shift(64)
left_curly_brace: Shift(4)
if: Shift(59)
id: Shift(16)
block: Shift(63)
right_curly_brace: Shift(82)
loc: Shift(67)
===================
State 8:
(: Shift(9)
===================
State 9:
true: Shift(10)
rel: Shift(30)
bool: Shift(57)
factor: Shift(25)
unary: Shift(33)
equality: Shift(39)
(: Shift(18)
!: Shift(15)
term: Shift(19)
num: Shift(11)
false: Shift(12)
expr: Shift(13)
id: Shift(16)
-: Shift(17)
real: Shift(27)
loc: Shift(21)
join: Shift(31)
===================
State 10:
>=: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
!=: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
<=: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
): Reduce(ReduceDerivation { left: "factor", right: ["true"] })
||: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
&&: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
-: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
*: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
>: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
;: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
+: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
/: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
<: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
==: Reduce(ReduceDerivation { left: "factor", right: ["true"] })
===================
State 11:
<=: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
&&: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
==: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
>: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
-: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
||: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
*: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
/: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
<: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
!=: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
>=: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
): Reduce(ReduceDerivation { left: "factor", right: ["num"] })
;: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
+: Reduce(ReduceDerivation { left: "factor", right: ["num"] })
===================
State 12:
==: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
;: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
!=: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
<: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
&&: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
/: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
<=: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
>=: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
>: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
||: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
-: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
): Reduce(ReduceDerivation { left: "factor", right: ["false"] })
*: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
+: Reduce(ReduceDerivation { left: "factor", right: ["false"] })
===================
State 13:
<: Shift(47)
;: Reduce(ReduceDerivation { left: "rel", right: ["expr"] })
>=: Shift(53)
>: Shift(55)
): Reduce(ReduceDerivation { left: "rel", right: ["expr"] })
&&: Reduce(ReduceDerivation { left: "rel", right: ["expr"] })
||: Reduce(ReduceDerivation { left: "rel", right: ["expr"] })
+: Shift(14)
<=: Shift(51)
==: Reduce(ReduceDerivation { left: "rel", right: ["expr"] })
-: Shift(49)
!=: Reduce(ReduceDerivation { left: "rel", right: ["expr"] })
===================
State 14:
loc: Shift(21)
(: Shift(18)
-: Shift(17)
false: Shift(12)
num: Shift(11)
factor: Shift(25)
true: Shift(10)
term: Shift(46)
!: Shift(15)
id: Shift(16)
real: Shift(27)
unary: Shift(33)
===================
State 15:
loc: Shift(21)
!: Shift(15)
(: Shift(18)
false: Shift(12)
factor: Shift(25)
id: Shift(16)
num: Shift(11)
unary: Shift(45)
true: Shift(10)
real: Shift(27)
-: Shift(17)
===================
State 16:
/: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
&&: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
;: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
+: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
||: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
-: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
=: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
*: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
>: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
[: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
<: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
<=: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
!=: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
>=: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
==: Reduce(ReduceDerivation { left: "loc", right: ["id"] })
): Reduce(ReduceDerivation { left: "loc", right: ["id"] })
===================
State 17:
!: Shift(15)
-: Shift(17)
true: Shift(10)
unary: Shift(44)
id: Shift(16)
real: Shift(27)
num: Shift(11)
false: Shift(12)
factor: Shift(25)
(: Shift(18)
loc: Shift(21)
===================
State 18:
!: Shift(15)
equality: Shift(39)
join: Shift(31)
false: Shift(12)
rel: Shift(30)
bool: Shift(40)
num: Shift(11)
loc: Shift(21)
unary: Shift(33)
expr: Shift(13)
true: Shift(10)
real: Shift(27)
(: Shift(18)
term: Shift(19)
id: Shift(16)
factor: Shift(25)
-: Shift(17)
===================
State 19:
<: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
<=: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
*: Shift(28)
&&: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
>: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
==: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
+: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
||: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
;: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
/: Shift(20)
-: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
): Reduce(ReduceDerivation { left: "expr", right: ["term"] })
>=: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
!=: Reduce(ReduceDerivation { left: "expr", right: ["term"] })
===================
State 20:
(: Shift(18)
false: Shift(12)
loc: Shift(21)
-: Shift(17)
num: Shift(11)
true: Shift(10)
unary: Shift(26)
real: Shift(27)
!: Shift(15)
id: Shift(16)
factor: Shift(25)
===================
State 21:
<: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
<=: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
+: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
/: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
==: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
*: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
&&: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
>: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
[: Shift(22)
!=: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
>=: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
-: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
;: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
): Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
||: Reduce(ReduceDerivation { left: "factor", right: ["loc"] })
===================
State 22:
num: Shift(23)
===================
State 23:
]: Shift(24)
===================
State 24:
-: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
+: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
/: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
[: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
<=: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
==: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
>: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
&&: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
): Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
>=: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
<: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
;: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
=: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
||: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
!=: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
*: Reduce(ReduceDerivation { left: "loc", right: ["loc", "[", "num", "]"] })
===================
State 25:
): Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
!=: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
;: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
>=: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
<=: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
/: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
==: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
+: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
&&: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
>: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
*: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
-: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
||: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
<: Reduce(ReduceDerivation { left: "unary", right: ["factor"] })
===================
State 26:
<: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
==: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
-: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
||: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
<=: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
;: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
*: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
!=: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
&&: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
>=: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
+: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
>: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
/: Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
): Reduce(ReduceDerivation { left: "term", right: ["term", "/", "unary"] })
===================
State 27:
<: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
>=: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
/: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
>: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
!=: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
<=: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
): Reduce(ReduceDerivation { left: "factor", right: ["real"] })
&&: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
+: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
==: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
||: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
-: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
*: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
;: Reduce(ReduceDerivation { left: "factor", right: ["real"] })
===================
State 28:
true: Shift(10)
-: Shift(17)
real: Shift(27)
loc: Shift(21)
!: Shift(15)
(: Shift(18)
false: Shift(12)
num: Shift(11)
factor: Shift(25)
id: Shift(16)
unary: Shift(29)
===================
State 29:
==: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
*: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
||: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
<=: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
-: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
/: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
;: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
+: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
!=: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
<: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
>=: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
>: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
&&: Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
): Reduce(ReduceDerivation { left: "term", right: ["term", "*", "unary"] })
===================
State 30:
||: Reduce(ReduceDerivation { left: "equality", right: ["rel"] })
&&: Reduce(ReduceDerivation { left: "equality", right: ["rel"] })
;: Reduce(ReduceDerivation { left: "equality", right: ["rel"] })
!=: Reduce(ReduceDerivation { left: "equality", right: ["rel"] })
): Reduce(ReduceDerivation { left: "equality", right: ["rel"] })
==: Reduce(ReduceDerivation { left: "equality", right: ["rel"] })
===================
State 31:
&&: Shift(32)
||: Reduce(ReduceDerivation { left: "bool", right: ["join"] })
;: Reduce(ReduceDerivation { left: "bool", right: ["join"] })
): Reduce(ReduceDerivation { left: "bool", right: ["join"] })
===================
State 32:
real: Shift(27)
loc: Shift(21)
num: Shift(11)
term: Shift(19)
(: Shift(18)
equality: Shift(34)
!: Shift(15)
true: Shift(10)
rel: Shift(30)
unary: Shift(33)
-: Shift(17)
factor: Shift(25)
id: Shift(16)
false: Shift(12)
expr: Shift(13)
===================
State 33:
&&: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
/: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
>=: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
): Reduce(ReduceDerivation { left: "term", right: ["unary"] })
<=: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
-: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
==: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
!=: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
>: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
*: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
||: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
;: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
<: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
+: Reduce(ReduceDerivation { left: "term", right: ["unary"] })
===================
State 34:
;: Reduce(ReduceDerivation { left: "join", right: ["join", "&&", "equality"] })
!=: Shift(37)
&&: Reduce(ReduceDerivation { left: "join", right: ["join", "&&", "equality"] })
||: Reduce(ReduceDerivation { left: "join", right: ["join", "&&", "equality"] })
==: Shift(35)
): Reduce(ReduceDerivation { left: "join", right: ["join", "&&", "equality"] })
===================
State 35:
id: Shift(16)
!: Shift(15)
factor: Shift(25)
num: Shift(11)
loc: Shift(21)
false: Shift(12)
true: Shift(10)
unary: Shift(33)
real: Shift(27)
(: Shift(18)
rel: Shift(36)
-: Shift(17)
term: Shift(19)
expr: Shift(13)
===================
State 36:
&&: Reduce(ReduceDerivation { left: "equality", right: ["equality", "==", "rel"] })
;: Reduce(ReduceDerivation { left: "equality", right: ["equality", "==", "rel"] })
): Reduce(ReduceDerivation { left: "equality", right: ["equality", "==", "rel"] })
==: Reduce(ReduceDerivation { left: "equality", right: ["equality", "==", "rel"] })
||: Reduce(ReduceDerivation { left: "equality", right: ["equality", "==", "rel"] })
!=: Reduce(ReduceDerivation { left: "equality", right: ["equality", "==", "rel"] })
===================
State 37:
rel: Shift(38)
-: Shift(17)
expr: Shift(13)
(: Shift(18)
!: Shift(15)
true: Shift(10)
real: Shift(27)
id: Shift(16)
term: Shift(19)
factor: Shift(25)
unary: Shift(33)
num: Shift(11)
loc: Shift(21)
false: Shift(12)
===================
State 38:
): Reduce(ReduceDerivation { left: "equality", right: ["equality", "!=", "rel"] })
||: Reduce(ReduceDerivation { left: "equality", right: ["equality", "!=", "rel"] })
&&: Reduce(ReduceDerivation { left: "equality", right: ["equality", "!=", "rel"] })
==: Reduce(ReduceDerivation { left: "equality", right: ["equality", "!=", "rel"] })
!=: Reduce(ReduceDerivation { left: "equality", right: ["equality", "!=", "rel"] })
;: Reduce(ReduceDerivation { left: "equality", right: ["equality", "!=", "rel"] })
===================
State 39:
!=: Shift(37)
&&: Reduce(ReduceDerivation { left: "join", right: ["equality"] })
): Reduce(ReduceDerivation { left: "join", right: ["equality"] })
==: Shift(35)
||: Reduce(ReduceDerivation { left: "join", right: ["equality"] })
;: Reduce(ReduceDerivation { left: "join", right: ["equality"] })
===================
State 40:
||: Shift(42)
): Shift(41)
===================
State 41:
): Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
/: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
-: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
||: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
>=: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
+: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
*: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
<=: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
!=: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
==: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
<: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
;: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
&&: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
>: Reduce(ReduceDerivation { left: "factor", right: ["(", "bool", ")"] })
===================
State 42:
expr: Shift(13)
(: Shift(18)
loc: Shift(21)
false: Shift(12)
join: Shift(43)
true: Shift(10)
rel: Shift(30)
num: Shift(11)
unary: Shift(33)
real: Shift(27)
factor: Shift(25)
equality: Shift(39)
term: Shift(19)
!: Shift(15)
-: Shift(17)
id: Shift(16)
===================
State 43:
): Reduce(ReduceDerivation { left: "bool", right: ["bool", "||", "join"] })
;: Reduce(ReduceDerivation { left: "bool", right: ["bool", "||", "join"] })
&&: Shift(32)
||: Reduce(ReduceDerivation { left: "bool", right: ["bool", "||", "join"] })
===================
State 44:
!=: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
*: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
): Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
||: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
&&: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
;: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
<=: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
/: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
<: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
+: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
-: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
>: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
==: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
>=: Reduce(ReduceDerivation { left: "unary", right: ["-", "unary"] })
===================
State 45:
<=: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
-: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
): Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
<: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
>: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
==: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
+: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
/: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
>=: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
||: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
;: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
*: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
&&: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
!=: Reduce(ReduceDerivation { left: "unary", right: ["!", "unary"] })
===================
State 46:
>=: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
!=: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
-: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
||: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
>: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
==: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
+: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
<=: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
<: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
/: Shift(20)
*: Shift(28)
&&: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
;: Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
): Reduce(ReduceDerivation { left: "expr", right: ["expr", "+", "term"] })
===================
State 47:
id: Shift(16)
expr: Shift(48)
true: Shift(10)
term: Shift(19)
-: Shift(17)
num: Shift(11)
loc: Shift(21)
unary: Shift(33)
!: Shift(15)
false: Shift(12)
real: Shift(27)
(: Shift(18)
factor: Shift(25)
===================
State 48:
-: Shift(49)
||: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<", "expr"] })
!=: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<", "expr"] })
): Reduce(ReduceDerivation { left: "rel", right: ["expr", "<", "expr"] })
+: Shift(14)
;: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<", "expr"] })
&&: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<", "expr"] })
==: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<", "expr"] })
===================
State 49:
!: Shift(15)
unary: Shift(33)
factor: Shift(25)
(: Shift(18)
id: Shift(16)
-: Shift(17)
true: Shift(10)
loc: Shift(21)
false: Shift(12)
num: Shift(11)
real: Shift(27)
term: Shift(50)
===================
State 50:
*: Shift(28)
-: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
>: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
+: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
): Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
!=: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
;: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
/: Shift(20)
||: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
<=: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
&&: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
>=: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
<: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
==: Reduce(ReduceDerivation { left: "expr", right: ["expr", "-", "term"] })
===================
State 51:
factor: Shift(25)
loc: Shift(21)
false: Shift(12)
unary: Shift(33)
true: Shift(10)
real: Shift(27)
id: Shift(16)
expr: Shift(52)
-: Shift(17)
(: Shift(18)
!: Shift(15)
term: Shift(19)
num: Shift(11)
===================
State 52:
-: Shift(49)
||: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<=", "expr"] })
!=: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<=", "expr"] })
): Reduce(ReduceDerivation { left: "rel", right: ["expr", "<=", "expr"] })
==: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<=", "expr"] })
+: Shift(14)
&&: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<=", "expr"] })
;: Reduce(ReduceDerivation { left: "rel", right: ["expr", "<=", "expr"] })
===================
State 53:
expr: Shift(54)
!: Shift(15)
factor: Shift(25)
(: Shift(18)
-: Shift(17)
true: Shift(10)
unary: Shift(33)
real: Shift(27)
loc: Shift(21)
id: Shift(16)
false: Shift(12)
num: Shift(11)
term: Shift(19)
===================
State 54:
+: Shift(14)
&&: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">=", "expr"] })
||: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">=", "expr"] })
): Reduce(ReduceDerivation { left: "rel", right: ["expr", ">=", "expr"] })
==: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">=", "expr"] })
!=: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">=", "expr"] })
;: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">=", "expr"] })
-: Shift(49)
===================
State 55:
expr: Shift(56)
id: Shift(16)
!: Shift(15)
factor: Shift(25)
num: Shift(11)
loc: Shift(21)
term: Shift(19)
false: Shift(12)
unary: Shift(33)
(: Shift(18)
true: Shift(10)
-: Shift(17)
real: Shift(27)
===================
State 56:
==: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">", "expr"] })
;: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">", "expr"] })
): Reduce(ReduceDerivation { left: "rel", right: ["expr", ">", "expr"] })
&&: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">", "expr"] })
-: Shift(49)
!=: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">", "expr"] })
||: Reduce(ReduceDerivation { left: "rel", right: ["expr", ">", "expr"] })
+: Shift(14)
===================
State 57:
): Shift(58)
||: Shift(42)
===================
State 58:
loc: Shift(67)
block: Shift(63)
left_curly_brace: Shift(4)
do: Shift(64)
if: Shift(59)
stmt: Shift(80)
id: Shift(16)
break: Shift(65)
while: Shift(8)
===================
State 59:
(: Shift(60)
===================
State 60:
id: Shift(16)
!: Shift(15)
term: Shift(19)
false: Shift(12)
true: Shift(10)
unary: Shift(33)
join: Shift(31)
num: Shift(11)
factor: Shift(25)
loc: Shift(21)
-: Shift(17)
expr: Shift(13)
equality: Shift(39)
rel: Shift(30)
(: Shift(18)
bool: Shift(61)
real: Shift(27)
===================
State 61:
): Shift(62)
||: Shift(42)
===================
State 62:
loc: Shift(67)
id: Shift(16)
if: Shift(59)
stmt: Shift(77)
break: Shift(65)
block: Shift(63)
while: Shift(8)
do: Shift(64)
left_curly_brace: Shift(4)
===================
State 63:
else: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
do: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
id: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
break: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
right_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
if: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
while: Reduce(ReduceDerivation { left: "stmt", right: ["block"] })
===================
State 64:
id: Shift(16)
block: Shift(63)
while: Shift(8)
do: Shift(64)
break: Shift(65)
if: Shift(59)
stmt: Shift(71)
loc: Shift(67)
left_curly_brace: Shift(4)
===================
State 65:
;: Shift(66)
===================
State 66:
break: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
id: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
else: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
while: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
right_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
if: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
do: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["break", ";"] })
===================
State 67:
=: Shift(68)
[: Shift(22)
===================
State 68:
factor: Shift(25)
rel: Shift(30)
expr: Shift(13)
unary: Shift(33)
true: Shift(10)
false: Shift(12)
term: Shift(19)
-: Shift(17)
equality: Shift(39)
bool: Shift(69)
num: Shift(11)
real: Shift(27)
id: Shift(16)
(: Shift(18)
join: Shift(31)
!: Shift(15)
loc: Shift(21)
===================
State 69:
;: Shift(70)
||: Shift(42)
===================
State 70:
right_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
while: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
do: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
if: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
break: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
id: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
else: Reduce(ReduceDerivation { left: "stmt", right: ["loc", "=", "bool", ";"] })
===================
State 71:
while: Shift(72)
===================
State 72:
(: Shift(73)
===================
State 73:
unary: Shift(33)
equality: Shift(39)
term: Shift(19)
(: Shift(18)
!: Shift(15)
join: Shift(31)
id: Shift(16)
false: Shift(12)
real: Shift(27)
bool: Shift(74)
rel: Shift(30)
expr: Shift(13)
factor: Shift(25)
num: Shift(11)
true: Shift(10)
-: Shift(17)
loc: Shift(21)
===================
State 74:
): Shift(75)
||: Shift(42)
===================
State 75:
;: Shift(76)
===================
State 76:
id: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
else: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
while: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
right_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
if: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
do: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
break: Reduce(ReduceDerivation { left: "stmt", right: ["do", "stmt", "while", "(", "bool", ")", ";"] })
===================
State 77:
while: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
right_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
id: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
else: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
do: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
if: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
break: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt"] })
===================
State 78:
left_curly_brace: Shift(4)
if: Shift(59)
do: Shift(64)
while: Shift(8)
id: Shift(16)
block: Shift(63)
stmt: Shift(79)
break: Shift(65)
loc: Shift(67)
===================
State 79:
if: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
right_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
id: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
break: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
else: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
do: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
while: Reduce(ReduceDerivation { left: "stmt", right: ["if", "(", "bool", ")", "stmt", "else", "stmt"] })
===================
State 80:
id: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
else: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
right_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
do: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
break: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
while: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
if: Reduce(ReduceDerivation { left: "stmt", right: ["while", "(", "bool", ")", "stmt"] })
===================
State 81:
if: Reduce(ReduceDerivation { left: "stmts", right: ["stmts", "stmt"] })
while: Reduce(ReduceDerivation { left: "stmts", right: ["stmts", "stmt"] })
left_curly_brace: Reduce(ReduceDerivation { left: "stmts", right: ["stmts", "stmt"] })
do: Reduce(ReduceDerivation { left: "stmts", right: ["stmts", "stmt"] })
break: Reduce(ReduceDerivation { left: "stmts", right: ["stmts", "stmt"] })
right_curly_brace: Reduce(ReduceDerivation { left: "stmts", right: ["stmts", "stmt"] })
id: Reduce(ReduceDerivation { left: "stmts", right: ["stmts", "stmt"] })
===================
State 82:
if: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
while: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
else: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
id: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
right_curly_brace: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
__$__: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
break: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
left_curly_brace: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
do: Reduce(ReduceDerivation { left: "block", right: ["left_curly_brace", "decls", "stmts", "right_curly_brace"] })
===================
State 83:
[: Reduce(ReduceDerivation { left: "type", right: ["basic"] })
id: Reduce(ReduceDerivation { left: "type", right: ["basic"] })
===================
State 84:
id: Shift(88)
[: Shift(85)
===================
State 85:
num: Shift(86)
===================
State 86:
]: Shift(87)
===================
State 87:
[: Reduce(ReduceDerivation { left: "type", right: ["type", "[", "num", "]"] })
id: Reduce(ReduceDerivation { left: "type", right: ["type", "[", "num", "]"] })
===================
State 88:
;: Shift(89)
===================
State 89:
break: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
if: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
right_curly_brace: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
while: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
left_curly_brace: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
do: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
basic: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
id: Reduce(ReduceDerivation { left: "decl", right: ["type", "id", ";"] })
===================

```
---
generated by rparser
RockRockWhite 2023
    