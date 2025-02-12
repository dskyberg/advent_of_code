pub static TEST_INPUT: &str = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#;

pub static INPUT: &str = r#"wrrr, ubwu, ugr, wgbw, uuw, wrrrg, gburgbrw, wrbuuug, urgbu, rr, ubrbrug, wurb, rbugrb, gubbug, bggwurg, grbgw, wbwwb, bgrw, ruguu, uu, grrr, ubgwb, ggbbb, gurr, rrg, ruugruu, wrw, uwwgg, wrrur, bubww, ubru, bubwr, ggugw, gwugru, bgguugb, brub, rguw, ubggb, gbrgw, rru, rrurbub, urbu, gwg, bggwbrwb, bbwbgrr, uwrbwr, wurbuw, bw, ugwu, uwr, wu, bwug, rrrb, rububbb, gbw, wuu, rbgwrg, ubggwggb, ubgbb, bbb, ugbbwr, gr, wwrgurwb, rrru, gurrg, bbbgb, gwuwg, brr, ggb, ug, bwgr, bubug, gwwbb, wwbrw, uuurrb, rrr, rwwg, gguuw, grgbug, uur, ggwg, wuuu, wbww, ubw, wgb, grbg, gwgrbggg, grw, wbu, gruwru, rrrrgb, ururggg, rgb, wrbbwgb, bwgbr, bwu, wwu, wuwbu, wwb, wbr, ugw, wwrbur, ubr, www, urrb, rrwwr, bwwr, wbb, wwbrrug, guuruub, gg, ggugbgu, bwwbrb, bgubuw, uuubg, ggwbg, gbg, urbg, bubbru, gubgg, grb, ugbb, w, buuugrb, wubbrwr, bbugw, grww, bub, rgurg, rb, rw, rwu, wwr, bgbg, rrguw, ruruwubw, bgw, gwurruw, wgu, wgr, guwug, wgwg, bru, wuwgbg, urugu, wwuuug, buwb, bwrbbb, wgwu, urruu, gww, rbb, bgr, ru, uwugub, gburu, rrwgbu, wuwb, bgg, wbwuru, rwr, rbwb, bggrugur, wbw, wgrub, wwwb, wwbgrw, wbg, wru, urwu, burwug, gwu, wug, gw, wbbu, u, wurr, bbbgg, bgrwgb, ubbwbw, ugubub, uubr, bbrwu, urgr, bb, ubgbuwub, b, rgrrru, rgr, uuuwr, bww, guu, gbr, gwr, wrruuw, rwur, wwgw, wbbgw, buuu, rwrb, ugb, wgg, buu, rrug, ggu, rub, ugg, ggw, gbbgw, gbgwgwu, rrb, wwubw, bur, gru, wrb, gu, wugrb, rbbg, gb, rbuug, wuw, wbwgrru, bwrwb, rgbubugb, gurbwb, ruw, wwguu, grgggr, brwgbb, wrgu, gbb, rubwr, urg, uwbuw, ubb, bgbb, uwbbww, rggr, uruuu, gwb, ggwb, rgw, urw, wwgruw, wwgg, gubb, ruu, ggwuwrgw, uubuggg, wbwurbg, bugb, ugubbrub, ugwbggw, wbrubg, wbuw, ubu, guwuggw, guuu, ugu, ruwu, ubgu, ww, rbg, uwb, bgru, gbbgu, rbw, bwgwbr, buwbw, uwu, ruwugru, ugrw, buuwwr, grrw, guw, brwbubr, uwrrww, uwg, wgwwwwr, rbbwrwg, bwgrwu, uuru, rbgrbuw, rwrg, wb, gub, urww, gruw, wrbb, urb, ruwbgrr, g, gwugrr, gug, brwu, bbg, ur, rurgbwrr, rrrruru, rggruuww, rwb, wub, bbu, rrwbbu, uuugr, bbwb, guwg, bgb, bwrwg, gur, wwguuw, wuwg, uwuuugu, ubrr, uwugu, bgrrgrw, bbbww, wwggg, buwr, uw, uwbru, rgwg, ggurrrb, uguwwb, buug, uwgu, buwrwu, gwwur, wrwu, bwg, uurrr, ugwrb, ugbbugr, ruwbu, brb, grg, bbw, rwurrg, ggr, bbgg, rww, rgu, bu, rrw, bwrgr, uww, wuggg, gugg, bwb, bbbrw, bugwrbr, grbburgr, uwrr, gruggg, rrbbwru, bbr, urbgww, rgrwru, wr, ugbg, urr, bubwrr, rgbgb, wgugwurg, gwru, wrwbgbb, rbbw, wggurg, uuwuw, ggg, rgrr, bwr, ggbgbr, rrgwbg, bbwg, brw, grr, uru, ruwuwuwu, rubrb, wur, uugb, grbbg, wuwuwgg, bwugr, buw, bwbu, uurg, brg, wg, rwbwu, rgrrr, bggwbg, rwbwbbb, uuwr, bug, bwrg, ub, uub, uwbbrrwu, wgru, uuu, rbr, ubwr, ugwuwg, wrbgurgb, ruwru, ruwbg, bbbwr, ggbwu, grru, rwwr, rgbw, wrr, rgbbu, rwgr, bbwrur, bg, wgw, gbwgwg, gwrw, rwgug, wrg, ubg, wwg, rrwbwr, gbuu, wbub, rur, wbguu, wgguggw, rgur, buur, bbgr, buwbrbur, wuuuggg, bgu, wbgr, buubu, wwbbgw, bwwwu, guuw, rwg, uwbwgwgb, ugbgwg, urbwgu, urruuw

brwrgurgrwwgguwguggruurubgwrgubgbrgubggugbgrugruguburu
rbwgwggbggbgwbuugrububbbgggwwgggwbwrwbgrwuu
wbuwwubruugbrrgwrwrgbugbrrwwbrwbbgrrrbuwwuuww
rggbwrrgwrwuwubwugruurgrrrugwrrrguugururbwbubg
rgggwbburbuguwgrwwguuuwrrrwuuwrubwguuurwbbgbbwwwgubgrwwwgu
urguwwrgrggrgggbuugwubugbgubwbrgwwbwbruugurbrrgrrgbubb
gwbbbbrburwggwgguburwrrbbrbwrwgwuwugrbwwuugruw
bbugrugrgwggbgubwgbbbbubuugguwwbubguburgwuugwgbgwuurwwbwu
rggubbgugrwuurburbwrbruubuwgbwrwgurbubbgrgrrbbww
gugruwugrruggwwgrwgugwwrrwgururgwrrurwbbwb
rggbgwggggugggurwggurrrburgwwwwburugwwwggbwbrrgbugrbgbuurubw
rggggugwwwrrwwwwrruubrrgbubwbubrgwgrwggugbrb
rggubbbwwguwwburwgguugwrwurgugwubrgwgrrbugugubrwwg
rggurrruuurgwwwurbrgwbwwubgbrbwuurbggwuwrggrgu
rgggggrrwurubgbwugburbbwubbwgwubggbbwurbuguwuugrwww
uggrbwugrrububbrwgwbbruurrgbwgbgwbwrbgwwuguwurb
rggwrugwgwbgbwwrwgbwwruurgugugwbgurbrwurrbrrrbrrbrwwu
rgguuguruuwwrwrbuubrbrbuuurwgwwwuwuuwgwrbugw
rgggugrwbububrurrbrggrbrwrgwruubrruruuwbugrgg
gwbuguuubuwbubrburrrurwrrbwbbbwwbwgwwggrububrgrgurgbuu
burugbruubrrruwuurbwuubbwubbgguwwbuburrgrggwuwwggrurbwggrb
wbrrrgrwggrwwrrrrgugbubbruuurbgrgugrrrwbwbubgrrwb
rggwrgwrgwrbrwwwrgrggugggbgbwbbwwwbrwuurubwbbgurgbrububwg
rggbwgrgrrbbgrruwwguwgbbrubgurruwbuubuwuugugurrw
urrbbggwbrwbrrwbwwwbwgwwggrruugbgbgbugwuggrwwu
buwbuwurubwwrrbruwbuuwgwwruuwuwurubbwwurgugwwug
guurbruwgrgggrbugubgubbubuwburuuwwuwwrbbbuuwgrwwgwur
rgguwuggwgwuuwbgwwwuggurbgbrrbwbgwwbwwgw
rgbrrgrbgwgrgurbbururrwgrwwburwubgwrbgbbuurrrguwbg
rggubrbgrurggbbwbwbwbbugurgrurbugubwwwrwgwgwg
ubwurruubuwbbwbubuwwrgurwbbgrgrgbwrwwbwgurgbugruwugbggwrbg
rggbwbwrrwwwgrbrbrurbgrgubwwurwubwuubgrrbugrrbwrbrwubbu
uugbubwgwbuubgwubbrggrwurguwgubbrbwrbgbwurbwbbbgbrwrgrugru
bwuwbbgwwuwrrguuurubugrurgbuwwbrurrrrbgbbwbw
rggwugbgbwburugwwgrbrggwuguwwwrgrbbrwggbubbrgbwwrb
rggguuubrrrubwrugggbrrgbwrwuguuguuwgbugwggbubwwbu
gwgbrwbwuwrruurgwbbwugwruwgrugrgbguuubuubbbbrgrgrwwgru
rggwgwuwbbbgbuwwbgrrbururgubwbuburbuugrbbrgg
ggbwuwwrbubbuwrubbwggwrbgugwuwwgrruwwgggrwguwb
brwubuubgrbguubgwbugbbbuggrgwgubugbwrgruwrbwwbuwgg
brubrrubuggggbwrbguugbgrubgwbgbwrrrguuwgrurrgwbruwugwubgw
rggwuuburgugwgbrrbwwugrbrbgwuwbgwrwwwugrgugwgbuggbuurbuuubwb
rggurburrbubrbuwgbbgubwwwuwuwrubururbrbbwggbwg
buguwbguburgguwbbuuwgbwbubbbwwuwwwrgrguwbrg
burbrbwwgggguwrbguurugbwwwwbuubuubrbwwgwurururgwwwr
bgwbuwurrrruuuruwgrurububwwbubburgugruubbrrrgwrg
ubrbugwbubrgugwgbubruwuuuguwgwbwubgbbruuwgrgbwrubbbrugr
rgggbugrugrrgwgbwbubbwgubrwggbrrggwbuwbwug
bggrwwrrwbbrbbrwrwbwwuwugwbbwgbbwubugugrrwruwuuwrrwbrw
rgguubguwguggbrguwgwwbwrgubbbuurbwrgbrrgrr
rggguwruuwrgwbwbrrgrggrrrrwgbwwrrgugrrrrurgwrrurwurr
ruggrrbubuwrbugubgguuwubbgubbbbgrbrruurgbwurwgbrgr
rggbwgwrwbrurbrgrwbwruwwrubbrrugugrbururwbwuguwwbb
rggugwrbrbwruwurrwbbwwrbuubwgwggbwwwggwurb
grgurgwwgwwwrrbbgrwrgwwrbgrguguwurbrrgrwrrrbgurrwbwgg
bwwrggwbggbggwgbrrbbwuruuuubrgrwwrrggrbggrggubbwu
rrrugugurgbrbubgbuwubguwgbgrwbrrrwruwgbbruwrwrbrbbg
rggwrbugrrrbbgguwggrbbwbugurrgggwurbwrbuu
rggwgrrwbrubrguwgwbggwgwugwbbgbgrbguubbuwbbwwwruwbuw
rugbgrgbgwubrgrwrwwggugrrrggbuwgbggrgrwgurgbrbrwbgw
rgggggruwgwgurgrubwbwbgburgbwbbwwruggrbubgguug
rgwbwbggurbgwbrrubgbbbrwwgbrggrububrgubgwrrbgu
guwbwggggbgwrwgrburbuwwgrguwgrbubgbgggwrruwrwgbgg
rgurrbuwbgbwgwubbgwwwrrgbbugbwrbgurgbububrwwwb
rbwwgrubgurwrwwrwubwbgrburrbugwwbugbruurbub
rggbrbrrwbwrubrwubrrgrbwbbgbrbrubgurbrbgbrurggrbubbrggbwwubb
rwururwwuwbgwguwwrwggwuruggwwgbuwwwwurwgbggwbrw
rgggggggbwwbbgguwbugrgwwruurwgububwburbggrw
rggwrrurgwbguwuwbruwgguugwgrgwggrwrbrgubrgggrwwr
rggwrgrwwuubwuubrrwggbuubbgbgubbgrrrbbwgug
uubbwggwwrgbugwgrubwbrwwgwwuuwgbgbrbrbrbgwugrwrbwbbubbw
ggwgbbwwbbugrwbgbrbubbwrwuugrbgurwgugubgugubbrubggbrwugu
rwrbrbwgwgbuwwrbrugbrbuguuwrrrrbggbgurwrbubbgb
rwgruwbbbwbuwgrbwurgbbubbburwgruguuurbruggrbbwrurwrbw
rwuuwwwbwgrbwbrguggrrgrbbuggrwwbrwbwwbrwbrwrrrbgrw
rggbgwgbbgrgrbwbrurrubuuubuwrggrwuugrrwrgrg
rggurwwwrrbrgrwwggurgwguggguuwugbbrrgrubbgbwbggw
wrrguggbbgwbgrgwwggbgubrrbruguuuguguwuwugrgurgrwguurrggruw
rggwgugbgrwwburrruwuugwrwruurwwbrbgbrbbrrrrrrgwguburubgbwgbw
wwugrbbbbuwbgggruwbuwubgrbruruuwrbgwuruwgwuuguubw
ugurgwwwbbrggrbwrbugwwbburggbggrrrrbguguwuwguugwubg
rggwbggubuwgggwbuggubuwwuwbwgwgbubrgbbrgrgwug
rgggbrgrugubwrrubggbrurbugrbrbuwbguubwubbrwgguwbrwbgbgbwrbu
gwbrwbwggwgurbgwggrwgugburbrwuuuuurrubrrgrrwrbguggrgubg
rgguggwgrbbgbgwwrwruubwwrububbgguwrgbbubgwruugr
rggbrrwruuwwrbgbgbwbwuurgwrrrubbbwwguruwbuuwrbbbbugguwuurbw
gwgwbrwubbrbguwwwbguubrbrgbwgugbrgbbrbbgbbgrwuwggrgubg
rggggwrggggwwuubrgbbggrggrwrugruubuuwurubuwrrbb
uwuwbbgggggwgwrggrgrrggurgubggwguurbbbrwbgrgg
buwwgbwwggbbguuggubbgguuubguwruwbgwwuuwbrbwbbb
rggbgwwbwbgggbggrwbrrrbbuubbrggbrgrgwrwbbwgubrrbguuwrwgurg
rgguurbbrbrgrgbgrgguwgbrwgurwbbggubwrubrbrrrggwrubrrrwwrrbwg
ugrgwwuguugwrwrgrguuwwwrrbugurwwgbbbwbbgbgu
rggbggrwrugbrggbrgurgrgruwwgwbwggggrbrwbg
ruuguwwbrrwwgguuruubgurbrrwwrgwwrrwrrbwrurbgwrrrggrwbruuwg
ruwrwrwwbrubwgwguubburrubbwwubwwwbrruwbwbguruwgwggbbrw
burwgrubgrbbrugwuwbgrwurubuwrbburwugrrwwbrguwbuwbub
rwbwrgbwrgrwbrwbwgwrgrrbwuuwwburwrrbuuwugu
ggurbubwgrwwrgwwbrrgurwbggbbrbrggwwwubgwgbbgubwuur
rggwuwrugbuugubwgbubuwrrgbrubwggwgggggwrbwwrr
uuuwbgbruubwgbggubrbbgrrwrrgbrwwwuurrbugguurrg
rrubbbwbbwgrwwwbgggwuwubugrwwrgrwbgbgggrbuuwbbbwggwbrw
rwwgwubbruguwrrbgwwrgbwrurggwurugwwrruwgrggubggwgwb
rggbbwuuwggrbwwbwrbubrurgburbrbubrrgbuwgrbrrrwwwgrwb
rggbuuwbgburwgrubwuwruuurwgwuubbbgbwbuubgbwgwrwwbgwrgbguurbw
gwgrbuggbrrgrbgwubuwbwbubuwgrubbggrwbgbggrbburgrugu
rggguuubrbubgbwgwguuwrurbbrubwgrbwruuwurwgbgbgg
rggbuurbuurbwwgubugburuwwbwgrwgwrurgbwgwbbb
wbgrrurbbrbwbgwugbbbwrwwbrwgubuwgwbbbbrwgrbgrugrwu
rgguguwrrrgbrbugbgwrbwruurgurbgbrwgugbrgbuwwgrrurw
bugwrurrguwgrwwrugugbgrrruggubbwugbruurrgwurwgwbwg
rgggguwbrwruuuuwwwgbubwbwbwrugwubgwuwbrrwguguwwwrgubgrww
bgbuuwrwwugrgguwgugrrubbrwgrrwwurugubuwbbgrwubbg
rgguugbbrubgurrwuburbbbwbggwgrbbuwgbuwgwgwuu
wbrbbubwugrgwwrgwbggrbbrurrgrbwugwbgwrwbbb
gwgggwwubguuguuwgbugbruwrwuguwrgggbwgrgrrwbbrwrbru
rgggrbbwbrwbbguwrbuguwbwwbwgrugwwubgwuwuuwuwwbrw
rgggbuurgrurbbururwbwrwwwggurbwbgrugrwruggwugruwbubgrwbwbwu
rggwrugurbgwwwuruwbbrrruwurugburubrbgrggggbugguuw
rgguwbuugbrwrrwwuubuggurbgrgrgbruruuuurgburwguuuuubggbruub
wbbubburgrrwrwuwbbrruwrwgwubbrrrwbubbrgugwrwgrg
grururwbrbwwrrruuwgrbrrwgwbrwwggwwubgwbbubbwu
rggbuuwwurwwwburbrgbubwrbgbwrugggwurbbwwbbruwugrwwwwggbgwugr
rggwuwrgwuugrbrrubwbwbrurruwrggbruwbbbggwggwwrwugbbwgrwrubw
rrrguuwrbgbwrwrruwugruuuwwururwbgubbgugwruwg
rgggugbrggruuurrrruwurruurgrwrwgbbgbubruggwuubgbwrrugb
wbrgbuburbrwgwggbrugbbwguwbrbwgguurbrbrwbrbrrbrggwbrrruu
uugwrrurgbwrrurbbuwbrgruuugwgubbrurggwbwwbbggbwbuubrruug
wguburbggbwbgburrbuubwbbwrbwbwwrgrwubbgwwgbggbgwwwu
rgggggbgguuwbrwrurgbwrrurrgwgugrrbubwwrgguurrgbwrwgurug
rggwrbwggwrwwbrgbburrgwbrugwbgrwuurubwub
rggwbrgwguwgrgbbrgubgrbbbgurwwgwrbwbrwuubrwuugwug
buuugwwbgugrugugbbrbwuwgwbwrubgwwguurbwubuubrru
rggbwrrbggrbburgrwurwwbrrwwbrubbggbrrbgbubwwuuguuugrurwubrw
rgguggrgububbuuwwuwuuuggwrrwbrwuuguwwrugwwwwwburwwur
gbuwrwgwbburuwbbwurrrwrrbubrrburbwrwuuwbbruurbbwgw
ubwrgwbuwbuuurgbbwrwgurbuuuwruggwwrwubgbrgr
rgbrggwbggwbugwgbgugrugbwwbubwggbgbggrubbgrbgbbrrrrguuurwrb
wruugbrrbuwgbuguwwugbwrwrrrbburwrwwggrrrbwbbruuuwubr
rgguwwgbgbwurguurrgurbgbrbgrrgrgwrubgugr
uugbgbubugbburwrburugwubggubuubrbrbrrrwgwwurbgrbrgww
ggwwbwbguggrwbwgrbgrurugwbwgbubbrugbgrbrurururrwgburwwwrug
brrbwbuggugwrugrwbgbwgggbwgwrgubwugbrwwwwwbg
rgguubrrbwbbwgrwruwggguwgurrugwurwubbruwwrbugrggurrb
ubggbrwwbrubgubbrgwuurwbgwrggburrrbrggrrbubbubbrruuuuuwgb
wuwbbrrwbgggwwguuubuguwbrwgwwrgbwgbgbrburbrurbwbbwwwubgb
rgggbugbwubwgwbwubggubgwuuwbgwuggubwrgurwbrrgrbggbuwgbuuru
grrrruwwrbgugbbwugrurbrgbruuwrrurgbwbwgggggbru
rgggurwrwurubrwbwbgrgugwwgrrubbwwwbgruuwgurgugbrwuguwwubg
rgggggrbgurwrrggbgbwwrrwbubgubbruruwuwbrbwuruurgrbbgubbuuru
uuubuwgbggguruwurrbbbrugugbwubgwrbuwgubwwruwgrugwrgwwrgrwb
rggbrbrrbgrbgburrguwbgrgbbubgwbuurruuwuwrww
wuugwrwuwburbbguwwggrwbrguwwrubrubburrggwubrwurbguwb
ggbrgrgubububgwggwbrrwubgugwgwruuwwgbbruubuwbgrbuwwb
ruguugguugrwwrgggrbuwbrburbubrgwgrwggbugbwurgrrwrgru
rggbbrbuuuggwuburrgwrwrbgbugwwwwgbgbrwgbrbrwuwuu
gbrrurwurrbwwwrwrrrrgwuggwwrrgbuuuubbwrbbwbgbuggwuw
buuwwrbbrggbwbwubwuwugrgbggugbbbgbgbugwwrrgrgubub
rgggggrbrrwrbwuwwgguruwgggrrbrgbugwrrggbgwrugrrgrbbug
wubruuwgurgrgggrurrbrwggrurugrugbruwbwggbuwrbbuugwubbuub
uuwwrrubwurbbwugwbguwbubgbwgbwbbbrurbgruwrgrg
rburwbbrwugwrrruburggbuwwruwwrgrwrgrbugbrubbwrgbubwr
rgbrgggggurgruwubwurwwuurwugbrubggbuuuurggrwuuwwuubwuruwugw
gggguwgbggrwbbgrwwgurbbugbwguwgrgbbwubgwbwugwguwruwwubbggu
rggbbgugubbrubbgurrguwggrugbuwwrrgrugbuubruwrg
rguwwggrugrgubbbwbgwbubbbbuwrrgwrgwrwwbuwrwuuuwuuggurugwbb
rgggbrbuurrwwbbgrwwgwbrwguwwuuggwrrrburwuubwbwuugubwbwuw
wuuuuguuwbrgrbbgbrrrggbrwrubwbgrgwgrwgurrwruuwr
rggguubuwruwuwuwubwgrgbgrbbgbrgwubwwrwuwgguuwwrg
gubrwurwrrgrwgbrubuwbguwbbrrwubbrwwbgbwrrr
ggwubbrurbbgbguuubbwgrbuwugwrrububwbgrrggurbgubbggrugur
wrrwgwwuguwubwwuwgwuwggwwgubwwggwbugurwgwuu
rggwwrwrwuwbrwuggwbwubwbwbugurgbgrruruwubwwbrbggrruburugbur
rggbwururwgbrrbubggrwbbbrbggrrbugwbrugrwgruu
rggubgbgggwbrrbgugbugurubrbrrgbrwrwugwwguuugrrr
wgrbwruuururrbbruwuwuwurrwugrgbwbrwguwbgbgu
wugwgwgwugurbruggugbruubwrrbwbrrbbubuwugbuwbguruwwgwubu
rrbwrrburggbbrubwburwwuwbruurwwwrbguubgrwrwgbgwrbrb
rggurbwwbbubrbbrguubwbbwbrwgubbrwwwwugwgurrwrbg
rggbbrgrwuwgrwrrbuuwrgruwbgurwwrrwuwggubbbggub
bbwrrruububrwrburwwrbwrrwwurwrurggguwbrwwg
rgguwubgrrwbgwgwwuurrwbuuburggwwrurbggwbwguu
rggwrwuuwuwbwbwbuuwrguguwgrgurrgwbruggbwbbubwrru
rggbubrggggbwruggwbbwwrwbguwubwurguurwbw
rggwuwggwrrbwrgrwwbbrububrurwggrubbrbggrrbugwwuwuggrwgbubg
rugrggbrrubuubbrwrbwrgbgbgbgugrrbrbugggrurwuruwgwgw
rgggubbwgubwbwuurwbubrwruubwguwbuwuwbwrugu
rggurgruwuwwgurbuuubrwgrburbgbgbgbgrrubbgwgwbrrggurwrbbrbuwb
rgguwbbgrbuggwruuruuwbbwrrbrbbrwrwbwubgwuuwwuggggwbrru
uwbuuuuuurgwwbwwwrguuwrgguruwrwwgubwwuurbwwwuuwruw
wguggrgubwrrbwubbwbbwuugguuwgrwrwgwurwuugrruguwgbrrurrr
bbuuwrrrugrggrbrbuwrwwguwubwgwgrgwuugwguugwrruwruuwwub
rggbrrwuugwwwrgbgruwrrrrgruuwwuububbruuwguwgguwbbbb
ggrbuugwrguurbgwgwgwbbguwbrurgwugguubuurwrugwbrg
wbbwrguwuwbgbugugbuggrwwwgbgrgrggwgwgbrrurrbrgburbbgrrr
rggugwugbbuububrwgrggrrgbuuruwwwuurrbrgugruggrbbrbugrr
buwwbwrbwubrbgbgbrrbugugbbgubbbuwwgwurwguuggwuw
uggbuugbgugbruugwgrwruugbggwwwrbgwwbwguuwrwbwurbbrrgrg
rgwbrruwubrbbrgbbgburgbwrrgwguuwgbbuurwugruuggrugww
rgguuggwgrbwgrwwbrbggbrgbgrubbbwwwgrrbgubrr
uuuurwuuruuuuwbwrgrwgrbbbwburrgrbgguubrugggbgruwgwubgbw
rggurggbwrwgwgbwbubrwwbrwgrburrgbuwrrrrrbuuubwgubrur
rggbgrruwgubwrgrrrrrrwwgwuwbwgwwwgbggrgbrggrrrg
rrbrwbrbrwrgwrbbwrwgubgrwurbgrggwbrwubrbugwwwrr
wwubrgrbrgbuwwuggrrrbwrbwuubwgbwbgbruwrrwgguuuuwu
rggbuwuwuguuwgwuugwuwbrwugbrgurbrwbgbrbubbbbwbrubbwuuguww
uuwbrurbwurwrbwurwgbgwbuwrwwbgwuurbwrrwrugwbwwwwwgbb
ggubbwuwrrrbwrurgwgurbwuwbgwwbuguguurbwrurwubguruuurwururu
bubbwgrwrrwrbgrubwbggbwwbgrgwwugbrbrbwbwbwuguurbwwurbuw
bgugrurubgubwbgrwguruwubrurugwrburwbwbbuwu
rggbwuwwwbrrggwubwggwrurbgwbbrgwgbggruubggwggbwbwrbbwgrbbwww
rggwrbwurrrwbbuggrwrrbubgbbrbgguwrgugwbbw
uuuguururrgbwuggwbwubbrbrggbwbwrbuubwwbbrgbrubrggwwuuwwug
bwwrwwwrwwgwwrgbwubbuugwuubuugwrubbggwwbgwubwbbwbrrwubwbw
rggubwgwurgrbggwwuugrgwubggbbgugbwgrbuwgwrrwbuggrgwubrbubuw
uwwrwwwuwrburuwwbruwrgbrbrwuwubruubgrubwuubguwubgrwwwb
uuuubuuwgwgwgwggugbrubrruwgrubgwuwgwbbbgwbbgwrrrrgbwur
rggwurwguburbbbrggwgbbrguuwruwgguwrbgwbgwbugbbrggwbb
uwbrrwbwurwgrbgburugbbrgguwgugugbwgrurwgurruwggrwurwb
rgguuguwbgbgugwuguuggurruurrrwgrgrwurggwurbrwbwuwbubwugbubr
bwggwwrrbrwbubrrugruurrbrrgwuwrrbwrruwwwbrbrurubggwuurwr
rggwrbbubwbuububrwubbwubrwrrurrrggugrgugwbbbrurgggbrugwubu
ggwwwrgwbguubgwwbrwrggwurgbbuuguwbrbwbugubwuubw
rggggbrgwrwbgbbbggurbrurrrwrwbgbugguurwrwrwww
ubwbubwrgwgbwggbbrbbuwburbrgrguwwgbuggrgbwwrwgguubur
gbrwbuwbrwuuggbbuwwgbbgwbbbwwrrgwwwrgwwrrruggwrbgb
rbwrbgggrbguggurgrrwggrurrguuwrgrrubuuuuub
gbrbbrbbrgbruruwwggwwrgbbgbrggwubuwbgwubgg
rggwubwbrgguugurwuwwrgurwbwwbwwugwgrwgrb
rgwrrwbwrwurgwgwbgbguuwwubgbbwgurrbgrbrwguwrg
bggbubrgrwubgugugbuwgwburwgbrubrrbrwruwwguuu
rgguwggbwuugrbwgrrbrbugbbgggwrbuuwrgrbgrggbugbbr
brrbuubguubbuurububwwrwwbgruwbwbubbrbugrruuubwrbwugwwrg
wubgugwubbgrrruwrwwrwgbrwgruwburwgbrbgwwbwrrrgbbgwggwuwww
rggurwuruuugwuwgbwruwrwrugwwrrbrwgguurwwwgugubrrgbbgu
rgguurwuugguugwrbuwwrggrgrwrrrgwbbgurrwbbrwrg
rggurggruuwwrbrrrruburwgrurwurbbggwbwugbrbgrwburbwwgwrbw
rrggguuubggubbwgubbgwwbgbbwuguggrwwuuwubwgbbbbwuu
bwbrurbbggwubbbgwrrubrwubbugubggwrwrugbrbbwg
rggugggwgwgwwwburuubwggrbbrrwbuwwwuwrubggwwrwuuubb
urggbrbgwbbgubwgbugbgrugubgbwgrgbubgwbburwrbwgrb
rgguurgburgbrwwubgwbggwbuubgwugurrbubwgwggwgrugw
rggwwbwgggugrgrgugbwrgwbrgbgggrgwbgwgrbg
wurggurgbruwgrrwbwgwubgruuwwbwrbbrbbbwubuubgbgrgggbburgr
rggggwbbbububwbggwbbrwruggburugbwbrruuuw
rgggwbgguwbgggbrurugbgbbbburrrwwuuguwgrurugrwg
burbbguubbuwbwbwgugwwwbwggwggruubwwwrrbgrwuwbwrrbwbgrr
urgbwurwgbrubgrwgbwbwuwrwurgrrgrrurgrgbuwbuwwurbgu
rggwurrwwgwurwwwrwuwurrggrubgrgwrbgwrwrwuuruuuggugwugbrg
rgguuggwurbwuruuuuwgbwrrbbrwuwuuuuwrrwrgwgrbgrrb
rggbbugrrgrwrgrwrwgrrguwgbrwgggubbwgbuggubgrgrrgr
ggguwbggurgbubggwggbbwwgbuurrggwugwwbwuruuwb
rggbwugbuwwubbburwbrburwrrgrgggwubwwbwurgrbuubburwwg
bwbgbbbgrrggububrwbrgwrruguruggbugwgbwgwrbbgbrwgugwrwugbw
rgggbwbgrgugggrwwrggbwugbbbrwguwgrwubrrrggwgwb
rggugrguubuurwwurbuwrgguggbbbrbggruguwrbbwubbruwwgwwugrr
rggwbgrubugwrgrugbuggwrrbgguguwbguuuwbubwuguuuguggurub
rggurbwwubrgggwbrrbububgbuwubrbrguubgurwbwuuubg
bgbguuwurwbbggbgwugbgbrrgrrrggwruwuubrwbwuuwwgrww
bgggwgrbgggururbwwugwwbugrwgbwugwrwwrrbrurbubgbwrugwgrwgw
rgguuwgbbuwbbrwggbrrrrwuwwuubguururwugrurwubwgbur
gurbrgrbrbgggwrwwrgubbgbuuurgruwgwrrgrwrwrruuwgrbbgwwruuww
rgguugggwuubrrwugbbubguubrgwwgbbbrrwgwgggwbgwuug
rggubwruguwgrrbuwwrububwrguwwwgbuuuguwwrrwgu
wrugubwbwggbggrgrggrrguggrrggwwbgwgwbrbgbrgwwg
rgugbbuugwwuwbgwggwbbbuggrubbruuubgurrgwbuu
rggurugugrrwuwgrrggwgruugbbgwwbuuggrbwrbwgubwwwr
wwrruggruguruwgwbwwrgugguwubugrrgbbuuubrgg
uggbwwugwgbwbrugwgurubgbbbuwwuubgggwrrgugwbwuwbubrubwb
wgwubwwwwwwrrwwgbguugbbrurbbuuuugbuwubbwburwrgwbu
rggwrgbuwrubrubgggwwbbgwwbbwrbbrrubggurgbbgrr
rurbuubrbubwbwrgbbugbwruuubgubrbwwwgbuggbrr
rggwuggwgrruwgugbgrwruurgrubwuuwgruuwgbbgbgwbww
wrrugugbwgbbrwguguggburruwgburwwrubgwrurrrub
gbbggwuwrgwrrwbwbuwwbwrwrrgrruwuwgwrrurrugbrbbbuuwgb
rggwwwwurggwurwwuwrbbwwwrbwurwgbwbbwwuwwgwuruuwbuugbwbgugb
rrbgurwrwruubrrgbbgwuwuuwuugrrgrggwwwbugwrrgwgu
wrwbwurrbwgwbwgugurwggrwrugururwrggwrrubugguubgbbgrburbbrr
wgrgrgrwbbrwwbgbuugwwbrrrggggubrrbwurrwwwguwuggubwwww
rggbgubwugrgbbuuubbbuubbrbgbwwubwbrruubgwbrbubrgrbbggrw
rggwbbwgrgubrbbgubuwbgwwrrwubbwrggwugbuugrwugggwrr
rggbggwbubbbugwbwggwuggbrwubbrwwrgguuuggbbbbrwr
ubgwgrbuuurguggwwggruwwrrgbrbwbburwwuuuurguwbwwggbgrwbgw
guwugwbgrggbrwrrurrrwwuubgbbwggbrrgburuburwgwbwguw
wuggrrrwgwbbugwrguwbwrwwbwuubggruruwubwguguwruuu
rggwbwubgugbwbuggbbgruwurbwrggubrwgbububwbwubrbbwuubwgbbg
wggrwrrwrrbgrgbguburbbubgurgwruuuubgrbguuuggg
bguuwwwrwrbbguwggwgrbubwgbubbguwbrgguwrggbbwugwgrbrwrgwbb
wruwrgurbgggwgrubrgwwbbwggwwbuwrbguwwwubwgwrguwwrwurburgu
rggbuugwubuuuurbugrrbrrwrwwrbrbwguwrrgggrwbrwrwg
rgguubrwbuwguurguwwbgbbrgrrbubrgwbbbgrrbrugrurwu
rggbgrwggruwwubgbggrwgwggbwwwbuugugwgbgrburggurbbuugr
rbuugbbrrrgwwbbrrbwrrgrwgbugwrrwwrwurwwrubrw
ubrubgwwgubwwurbbwrugwuwguuwbbwubuugurrrggbgbgrbwwbrw
wuguwuwruwbwwruurbgwbugbwubgrgwwrwuruwbwrwggrubwgbuwwugbb
rggbwrugrbwggrbbbgrrrbrubgwgbrrwggurubgrbgrrbbrgwg
gurrgrugrwrggrgbbgbugbrbgwruwuurbuubwwrbwwgubbu
wwrwwugrrwggrubbgrggbuggrgurrwbbrrwgbwrrwrbrbgrgburgrwwuww
wrugwuwubwggugrgwwurrurwugurwbbuggrrwruwbbbrb
rrwuwbrguwrrbwuuubgbuwugurruwbuubwgwwrbwbgurgugwrgrwbrrbgg
rgguwrwbwrrgwbgguuguuuugbguuuuwugwrwrwgbwrubwr
rrrwbrguguugrggrrrwggrrwgwrbbrrwwgbguggggurwwrrwbuggrgwgw
rggbwubugugwgwrrurrwubgubrbuguwwruwwwuuwwurbgugrbrbgu
rggwuwwgugbbburbgruurwgurbwbrwgbgwwgbwwwurrbuurgg
rggbrbuwbwgrgbuurbbwbwgbrbbwruwurwbwwbbwgwbguububrbwbwr
rggbwwbgrugrwgbwubgwurwwbrbbwgbrbgrbbrruwuubgw
rggbgrruwugguugwuwbrbgurwurrgrguwwugwbrbbrguuwrwgrbgugbw
bwgwrrubwguwrrwgwuwrrbwwgrrbuubbuwbrbubgrbbrubugubgg
rggburrwubgwubuwugrruwuugugurgrwrwgrrbrbgggurrbrgwwwrwuwu
rggbuwgbuggbbuugwwgrrwbruugwwgrgrwrruwbuggwuwrrgwbgguu
rggubwgrrwbubwwwwwguurbrwuwbugbwwgrgbuwuuwrr
rggbguguugwgrbgggbbwrbwugwbbwbugbggbbrrgrgbuuruggrgwbwuubbr
rggurwwwwuuwggubbbgwgbrbruuurbruubwwgggbbgw
rgggubwwrgwrbrwwwwrrrgrwbwurbbbbbwwuurbwwrgwgbub
wrrwwrgrrgwgugwbuurguggrrbbbbrwbbuwuruubrbuuuwurbw
gbrwbbbugbgbwwrgugrrrruurbgwwguwuwgggbwuwwurbwrwrwbrru
wrbuubbgbwrrwwuggrrurgbgrwggwrbggwrwrbubwwru
rgguwbguwrgubrbwgubbguwgrwurwbwbrbbwbugrwgbugrgrwubgg
rggugrbgurgubrwgwgwbgruwwgbuggbrwwwrrwruwuwguw
guwwuguuugguuuwwgrwwgrrgbubugbbrrubwbuuguuwbbwwugbggrgwb
rggwwgbggbwrruwwrbbgbwwuruubuurgbgwrgubwrwwrg
rggggrbuuwuburruubwgwbgrruugbrububuuwggwbugbbbrwbwrwrgbww
gurbburbrgburguwubbuwubwbgwwuubwrwwuuwuubggrbgwgrubgggwbwb
rggbuguurbwubrbrwwbbrbbwurggwgrruwgwwwbubwurwbbgrwbrbrg
grbwrrgrrgggurbwrggruuwwrbubwubbwbuubgubbwwubggwrw
rggwrrrbbruugugwbwrugwrrrurbbgruwwugbwrubbr
rggbbubrwuurwggugwubguurubggrubgrwbrggwuwuwbbrrwuug
rggbrurbrwggbrgubuwbwbrruwwubwugbrgubgbwbgubgrwwbgwgugwurgrb
bguguwwbubrgbrubgggrgrburwwrrgbbgwuugugwbrwrbwwwrwwurug
wuuwuugubwuguuwrgwrggrwwgugbuugggurwgwurwrbwrrbbwgwguwbuu
rwubwbgrbruurbruwrwbuurubbbwgbruwuurwbbuwbw
rggbugrwbbgbgbrbbrbugbwggurggbwwubburgwuuwuggwbrrgwwrgguwu
rggbrrrugruwurwwwrrrgubwrubwubwrbruwwbwwbuuubwbbgbrgrrrrbb
ubgurgurubgwguwgrwwurugrbwrrgrbwwwggbwwwwwbgbrrubugwgbuwbr
rgggwuwrggbwgwwurwugugwwgbruwbbuwrrbwrruggwwbwgbgwuggwg
rggurrwgruubwgubrbbgbguuuwwguwgbrwgrwwrwbugwrbug
wbrbrbggwrwugwbrbgbwwwggggggugubwgbwbbuwuuguugg
bwbugwrbwgugugrbbbbrbbbbugurgwgruwwwwbbwgrwwwugbrbbgbwu
ruuuuubwgwbuwgugugugrgurrgwwuuguurwgubwbwgbuwgrbu
rggbubgwgwwrrwubgbrrgbgwgbwrgurrubgbubrbrgbgwuuwwwwr
rggwgbwwbuwuwurrburwwugwugwwgbbwuwrururubwrgwbb
rggwuurrwggwrugrrgwuugbrggbubbwwbbgbbbbgwgwugur
uugggububrbururwrbgrrwbgwrubrbugrwgggwwruruwbbwguww
rggugubwbgbugwwbugwgggburbbgwbggrugurbwwuwu
rggbrbwwgwuugggwwugwwrggwbrgrbrburbgbgwwbbwwuwrbr
uwwwwbbbgbwrgrruruuurburuuruugurwgwgggrubwwwgbrurgrg
gggbbbubugrbgurguguugrurguurugggbbruwuuruwwwwwr
wgrgbwrwwuugwwuugwbwuwubbrgubburrggrbgubruwbwrbwgburbu
rggwwwwgwgrbggwbrwbggubgbwgwubugwwgrbwwwg
ggbuwruuggrrurwuuugwbuguggwgbgwrugrgbbgwwrbwbrggrwbwugw
wwwwbbrwwubbugrrwuurbugwburbrurgggurwuwbub
rggbrruggwurggwwbbrwgwgruuurbuuwrgwuwgbruuubgurwgwrrrgwgw
wwurbuubgbbrwrugbugubwuugrrbwbbwgggurggwwuwwwruggurgwgr
ggrbgwrbrgwguwrwwwwgugrwbgugrrgbbwrbrrrbwbwgruurgbggbb
rgggrwbrwrgruwrbgurgbwwrrgbrbbwuwburgwubbrrbubwbrg
rgguggggbrrubwrwubbrwrrwguuuuubruwggbggrrbrgubrgwruggggrbug
bubgguguububrgrbgruurbbbbwwgrwurbuwbgbwrgwwuubwrbwwgbub
ruwurrrubwubuubgbggurgruruwuguwruurruwbbbwuwu
rgguwbwbbwrburwwgrbwwwrbwuugrbbruubbbwrbwuubwbuuwru
ruwwgggbrwwwbwrgrwgwuwbbugrgugbbubrbrbgbbwwubrbrbubbruu
rggwuwggbruubwwgggrbgrguwwgbgburwbbbugrggbwruwwuwubuwbuu
rgguwrrugwuwbbrwgggggwrrbrrwrruwwwrgubgr
rgguurburrrrubgrgbubugbubbggrrwwruuuurgubwgwggwbbbr
rggurgrugrrbuggwwwwrggbrggrwggbuwugbrrgbuubrrrggruggrwbgbrr
rggugrruuugbbwugwrbgrgbgwuuurbrrrgbgwwbwwwwrubrwuuwbbbbwrg
rggbrbwgwbbrrwrbwbwuggrwwbguugurbgbbrrbgrggwrggbwbwgub
wwrgbuwwububurguurwbubgubruwwuwwgwbrgrguwu
rgguurggbgwbgggrbggwgwuguwgbbrbwrgbbggbggbwb
rgggwbwbburbgubrguwwwbwuwbuwwgugbrrbwbuuuugggwurbwubwwbr
gwbwgrwbrwrbbwwrgubggrbrggugubugbwuubbgggruubbgrg
gwbgggrgrrrwubbrwrgrgbgwbgurrbbugrwuguggrrrrurbrr
gwwrwwbrbrbbuwwrurrbbgrugrwuwgugwurgggwubrburbrburwrgbrg
gbugggubrggwwwwrbrgwubrrurgbwwgrgbgwuurggguwrurgruuu
wwbuubuwguguwrwrgbrbuwubgrwwgrgrgburgbrwwuu
rgguwrbrrrwbggwbbubwgugbrguuwrugwbbruwbrwuggwuggu
bwwrrrrgrgwbggbrggggwwbwgwbbguuwgbububggwguuruugrrburr
ubwbwgubgrwwwrbgrbrgwrgrbwugrwrrguuugwwbuubbgbrbrbwgwbgrg
rggbgwubwrwrgwrubbgwgwgwwburuwgrgburgggbrrbgr
rgggwbrbrrugbwgbubugwbwrbbbuwrrrgbgburbbrurgggruuwwwrrubr
rgguwruwubwwurgrgrwuwgwurrgwguubbrurbwbubbrgwrbwuguugg
bgrruuuwbgguwuggrrwruwwwuguguururruugguwgwwubwg
ubwrwrbwgwuwuwgrurbuwbrbrrwbwbuwuuwggbwbgwwgu
rgguuwwgwgwbbgubuuuuugbubugruuwrgbbwwgrrwwr
ururguugbwwuwrrbrrgrgbbgrrbwguurubwgbbbrbrwgwgbgrbu
wwggubgbbgwurwbugbrwrwrgrgbuururuubbburgwwgrwrb
rgguwwwwwrwubrbwggwrbruwwrwgwrggwgrbrbbrgbuurwuuwuugbrggwbur
rbbrgwbgbbggbgwbwrbggurgrugbubwbwrwgwwrwrwbwbuwuggbrgwbwb
ubrwbgwuubwguubwuggrrwubbrbubgbrggrrbbgwgwrbrbuwgbuu
ubbubwugrrbwbwrguubwuwbwgwgbbuggbugrgrrbrrggbggggw
rggwgwubbbwrwwgwbbgbrbbrgwurugggwwuururgrbbgbbr
rggubbrubwuurguwburwgrggrbgrggbrugbwwuwwgbw
rggbbrbuguwrwggrgrrggurruwwbgwburrwrgwgbbbuuwrburbgrwuw
rgguguwwurbrguubgruggubgugrbubguwbbguwbwrw
bbgwuggrggwgwbwguuuuruwrgwuguwgwrbrwwwwwuggburuwgbuwwbrwbu
rgggwgwrugubuwggbwuugrurgbwwgwgrbwgwwbrwbwwuggwurrbbwbg
rgguubwwuuggwguubrbbubwrgggubrgguwwrwbugwbwbbrwgbwuwruubbr
bbwggrggrggwgwurruguuwbubbgurwbrwwbgggurrrbruu
rggwurrwugwbugwgwuruwrbrbggwugbgrbwgwbgruwbrrwwwuurb
uggwbgwwbwwbggbgruwbrrugwrgwubwbrbrrggwrruwwgbwwbbb
rggbbbwrrgwrubggbuwbrburuwggrgubwbrrgrrgbrwgwb
"#;
