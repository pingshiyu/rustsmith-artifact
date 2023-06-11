#![allow(warnings, unused, unconditional_panic)]
use std::env;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
const CONST1: i32 = -266582746i32;
const CONST2: u16 = 45528u16;
const CONST3: f32 = 0.0040058494f32;
const CONST4: i8 = 78i8;
const CONST5: i16 = 29419i16;
const CONST6: f32 = 0.42273867f32;
const CONST7: u128 = 64195594437284793612510038890125745280u128;
const CONST8: u64 = 2655251870960170043u64;
macro_rules! reconditioned_div{
    ($a:expr,$b:expr, $zero: expr) => {
        {
            let denominator = $b;
            if (denominator != $zero) {($a / denominator)} else {$zero}
        }
    }
}
macro_rules! reconditioned_access{
    ($a:expr,$b:expr) => {{
        let arrLength = $a.len();
        let index = $b;
        $a[if (index < arrLength) { index } else { 0 }]
    }};
}
macro_rules! reconditioned_mod{
    ($a:expr,$b:expr, $zero: expr) => {
        {
            let denominator = $b;
            if (denominator != $zero) {($a % denominator)} else {$zero}
        }
    }
}
#[derive(Debug)]
struct Struct1 {
var1: i16,
var2: Box<u32>,
var3: i64,
var4: i32,
}

impl Struct1 {
 #[inline(never)]
fn fun34(&self, hasher: &mut DefaultHasher) -> u16 {
let mut var919: f32 = 0.62428874f32;
var919 = (0.50143915f32);
13295605437436694280usize;
var919 = 0.694732f32;
String::from("3Cu3UrdoFno7nv45RbhL9t8Lkp5rp9E1Y2j7useMFHfVRRd6akcpmXZdxKjXuBKxV");
true;
let var922: Vec<i16> = vec![1556i16,26307i16,297i16];
1268789357u32;
return 65352u16;
7220u16
}
 
}
#[derive(Debug)]
struct Struct2<'a2> {
var5: &'a2 u16,
var6: Box<i16>,
}

impl<'a2> Struct2<'a2> {
 
fn fun3(&self, var33: f32, var34: u32, var35: bool, hasher: &mut DefaultHasher) -> Box<u32> {
format!("{:?}", var35).hash(hasher);
let mut var36: i128 = 105965048152359495086369184288160756488i128;
let mut var37: i128 = 164508840400142805427093918079544399806i128;
vec![119613262387088925235039776191958147218i128,105794373984434997952251997307102676260i128,var36,var37].push(85017461705310566596192785097885402271i128);
format!("{:?}", var34).hash(hasher);
let var38: f32 = 0.14568812f32;
var38;
let var39: i128 = 48693753298225530029170346166082211048i128;
var37 = var39;
var37 = 131907680665481811691990155250967670169i128;
var37 = 151773868481334722796312112264724164146i128;
let var40: f32 = 0.62040126f32;
var40;
let var41: i32 = -61075839i32;
format!("{:?}", self).hash(hasher);
format!("{:?}", var41).hash(hasher);
var36 = 100393430720413229682195447202998036908i128;
var36 = var39;
format!("{:?}", var34).hash(hasher);
format!("{:?}", var41).hash(hasher);
let var42: Box<u32> = Box::new(37175875u32);
return var42;
let var43: u32 = 165111310u32;
Box::new(var43)
}

#[inline(never)]
fn fun12(&self, var219: i64, var220: u32, hasher: &mut DefaultHasher) -> i16 {
return 27499i16;
10925i16
}
 
}
#[derive(Debug)]
struct Struct3 {
var7: f32,
}

impl Struct3 {
  
}
#[derive(Debug)]
struct Struct4 {
var48: u16,
var49: usize,
var50: Struct3<>,
}

impl Struct4 {
 
fn fun16(&self, hasher: &mut DefaultHasher) -> (bool,Struct1) {
let var256: Option<f32> = None::<f32>;
format!("{:?}", self).hash(hasher);
6053449850752562112u64;
let var257: i8 = 62i8;
format!("{:?}", var256).hash(hasher);
let mut var258: u16 = 26695u16;
var258 = 65312u16;
format!("{:?}", self).hash(hasher);
var258 = 15096u16;
Some::<i16>(14658i16);
1838202608567025618u64;
Box::new(25692i16);
true;
12898i16;
format!("{:?}", self).hash(hasher);
format!("{:?}", var257).hash(hasher);
Box::new(-1462054807i32);
String::from("4aeC6sthifevjSHO1yfLIL6YPsSrfX6lX6jufD1lggLmhErUGTa5BWOTBnDZC4S1jFrCfb5FCGTouW71GbrMqs42dVyjp5Z1");
var258 = 60356u16;
var258 = 53437u16;
var258 = 12961u16;
(true,Struct1 {var1: 3137i16, var2: Box::new(3270143991u32), var3: -4245122337816415099i64, var4: -2050092191i32,})
}
 
}
#[derive(Debug)]
struct Struct5 {
var91: u16,
}

impl Struct5 {
  
}
#[derive(Debug)]
struct Struct6 {
var233: f64,
}

impl Struct6 {
 
fn fun14(&self, var234: &mut usize, var235: u32, var236: String, var237: i16, hasher: &mut DefaultHasher) -> Struct1 {
(*var234) = vec![14141111455743991101u64,4658368517783466471u64,15499265218169908785u64,7194388885390662172u64,8803962363167867557u64].len();
let mut var238: u64 = 8984029918600967609u64;
format!("{:?}", var237).hash(hasher);
let mut var239: i128 = 112463980733805674657636855726434238545i128;
true;
var238 = 4771001243508489791u64;
var239 = 15515554250773837224900918438179985281i128;
Some::<String>(String::from("EZIlG88GlmiJOvVzVqiYHW2yXZBNLfu21hihRXmAd9xGj111fIN7SAXEti39Q2v"));
let mut var240: bool = true;
fun2(false,1879979441i32,107u8,232u8,hasher);
var239 = 161310185853441796290716586064697909289i128;
let mut var241: bool = true;
1301496320i32;
let mut var242: i8 = 37i8;
8152404990462228600usize;
format!("{:?}", var242).hash(hasher);
var241 = true;
Struct1 {var1: 8600i16, var2: Box::new(1772259266u32), var3: -7564710874106126468i64, var4: 849294246i32,}
}


fn fun18(&self, var291: Struct3, hasher: &mut DefaultHasher) -> u64 {
let var294: f64 = 0.3769450340674503f64;
let mut var295: u16 = 46530u16;
&mut (var295);
format!("{:?}", var291).hash(hasher);
let mut var296: u64 = 3995089474359232469u64;
let mut var297: u64 = 16141024130014012733u64;
let mut var298: u64 = 14379012120694796784u64;
vec![var296,var297,var298,6020553415879254179u64,11837074513678181938u64].push(5281329214708061861u64);
68u8;
var296 = CONST8;
let var299: u8 = 16u8;
var299;
let var300: Option<u64> = None::<u64>;
var300;
let var302: u128 = 93026470190854645914033028057007282757u128;
let var301: u128 = var302;
let var303: i8 = 20i8;
var303;
let var304: u8 = 77u8;
38i8;
749704078475792040usize;
var297 = 1548258208843181209u64;
72i8;
format!("{:?}", self).hash(hasher);
return 16874467855101195031u64;
let var305: u64 = 6551457714695901788u64;
var305
}
 
}
#[derive(Debug)]
struct Struct7 {
var352: Box<i16>,
}

impl Struct7 {
  
}
#[derive(Debug)]
struct Struct8 {
var391: f32,
}

impl Struct8 {
  
}
#[derive(Debug)]
struct Struct9 {
var589: i32,
var590: usize,
}

impl Struct9 {
 #[inline(never)]
fn fun30(&self, var591: &i8, var592: i8, var593: Type2, hasher: &mut DefaultHasher) -> (i8,f64) {
format!("{:?}", var591).hash(hasher);
let var594: bool = false;
let mut var595: i128 = 4822229801568239536370071265235678085i128;
var595 = 54407240986195092889465467285276441563i128;
return (99i8,0.8763813836482099f64);
(92i8,0.7316167340019115f64)
}

#[inline(never)]
fn fun36(&self, var968: String, var969: u32, var970: Option<(i8,f64)>, hasher: &mut DefaultHasher) -> f64 {
663068601659476828u64;
let mut var972: u8 = 223u8;
var972 = 97u8;
3765i16;
61867u16;
vec![Box::new(168447506795625175269323393593275104794i128),Box::new(24229263593878486736486641440133673274i128),Box::new({
format!("{:?}", var968).hash(hasher);
var972 = 236u8;
Box::new(0.4855435827953647f64);
false;
let mut var973: (i8,f64) = (7i8,0.754597329941143f64);
let mut var974: usize = vec![-9046837602353874208i64].len();
var973.1 = 0.8547939599483687f64;
Box::new(true);
format!("{:?}", self).hash(hasher);
Struct11 {var975: vec![Struct1 {var1: 9568i16, var2: Box::new(1062701149u32), var3: -3140266169361909620i64, var4: 1205108019i32,},match (None::<f64>) {
None => {
195u8;
144350204393108544653546654765553069613u128;
format!("{:?}", var973).hash(hasher);
let mut var984: i128 = 23425068766008369812818489547036770210i128;
var973 = (2i8,0.924649140666199f64);
var972 = 72u8;
let mut var985: i64 = -6025554287169201961i64;
var973 = (104i8,0.5628703401274695f64);
format!("{:?}", self).hash(hasher);
let var986: u128 = 61290459008821442887685347646543310907u128;
var984 = 152256990671163479501170961574868266893i128;
2353i16;
let var987: f32 = 0.93436944f32;
format!("{:?}", var969).hash(hasher);
format!("{:?}", var969).hash(hasher);
var973.0 = 18i8;
15648i16;
135u8;
3619552097u32;
12621568204125935231usize;
let var988: Box<bool> = Box::new(true);
103u8;
Struct1 {var1: 1678i16, var2: Box::new(1053824599u32), var3: 7406218402016990241i64, var4: 1123151537i32,}},
 Some(var982) => {
var972 = 108u8;
let mut var983: Struct3 = Struct3 {var7: 0.5109972f32,};
return 0.544855047520335f64;
Struct1 {var1: 17279i16, var2: Box::new(2679437144u32), var3: -643088553365648182i64, var4: -280107662i32,}
}
}
,Struct1 {var1: 6164i16, var2: Box::new(1047025110u32), var3: -1029405862682242852i64, var4: -6577461i32,},Struct1 {var1: 14106i16, var2: Box::new(465274075u32), var3: 489941124515278589i64, var4: 1128008601i32,},fun38(3809790404102989096usize,83496630094580822423518206456481794717u128,hasher),Struct1 {var1: 13327i16, var2: Box::new(185928155u32), var3: -5654288322105662473i64, var4: -1369645938i32,},Struct1 {var1: 25640i16, var2: Box::new(994456142u32), var3: -2657985510706728310i64, var4: 246193549i32,},Struct1 {var1: 16324i16, var2: Box::new(953413565u32), var3: 5219827746183113807i64, var4: 746554522i32,},Struct1 {var1: 297i16, var2: Box::new(3020195079u32), var3: -3490760713194297642i64, var4: 1307401891i32,}],}.fun37(7394622396446141950u64,842120900i32,(true,Struct1 {var1: 14512i16, var2: Box::new(1215011019u32), var3: -6061520995272647673i64, var4: 2022478579i32,}),28236i16,hasher).push((None::<i16>,28598i16,17171i16));
return 0.015155886670061403f64;
120694661821695438912621384143267287358i128
}),Box::new(156545960463609002816194912453432096034i128),Box::new(121003630232753419806875058949547162036i128),Box::new(126445227761112041545684671113127423331i128),Box::new(167943142842212943332044380891777383133i128),Box::new(134530537862178751736593666560339959945i128)].len();
37360u16;
format!("{:?}", self).hash(hasher);
format!("{:?}", var969).hash(hasher);
var972 = 113u8;
0.6356815f32;
144428335124317478354780697770331337340u128;
None::<Vec<f64>>;
format!("{:?}", var972).hash(hasher);
let var1002: usize = 1091971126590117446usize;
var972 = 219u8;
var972 = 10u8;
var972 = 242u8;
format!("{:?}", var969).hash(hasher);
0.9892230416008546f64
}
 
}
#[derive(Debug)]
struct Struct10 {
var763: Box<u16>,
var764: u32,
var765: bool,
}

impl Struct10 {
  
}
#[derive(Debug)]
struct Struct11 {
var975: Vec<Struct1<>>,
}

impl Struct11 {
 #[inline(never)]
fn fun37(&self, var976: u64, var977: i32, var978: (bool,Struct1), var979: i16, hasher: &mut DefaultHasher) -> Vec<(Option<i16>,i16,i16)> {
format!("{:?}", var977).hash(hasher);
let mut var980: u8 = 81u8;
var980 = 163u8;
var980 = 236u8;
var980 = 112u8;
0.16417342f32;
1660499850i32;
false;
(0.29985857f32,0.4677412352551005f64,Box::new(1611262072i32));
5203492452942570565u64;
var980 = 124u8;
let mut var981: Option<i8> = None::<i8>;
return vec![(None::<i16>,26486i16,11622i16),(Some::<i16>(19787i16),6552i16,20229i16),(None::<i16>,8330i16,26192i16),(None::<i16>,22593i16,13133i16),(Some::<i16>(31821i16),17438i16,12386i16),(Some::<i16>(16422i16),26734i16,6612i16),(Some::<i16>(32173i16),20580i16,15355i16),(Some::<i16>(22662i16),28416i16,3670i16),(Some::<i16>(24551i16),22628i16,15291i16)];
vec![(Some::<i16>(13650i16),30624i16,10116i16),(None::<i16>,16101i16,8969i16),(None::<i16>,18561i16,1603i16),(Some::<i16>(28851i16),13101i16,9031i16),(Some::<i16>(27711i16),24429i16,26523i16),(Some::<i16>(29048i16),27891i16,4170i16),(None::<i16>,15200i16,15530i16),(None::<i16>,7351i16,22197i16)]
}
 
}
type Type1 = Option<u64>;
type Type2 = f32;
type Type3 = u128;
#[inline(never)]
fn fun2( var16: bool, var17: i32, var18: u8, var19: u8, hasher: &mut DefaultHasher) -> i8 {
let var20: bool = false;
let var21: Box<u32> = Box::new(3461499307u32);
let var22: i32 = 516148336i32;
(var20,Struct1 {var1: 2539i16, var2: var21, var3: -7758710182641630403i64, var4: var22,});
let var24: i32 = 381409005i32;
Box::new(&(var24));
let var26: u8 = 19u8;
let var25: u8 = var26;
let var27: f32 = 0.80089605f32;
var27;
let mut var28: u128 = 43108619382349728538300654585112264390u128;
let var29: u128 = 126999774865123256391899297719401996400u128;
var28 = var29;
let mut var30: u64 = match (Some::<u128>(102744293693942500745793291515627234872u128)) {
None => {
0.7550062981640708f64;
let mut var46: u16 = 17911u16;
let var47: u16 = 17508u16.wrapping_add(49688u16);
var47;
return 61i8;
12433419945916644956u64},
 Some(var31) => {
return (111i8 | 87i8);
16003279268470927340u64
}
}
;
var30 = 14393689051030210602u64;
let var52: Struct4 = Struct4 {var48: 12122u16, var49: 14706789006029987286usize, var50: Struct3 {var7: 0.9353558f32,},};
let mut var51: Struct4 = var52;
let var53: Vec<Option<Option<u16>>> = vec![Some::<Option<u16>>(None::<u16>)];
var53.len();
var30 = CONST8;
let var54: bool = true;
var54;
var28 = 76733436206110805911280251678387649842u128;
let var55: usize = vec![68677104795825257404598543676364372108i128].len();
var55;
let var56: u64 = 2382203739105662932u64;
var56;
let var57: u8 = 15u8;
&(var57);
0.6922890065856164f64;
format!("{:?}", var28).hash(hasher);
let var58: i8 = 66i8;
var58
}


fn fun4( hasher: &mut DefaultHasher) -> Box<i32> {
let mut var86: bool = false;
format!("{:?}", var86).hash(hasher);
let mut var87: u16 = 24919u16;
var87 = CONST2;
format!("{:?}", var86).hash(hasher);
let var88: Box<i32> = Box::new(-1132307854i32);
return var88;
Box::new(1332026052i32)
}

#[inline(never)]
fn fun5( var111: i64, hasher: &mut DefaultHasher) -> i128 {
let var113: i32 = 1106681664i32;
let var112: i32 = var113;
let var117: i128 = 103224535227618111281155437741536334386i128;
let var116: i128 = var117;
();
let var120: f32 = 0.5867411f32;
let mut var119: f32 = var120;
let var121: f32 = 0.87812734f32;
var119 = var121;
var119 = 0.74522126f32;
let var124: f64 = 0.13772591326773642f64;
(113i8,var124);
let var125: i128 = 46979864893269609559573583914212962453i128;
var125;
var119 = 0.72009885f32;
let var126: f32 = 0.63311386f32;
&(var126);
0.6677399853312795f64;
format!("{:?}", var125).hash(hasher);
let var127: usize = 10372697105607044544usize;
var127;
let var128: Box<i128> = Box::new(40788860594627558405089595034678826311i128);
vec![var128];
();
format!("{:?}", var127).hash(hasher);
let var129: i128 = 90520644226772964442649759407295088144i128;
var129
}

#[inline(never)]
fn fun6( var131: f32, var132: u8, hasher: &mut DefaultHasher) -> i16 {
let var133: i8 = 4i8;
let mut var134: i8 = 101i8;
var134 = 38i8;
20781i16;
format!("{:?}", var133).hash(hasher);
String::from("UKlyn1tTvfjSnJycRSf79qLbZnDKzhD5kCYUCL4R6B");
24537i16;
format!("{:?}", var133).hash(hasher);
var134 = 103i8;
6919257876156232877u64;
var134 = 127i8;
format!("{:?}", var134).hash(hasher);
var134 = 30i8;
true;
var134 = 61i8;
format!("{:?}", var134).hash(hasher);
let var135: f64 = 0.7717461857045927f64;
format!("{:?}", var131).hash(hasher);
var134 = 39i8;
8134i16
}

#[inline(never)]
fn fun7( hasher: &mut DefaultHasher) -> Box<u32> {
let mut var136: String = String::from("biFd4tG4zEJ7aj0nRkQu0WCFVYdfKEVOFhAAc8Qmj7EVEyeDoT2B");
var136 = String::from("dFlmYozgZ7If2auvBHgexi");
29956368u32;
4075785520009668617i64;
format!("{:?}", var136).hash(hasher);
let var137: u128 = 30321323893214728849832374495072619290u128;
format!("{:?}", var137).hash(hasher);
10078574524688303259u64;
format!("{:?}", var137).hash(hasher);
let mut var138: String = String::from("QgIk6rDbxobVDnfpgrDgA4fW2LqfskHGfabpkpO3UgtuANYSsDlt9icU");
var138 = String::from("GJqmXBmnhl");
return Box::new(1003687715u32);
Box::new(559820013u32)
}

#[inline(never)]
fn fun8( var141: &i64, var142: (bool,Struct1), var143: Box<i16>, hasher: &mut DefaultHasher) -> Struct1 {
let mut var144: i32 = var142.1.var4;
let var145: bool = true;
var145;
format!("{:?}", var144).hash(hasher);
let var146: bool = true;
var146;
let var147: Struct1 = Struct1 {var1: 4307i16, var2: Box::new(2333776016u32), var3: -7888286320292462444i64, var4: 2002253024i32,};
return var147;
let var148: Struct1 = Struct1 {var1: 21841i16, var2: Box::new(2542542735u32), var3: -5279030242230314766i64, var4: -648514421i32,};
var148
}


fn fun9( var153: f32, var154: &mut i16, hasher: &mut DefaultHasher) -> Vec<Struct1> {
let mut var155: Struct5 = Struct5 {var91: 48729u16,};
let var156: i128 = 138814674121383108229405261507445648849i128;
vec![Box::new(111101980684818427910630671474784863153i128),Box::new(154670397613391814860082069948170671241i128),Box::new(88927318290498284006414138743543104898i128),Box::new(31207565995493579332934756258297932105i128),Box::new(16060730379733522848231178135759582344i128),Box::new(98196777122718847528180897235963887834i128),Box::new(52396628709572521741872934802760965394i128),Box::new(6168789791515784131791208827688676224i128)].push(Box::new(113197755949047901913791393784906351683i128));
73i8;
let var157: i8 = 67i8;
18084545529583277025u64;
format!("{:?}", var153).hash(hasher);
let var158: i16 = 3177i16;
12u8;
false;
format!("{:?}", var155).hash(hasher);
(*var154) = 11824i16;
160805683660847514501332467209697936526i128;
228u8;
let var159: (i8,f64) = (92i8,0.7105321132724169f64);
let mut var160: f32 = 0.45440364f32;
87762663902949635735175545644438375780u128;
41u8;
let var161: i8 = 97i8;
let var162: i128 = 63575091781533125548835530624095984354i128;
84u8;
Box::new(61211u16);
var160 = 0.87586653f32;
vec![Struct1 {var1: 27608i16, var2: Box::new(1343598930u32), var3: -3169269013208519550i64, var4: -143697060i32,},Struct1 {var1: 5872i16, var2: Box::new(1164366946u32), var3: -3513971741448912394i64, var4: 689602197i32,},Struct1 {var1: 18786i16, var2: Box::new(3488835781u32), var3: 317475568231145257i64, var4: 296177716i32,},Struct1 {var1: 442i16, var2: Box::new(1591647641u32), var3: -6070864078969223508i64, var4: -396899919i32,}]
}

#[inline(never)]
fn fun10( var169: i16, var170: Option<Vec<Box<i128>>>, var171: usize, var172: i16, hasher: &mut DefaultHasher) -> f64 {
1423775453779809612u64;
format!("{:?}", var169).hash(hasher);
format!("{:?}", var170).hash(hasher);
let var173: i32 = -782136604i32;
var173;
let var175: u16 = 47980u16;
let mut var174: u16 = var175;
let var176: u16 = 46436u16;
var174 = var176;
format!("{:?}", var174).hash(hasher);
let mut var177: i128 = 21008579590459322717874937026053926589i128;
let mut var178: Box<i128> = Box::new(65682960885325562981678374363673551311i128);
let mut var179: Box<i128> = Box::new(74084732828503426106876472535277463288i128);
let mut var180: Box<i128> = Box::new(96043779171308393236583360117166568632i128);
let var181: Box<i128> = Box::new(132204904272204911113062308192315344917i128);
vec![Box::new(var177),var178,var179,var180,Box::new(51573397758813418103251358538726291000i128)].push(var181);
let var183: i128 = 5547372471732515980334159669917124835i128;
let mut var182: i128 = var183;
let var184: String = String::from("F9mBGXUXS9UtOOPglXBOwTfO6I5OEAiw0UPF9FtQJ5S2uv1HwkP0rfE7YPbuG4vO0hyuP6ItNqpkIU2T");
&(var184);
format!("{:?}", var175).hash(hasher);
format!("{:?}", var175).hash(hasher);
let var185: Vec<i128> = vec![84151575198223606959482554885191527531i128,23024552176323152727825855696034773626i128,5563268485258478538679151874312374395i128,128910825312494037889070869440574915233i128,137265018450522337167830804550105385899i128];
var185;
var174 = 13326u16;
return 0.665136365973627f64;
0.43591054934458373f64
}


fn fun11( var206: i16, var207: u128, var208: i64, hasher: &mut DefaultHasher) -> u16 {
format!("{:?}", var206).hash(hasher);
let var209: i128 = 62645966197544336029144545040942096046i128;
Some::<i128>(var209);
1559889156u32;
let var210: i32 = 160892335i32;
format!("{:?}", var210).hash(hasher);
Box::new(57290u16);
false;
let var213: i128 = 4567149069288003995094199897985068188i128;
let var212: i128 = var213;
26600u16;
let var214: u128 = {
return 27848u16;
103050085651077507740537179928885745694u128
};
var214;
format!("{:?}", var214).hash(hasher);
let var215: u16 = 9681u16;
return var215;
let var216: u16 = 54847u16;
var216
}

#[inline(never)]
fn fun13( hasher: &mut DefaultHasher) -> i32 {
let mut var226: i32 = -1292563932i32;
var226 = 989654008i32;
var226 = {
671607811175599865i64;
let var227: f32 = 0.31123817f32;
(true,Struct1 {var1: 18388i16, var2: Box::new(3573475462u32), var3: -7996592609704527811i64, var4: 458561479i32,});
3862i16;
format!("{:?}", var227).hash(hasher);
();
format!("{:?}", var227).hash(hasher);
let mut var228: i8 = 8i8;
vec![Box::new(53627692188731067575048549878418892002i128),Box::new(27887684895234041335706195264475998328i128),Box::new(112347517153032694000412426039424720581i128),Box::new(144744914709314189333329667562675652259i128),Box::new(103671751972192731710926133974110640201i128)];
vec![Struct1 {var1: 5693i16, var2: Box::new(2133447606u32), var3: 8583381965751598926i64, var4: 784668973i32,},Struct1 {var1: 29406i16, var2: Box::new(4255773492u32), var3: -3358186589752136116i64, var4: 1203253525i32,}].len();
var228 = 71i8;
5735842618769418239637504821701828893i128;
false;
format!("{:?}", var227).hash(hasher);
format!("{:?}", var227).hash(hasher);
let var229: usize = vec![97949035944267727729947808305284762042i128,93848420609144511101985164348410463177i128,28662736348069954203614030035214574280i128].len();
98835643i32
};
();
var226 = -1540312542i32;
var226 = -1712256739i32;
false;
format!("{:?}", var226).hash(hasher);
var226 = 476496996i32;
let var230: i32 = 775586687i32;
Box::new(2233i16);
return -207600812i32;
860776609i32
}

#[inline(never)]
fn fun15( var246: i128, var247: String, hasher: &mut DefaultHasher) -> Vec<Box<i128>> {
();
let var249: f32 = 0.18789607f32;
let var248: f32 = var249;
let var250: i128 = 6544537401897339605552412049830737032i128;
let var266: Box<i128> = Box::new(29902222646856821288049617281944390923i128);
let var267: i128 = 19096844776373392093429882705785318044i128;
let var268: Box<i128> = Box::new(95376658448641045596689863765668332809i128);
let var269: Box<i128> = Box::new(12458897469598490644646560936956470705i128);
let var270: i128 = 21115328777467678619141063063316636617i128;
let var271: i128 = 76754056336186286803163745669914356025i128;
return vec![Box::new(var250),{
let var252: f64 = 0.30587809696281687f64;
let mut var251: (i8,f64) = ((22i8,var252));
let var253: (i8,f64) = (27i8,0.7622108645752826f64);
var251 = var253;
let var254: u32 = 3513233716u32;
(true,(Struct1 {var1: 11350i16, var2: Box::new(var254), var3: -191859889328876478i64, var4: 653339317i32,}));
let var255: Box<(bool,Struct1)> = Box::new(Struct4 {var48: 2847u16, var49: 13635657059011251428usize, var50: Struct3 {var7: reconditioned_div!(0.61417896f32, 0.067162275f32, 0.0f32),},}.fun16(hasher));
var255;
-1335686890i32;
4270522387814422054i64;
();
format!("{:?}", var246).hash(hasher);
let var259: Vec<Box<i128>> = vec![Box::new(151566791724456125155273915921662153055i128),Box::new(59183220813987817008824212048921252877i128),Box::new(128966067262863873413497673567657166783i128),match (None::<String>) {
None => {
let var264: (Option<i16>,i16,i16) = (Some::<i16>(13754i16),29941i16,13180i16);
var251.1 = 0.9887365144910109f64;
return vec![Box::new(62343074100288437481655845230075995364i128),Box::new(96887332384695051188217030099772835784i128),Box::new(101737727892963398992075879693901539648i128),Box::new(27912510977011368102403628048543496193i128),Box::new(136687154588627711743223027589724688435i128),Box::new(37598421739017334668538714376167989658i128),Box::new(42536338988406941206568686595232295917i128),Box::new(5266723747337824205347525088793273241i128)];
Box::new(127115636011239133018507734627027683815i128)},
 Some(var260) => {
0.49760774044801126f64;
vec![Box::new(145648863185534975649467292822385844519i128),Box::new(126597036436556251860038890194010613395i128),Box::new(44479843338878978570924821391134517369i128)];
format!("{:?}", var251).hash(hasher);
27u8;
();
format!("{:?}", var260).hash(hasher);
29656i16;
var251.0 = 84i8;
5339428501921072313u64;
17458925255179548372u64;
var251.1 = 0.8692828231190863f64;
0.06859511241821037f64;
let var261: f32 = 0.6113951f32;
format!("{:?}", var251).hash(hasher);
let var262: bool = false;
67584268i32;
let var263: u128 = 35004093113043309160990733591900217519u128;
Box::new(35742388645375112099766507350853570950i128)
}
}
,Box::new(114540305360351039219894237682669783743i128)];
return var259;
let var265: Box<i128> = Box::new(101584370140439902906853597160723979478i128);
var265
},var266,Box::new(var267),var268,var269,Box::new(var270),Box::new(33790915329141176044896497845436788190i128),Box::new(var271)];
let var272: Box<i128> = Box::new(65480291289306421438926178244022570768i128);
let var273: i128 = 39791192586801815518818768702085346844i128;
let var274: Box<i128> = Box::new(79050358119027124644155732952033812629i128);
let var275: Box<i128> = Box::new(92011211341942413284315567936853443988i128);
vec![var272,Box::new(53898897678892690034147871587555700663i128),Box::new(168264646930342950917011081368567056662i128),Box::new(var273),var274,var275,Box::new(57456899845483784175682575151327926425i128)]
}


fn fun1( var10: Option<usize>, var11: bool, hasher: &mut DefaultHasher) -> u128 {
let var61: i8 = 29i8;
let var60: i8 = var61;
let var59: i8 = var60;
let var64: i8 = 27i8;
let var66: i8 = 65i8;
let var65: i8 = var66;
let var63: i8 = var64.wrapping_mul((var65));
let var62: i8 = var63;
let var67: i32 = -1353418972i32;
let var15: i8 = fun2((var59 <= var62),var67,77u8,246u8,hasher);
let var14: i8 = var15;
let var13: i8 = var14;
let mut var12: i8 = var13;
format!("{:?}", var64).hash(hasher);
let mut var68: f64 = 0.6292591652454804f64;
let var74: i16 = 12730i16;
let var75: i16 = 13757i16;
let var73: (Option<i16>,i16,i16) = (Some::<i16>(16079i16),var74,var75);
let var72: (Option<i16>,i16,i16) = var73;
let var71: (Option<i16>,i16,i16) = var72;
let var70: (Option<i16>,i16,i16) = var71;
let var69: (Option<i16>,i16,i16) = var70;
var12 = var65;
2323800283u32;
let mut var244: Box<u32> = Box::new(3188890754u32);
let var245: Vec<Box<i128>> = fun15(71983848684346832368712482728578908187i128,String::from("olxUwf9ZI0uUy1eOgkF1kNpeDKTewwpq"),hasher);
var245;
let mut var276: i16 = 31633i16;
format!("{:?}", var65).hash(hasher);
format!("{:?}", var10).hash(hasher);
let var279: f32 = 0.3000087f32;
let var278: f32 = var279;
let var277: Struct3 = Struct3 {var7: var278,};
var277;
format!("{:?}", var67).hash(hasher);
let var280: u8 = 95u8;
let var281: u64 = 12092968493347354530u64;
var281;
let var282: u128 = 30444060224405945646346961681927437158u128;
var282
}

#[inline(never)]
fn fun19( hasher: &mut DefaultHasher) -> bool {
let mut var315: bool = false;
let var316: Vec<bool> = vec![false,(true & true),(14551778920039970841u64 > 1930132414047873284u64)];
let var317: usize = vec![-5809073910884374957i64,4288956354587806732i64,-1581090630279283357i64].len();
var315 = reconditioned_access!(var316, var317);
(70u8);
let mut var318: u32 = 960371507u32;
var318 = 2706309241u32;
let mut var319: f32 = 0.17661583f32;
format!("{:?}", var319).hash(hasher);
let var320: usize = vec![734641776761251220u64,2870094453008229075u64,13818020208612665792u64,2867480191716646111u64].len();
var320;
7122108678387932318i64;
return true;
let var321: bool = true;
var321
}

#[inline(never)]
fn fun20( hasher: &mut DefaultHasher) -> u64 {
let mut var340: i32 = 1006283055i32;
var340 = -207654600i32;
();
var340 = 2016667961i32;
var340 = -1090312114i32;
return 9104260182428777676u64;
10064842604767675142u64
}

#[inline(never)]
fn fun21( hasher: &mut DefaultHasher) -> i16 {
let mut var371: (Option<i16>,i16,i16) = (None::<i16>,172i16,19049i16);
var371.2 = 455i16;
44671218968599917213170154866600710421i128;
false;
1982924788i32;
var371 = (Some::<i16>(30287i16),4345i16,5961i16);
Box::new(3099877763u32);
0.9956341f32;
let mut var372: u8 = 95u8;
27808205196662065198364247384130570410u128;
let var373: u8 = 5u8;
format!("{:?}", var372).hash(hasher);
var371.1 = 26352i16;
None::<(u16,bool)>;
return 6416i16;
9688i16
}

#[inline(never)]
fn fun22( var390: (i8,f64), hasher: &mut DefaultHasher) -> Option<i16> {
let mut var392: Struct8 = Struct8 {var391: 0.09472811f32,};
let var393: Struct8 = Struct8 {var391: 0.28808534f32,};
var392 = var393;
let var397: Vec<u64> = vec![16955152006189543005u64,8542759027828712830u64,11988030978362492008u64,316656499560713855u64,9650911183223142449u64,4240567021221595355u64,12464598231009234251u64,14286697905752110891u64,11223496571371711474u64];
let mut var396: usize = var397.len();
var392.var391 = 0.53461725f32;
format!("{:?}", var390).hash(hasher);
var392.var391 = 0.8800414f32;
let var398: i64 = -3801127301905474357i64;
var398;
let var399: usize = vec![17727761128768229227u64,5800284227850890389u64,1664121865040497396u64,18166651851190162897u64].len();
var396 = var399;
let var401: i128 = (112961654443935910368794073665917903949i128 & 21318853642826847241458956488211274654i128);
let var402: i128 = 102909734188318598397547552195183416158i128;
let var403: i128 = 138630082600668212282974050179915546912i128;
let var404: i128 = 110941975464687415138121893425122472203i128;
let var405: i128 = 142021704831232167667585187757657296138i128;
let var406: i128 = 39294735264412887059332997538183505969i128;
let var407: i128 = 30091105194875051489066288208815700925i128;
vec![(143985944243817996588629082168017496060i128 | 85398975130221992381276609678610339900i128),var401,128401355214374268304084208684216640286i128,var402,var403,var404,var405,var406,var407];
let var409: bool = false;
let mut var408: bool = var409;
let var410: Struct3 = Struct3 {var7: 0.5435108f32,};
var410;
let mut var412: f64 = 0.23749369175906565f64;
let mut var413: f64 = 0.7625884049315639f64;
vec![0.5357224025236844f64,var412,0.8741170226441312f64,0.3586213285179032f64,0.8092614098104894f64,var413].push(0.5333327451563805f64);
Struct6 {var233: 0.1607642048208423f64,};
vec![6875u16];
let var414: bool = false;
&(var414);
31954i16;
let var419: i64 = 3999805415823983132i64;
None::<i16>
}

#[inline(never)]
fn fun23( var435: Option<u32>, var436: &i32, var437: Box<i16>, var438: Struct4, hasher: &mut DefaultHasher) -> i64 {
let mut var439: bool = (true | false);
var439 = true;
let var440: i128 = 85331350555749324870907762115075900003i128;
format!("{:?}", var435).hash(hasher);
return 1117056813818842948i64;
1401259362395649013i64
}


fn fun24( var448: Vec<f64>, var449: String, hasher: &mut DefaultHasher) -> String {
let var451: u16 = 32412u16;
let mut var450: u16 = var451;
var450 = 10574u16;
var450 = CONST2;
format!("{:?}", var449).hash(hasher);
var450 = CONST2;
let var452: (u16,bool) = (14715u16,false);
var452;
var450 = 18567u16;
format!("{:?}", var448).hash(hasher);
31i8;
var450 = var452.0;
var450 = 54349u16;
var450 = 15558u16;
let var453: (Option<i16>,i16,i16) = (Some::<i16>(28518i16),15000i16,30092i16);
var453;
let var454: i64 = -2614300038622537502i64;
var454;
format!("{:?}", var453).hash(hasher);
let var455: String = String::from("xV2fJXCNwj6eJ4kJ58f63QQOyC5yO");
var455;
let var456: i32 = 1343887267i32;
var456;
var450 = var451;
format!("{:?}", var452).hash(hasher);
String::from("bF8xYWW")
}

#[inline(never)]
fn fun25( var459: u64, var460: u128, var461: i16, var462: u16, hasher: &mut DefaultHasher) -> Type1 {
false;
return Some::<u64>(4866198137808386568u64);
None::<u64>
}


fn fun26( hasher: &mut DefaultHasher) -> (Option<i16>,i16,i16) {
vec![None::<Option<u16>>,Some::<Option<u16>>(Some::<u16>(47335u16)),Some::<Option<u16>>(Some::<u16>(838u16)),Some::<Option<u16>>(Some::<u16>(32680u16)),None::<Option<u16>>,Some::<Option<u16>>(None::<u16>),None::<Option<u16>>,Some::<Option<u16>>(None::<u16>)].push(None::<Option<u16>>);
let mut var467: usize = 11138609463388078723usize;
format!("{:?}", var467).hash(hasher);
vec![7951u16,54776u16,32295u16,25958u16,38708u16,55844u16,46505u16].len();
let mut var468: u128 = 51706682633636600297983836545144689111u128;
format!("{:?}", var467).hash(hasher);
format!("{:?}", var468).hash(hasher);
var467 = 17523961174734701704usize;
var468 = 20189860029767563992188186449581834578u128;
var468 = 12852640484702951151067618721746907941u128;
format!("{:?}", var468).hash(hasher);
None::<usize>;
let mut var470: (Option<i16>,i16,i16) = (None::<i16>,3370i16,16654i16);
false;
let mut var471: i128 = 101995991723910516691172377167108274559i128;
return (None::<i16>,11866i16,5038i16);
(None::<i16>,16247i16,27455i16)
}


fn fun27( var490: i128, var491: &String, var492: i8, var493: u16, hasher: &mut DefaultHasher) -> Box<i128> {
let mut var494: String = String::from("eC4ZWZVlxWTvCmiGnhTNwnOnZjRI6qLRPn36dHfDMZWcsIdMxrFgEd1U");
var494 = String::from("mjrVvJy1v");
format!("{:?}", var494).hash(hasher);
157u8;
let mut var495: u8 = 137u8;
var495 = 139u8;
String::from("ZOCI3OxWJHWKIb9IhX9KpWcGnAWA6pArwwWm0c0WuE5uXiTQEZMLaQsfAMGAY7dbQmF8OrWRCwpJVSO");
format!("{:?}", var492).hash(hasher);
vec![(None::<i16>,1671i16,24080i16),(None::<i16>,2352i16,12033i16),(None::<i16>,12165i16,21354i16),(Some::<i16>(10659i16),1003i16,4590i16),(Some::<i16>(12723i16),31369i16,1017i16),(None::<i16>,23987i16,3934i16),(Some::<i16>(31537i16),2869i16,2376i16),(Some::<i16>(30491i16),23921i16,836i16)].push((None::<i16>,31289i16,7228i16));
format!("{:?}", var493).hash(hasher);
let var496: i8 = 71i8;
let mut var497: Option<i8> = Some::<i8>(13i8);
return Box::new(14127981686668720671775770920734584792i128);
Box::new(122715830340620316189266328084261113745i128)
}


fn fun28( hasher: &mut DefaultHasher) -> Struct3 {
-3188467742847752921i64;
let mut var502: i32 = -1409157257i32;
format!("{:?}", var502).hash(hasher);
vec![0.7698880538320075f64,0.6025815280849008f64,0.018258224887629404f64].push(0.159412791741394f64);
2601729661276295998i64;
let var503: Option<i32> = Some::<i32>(-892775239i32);
let mut var504: Option<i8> = None::<i8>;
var504 = None::<i8>;
();
var502 = 1112432593i32;
return (Struct3 {var7: 0.87658435f32,});
Struct3 {var7: 0.80672896f32,}
}


fn fun29( var506: f64, var507: i64, var508: u16, var509: bool, hasher: &mut DefaultHasher) -> u32 {
let mut var510: i32 = -305258612i32;
var510 = 1277685584i32;
let var511: i8 = 115i8;
0.020482094516178107f64;
return 1424349605u32;
3567425880u32
}


fn fun17( var284: i128, hasher: &mut DefaultHasher) -> (Option<i16>,i16,i16) {
if (fun19(hasher)) {
 format!("{:?}", var284).hash(hasher);
0.5781773f32;
format!("{:?}", var284).hash(hasher);
42611u16;
format!("{:?}", var284).hash(hasher);
let mut var285: i128 = 54400797175044202506907546843225051869i128;
var285 = 55252733671527569404128011793083784576i128;
let var287: u32 = 515139066u32;
let mut var286: u32 = var287;
let var288: u64 = 722475705524892633u64;
let var289: u64 = 12928991374869151654u64;
let var290: u64 = 5989689481784006547u64;
let var306: f64 = 0.6577111203258407f64;
vec![var288,var289,var290,(18054949643393224328u64 | Struct6 {var233: var306,}.fun18(Struct3 {var7: 0.0084270835f32,},hasher)),9721613918870418525u64,2852775361344190162u64].len();
let var308: f64 = 0.14024950599546515f64;
let mut var307: &f64 = &(var308);
format!("{:?}", var289).hash(hasher);
format!("{:?}", var285).hash(hasher);
-3692993215801498855i64;
format!("{:?}", var284).hash(hasher);
format!("{:?}", var288).hash(hasher);
let var310: String = String::from("KXtYa");
var310;
let var311: u64 = 7883058056798241959u64;
var311;
let var312: u16 = 42494u16;
var312;
let var314: (Option<i16>,i16,i16) = (None::<i16>,27192i16,154i16);
let mut var313: (Option<i16>,i16,i16) = var314;
var313.0 = var314.0;
String::from("urloWP2ImBfbsSWJUie12zFeApQvFBlokd") 
} else {
 let var323: Option<i16> = Some::<i16>(16845i16);
let mut var322: Option<i16> = var323;
var322 = Some::<i16>(25394i16);
var322 = None::<i16>;
let var324: u64 = 17416864148912122218u64;
var324;
let mut var325: i64 = 3255962531471514820i64;
let var326: i8 = 102i8;
var325 = 4412181160507041735i64;
();
let var328: (u16,bool) = (41421u16.wrapping_mul(fun11(24713i16,86690834350489287620999727872185213683u128,7229110685615732368i64,hasher)),true);
var328;
0.5757291524138501f64;
let var330: i64 = 4664713781401191902i64;
let var329: i128 = fun5(var330,hasher);
format!("{:?}", var322).hash(hasher);
let var331: u128 = 37203295965867930703041614839158262439u128;
var331;
let var332: u32 = 1667930410u32;
var332;
let mut var333: Vec<Option<Option<u16>>> = vec![None::<Option<u16>>];
let mut var334: i128 = 46175999442114505766144322937054600950i128;
vec![127688359482249954388540253110919817775i128.wrapping_add(var334)].push(123771639301092100748899242084489078868i128);
0.1290150346879423f64;
format!("{:?}", var329).hash(hasher);
String::from("gSdZIZ47jnIlbN") 
};
19754120712865199507102615759871009052u128;
let var335: String = String::from("XgO2pdjIyDhCH7lvpL3KKwl59UYZ11tFM8TS66JvyAxGRlTf8BdIORea4g61sPVMQIjH");
var335;
9076i16;
None::<bool>;
();
format!("{:?}", var284).hash(hasher);
let var377: f64 = 0.44445793788960586f64;
var377;
let var379: usize = 18165806935456928944usize;
var379;
let var384: (i8,f64) = (fun2(false,-1561240466i32,209u8,179u8,hasher),0.054768981406807526f64);
let mut var383: (i8,f64) = var384;
let mut var385: Option<bool> = None::<bool>;
format!("{:?}", var385).hash(hasher);
format!("{:?}", var383).hash(hasher);
let var386: i16 = 16862i16;
let var387: i16 = 8737i16;
return (Some::<i16>(22668i16),var386,var387);
let var422: bool = false;
if (var422) {
 let var388: i64 = -4976064814323306419i64;
let var389: i16 = 14559i16;
return (None::<i16>,var389,24159i16);
let var420: i16 = 30341i16;
let var421: i16 = 28967i16;
(fun22((17i8,var384.1),hasher),var420,var421) 
} else {
 let var424: u8 = 123u8;
let mut var423: u8 = (var424);
var385 = None::<bool>;
format!("{:?}", var387).hash(hasher);
let var425: u128 = 12164768109021900828051132743112261745u128;
var425;
var383.0 = 59i8;
let mut var426: Vec<u16> = vec![59775u16,26211u16,(17859u16 | 9458u16),57512u16,5746u16,17129u16,16705u16,43017u16,56431u16];
let var427: u16 = 27111u16;
var426.push(var427);
let var428: String = String::from("UgOrDQdw3QOmibTyrf6fZuFzbiPs8o3Z4RtBa");
var428;
let mut var430: i16 = 7247i16;
let var429: &mut i16 = &mut (var430);
let var431: u16 = 3369u16;
var431;
let var433: u8 = 248u8;
let mut var432: u8 = var433;
1523764596168924122u64;
Box::new(21965i16);
Box::new({
var383 = var384;
let mut var442: f64 = var384.1;
93431110174821055732265919575705063939u128;
format!("{:?}", var429).hash(hasher);
457670589031135503i64;
55i8;
();
var442 = var384.1;
1992445927986448715u64;
true;
let var445: Option<Option<u128>> = Some::<Option<u128>>(Some::<u128>((112857281449234719967814767906781328322u128)));
var445;
var432 = var433;
let var446: (Option<i16>,i16,i16) = (None::<i16>,15146i16,7012i16);
return var446;
let var447: u16 = 28973u16;
Box::new(var447)
});
let var457: String = String::from("C6KXbLJd9vgOfEbTqjdUJA7QZqaUEoO3lrVyEmneIl07hkfXu8UUxQ7juZHM44puxgSYNBQ2");
fun24(vec![var384.1,0.04128884097382157f64,var384.1,var384.1,var384.1,var384.1],var457,hasher);
let var458: Type1 = fun25(16480124864225731028u64,22440554073594434957734463221385157251u128,9207i16,(40266u16 ^ 45095u16),hasher);
var458;
var383.0 = var384.0;
format!("{:?}", var427).hash(hasher);
let mut var463: i8 = 34i8;
let var464: f64 = var384.1;
format!("{:?}", var284).hash(hasher);
let var465: i32 = if (false) {
 Struct7 {var352: Box::new(fun6(0.665867f32,111u8,hasher)),};
var423 = 39u8;
let var466: u128 = fun1(None::<usize>,false,hasher);
fun26(hasher);
vec![26075959466568537367861075759622593810i128].push(169973823725526663155440601595973562885i128);
let mut var472: i32 = 1996855534i32;
31331u16;
100533656256959820303395824437518593271u128;
var423 = 191u8;
format!("{:?}", var387).hash(hasher);
let var473: i16 = fun21(hasher);
fun13(hasher);
format!("{:?}", var383).hash(hasher);
return (None::<i16>,9549i16,31001i16);
-133858494i32 
} else {
 0.5271943f32;
let var474: i16 = 4921i16;
var463 = 104i8;
format!("{:?}", var463).hash(hasher);
format!("{:?}", var458).hash(hasher);
let var475: u128 = 113923901631216329069247764110638549191u128;
0.7582493855359095f64;
format!("{:?}", var385).hash(hasher);
var432 = 9u8;
format!("{:?}", var464).hash(hasher);
vec![18749i16,fun6(0.13580811f32,209u8,hasher),22514i16];
-5577212153385232960i64;
2048i16;
format!("{:?}", var464).hash(hasher);
let mut var476: i64 = -4634340458445024968i64;
let mut var477: i32 = 1551673804i32;
format!("{:?}", var423).hash(hasher);
return (Some::<i16>(2307i16),16149i16,22195i16);
reconditioned_mod!(1033090714i32, -598177741i32, 0i32) 
};
var465;
let mut var478: usize = 14380038026103524045usize;
let var501: Struct3 = fun28(hasher);
var501;
let var505: u32 = fun29(0.7780242329435605f64,-5296944063757153901i64,8437u16,true,hasher);
var505;
let var512: i16 = 26749i16;
(None::<i16>,1933i16,var512) 
}
}


fn fun31( hasher: &mut DefaultHasher) -> Vec<(Option<i16>,i16,i16)> {
let mut var612: f32 = 0.787041f32;
var612 = 0.4297158f32;
Box::new(32505i16);
3598023045u32;
vec![(Some::<i16>(24020i16),13371i16,3960i16),(None::<i16>,9065i16,22031i16),(Some::<i16>(2643i16),20656i16,27941i16),(None::<i16>,15807i16,32675i16),(None::<i16>,9838i16,18256i16),(Some::<i16>(23686i16),9104i16,18655i16),(Some::<i16>(3104i16),21799i16,3383i16),(Some::<i16>(23731i16),21148i16,12605i16),(Some::<i16>(11338i16),17534i16,2332i16)].len();
384567110u32;
format!("{:?}", var612).hash(hasher);
var612 = 0.71380484f32;
Box::new(55740u16);
format!("{:?}", var612).hash(hasher);
let var613: usize = vec![12321454423496998359u64,7508703027789810054u64,4967065557978982690u64,2240559237655712554u64,17694780796848699536u64,17421509435561156588u64].len();
vec![Struct1 {var1: 3372i16, var2: Box::new(1987748271u32), var3: -113404656351795449i64, var4: -1971346734i32,}].len();
143373175628959195922883260310165047331i128;
let var614: i32 = -1169770900i32;
format!("{:?}", var613).hash(hasher);
let var615: bool = true;
var612 = 0.4365155f32;
let var616: f64 = 0.13293596202254965f64;
Some::<u32>(4171898118u32);
0.7723138087171626f64;
var612 = 0.055266738f32;
let var617: Type1 = None::<u64>;
var612 = 0.30014282f32;
var612 = 0.71621007f32;
vec![(None::<i16>,29037i16,15638i16),(Some::<i16>(3885i16),10403i16,20974i16)]
}

#[inline(never)]
fn fun32( hasher: &mut DefaultHasher) -> () {
let var675: i16 = 18363i16;
let mut var674: i16 = var675;
format!("{:?}", var674).hash(hasher);
format!("{:?}", var675).hash(hasher);
true;
let var678: u16 = 26453u16;
let var677: u16 = var678;
let mut var676: u16 = var677;
let mut var679: f64 = 0.7147043117398085f64;
{
let var681: String = String::from("Wt7CO6wm0SJcS");
let var680: String = var681;
var680;
return ();
};
let mut var683: u16 = 22820u16;
let mut var682: &mut u16 = &mut (var683);
let var684: u64 = 6237543143530802770u64;
var684;
let var685: u128 = 57873080305171432559083244396681935650u128;
var676 = var678;
format!("{:?}", var678).hash(hasher);
0.834702297428296f64;
let mut var686: u128 = 121114187616392246618039529253923907050u128;
let var710: u64 = 4558209744602338324u64;
let mut var709: u64 = var710;
let var708: &mut u64 = &mut (var709);
let var707: &mut u64 = var708;
let var706: &mut u64 = var707;
let mut var705: &mut u64 = var706;
let var711: f64 = 0.039968870521233324f64;
var711;
format!("{:?}", var674).hash(hasher);
55i8;
format!("{:?}", var685).hash(hasher);
let var712: u64 = 6002070838026655125u64;
var712;
let var718: f64 = 0.22697387641016653f64;
let var717: f64 = var718;
let var716: &f64 = &(var717);
let var715: &f64 = var716;
let var714: &f64 = var715;
let mut var713: &f64 = var714;
let var721: i128 = 82624830604175559599808686045401610498i128;
let var720: i128 = var721;
let var719: i128 = var720;
(Box::new(var719));
}

#[inline(never)]
fn fun33( var762: (u8,Box<i32>,&f64), hasher: &mut DefaultHasher) -> usize {
Some::<u16>(22471u16);
format!("{:?}", var762).hash(hasher);
let var766: u16 = 54939u16;
Struct10 {var763: Box::new(var766), var764: 4147115527u32, var765: false,};
let var769: f64 = 0.14940676556264987f64;
let var768: f64 = var769;
let var767: f64 = var768;
var767;
let var771: u32 = 3986677452u32;
let var770: u32 = var771;
var770;
false;
format!("{:?}", var766).hash(hasher);
();
let var774: String = String::from("yP2C3qgdXRLC8JBlnTm0CmzzWt");
let var773: String = var774;
let mut var772: String = var773;
var772 = String::from("TRQ");
let var781: String = String::from("yDknsLuzS5viEGDcGOWDPtblpd5IVJYslec5MSbvg0MGD3k7iy9Ccxulb62kF0dMjmmuquZLgH4watFG0RD");
let var780: String = var781;
let var779: String = var780;
let var778: String = var779;
let var777: String = var778;
let var776: String = var777;
let var775: String = var776;
var772 = var775;
7275u16;
0.32582969841885323f64;
true;
let var783: f64 = 0.24574601161549237f64;
let var784: f64 = 0.7818517565635223f64;
let var785: f64 = 0.8973267181029065f64;
let var782: usize = vec![var783,var784,var785,0.7909246201120099f64].len();
return var782;
6171417480373957186usize
}


fn fun38( var989: usize, var990: u128, hasher: &mut DefaultHasher) -> Struct1 {
format!("{:?}", var990).hash(hasher);
return Struct1 {var1: 18690i16, var2: Box::new(3528094210u32), var3: 6386189540496007907i64, var4: 1335052093i32,};
Struct1 {var1: 15210i16, var2: Box::new(2777883328u32), var3: 3526950176502603543i64, var4: -628124624i32,}
}


fn main( ) -> () {
let cli_args: Vec<String> = env::args().collect();
let mut s = DefaultHasher::new();
let hasher = &mut s;
let var667: u16 = cli_args[14].clone().parse::<u16>().unwrap();
None::<Vec<(Option<i16>,i16,i16)>>;
let var668: i128 = cli_args[5].clone().parse::<i128>().unwrap();
let mut var1003: u64 = cli_args[6].clone().parse::<u64>().unwrap();
let var1004: u16 = 6039u16;
var1003 = 12612008494902443206u64;
format!("{:?}", var667).hash(hasher);
let mut var1005: u64 = 6907361457973365412u64;
&mut (var1005);
let var1006: bool = cli_args[2].clone().parse::<bool>().unwrap();
var1003 = CONST8;
40493673479722785344891363927682818057u128;
var1003 = cli_args[6].clone().parse::<u64>().unwrap();
var1003 = CONST8;
var1003 = CONST8;
format!("{:?}", var1003).hash(hasher);
false;
var1003 = (1238139013960603115u64 | 11623769572248329702u64);
format!("{:?}", CONST1).hash(hasher);
format!("{:?}", CONST2).hash(hasher);
format!("{:?}", CONST3).hash(hasher);
format!("{:?}", CONST4).hash(hasher);
format!("{:?}", CONST5).hash(hasher);
format!("{:?}", CONST6).hash(hasher);
format!("{:?}", CONST7).hash(hasher);
format!("{:?}", CONST8).hash(hasher);
format!("{:?}", var1003).hash(hasher);
format!("{:?}", var1004).hash(hasher);
format!("{:?}", var1006).hash(hasher);
format!("{:?}", var667).hash(hasher);
format!("{:?}", var668).hash(hasher);
println!("Program Seed: {:?}", -6381862764418016634i64);
println!("{:?}", hasher.finish());
}
