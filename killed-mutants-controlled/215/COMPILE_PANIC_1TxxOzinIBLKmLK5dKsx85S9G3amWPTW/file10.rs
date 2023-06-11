#![allow(warnings, unused, unconditional_panic)]
use std::env;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
const CONST1: i128 = 81831085192004573965199924909173649504i128;
const CONST2: i32 = 1465251071i32;
const CONST3: f32 = 0.9304469f32;
const CONST4: f32 = 0.32282257f32;
const CONST5: i16 = 30296i16;
const CONST6: i64 = -3236525953864574131i64;
const CONST7: u128 = 22853575958234638175577463725726829180u128;
const CONST8: i8 = 112i8;
const CONST9: i32 = 1865295069i32;
macro_rules! reconditioned_mod{
    ($a:expr,$b:expr, $zero: expr) => {
        {
            let denominator = $b;
            if (denominator != $zero) {($a % denominator)} else {$zero}
        }
    }
}
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
#[derive(Debug)]
struct Struct1 {
var18: bool,
var19: u16,
}

impl Struct1 {
 
fn fun5(&self, var42: (u128,Option<Vec<u64>>,u64), var43: (i32,u64,i16,u8), hasher: &mut DefaultHasher) -> u16 {
let mut var44: Option<u128> = Some::<u128>(100627945620665993261675585457746722534u128);
var44 = Some::<u128>(67798513541038193775146527100558408749u128);
format!("{:?}", var44).hash(hasher);
format!("{:?}", var42).hash(hasher);
let mut var45: String = String::from("zYiglAoEI01iJAcFlusEEICpkzVR68GOI07wMASWi2");
var45 = String::from("nYSj7bNSJO33GcLV9Yx5nGrHgQPDx");
var45 = String::from("KsF3eZtjWa87LCmDMmPlcQ4uTJREXSpcLavOWIkiz9Oc5Cnwi3tu5EU9LhdDJ");
let mut var47: f32 = 0.014326453f32;
var44 = None::<u128>;
format!("{:?}", var47).hash(hasher);
let mut var48: (u128,Option<Vec<u64>>,u64) = (45674562221189502427746529425216288754u128,Some::<Vec<u64>>(vec![17797897388354227956u64]),17189618349637040069u64);
8001436607741376217i64;
format!("{:?}", var44).hash(hasher);
0.6011382639173201f64;
2195147530u32;
0.29205704f32;
41542u16
}
 
}
#[derive(Debug)]
struct Struct2 {
var51: u128,
var52: u16,
}

impl Struct2 {
 
fn fun9(&self, hasher: &mut DefaultHasher) -> Option<u64> {
format!("{:?}", self).hash(hasher);
let mut var70: i16 = 14960i16;
var70 = 16800i16;
format!("{:?}", self).hash(hasher);
let var71: i128 = 147104060909310607400833590183198093071i128;
true;
let mut var72: Box<u128> = Box::new(29380041811994411883826446648708689027u128);
var72 = Box::new(150770902646357680189910496396703445544u128);
Struct5 {var73: String::from("ubNxHbc"),};
Struct1 {var18: false, var19: 34111u16,};
let mut var74: u128 = 94112054426232658279824407978886552465u128;
format!("{:?}", var70).hash(hasher);
var70 = 12302i16;
vec![0.24392277f32,0.9290433f32,0.15932703f32,0.37315464f32,0.9432688f32,0.21365541f32,0.57182777f32,0.113618195f32,0.692036f32].push(0.87379146f32);
var74 = 140344626533730827894129365638674250157u128;
var74 = 139158269999536839023816144281029914587u128;
0.22567744795092992f64;
var70 = 29492i16;
13644602917279278452usize;
None::<u64>
}
 
}
#[derive(Debug)]
struct Struct4 {
var60: u16,
}

impl Struct4 {
  
}
#[derive(Debug)]
struct Struct3<'a3> {
var59: Option<Struct4<>>,
var61: &'a3 mut i32,
}

impl<'a3> Struct3<'a3> {
  
}
#[derive(Debug)]
struct Struct5 {
var73: String,
}

impl Struct5 {
  
}
type Type1 = i128;

fn fun1( var4: u128, var5: u32, var6: i16, var7: u32, hasher: &mut DefaultHasher) -> i128 {
format!("{:?}", var5).hash(hasher);
let var8: i128 = 38562619839600739504144704785244837315i128;
return var8;
156238527248094267443125445138799146650i128
}

#[inline(never)]
fn fun3( var21: u64, var22: i32, var23: i64, hasher: &mut DefaultHasher) -> Box<u128> {
let var24: String = String::from("5YCcfqzbZpLYqRq7HIE");
format!("{:?}", var23).hash(hasher);
false;
format!("{:?}", var24).hash(hasher);
if (false) {
 return Box::new(34774682853768254800166537881460790812u128);
0.22291644956222412f64 
} else {
 format!("{:?}", var21).hash(hasher);
format!("{:?}", var21).hash(hasher);
let var25: i16 = 25498i16;
vec![164422181069862364373375012853439085617u128].push(40019522120726065569452143851207526791u128);
let var26: Type1 = 25972888248665503378702793395732102844i128;
let mut var27: u32 = 2240375442u32;
var27 = 4001824732u32;
vec![18204102808202639211u64,16498369557063574218u64,10965712787325269343u64,17820741174085125883u64,5352941707039778082u64,13784570457002196627u64,9179600882184360850u64,4167671175244466658u64,4506440535633882433u64].len();
var27 = 2457522723u32;
Some::<i8>(82i8);
131552383628979469511283485890140230019i128;
{
String::from("9ejmE8UIoaNnu7kZ2GNog7GjnGUtnxptY");
format!("{:?}", var23).hash(hasher);
return Box::new(148304068915571038028936738452991173790u128);
2823058904u32
};
24702u16;
let var28: u32 = 1784541319u32;
var27 = 2837633063u32;
var27 = 2396887374u32;
Struct1 {var18: false, var19: 18085u16,};
0.19654861863840944f64;
return Box::new(78393901100286104384137842302186188156u128);
0.6355707503403635f64 
};
let mut var30: i8 = 16i8;
var30 = 115i8;
169143933809644170010932006414992138873u128;
let var31: usize = vec![(0.099359035f32),0.39841098f32,0.9857603f32].len();
104i8;
103u8;
33396u16;
Box::new(44153256696690597903678266271845520061u128);
let mut var32: i128 = 92681948229502980213334570046442721902i128;
107376434860685091769460428905392218460i128;
format!("{:?}", var31).hash(hasher);
();
Box::new(30207505153103652176876474004510721806u128)
}


fn fun2( hasher: &mut DefaultHasher) -> Box<u128> {
let mut var17: String = String::from("LZrdXIsOe8OEomgzvmh");
var17 = String::from("ikbvqMwGosEH6wA4gSMMDQbDY8vALvxnKo6VJLrJzbMnxuNzp4PGK9qaafYYGFkC3NbYLA5t6XOR1BD9PW6pOiW9lMZ");
2445606866u32;
30958348312292099254533166672865133876i128;
let mut var20: Struct1 = Struct1 {var18: false, var19: 55993u16,};
-7004952199829153583i64;
Box::new(26857950086033655181925444424007210846u128);
return fun3(623478453418784751u64,reconditioned_mod!(884816439i32, 1666375910i32, 0i32),-8215694334392657802i64,hasher);
Box::new(128375026117097608165417922471584152356u128)
}


fn fun6( hasher: &mut DefaultHasher) -> i32 {
38119u16.wrapping_mul(59080u16);
let mut var49: usize = 6305518071969896755usize;
format!("{:?}", var49).hash(hasher);
(vec![17835765107785715402u64,1727994629018973868u64,12732014554567197534u64,6215516949041622382u64],245u8);
15230331383299076308u64;
var49 = vec![92041299747715270098637523727098587906u128,148229097685039062310320113013668230606u128,16899313765530139240843204361409780435u128,154767111416810029085533225392652614124u128.wrapping_mul(25307374043387742602266218160671055790u128),90625438643289989655092076357516459865u128,159577221354153664523765091679721700321u128,117627241155404979660398710732309875198u128,141513545479991024647823657500297202113u128,142032602347474006622597200099713031309u128].len();
var49 = 916521744707683290usize;
let var53: Struct2 = Struct2 {var51: 167887577318039095421506168338075645936u128, var52: 45270u16,};
245u8;
format!("{:?}", var53).hash(hasher);
return -1391873830i32;
1388562003i32
}

#[inline(never)]
fn fun7( var54: &f32, hasher: &mut DefaultHasher) -> i16 {
String::from("0gVmKUOVTR7QYgBL7ZzhAQGwHubOBIcQbNMSu9QZMxfeNJ5zVXmusFOIiF8NbNWh");
49977u16;
71506947549044121398306949152012979445u128;
47i8;
-8289597623529683549i64;
(vec![9074788125744766299u64.wrapping_add(1049253666831296023u64),17700526934409214225u64,4702018520226673478u64,4020107846540105741u64,912778097873576192u64,8536141907658128742u64,10568061700241807552u64,14540176140826198337u64,15985874032245430309u64],201u8.wrapping_mul(242u8));
let mut var55: i8 = 123i8;
10132i16;
let var56: i16 = 21530i16;
94i8;
let var57: u16 = 60580u16;
2175155895u32;
49342844450301080i64;
let var58: i64 = 4510344759777041002i64;
format!("{:?}", var55).hash(hasher);
(764550967i32,13126970515760068068u64,5298i16,123u8);
vec![12588165523679334073u64,5606869795263036565u64,18353486669245095877u64,905639967874260789u64,2594255141189121657u64,8932667078602229169u64,12406502335547802207u64].push((18364188140134994152u64 & 10310732476748763069u64));
36902u16;
let mut var63: i16 = 15834i16;
5821i16
}

#[inline(never)]
fn fun8( var66: bool, var67: Struct1, var68: &mut usize, hasher: &mut DefaultHasher) -> u16 {
let mut var69: Option<u64> = Struct2 {var51: 134530078529039700014132947629934427126u128, var52: 53875u16,}.fun9(hasher);
let mut var75: bool = true;
169271151542819747253395354637365827684i128;
78i8;
var75 = false;
vec![0.5182616f32,0.62770385f32,0.6019216f32,0.035873413f32,0.7601387f32,0.32775098f32].len();
146186938867836037473570404870155193944i128.wrapping_mul(108976484384603625969330974977312975124i128);
(reconditioned_mod!(-160969897i32, -1764056916i32, 0i32),10899840435946063954u64,2319i16,9u8);
159190143581070803270876363166187349206u128;
format!("{:?}", var75).hash(hasher);
var69 = Some::<u64>(1474595824631398379u64);
154277269835853728283574754924478364710u128;
var75 = true;
var75 = true;
let var76: Struct4 = Struct4 {var60: 60597u16,};
119503292503241509184515568348935056454i128;
1498i16;
return 20791u16;
16389u16
}


fn fun10( var81: u8, hasher: &mut DefaultHasher) -> u8 {
let mut var82: i128 = 25025385046663525556452685764503224059i128;
var82 = 156232674491684230576926059748910397826i128;
format!("{:?}", var82).hash(hasher);
let mut var83: u16 = 11020u16;
15221202534151714377u64;
var82 = 87702610308901082385207005627469138647i128;
5507272805805901180usize;
let var84: u128 = 26091590227444824504553678229052138480u128;
var82 = 106625338797876957708070287952200941703i128;
return 126u8;
144u8
}


fn fun4( var39: f64, hasher: &mut DefaultHasher) -> u16 {
();
format!("{:?}", var39).hash(hasher);
let mut var40: bool = {
let mut var41: String = if (true) {
 return Struct1 {var18: true, var19: 11521u16,}.fun5((41070318981529642838221551075798767913u128,None::<Vec<u64>>,941951859351565662u64),(1975304208i32,10700761785868389784u64,14033i16,81u8),hasher);
String::from("drdW1SEdsNmPxVyW5mMUw8") 
} else {
 return Struct1 {var18: true, var19: 11521u16,}.fun5((41070318981529642838221551075798767913u128,None::<Vec<u64>>,941951859351565662u64),(1975304208i32,10700761785868389784u64,14033i16,81u8),hasher);
String::from("drdW1SEdsNmPxVyW5mMUw8") 
};
var41 = String::from("v6HuVfiR23EKdT8uCxW5eGhcnyiGTv5eYFRs");
114u8;
format!("{:?}", var39).hash(hasher);
59504u16;
let mut var65: i128 = 4030981417512059494285783368320321323i128;
let var78: Box<u128> = Box::new(64866216918105605655293462060326876684u128);
let mut var79: (i32,u64,i16,u8) = (982175990i32,9523180518723440332u64,19634i16,178u8);
let var80: Box<u16> = Box::new(59307u16);
Some::<i64>(4881863968764149334i64);
var65 = 125858014257039917733582194958248922710i128;
var79 = (1169083169i32,3827213407214228915u64,3380i16,reconditioned_div!(23u8, fun10(36u8,hasher), 0u8));
var79.0 = -981018413i32;
let mut var85: u128 = (77331927843574198511379879565971636964u128 ^ 28388113743703625879667933646131503503u128);
format!("{:?}", var78).hash(hasher);
format!("{:?}", var80).hash(hasher);
format!("{:?}", var41).hash(hasher);
let var86: f64 = 0.08794598453792213f64;
String::from("zu2LXpjQZrwkrDpY0SInv7HAStjkh9g7");
true
};
None::<u128>;
None::<Struct4>;
String::from("eXO4qc19KRXRS6mgOyH6mgG6jR9F5QNsj5qGTsayGty38bsd8ChVZi4vq7BlSUpwLeitOKMcZppA30JLYpaX1HRbKVY");
format!("{:?}", var39).hash(hasher);
format!("{:?}", var40).hash(hasher);
11126171293877602207530495799148050249i128;
format!("{:?}", var40).hash(hasher);
let mut var87: i8 = 96i8;
var40 = true;
var87 = 46i8;
28i8;
None::<i8>;
15736u16
}


fn fun11( var97: Vec<usize>, var98: f64, hasher: &mut DefaultHasher) -> f32 {
true;
let var99: f32 = 0.2122488f32;
return var99;
0.86345565f32
}


fn main( ) -> () {
let cli_args: Vec<String> = env::args().collect();
let mut s = DefaultHasher::new();
let hasher = &mut s;
let var2: i128 = cli_args[1].clone().parse::<i128>().unwrap();
let var1: i128 = var2;
let var3: i128 = fun1(37899129163190139525115623662305160905u128,cli_args[2].clone().parse::<u32>().unwrap(),290i16,cli_args[2].clone().parse::<u32>().unwrap(),hasher);
(var1 | var3);
637834943u32;
0.8917993f32;
let mut var9: u16 = cli_args[3].clone().parse::<u16>().unwrap();
var9 = 55858u16;
let var10: i8 = {
();
let var14: f32 = 0.9769193f32;
let var13: f32 = var14;
cli_args[4].clone().parse::<u8>().unwrap();
cli_args[5].clone().parse::<bool>().unwrap();
let var15: f64 = 0.5809755615217403f64;
var15;
let var16: Box<u128> = fun2(hasher);
var16;
var9 = 4676u16;
format!("{:?}", var14).hash(hasher);
let var33: u16 = 25017u16;
var9 = var33;
format!("{:?}", var15).hash(hasher);
let var34: i64 = cli_args[6].clone().parse::<i64>().unwrap();
var9 = cli_args[3].clone().parse::<u16>().unwrap();
var9 = var33;
cli_args[6].clone().parse::<i64>().unwrap();
format!("{:?}", var13).hash(hasher);
let var35: u32 = 680822u32;
var35;
var9 = 49742u16;
cli_args[7].clone().parse::<usize>().unwrap();
let mut var36: i128 = cli_args[1].clone().parse::<i128>().unwrap();
let var37: f64 = cli_args[8].clone().parse::<f64>().unwrap();
var37;
let var38: Struct1 = Struct1 {var18: false, var19: fun4(cli_args[8].clone().parse::<f64>().unwrap(),hasher),};
var38;
let var88: String = cli_args[9].clone().parse::<String>().unwrap();
967547794u32;
let var89: i8 = cli_args[10].clone().parse::<i8>().unwrap();
let var90: i8 = cli_args[10].clone().parse::<i8>().unwrap();
(var89 & var90)
};
var10;
cli_args[9].clone().parse::<String>().unwrap();
cli_args[11].clone().parse::<i32>().unwrap();
let var94: u64 = 6963235697951699201u64;
let var93: u64 = var94;
let var92: Vec<u64> = vec![cli_args[12].clone().parse::<u64>().unwrap(),6522389056462495771u64,833533825950219u64,var93];
let var100: f32 = 0.35115522f32;
let var101: usize = (cli_args[7].clone().parse::<usize>().unwrap() & 16122663211193960599usize);
let var102: usize = cli_args[7].clone().parse::<usize>().unwrap();
let var96: f32 = fun11(vec![cli_args[7].clone().parse::<usize>().unwrap(),cli_args[7].clone().parse::<usize>().unwrap(),vec![var100].len(),var101,var102,cli_args[7].clone().parse::<usize>().unwrap()],cli_args[8].clone().parse::<f64>().unwrap(),hasher);
let var95: usize = vec![var96,cli_args[13].clone().parse::<f32>().unwrap()].len();
let var91: u64 = reconditioned_access!(var92, var95);
var91;
();
let var103: Option<i8> = None::<i8>;
let var104: i32 = cli_args[11].clone().parse::<i32>().unwrap();
let mut var105: u64 = cli_args[12].clone().parse::<u64>().unwrap();
fun10(195u8,hasher);
format!("{:?}", var96).hash(hasher);
format!("{:?}", var1).hash(hasher);
format!("{:?}", CONST1).hash(hasher);
format!("{:?}", CONST2).hash(hasher);
format!("{:?}", CONST3).hash(hasher);
format!("{:?}", CONST4).hash(hasher);
format!("{:?}", CONST5).hash(hasher);
format!("{:?}", CONST6).hash(hasher);
format!("{:?}", CONST7).hash(hasher);
format!("{:?}", CONST8).hash(hasher);
format!("{:?}", CONST9).hash(hasher);
format!("{:?}", var1).hash(hasher);
format!("{:?}", var10).hash(hasher);
format!("{:?}", var100).hash(hasher);
format!("{:?}", var101).hash(hasher);
format!("{:?}", var102).hash(hasher);
format!("{:?}", var103).hash(hasher);
format!("{:?}", var104).hash(hasher);
format!("{:?}", var105).hash(hasher);
format!("{:?}", var2).hash(hasher);
format!("{:?}", var3).hash(hasher);
format!("{:?}", var9).hash(hasher);
format!("{:?}", var91).hash(hasher);
format!("{:?}", var93).hash(hasher);
format!("{:?}", var94).hash(hasher);
format!("{:?}", var95).hash(hasher);
format!("{:?}", var96).hash(hasher);
println!("Program Seed: {:?}", -8997745538835215249i64);
println!("{:?}", hasher.finish());
}
