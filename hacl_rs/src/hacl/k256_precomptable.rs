#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unreachable_patterns)]
#![allow(const_item_mutation)]

pub const precomp_basepoint_table_w4: [u64; 240] =
    [0u64, 0u64, 0u64, 0u64, 0u64, 1u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
        705178180786072u64, 3855836460717471u64, 4089131105950716u64, 3301581525494108u64,
        133858670344668u64, 2199641648059576u64, 1278080618437060u64, 3959378566518708u64,
        3455034269351872u64, 79417610544803u64, 1u64, 0u64, 0u64, 0u64, 0u64, 1282049064345544u64,
        971732600440099u64, 1014594595727339u64, 4392159187541980u64, 268327875692285u64,
        2411661712280539u64, 1092576199280126u64, 4328619610718051u64, 3535440816471627u64,
        95182251488556u64, 1893725512243753u64, 3619861457111820u64, 879374960417905u64,
        2868056058129113u64, 273195291893682u64, 2044797305960112u64, 2357106853933780u64,
        3563112438336058u64, 2430811541762558u64, 106443809495428u64, 2231357633909668u64,
        3641705835951936u64, 80642569314189u64, 2254841882373268u64, 149848031966573u64,
        2304615661367764u64, 2410957403736446u64, 2712754805859804u64, 2440183877540536u64,
        99784623895865u64, 3667773127482758u64, 1354899394473308u64, 3636602998800808u64,
        2709296679846364u64, 7253362091963u64, 3585950735562744u64, 935775991758415u64,
        4108078106735201u64, 556081800336307u64, 229585977163057u64, 4055594186679801u64,
        1767681004944933u64, 1432634922083242u64, 534935602949197u64, 251753159522567u64,
        2846474078499321u64, 4488649590348702u64, 2437476916025038u64, 3040577412822874u64,
        79405234918614u64, 3030621226551508u64, 2801117003929806u64, 1642927515498422u64,
        2802725079726297u64, 8472780626107u64, 866068070352655u64, 188080768545106u64,
        2152119998903058u64, 3391239985029665u64, 23820026013564u64, 2965064154891949u64,
        1846516097921398u64, 4418379948133146u64, 3137755426942400u64, 47705291301781u64,
        4278533051105665u64, 3453643211214931u64, 3379734319145156u64, 3762442192097039u64,
        40243003528694u64, 4063448994211201u64, 5697015368785u64, 1006545411838613u64,
        4242291693755210u64, 135184629190512u64, 264898689131035u64, 611796474823597u64,
        3255382250029089u64, 3490429246984696u64, 236558595864362u64, 2055934691551704u64,
        1487711670114502u64, 1823930698221632u64, 2130937287438472u64, 154610053389779u64,
        2746573287023216u64, 2430987262221221u64, 1668741642878689u64, 904982541243977u64,
        56087343124948u64, 393905062353536u64, 412681877350188u64, 3153602040979977u64,
        4466820876224989u64, 146579165617857u64, 2628741216508991u64, 747994231529806u64,
        750506569317681u64, 1887492790748779u64, 35259008682771u64, 2085116434894208u64,
        543291398921711u64, 1144362007901552u64, 679305136036846u64, 141090902244489u64,
        632480954474859u64, 2384513102652591u64, 2225529790159790u64, 692258664851625u64,
        198681843567699u64, 2397092587228181u64, 145862822166614u64, 196976540479452u64,
        3321831130141455u64, 69266673089832u64, 4469644227342284u64, 3899271145504796u64,
        1261890974076660u64, 525357673886694u64, 182135997828583u64, 4292760618810332u64,
        3404186545541683u64, 312297386688768u64, 204377466824608u64, 230900767857952u64,
        3871485172339693u64, 779449329662955u64, 978655822464694u64, 2278252139594027u64,
        104641527040382u64, 3528840153625765u64, 4484699080275273u64, 1463971951102316u64,
        4013910812844749u64, 228915589433620u64, 1209641433482461u64, 4043178788774759u64,
        3008668238856634u64, 1448425089071412u64, 26269719725037u64, 3330785027545223u64,
        852657975349259u64, 227245054466105u64, 1534632353984777u64, 207715098574660u64,
        3209837527352280u64, 4051688046309066u64, 3839009590725955u64, 1321506437398842u64,
        68340219159928u64, 1806950276956275u64, 3923908055275295u64, 743963253393575u64,
        42162407478783u64, 261334584474610u64, 3728224928885214u64, 4004701081842869u64,
        709043201644674u64, 4267294249150171u64, 255540582975025u64, 875490593722211u64,
        796393708218375u64, 14774425627956u64, 1500040516752097u64, 141076627721678u64,
        2634539368480628u64, 1106488853550103u64, 2346231921151930u64, 897108283954283u64,
        64616679559843u64, 400244949840943u64, 1731263826831733u64, 1649996579904651u64,
        3643693449640761u64, 172543068638991u64, 329537981097182u64, 2029799860802869u64,
        4377737515208862u64, 29103311051334u64, 265583594111499u64, 3798074876561255u64,
        184749333259352u64, 3117395073661801u64, 3695784565008833u64, 64282709896721u64,
        1618968913246422u64, 3185235128095257u64, 3288745068118692u64, 1963818603508782u64,
        281054350739495u64, 1658639050810346u64, 3061097601679552u64, 3023781433263746u64,
        2770283391242475u64, 144508864751908u64, 173576288079856u64, 46114579547054u64,
        1679480127300211u64, 1683062051644007u64, 117183826129323u64, 1894068608117440u64,
        3846899838975733u64, 4289279019496192u64, 176995887914031u64, 78074942938713u64,
        454207263265292u64, 972683614054061u64, 808474205144361u64, 942703935951735u64,
        134460241077887u64];

pub const precomp_g_pow2_64_table_w4: [u64; 240] =
    [0u64, 0u64, 0u64, 0u64, 0u64, 1u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
        4496295042185355u64, 3125448202219451u64, 1239608518490046u64, 2687445637493112u64,
        77979604880139u64, 3360310474215011u64, 1216410458165163u64, 177901593587973u64,
        3209978938104985u64, 118285133003718u64, 434519962075150u64, 1114612377498854u64,
        3488596944003813u64, 450716531072892u64, 66044973203836u64, 2822827191156652u64,
        2417714248626059u64, 2173117567943u64, 961513119252459u64, 233852556538333u64,
        3014783730323962u64, 2955192634004574u64, 580546524951282u64, 2982973948711252u64,
        226295722018730u64, 26457116218543u64, 3401523493637663u64, 2597746825024790u64,
        1789211180483113u64, 155862365823427u64, 4056806876632134u64, 1742291745730568u64,
        3527759000626890u64, 3740578471192596u64, 177295097700537u64, 1533961415657770u64,
        4305228982382487u64, 4069090871282711u64, 4090877481646667u64, 220939617041498u64,
        2057548127959588u64, 45185623103252u64, 2871963270423449u64, 3312974792248749u64,
        8710601879528u64, 570612225194540u64, 2045632925323972u64, 1263913878297555u64,
        1294592284757719u64, 238067747295054u64, 1576659948829386u64, 2315159636629917u64,
        3624867787891655u64, 647628266663887u64, 75788399640253u64, 710811707847797u64,
        130020650130128u64, 1975045425972589u64, 136351545314094u64, 229292031212337u64,
        1061471455264148u64, 3281312694184822u64, 1692442293921797u64, 4171008525509513u64,
        275424696197549u64, 1170296303921965u64, 4154092952807735u64, 4371262070870741u64,
        835769811036496u64, 275812646528189u64, 4006745785521764u64, 1965172239781114u64,
        4121055644916429u64, 3578995380229569u64, 169798870760022u64, 1834234783016431u64,
        3186919121688538u64, 1894269993170652u64, 868603832348691u64, 110978471368876u64,
        1659296605881532u64, 3257830829309297u64, 3381509832701119u64, 4016163121121296u64,
        265240263496294u64, 4411285343933251u64, 728746770806400u64, 1767819098558739u64,
        3002081480892841u64, 96312133241935u64, 468184501392107u64, 2061529496271208u64,
        801565111628867u64, 3380678576799273u64, 121814978170941u64, 3340363319165433u64,
        2764604325746928u64, 4475755976431968u64, 3678073419927081u64, 237001357924061u64,
        4110487014553450u64, 442517757833404u64, 3976758767423859u64, 2559863799262476u64,
        178144664279213u64, 2488702171798051u64, 4292079598620208u64, 1642918280217329u64,
        3694920319798108u64, 111735528281657u64, 2904433967156033u64, 4391518032143166u64,
        3018885875516259u64, 3730342681447122u64, 10320273322750u64, 555845881555519u64,
        58355404017985u64, 379009359053696u64, 450317203955503u64, 271063299686173u64,
        910340241794202u64, 4145234574853890u64, 2059755654702755u64, 626530377112246u64,
        188918989156857u64, 3316657461542117u64, 778033563170765u64, 3568562306532187u64,
        2888619469733481u64, 4364919962337u64, 4095057288587059u64, 2275461355379988u64,
        1507422995910897u64, 3737691697116252u64, 28779913258578u64, 131453301647952u64,
        3613515597508469u64, 2389606941441321u64, 2135459302594806u64, 105517262484263u64,
        2973432939331401u64, 3447096622477885u64, 684654106536844u64, 2815198316729695u64,
        280303067216071u64, 1841014812927024u64, 1181026273060917u64, 4092989148457730u64,
        1381045116206278u64, 112475725893965u64, 2309144740156686u64, 1558825847609352u64,
        2008068002046292u64, 3153511625856423u64, 38469701427673u64, 4240572315518056u64,
        2295170987320580u64, 187734093837094u64, 301041528077172u64, 234553141005715u64,
        4170513699279606u64, 1600132848196146u64, 3149113064155689u64, 2733255352600949u64,
        144915931419495u64, 1221012073888926u64, 4395668111081710u64, 2464799161496070u64,
        3664256125241313u64, 239705368981290u64, 1415181408539490u64, 2551836620449074u64,
        3003106895689578u64, 968947218886924u64, 270781532362673u64, 2905980714350372u64,
        3246927349288975u64, 2653377642686974u64, 1577457093418263u64, 279488238785848u64,
        568335962564552u64, 4251365041645758u64, 1257832559776007u64, 2424022444243863u64,
        261166122046343u64, 4399874608082116u64, 640509987891568u64, 3119706885332220u64,
        1990185416694007u64, 119390098529341u64, 220106534694050u64, 937225880034895u64,
        656288151358882u64, 1766967254772100u64, 197900790969750u64, 2992539221608875u64,
        3960297171111858u64, 3499202002925081u64, 1103060980924705u64, 13670895919578u64,
        430132744187721u64, 1206771838050953u64, 2474749300167198u64, 296299539510780u64,
        61565517686436u64, 752778559080573u64, 3049015829565410u64, 3538647632527371u64,
        1640473028662032u64, 182488721849306u64, 1234378482161516u64, 3736205988606381u64,
        2814216844344487u64, 3877249891529557u64, 51681412928433u64, 4275336620301239u64,
        3084074032750651u64, 42732308350456u64, 3648603591552229u64, 142450621701603u64,
        4020045475009854u64, 1050293952073054u64, 1974773673079851u64, 1815515638724020u64,
        104845375825434u64];

pub const precomp_g_pow2_128_table_w4: [u64; 240] =
    [0u64, 0u64, 0u64, 0u64, 0u64, 1u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
        1277614565900951u64, 378671684419493u64, 3176260448102880u64, 1575691435565077u64,
        167304528382180u64, 2600787765776588u64, 7497946149293u64, 2184272641272202u64,
        2200235265236628u64, 265969268774814u64, 1913228635640715u64, 2831959046949342u64,
        888030405442963u64, 1817092932985033u64, 101515844997121u64, 3309468394859588u64,
        3965334773689948u64, 1945272965790738u64, 4450939211427964u64, 211349698782702u64,
        2085160302160079u64, 212812506072603u64, 3646122434511764u64, 1711405092320514u64,
        95160920508464u64, 1677683368518073u64, 4384656939250953u64, 3548591046529893u64,
        1683233536091384u64, 105919586159941u64, 1941416002726455u64, 246264372248216u64,
        3063044110922228u64, 3772292170415825u64, 222933374989815u64, 2417211163452935u64,
        2018230365573200u64, 1985974538911047u64, 1387197705332739u64, 186400825584956u64,
        2469330487750329u64, 1291983813301638u64, 333416733706302u64, 3413315564261070u64,
        189444777569683u64, 1062005622360420u64, 1800197715938740u64, 3693110992551647u64,
        626990328941945u64, 40998857100520u64, 3921983552805085u64, 1016632437340656u64,
        4016615929950878u64, 2682554586771281u64, 7043555162389u64, 3333819830676567u64,
        4120091964944036u64, 1960788263484015u64, 1642145656273304u64, 252814075789128u64,
        3085777342821357u64, 4166637997604052u64, 1339401689756469u64, 845938529607551u64,
        223351828189283u64, 1148648705186890u64, 1230525014760605u64, 1869739475126720u64,
        4193966261205530u64, 175684010336013u64, 4476719358931508u64, 4209547487457638u64,
        2197536411673724u64, 3010838433412303u64, 169318997251483u64, 49493868302162u64,
        3594601099078584u64, 3662420905445942u64, 3606544932233685u64, 270643652662165u64,
        180681786228544u64, 2095882682308564u64, 813484483841391u64, 1622665392824698u64,
        113821770225137u64, 3075432444115417u64, 716502989978722u64, 2304779892217245u64,
        1760144151770127u64, 235719156963938u64, 3180013070471143u64, 1331027634540579u64,
        552273022992392u64, 2858693077461887u64, 197914407731510u64, 187252310910959u64,
        4160637171377125u64, 3225059526713298u64, 2574558217383978u64, 249695600622489u64,
        364988742814327u64, 4245298536326258u64, 1812464706589342u64, 2734857123772998u64,
        120105577124628u64, 160179251271109u64, 3604555733307834u64, 150380003195715u64,
        1574304909935121u64, 142190285600761u64, 1835385847725651u64, 3168087139615901u64,
        3201434861713736u64, 741757984537760u64, 163585009419543u64, 3837997981109783u64,
        3771946407870997u64, 2867641360295452u64, 3097548691501578u64, 124624912142104u64,
        2729896088769328u64, 1087786827035225u64, 3934000813818614u64, 1176792318645055u64,
        125311882169270u64, 3530709439299502u64, 1561477829834527u64, 3927894570196761u64,
        3957765307669212u64, 105720519513730u64, 3758969845816997u64, 2738320452287300u64,
        2380753632109507u64, 2762090901149075u64, 123455059136515u64, 4222807813169807u64,
        118064783651432u64, 2877694712254934u64, 3535027426396448u64, 100175663703417u64,
        3287921121213155u64, 4497246481824206u64, 1960809949007025u64, 3236854264159102u64,
        35028112623717u64, 338838627913273u64, 2827531947914645u64, 4231826783810670u64,
        1082490106100389u64, 13267544387448u64, 4249975884259105u64, 2844862161652484u64,
        262742197948971u64, 3525653802457116u64, 269963889261701u64, 3690062482117102u64,
        675413453822147u64, 2170937868437574u64, 2367632187022010u64, 214032802409445u64,
        2054007379612477u64, 3558050826739009u64, 266827184752634u64, 1946520293291195u64,
        238087872386556u64, 490056555385700u64, 794405769357386u64, 3886901294859702u64,
        3120414548626348u64, 84316625221136u64, 223073962531835u64, 4280846460577631u64,
        344296282849308u64, 3522116652699457u64, 171817232053075u64, 3296636283062273u64,
        3587303364425579u64, 1033485783633331u64, 3686984130812906u64, 268290803650477u64,
        2803988215834467u64, 3821246410529720u64, 1077722388925870u64, 4187137036866164u64,
        104696540795905u64, 998770003854764u64, 3960768137535019u64, 4293792474919135u64,
        3251297981727034u64, 192479028790101u64, 1175880869349935u64, 3506949259311937u64,
        2161711516160714u64, 2506820922270187u64, 131002200661047u64, 3532399477339994u64,
        2515815721228719u64, 4274974119021502u64, 265752394510924u64, 163144272153395u64,
        2824260010502991u64, 517077012665142u64, 602987073882924u64, 2939630061751780u64,
        59211609557440u64, 963423614549333u64, 495476232754434u64, 94274496109103u64,
        2245136222990187u64, 185414764872288u64, 2266067668609289u64, 3873978896235927u64,
        4428283513152105u64, 3881481480259312u64, 207746202010862u64, 1609437858011364u64,
        477585758421515u64, 3850430788664649u64, 2682299074459173u64, 149439089751274u64,
        3665760243877698u64, 1356661512658931u64, 1675903262368322u64, 3355649228050892u64,
        99772108898412u64];

pub const precomp_g_pow2_192_table_w4: [u64; 240] =
    [0u64, 0u64, 0u64, 0u64, 0u64, 1u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
        34056422761564u64, 3315864838337811u64, 3797032336888745u64, 2580641850480806u64,
        208048944042500u64, 1233795288689421u64, 1048795233382631u64, 646545158071530u64,
        1816025742137285u64, 12245672982162u64, 2119364213800870u64, 2034960311715107u64,
        3172697815804487u64, 4185144850224160u64, 2792055915674u64, 795534452139321u64,
        3647836177838185u64, 2681403398797991u64, 3149264270306207u64, 278704080615511u64,
        2752552368344718u64, 1363840972378818u64, 1877521512083293u64, 1862111388059470u64,
        36200324115014u64, 4183622899327217u64, 747381675363076u64, 2772916395314624u64,
        833767013119965u64, 246274452928088u64, 1526238021297781u64, 3327534966022747u64,
        1169012581910517u64, 4430894603030025u64, 149242742442115u64, 1002569704307172u64,
        2763252093432365u64, 3037748497732938u64, 2329811173939457u64, 270769113180752u64,
        4344092461623432u64, 892200524589382u64, 2511418516713970u64, 103575031265398u64,
        183736033430252u64, 583003071257308u64, 3357167344738425u64, 4038099763242651u64,
        1776250620957255u64, 51334115864192u64, 2616405698969611u64, 1196364755910565u64,
        3135228056210500u64, 533729417611761u64, 86564351229326u64, 98936129527281u64,
        4425305036630677u64, 2980296390253408u64, 2487091677325739u64, 10501977234280u64,
        1805646499831077u64, 3120615962395477u64, 3634629685307533u64, 3009632755291436u64,
        16794051906523u64, 2465481597883214u64, 211492787490403u64, 1120942867046103u64,
        486438308572108u64, 76058986271771u64, 2435216584587357u64, 3076359381968283u64,
        1071594491489655u64, 3148707450339154u64, 249332205737851u64, 4171051176626809u64,
        3165176227956388u64, 2400901591835233u64, 1435783621333022u64, 20312753440321u64,
        1767293887448005u64, 685150647587522u64, 2957187934449906u64, 382661319140439u64,
        177583591139601u64, 2083572648630743u64, 1083410277889419u64, 4267902097868310u64,
        679989918385081u64, 123155311554032u64, 2830267662472020u64, 4476040509735924u64,
        526697201585144u64, 3465306430573135u64, 2296616218591u64, 1270626872734279u64,
        1049740198790549u64, 4197567214843444u64, 1962225231320591u64, 186125026796856u64,
        737027567341142u64, 4364616098174u64, 3618884818756660u64, 1236837563717668u64,
        162873772439548u64, 3081542470065122u64, 910331750163991u64, 2110498143869827u64,
        3208473121852657u64, 94687786224509u64, 4113309027567819u64, 4272179438357536u64,
        1857418654076140u64, 1672678841741004u64, 94482160248411u64, 1928652436799020u64,
        1750866462381515u64, 4048060485672270u64, 4006680581258587u64, 14850434761312u64,
        2828734997081648u64, 1975589525873972u64, 3724347738416009u64, 597163266689736u64,
        14568362978551u64, 2203865455839744u64, 2237034958890595u64, 1863572986731818u64,
        2329774560279041u64, 245105447642201u64, 2179697447864822u64, 1769609498189882u64,
        1916950746430931u64, 847019613787312u64, 163210606565100u64, 3658248417400062u64,
        717138296045881u64, 42531212306121u64, 1040915917097532u64, 77364489101310u64,
        539253504015590u64, 732690726289841u64, 3401622034697806u64, 2864593278358513u64,
        142611941887017u64, 536364617506702u64, 845071859974284u64, 4461787417089721u64,
        2633811871939723u64, 113619731985610u64, 2535870015489566u64, 2146224665077830u64,
        2593725534662047u64, 1332349537449710u64, 153375287068096u64, 3689977177165276u64,
        3631865615314120u64, 184644878348929u64, 2220481726602813u64, 204002551273091u64,
        3022560051766785u64, 3125940458001213u64, 4258299086906325u64, 1072471915162030u64,
        2797562724530u64, 3974298156223059u64, 1624778551002554u64, 3490703864485971u64,
        2533877484212458u64, 176107782538555u64, 4275987398312137u64, 4397120757693722u64,
        3001292763847390u64, 1556490837621310u64, 70442953037671u64, 1558915972545974u64,
        744724505252845u64, 2697230204313363u64, 3495671924212144u64, 95744296878924u64,
        1508848630912047u64, 4163599342850968u64, 1234988733935901u64, 3789722472212706u64,
        219522007052022u64, 2106597506701262u64, 3231115099832239u64, 1296436890593905u64,
        1016795619587656u64, 231150565033388u64, 4205501688458754u64, 2271569140386062u64,
        3421769599058157u64, 4118408853784554u64, 276709341465173u64, 2681340614854362u64,
        2514413365628788u64, 62294545067341u64, 277610220069365u64, 252463150123799u64,
        2547353593759399u64, 1857438147448607u64, 2964811969681256u64, 3303706463835387u64,
        248936570980853u64, 3208982702478009u64, 2518671051730787u64, 727433853033835u64,
        1290389308223446u64, 220742793981035u64, 3851225361654709u64, 2307489307934273u64,
        1151710489948266u64, 289775285210516u64, 222685002397295u64, 1222117478082108u64,
        2822029169395728u64, 1172146252219882u64, 2626108105510259u64, 209803527887167u64,
        2718831919953281u64, 4348638387588593u64, 3761438313263183u64, 13169515318095u64,
        212893621229476u64];

pub const precomp_basepoint_table_w5: [u64; 480] =
    [0u64, 0u64, 0u64, 0u64, 0u64, 1u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
        705178180786072u64, 3855836460717471u64, 4089131105950716u64, 3301581525494108u64,
        133858670344668u64, 2199641648059576u64, 1278080618437060u64, 3959378566518708u64,
        3455034269351872u64, 79417610544803u64, 1u64, 0u64, 0u64, 0u64, 0u64, 1282049064345544u64,
        971732600440099u64, 1014594595727339u64, 4392159187541980u64, 268327875692285u64,
        2411661712280539u64, 1092576199280126u64, 4328619610718051u64, 3535440816471627u64,
        95182251488556u64, 1893725512243753u64, 3619861457111820u64, 879374960417905u64,
        2868056058129113u64, 273195291893682u64, 2044797305960112u64, 2357106853933780u64,
        3563112438336058u64, 2430811541762558u64, 106443809495428u64, 2231357633909668u64,
        3641705835951936u64, 80642569314189u64, 2254841882373268u64, 149848031966573u64,
        2304615661367764u64, 2410957403736446u64, 2712754805859804u64, 2440183877540536u64,
        99784623895865u64, 3667773127482758u64, 1354899394473308u64, 3636602998800808u64,
        2709296679846364u64, 7253362091963u64, 3585950735562744u64, 935775991758415u64,
        4108078106735201u64, 556081800336307u64, 229585977163057u64, 4055594186679801u64,
        1767681004944933u64, 1432634922083242u64, 534935602949197u64, 251753159522567u64,
        2846474078499321u64, 4488649590348702u64, 2437476916025038u64, 3040577412822874u64,
        79405234918614u64, 3030621226551508u64, 2801117003929806u64, 1642927515498422u64,
        2802725079726297u64, 8472780626107u64, 866068070352655u64, 188080768545106u64,
        2152119998903058u64, 3391239985029665u64, 23820026013564u64, 2965064154891949u64,
        1846516097921398u64, 4418379948133146u64, 3137755426942400u64, 47705291301781u64,
        4278533051105665u64, 3453643211214931u64, 3379734319145156u64, 3762442192097039u64,
        40243003528694u64, 4063448994211201u64, 5697015368785u64, 1006545411838613u64,
        4242291693755210u64, 135184629190512u64, 264898689131035u64, 611796474823597u64,
        3255382250029089u64, 3490429246984696u64, 236558595864362u64, 2055934691551704u64,
        1487711670114502u64, 1823930698221632u64, 2130937287438472u64, 154610053389779u64,
        2746573287023216u64, 2430987262221221u64, 1668741642878689u64, 904982541243977u64,
        56087343124948u64, 393905062353536u64, 412681877350188u64, 3153602040979977u64,
        4466820876224989u64, 146579165617857u64, 2628741216508991u64, 747994231529806u64,
        750506569317681u64, 1887492790748779u64, 35259008682771u64, 2085116434894208u64,
        543291398921711u64, 1144362007901552u64, 679305136036846u64, 141090902244489u64,
        632480954474859u64, 2384513102652591u64, 2225529790159790u64, 692258664851625u64,
        198681843567699u64, 2397092587228181u64, 145862822166614u64, 196976540479452u64,
        3321831130141455u64, 69266673089832u64, 4469644227342284u64, 3899271145504796u64,
        1261890974076660u64, 525357673886694u64, 182135997828583u64, 4292760618810332u64,
        3404186545541683u64, 312297386688768u64, 204377466824608u64, 230900767857952u64,
        3871485172339693u64, 779449329662955u64, 978655822464694u64, 2278252139594027u64,
        104641527040382u64, 3528840153625765u64, 4484699080275273u64, 1463971951102316u64,
        4013910812844749u64, 228915589433620u64, 1209641433482461u64, 4043178788774759u64,
        3008668238856634u64, 1448425089071412u64, 26269719725037u64, 3330785027545223u64,
        852657975349259u64, 227245054466105u64, 1534632353984777u64, 207715098574660u64,
        3209837527352280u64, 4051688046309066u64, 3839009590725955u64, 1321506437398842u64,
        68340219159928u64, 1806950276956275u64, 3923908055275295u64, 743963253393575u64,
        42162407478783u64, 261334584474610u64, 3728224928885214u64, 4004701081842869u64,
        709043201644674u64, 4267294249150171u64, 255540582975025u64, 875490593722211u64,
        796393708218375u64, 14774425627956u64, 1500040516752097u64, 141076627721678u64,
        2634539368480628u64, 1106488853550103u64, 2346231921151930u64, 897108283954283u64,
        64616679559843u64, 400244949840943u64, 1731263826831733u64, 1649996579904651u64,
        3643693449640761u64, 172543068638991u64, 329537981097182u64, 2029799860802869u64,
        4377737515208862u64, 29103311051334u64, 265583594111499u64, 3798074876561255u64,
        184749333259352u64, 3117395073661801u64, 3695784565008833u64, 64282709896721u64,
        1618968913246422u64, 3185235128095257u64, 3288745068118692u64, 1963818603508782u64,
        281054350739495u64, 1658639050810346u64, 3061097601679552u64, 3023781433263746u64,
        2770283391242475u64, 144508864751908u64, 173576288079856u64, 46114579547054u64,
        1679480127300211u64, 1683062051644007u64, 117183826129323u64, 1894068608117440u64,
        3846899838975733u64, 4289279019496192u64, 176995887914031u64, 78074942938713u64,
        454207263265292u64, 972683614054061u64, 808474205144361u64, 942703935951735u64,
        134460241077887u64, 2104196179349630u64, 501632371208418u64, 1666838991431177u64,
        445606193139838u64, 73704603396096u64, 3140284774064777u64, 1356066420820179u64,
        227054159419281u64, 1847611229198687u64, 82327838827660u64, 3704027573265803u64,
        1585260489220244u64, 4404647914931933u64, 2424649827425515u64, 206821944206116u64,
        1508635776287972u64, 1933584575629676u64, 1903635423783032u64, 4193642165165650u64,
        234321074690644u64, 210406774251925u64, 1965845668185599u64, 3059839433804731u64,
        1933300510683631u64, 150696600689211u64, 4069293682158567u64, 4346344602660044u64,
        312200249664561u64, 2495020807621840u64, 1912707714385u64, 299345978159762u64,
        1164752722686920u64, 225322433710338u64, 3128747381283759u64, 275659067815583u64,
        1489671057429039u64, 1567693343342676u64, 921672046098071u64, 3707418899384085u64,
        54646424931593u64, 4026733380127147u64, 2933435393699231u64, 3356593659521967u64,
        3637750749325529u64, 232939412379045u64, 2298399636043069u64, 270361546063041u64,
        2523933572551420u64, 3456896091572950u64, 185447004732850u64, 429322937697821u64,
        2579704215668222u64, 695065378803349u64, 3987916247731243u64, 255159546348233u64,
        3057777929921282u64, 1608970699916312u64, 1902369623063807u64, 1413619643652777u64,
        94983996321227u64, 2832873179548050u64, 4335430233622555u64, 1559023976028843u64,
        3297181988648895u64, 100072021232323u64, 2124984034109675u64, 4501252835618918u64,
        2053336899483297u64, 638807226463876u64, 278445213600634u64, 2311236445660555u64,
        303317664040012u64, 2659353858089024u64, 3598827423980130u64, 176059343827873u64,
        3891639526275437u64, 252823982819463u64, 3404823300622345u64, 2758370772497456u64,
        91397496598783u64, 2248661144141892u64, 491087075271969u64, 1786344894571315u64,
        452497694885923u64, 34039628873357u64, 2116503165025197u64, 4436733709429923u64,
        3045800776819238u64, 1385518906078375u64, 110495603336764u64, 4051447296249587u64,
        1103557421498625u64, 1840785058439622u64, 425322753992314u64, 98330046771676u64,
        365407468686431u64, 2611246859977123u64, 3050253933135339u64, 1006482220896688u64,
        166818196428389u64, 3415236093104372u64, 1762308883882288u64, 1327828123094558u64,
        3403946425556706u64, 96503464455441u64, 3893015304031471u64, 3740839477490397u64,
        2411470812852231u64, 940927462436211u64, 163825285911099u64, 1622441495640386u64,
        850224095680266u64, 76199085900939u64, 1941852365144042u64, 140326673652807u64,
        3161611011249524u64, 317297150009965u64, 2145053259340619u64, 2180498176457552u64,
        38457740506224u64, 394174899129468u64, 2687474560485245u64, 1542175980184516u64,
        1628502671124819u64, 48477401124385u64, 4474181600025082u64, 2142747956365708u64,
        1638299432475478u64, 2005869320353249u64, 112292630760956u64, 1887521965171588u64,
        457587531429696u64, 840994209504042u64, 4268060856325798u64, 195597993440388u64,
        4148484749020338u64, 2074885000909672u64, 2309839019263165u64, 2087616209681024u64,
        257214370719966u64, 2331363508376581u64, 1233124357504711u64, 2849542202650296u64,
        3790982825325736u64, 13381453503890u64, 1665246594531069u64, 4165624287443904u64,
        3418759698027493u64, 2118493255117399u64, 136249206366067u64, 4064050233283309u64,
        1368779887911300u64, 4370550759530269u64, 66992990631341u64, 84442368922270u64,
        2139322635321394u64, 2076163483726795u64, 657097866349103u64, 2095579409488071u64,
        226525774791341u64, 4445744257665359u64, 2035752839278107u64, 1998242662838304u64,
        1601548415521694u64, 151297684296198u64, 1350963039017303u64, 2624916349548281u64,
        2018863259670197u64, 2717274357461290u64, 94024796961533u64, 711335520409111u64,
        4322093765820263u64, 2041650358174649u64, 3439791603157577u64, 179292018616267u64,
        2436436921286669u64, 3905268797208340u64, 2829194895162985u64, 1355175382191543u64,
        55128779761539u64, 2648428998786922u64, 869805912573515u64, 3706708942847864u64,
        2785288916584667u64, 37156862850147u64, 1422245336293228u64, 4497066058933021u64,
        85588912978349u64, 2616252221194611u64, 53506393720989u64, 3727539190732644u64,
        872132446545237u64, 933583590986077u64, 3794591170581203u64, 167875550514069u64,
        2267466834993297u64, 3072652681756816u64, 2108499037430803u64, 1606735192928366u64,
        72339568815255u64, 3258484260684219u64, 3277927277719855u64, 2459560373011535u64,
        1672794293294033u64, 227460934880669u64, 3702454405413705u64, 106168148441676u64,
        1356617643071159u64, 3280896569942762u64, 142618711614302u64, 4291782740862057u64,
        4141020884874235u64, 3720787221267125u64, 552884940089351u64, 174626154407180u64,
        972071013326540u64, 4458530419931903u64, 4435168973822858u64, 1902967548748411u64,
        53007977605840u64, 2453997334323925u64, 3653077937283262u64, 850660265046356u64,
        312721924805450u64, 268503679240683u64, 256960167714122u64, 1474492507858350u64,
        2456345526438488u64, 3686029507160255u64, 279158933010398u64, 3646946293948063u64,
        704477527214036u64, 3387744169891031u64, 3772622670980241u64, 136368897543304u64,
        3744894052577607u64, 1976007214443430u64, 2090045379763451u64, 968565474458988u64,
        234295114806066u64];
