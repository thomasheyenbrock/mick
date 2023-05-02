use crate::{board::Board, castling_rights::CastlingRights, side::Side, square::Square};

pub struct Zorbist {
    piece_boards: [[u64; 64]; 12],
    side_to_move: u64,
    castling_rights: [u64; 16],
    en_passant_target: [u64; 8],
}

impl Zorbist {
    /// This was created using the `zorbist` subcommand with a seed value of 42
    pub const DEFAULT: Zorbist = Zorbist {
        side_to_move: 2939647976913485485,
        piece_boards: [
            [
                5612343741886906158,
                5452211403613372577,
                13439519126536318205,
                9947736379437784723,
                2786141136387072907,
                5873408492745772459,
                8472530172997478856,
                267327539070441925,
                4270017712548254976,
                4466556548229549858,
                1766236354768281404,
                7164793947985299523,
                16157068577368686473,
                14236266120913393914,
                10231234793437854686,
                14608185237329119605,
                1393193993351648064,
                7213958656856335785,
                3102907366991594050,
                4878962735898882738,
                11805809806811710801,
                16727713531311639580,
                7417648798680592675,
                16489703862847782354,
                10980180936451329420,
                17507736078531051105,
                5344423495932281295,
                6537282405044875692,
                6284393596469393488,
                11356421843933695168,
                2496306458581305670,
                8903956306026309760,
                9842481137521136272,
                17408020955249072875,
                17243606229328828371,
                1131426391891300369,
                600314297388711731,
                8597099033679854213,
                13718837376483754293,
                16531445070767262580,
                17228990081184085480,
                15779682836263366960,
                9905081232633771199,
                4446494979106085632,
                12564130647156897133,
                4079803179502755835,
                7881541547532500804,
                10746748058862982401,
                14340180309204008853,
                13239275394000542058,
                11523934645856619209,
                2315580213555741211,
                10156890315986494010,
                6776367085492898721,
                8220988790906199397,
                8081040131198311686,
                6667893088790125576,
                7086256258625245304,
                3622299775101032606,
                7800988731422817766,
                12412582268967783275,
                14577528805957479376,
                12501607310472568467,
                3401415414907309556,
            ],
            [
                15340639094564699554,
                7029319930420779671,
                9783901258014516662,
                1582574054400998814,
                14014502415505233625,
                3645965833991913067,
                11644122777352903966,
                9224738305457199727,
                7602873731125814513,
                2092157895417474523,
                13508346964887891660,
                17648094380590596864,
                16420730943866755893,
                11879586442257370585,
                14924565245109558857,
                7198802593224683682,
                11462022756737018654,
                3209896855485027182,
                11551585560991437771,
                17663005755440503286,
                17245023698953642185,
                12405035798359643853,
                12553229305299543023,
                7695815280132593518,
                17821718004141110471,
                7103762613606332663,
                15574015762201919121,
                5052938882619327297,
                9630389419857579818,
                11069618338850955899,
                5127268456743077249,
                12616022069555095821,
                4596760825754441095,
                919751611747505239,
                13310669937866117114,
                18273175602592545892,
                3489294817472397097,
                14857786097152718166,
                14640759213920901022,
                18376413739029455402,
                3046104705582903606,
                4936164893340648792,
                17778749702375626160,
                6995355174979595633,
                959456782792320370,
                10861185425451318556,
                7366582450340175304,
                11477705165591734290,
                3136743279244378325,
                2918808829521989577,
                13025386837651124063,
                2739331147012733189,
                15903983887928564598,
                11228163341105914529,
                12519719017236799647,
                218708686305671363,
                6307552614212610835,
                13238239811550384043,
                7171884571288751686,
                4586382777078038841,
                6325243916544195815,
                13616538686389811874,
                1444630766318400262,
                14467636247804844292,
            ],
            [
                5765965407730767477,
                13000796820593365081,
                10447353012008689492,
                17881657816780260899,
                18425977481508045639,
                18086287446240530822,
                11106689518366871193,
                12367945271758709426,
                9454237647276555413,
                7652410063349223906,
                2886633513760218076,
                11404059536174006203,
                17537369691971997820,
                2007070408068658691,
                6282668616304967646,
                3911072061850706440,
                13354233096755795521,
                15145209314210089342,
                6540279062252542439,
                7626628840213638163,
                18031558209681080552,
                15849986042211505588,
                2512773290360964607,
                12636577362725692038,
                13136202369294473999,
                7410639326963063373,
                12923368031134628752,
                11362794112794802338,
                9036153040252339876,
                17940818382242885068,
                16708485849945843147,
                86232906601378292,
                17775164011598072842,
                8667134452053012705,
                10454073968380525410,
                11647201961371308213,
                5125227118315651492,
                17821402862892242018,
                12492679947644069998,
                2193752859578825987,
                5207787906334773233,
                8030647428241739109,
                14623013423662393323,
                14182863550419301122,
                4758721897981647066,
                15953582832501574455,
                1775911664148786530,
                1285068408497100160,
                170816030284369967,
                11355269990886899672,
                13371785704570160305,
                4945019156001474740,
                6806074563804013549,
                9978967418305465890,
                16661057409795188625,
                8516208067136851701,
                13615386324279387394,
                8330500116078852977,
                11070677263400067433,
                16597872522475283710,
                6644052692133022419,
                9225916559022926662,
                10802501068881794971,
                13083269339112707982,
            ],
            [
                16556618046911552097,
                6699277736609481173,
                255998892947048692,
                15282059606684839989,
                871130425700908777,
                15092820786912891623,
                5294005526868283990,
                15594950535189393004,
                10677966215043445364,
                4792061299548822337,
                14415315640882124868,
                4741233543491986508,
                700504744769652247,
                13993691195201082678,
                4037719315534722838,
                2976359674463512005,
                11127358764282010900,
                5014568771541919133,
                10655927266064293502,
                15321621793263634104,
                1479770730067235007,
                5485744985652980879,
                11153942624210312607,
                14003850476544598751,
                13095381385858054215,
                3251794379699576655,
                8735730238448364848,
                3270434158107220218,
                14393303187287576404,
                13361778679094443971,
                10901168820783989428,
                9608646675829307627,
                349825150829971524,
                9487914566992690371,
                13854401744985839100,
                4200333515498394008,
                8983824066456022556,
                1507100217000782715,
                284752345007561758,
                14609598502803525505,
                10049278281621816028,
                9477839625314333673,
                1035424730274067799,
                669040097493621464,
                8073404841552897547,
                5738878881256939835,
                13469065257984904831,
                9035017501682502674,
                3909081041234703067,
                17549045293055734164,
                5253493205450980070,
                7421866557987897173,
                12677565852190658735,
                4446838891902019472,
                17791362045496289167,
                680247909208122052,
                8344310511762544567,
                6458541056218692087,
                1377840696971470705,
                10732950663339433175,
                22059957411888582,
                2417445602276994964,
                9784297071089440780,
                9032834351028057901,
            ],
            [
                610769765166370294,
                911897807697497629,
                16409820449803109110,
                17347406936562828904,
                758688081070388038,
                1052837353902608240,
                13732579238665386417,
                12584323671000977412,
                16280601852783414782,
                5430357067614062071,
                4319263478992847452,
                2472232985850744810,
                9997640126590306242,
                7357300040735880165,
                1983795526786410940,
                10897318142324907449,
                5617151536604588851,
                16345375869914853877,
                15613436025625777794,
                17071670741184314385,
                11344507909402671740,
                18382636477315401816,
                1806973606811541211,
                13620601958347827629,
                1334455495458074800,
                3986012468943582855,
                2468736569540485184,
                10470537558898989886,
                7793160721087630185,
                12446907375091913974,
                5220555343702630831,
                2460840881247518229,
                637853500893263807,
                4354294508428743909,
                3427219531310261864,
                1606535114128417730,
                2854205432469701912,
                8949530655050424951,
                12294830997022577646,
                16869008521541704644,
                10905916248057782094,
                1563793198800499734,
                10360885366428312736,
                15590027525933599062,
                5350378395454419594,
                1887148566247045410,
                14691302503989876813,
                4171802151700918079,
                16147649705069251620,
                14450495969503877727,
                8230036778917891865,
                6516053195974555870,
                320726819768297399,
                5244744479026372051,
                10813138081422588052,
                11899768630131244216,
                3355739531639819634,
                6563918689964024802,
                16444864242554109489,
                4437330000082608589,
                17949435503313576605,
                9868701814377964599,
                14807912241840095024,
                13935646870434242618,
            ],
            [
                9854215897367548701,
                9730750084943870341,
                13563168049208661547,
                5560804492063642519,
                9555805194777460792,
                16401648673792392340,
                703555595912463453,
                5013248599503041098,
                14587683169739052211,
                1138429726473671743,
                17550503356031073827,
                17820836617812444166,
                17850185867674467343,
                535276514515651738,
                7988894939758009741,
                12674191634731600523,
                9378522161052928790,
                18233564236723558816,
                3759558785891495248,
                17837710219961764489,
                3345590028697756767,
                16807653550263418906,
                7264791510193382680,
                4145860131475504772,
                12529068209295928379,
                11191882192633247379,
                14905843462512449042,
                17198184818291758172,
                11267267253954178179,
                13290983658511344400,
                15977670628233725218,
                3643478302130914362,
                4191310551569899334,
                4042050289377133712,
                9816605650378904660,
                10482866740130732381,
                17117288883892014231,
                16654278731479236660,
                6972779203184466602,
                17805936631252299599,
                1380776387998607022,
                17070219870043821661,
                6755493621983846785,
                15045939540430074447,
                6011663507753713329,
                8568335817009799311,
                18130778147101169474,
                16230885531813615907,
                8839611371380663869,
                1036690927795210728,
                13682457606322055933,
                14629381242342137216,
                9774742575309004507,
                8788389494826019318,
                14767818946205086529,
                13011717640350092941,
                6023633933754301561,
                9047857088623561687,
                14326185199337279644,
                10663241965194466956,
                6273127661719963871,
                1874132507220746711,
                14072679170816472410,
                5444825199871924075,
            ],
            [
                10505877806911894777,
                5060604126992073657,
                3732066736048002002,
                12720852624178863208,
                8921044399104107074,
                4688893831742648554,
                16540157436971142465,
                13753493543479791999,
                10823151794829149383,
                12499394239547024437,
                3666788043033837809,
                10252152034661597474,
                11198221278298351650,
                18071789331193753772,
                12997727582829399020,
                13719860882926823519,
                1135128634276244181,
                7771189181488785336,
                16001415109839438971,
                9586191687250666954,
                11303432131457541425,
                14296677925267109051,
                10456211953045485533,
                2204868839622993864,
                12006368005773512785,
                14578557085804327607,
                15090473469810753488,
                7804366887409283385,
                3567901308687579459,
                8985241945358381090,
                428935145468218533,
                8222059835093914237,
                18093839825887632084,
                12749985057561077381,
                1440178873962420794,
                154749612606030705,
                15478737147817032969,
                7215892369180172326,
                13690442616650845058,
                17946248720232506820,
                15503499836447108473,
                7326699563879258530,
                12158009128859093887,
                2699220349416148416,
                3157540492250447478,
                3603799398817911763,
                6972081646291616227,
                4114338914753357849,
                8205995889141947321,
                10705961071736858364,
                16130106330602609393,
                11102068051562072043,
                11217125090765354669,
                3355459101274580384,
                9675293880713979264,
                5777936856290797742,
                4551735356497097762,
                9995256687704507712,
                1744882110308833573,
                8863372264095627866,
                17220494430743143884,
                8966139631531512324,
                15010071736135502605,
                5882727606212751499,
            ],
            [
                14766678818513736311,
                8975383468811760637,
                8424897044823684694,
                83465627559838322,
                3337234196307878001,
                5656201487082842546,
                18136600616275156018,
                3195999085004408067,
                12008595655103434335,
                9374311273078960839,
                7266714652970033632,
                7565339083075978600,
                3140302511825372162,
                11941086966290790217,
                10784293955858593504,
                1842659292099546805,
                15433960773185579501,
                5517472415343469629,
                2141230538968447464,
                6712404984910303553,
                3253583592147533239,
                17346715360870942549,
                675293873209141534,
                10037678995741425226,
                11603872569863444489,
                2780512010396793222,
                616321702543788627,
                3200004002964160951,
                2822382662919180652,
                18171626285817968342,
                7190403688468539014,
                4385565233615230029,
                14425705223731678787,
                17490528735748597883,
                10359193296737564004,
                15332436202601719959,
                67912168723568244,
                17839681614144414443,
                2039970154001099977,
                2531725248315336128,
                8084066424274634931,
                9566966471709322668,
                15793057015441693300,
                1396348135741692815,
                3089789394088330481,
                5319109881551772893,
                1556795355439484740,
                5075677583131755664,
                6230107078959817454,
                10972085576509470942,
                10303631023497417943,
                18172887562684489462,
                1027926510365452218,
                2834677643851841867,
                14850418388709604338,
                8416135949381891912,
                7739012458624887439,
                10224056299114428260,
                11272289930409034115,
                5162883763825282494,
                16410899814762243513,
                11903075906701677629,
                16925682247967468120,
                1379379649729065590,
            ],
            [
                10658459256838047535,
                13795197068821932038,
                17940607277208736984,
                17406608926246373990,
                1683813603175168160,
                13273775920587338246,
                14186098186263490173,
                10223606346973533437,
                14132864853306480860,
                7201097137407490531,
                10086698354222198448,
                13308912316091799163,
                1325247613439760756,
                17880237415217499098,
                6780400620545754416,
                7050126362487698418,
                17321604913941501461,
                1238581793313943508,
                5592549349379608850,
                15240165289703070046,
                5397452064867396341,
                1502730661367769302,
                15291799223974306220,
                12923034583072326062,
                11044098699489723792,
                12516580981140052579,
                18386084863181473398,
                947533906898736275,
                14677237551830931359,
                4831446996704768864,
                14881021294267000025,
                6735123879410240713,
                1580307043173893335,
                15472943952222913734,
                14089378866827817258,
                16996575484700647288,
                14811343233491129550,
                11258168272974457364,
                283154228031381396,
                7562847123943739539,
                15199615737801923804,
                10913474063694993803,
                15117748570316879138,
                1876714235795589871,
                3344908620705566961,
                7977925083788380908,
                14951378818810454162,
                1676315283082908263,
                7197150893984759193,
                14617564134871380889,
                14934989701002710847,
                7590303306622301965,
                5741765539139756444,
                11692780609686838225,
                11015853075088867531,
                7173418946752234234,
                12774871362346456523,
                12915110512571969640,
                4466793848759179948,
                11313910529930121991,
                16875432755871313754,
                11987585400457881827,
                18440446168122346509,
                17743455258838532012,
            ],
            [
                5090445453578045758,
                792991733825602563,
                4441827158345888171,
                1809356097716120450,
                8679459921799739019,
                8811818661843118238,
                7946336275535304169,
                11592732803330949223,
                212624318637012242,
                2956825961395882215,
                8704185405594608934,
                10386600195635506799,
                11813670775349976223,
                15953161110290297823,
                5224940036467181869,
                10701590859781177062,
                16494225587906998939,
                8976109082397877816,
                2292529621964884724,
                11592711549756790773,
                9422296132141173506,
                320888398661956763,
                2344418060556387280,
                10389725258525710202,
                2961446164702641662,
                15896910165256298901,
                11143580705873078308,
                11613071851158038820,
                12198009920568399059,
                10206059987901693999,
                8944897994272370095,
                6718137275235859114,
                6296112275406009341,
                11027400246388601281,
                17701629409467187662,
                16302561957340317252,
                3117344335752507907,
                3422475828031858703,
                8494274540119760574,
                6329820106805439958,
                1859026830496299165,
                7152717919328599930,
                3160903418541886263,
                15077002370590814520,
                11182051970484759047,
                9672036227060373259,
                16366985342387384195,
                12319774006385919698,
                11030055349700315885,
                14717313009360298253,
                6193522391246250622,
                7993730839482260024,
                15947676712979226289,
                7112665590775422399,
                5574945034297335826,
                11862887102660404400,
                400676566917768951,
                9740307244436159519,
                13333721659245504795,
                14247218460501659437,
                12534114101082256941,
                17848383302653107193,
                4204558938907580872,
                12858727855432099450,
            ],
            [
                7626622640698942570,
                2159318212986799628,
                15836081011380512017,
                12126258116344289891,
                6159154678111084440,
                13293988907659766869,
                6166185380814685400,
                9887698028137810317,
                1621141050968566987,
                17800583284451458181,
                6602698996587814096,
                11834110722110321825,
                12911814717690833015,
                18225724665850306863,
                5773682227018782086,
                8224607132046031913,
                3853986058788125258,
                3868240855614291142,
                4227491103382636811,
                17331145363124025902,
                5840430040232795567,
                15029314592560665749,
                18145872295419024997,
                13488509450257185730,
                8680792993716407361,
                5462588501870997230,
                14457465577948979503,
                17086186519600183215,
                8299810844703090235,
                10209466361021314273,
                2147045563167260169,
                1082197907945680999,
                6850066816948487409,
                2005971652740825353,
                17200170552524848562,
                11824491336299280497,
                15682616618686180499,
                9048179079825984867,
                4592122548706610608,
                13638387780092703889,
                7215864317111281858,
                9322552397564197381,
                13197823813402759103,
                8504182214980835938,
                6253641910314903179,
                18306040429031572959,
                2315873532425319064,
                1836375608925421233,
                12895028656505050762,
                11626212312099621200,
                5642142504081892974,
                10611164464987067228,
                4660695642793026699,
                18121798720096319479,
                14387386988534364386,
                17490405302134190047,
                8778785979387803719,
                632888537827188116,
                13695870693928325369,
                5266008515899561752,
                18165334623730724210,
                12776806129316457570,
                12775492848798949999,
                10209634173844477401,
            ],
            [
                14441869474958855008,
                11190541526566992975,
                14761386288254762247,
                1049150981230032472,
                9828903187558478580,
                11626974921239442348,
                4464316942166330229,
                4362875584345123705,
                16346280314033724924,
                9695462998995147488,
                7024050752010390666,
                3715742805389751263,
                4323890414709832851,
                16304284298878928596,
                13315934340085186285,
                14936867190702221573,
                18237657476330610615,
                6476929890853315628,
                2126280272874814581,
                13240804803274667553,
                5659514628274349256,
                634399726606181825,
                14977496353674232411,
                344840795396116004,
                4824163463622293440,
                14880947355737558225,
                11181638602043364548,
                11118440814871985965,
                11826900833892521718,
                1596232466077638703,
                17996463970087151395,
                13422853568906292453,
                15474845427443740353,
                8128949732237424810,
                5349738506627811540,
                5333237445900005674,
                1809984124897330744,
                14945949356231293277,
                16950335996376950431,
                5347902625225082901,
                2941428093674281025,
                10155360963063131573,
                15167246183265019492,
                6299474100614756061,
                12544960712564738861,
                5360881431705147612,
                11707255300293339212,
                15818320364957441405,
                3870281825202351305,
                89973236677421680,
                10615841144432800928,
                598058586043593916,
                8475774910394549920,
                618397829430972633,
                10349770043551389071,
                1955011437684855167,
                1793560278272706038,
                2013675218766411629,
                5601222960406405948,
                18043253684789378793,
                14044980152172264626,
                6492822312477975272,
                4032127513910289851,
                7283315899762671379,
            ],
        ],
        castling_rights: [
            6571235847249673943,
            1407692895529808991,
            8017416733058094342,
            17174161043994964909,
            14010261613684750750,
            6749265715182960658,
            17201436012504997405,
            7692195806829441891,
            618922600466448659,
            9977899019142877944,
            6584548094983903006,
            11901552865359661386,
            13384647352087957342,
            6655747569867824196,
            13705265583141900721,
            761158951579815276,
        ],
        en_passant_target: [
            81219621786405762,
            17034661209676531226,
            10220277138238756656,
            12084541879564429561,
            9813363965369853949,
            18037792549846369496,
            13233332059395281206,
            7008673347836694607,
        ],
    };

    pub fn hash(
        &self,
        piece_boards: &[Board; 12],
        side_to_move: &Side,
        castling_rights: &CastlingRights,
        en_passant_target: &Option<Square>,
    ) -> u64 {
        let side_to_mode = if *side_to_move == Side::WHITE {
            0
        } else {
            self.side_to_move
        };
        let castling_rights = self.castling_rights[castling_rights.to_usize()];
        let en_passant_target =
            if let Some(file) = en_passant_target.as_ref().map(|square| square.file_index()) {
                self.en_passant_target[file]
            } else {
                0
            };

        let mut hash = side_to_mode ^ castling_rights ^ en_passant_target;

        for (piece, board) in piece_boards.iter().enumerate() {
            for (_, square) in board.iter() {
                hash ^= self.piece_boards[piece][square.to_usize()]
            }
        }

        hash
    }
}
