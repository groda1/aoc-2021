pub fn main() -> Result<(), ()> {
    let input = "1101,1,29,67,1102,0,1,65,1008,65,35,66,1005,66,28,1,67,65,20,4,0,1001,65,1,65,1106,0,8,99,35,67,101,99,105,32,110,39,101,115,116,32,112,97,115,32,117,110,101,32,105,110,116,99,111,100,101,32,112,114,111,103,114,97,109,10,315,1081,464,841,82,86,483,2,1163,1189,184,369,510,519,13,0,1127,899,1157,1018,3,1014,507,330,20,4,374,641,1768,773,1130,827,1132,166,217,3,460,51,767,484,109,48,143,977,790,1139,426,50,391,537,1140,478,41,1560,1457,180,814,138,338,751,1164,1088,157,1168,163,880,1477,1236,496,446,582,215,840,19,6,626,84,473,977,191,281,1166,230,643,772,985,65,290,389,409,366,1339,499,4,20,262,58,1294,1043,1625,77,284,415,220,583,110,70,639,424,273,28,669,920,1580,434,377,24,446,113,100,1040,1271,874,73,629,87,88,877,959,896,1478,377,14,409,1150,975,547,181,367,549,941,923,1175,19,293,91,1079,495,8,156,807,1423,337,544,505,694,45,968,349,472,1333,43,1339,139,383,676,105,81,0,208,397,628,1033,429,1642,1635,397,675,273,263,165,965,600,571,834,13,411,592,2,49,1706,1288,241,115,5,1046,1301,688,474,34,817,200,410,233,181,173,8,124,527,1266,238,33,482,1066,1074,381,197,139,828,45,843,133,823,265,685,463,680,958,561,101,258,339,7,578,38,176,1127,57,147,186,795,685,769,27,547,1089,252,125,1182,63,907,1300,1022,1040,613,1082,964,693,7,130,259,75,5,1172,1380,362,1203,1936,666,50,600,482,497,106,724,153,52,480,848,1216,814,156,730,20,831,441,160,81,4,750,81,537,1006,320,493,526,63,112,0,467,65,50,1228,105,121,1123,568,232,6,946,436,429,1116,309,13,1471,60,15,1143,385,34,374,140,413,24,593,97,7,127,504,221,1380,585,661,326,709,91,1323,722,256,346,43,296,5,477,356,182,476,1109,107,1137,151,1303,201,796,760,1144,51,410,694,68,49,78,32,24,781,1347,34,16,137,999,47,94,788,409,188,853,362,862,576,386,621,52,341,908,362,160,1373,1118,368,222,125,389,327,168,136,197,445,206,564,517,95,547,765,526,584,1115,150,674,312,246,33,168,266,92,535,23,442,518,145,358,531,150,174,545,1525,576,1721,5,1369,1112,787,9,1266,955,265,385,823,1129,62,1045,1682,4,157,268,449,77,340,12,401,818,28,722,232,116,41,1250,504,1448,108,748,161,42,634,975,1046,50,371,474,669,271,910,263,1091,566,35,181,29,163,652,497,1108,878,448,44,241,319,585,530,176,596,1334,2,181,63,807,53,738,851,952,502,127,983,59,0,22,1143,393,75,292,105,592,649,1709,1505,0,1,634,1815,1505,23,145,117,1286,1641,372,273,348,688,113,0,823,1829,607,89,110,30,667,997,987,354,804,766,243,211,783,76,152,401,667,477,555,280,504,252,287,448,495,59,83,353,219,112,198,174,1496,3,803,765,1166,446,1062,451,1365,778,115,381,102,3,209,236,8,87,169,145,139,1032,1702,176,525,436,73,5,35,1512,198,370,70,358,135,46,153,473,48,521,315,709,473,136,363,60,256,1048,81,1037,59,456,343,18,935,51,1329,1624,1134,189,526,578,190,1635,396,211,583,83,165,89,1073,1251,241,607,833,105,183,300,998,1849,1127,734,325,767,215,1375,326,300,228,246,1221,204,40,718,26,231,31,608,453,11,169,104,380,339,281,23,778,1023,385,251,972,47,101,779,122,471,1003,45,261,1223,493,48,92,948,1269,519,319,720,3,145,1509,24,65,219,25,687,781,465,38,554,244,99,335,55,817,13,1009,319,112,93,537,454,298,393,1217,941,673,356,142,213,140,422,11,1050,143,270,21,58,145,188,486,821,942,1,420,844,451,45,560,701,758,624,149,267,63,286,236,339,120,1145,747,1835,471,540,684,1549,204,285,0,335,387,729,81,17,162,24,25,212,64,1051,373,629,187,34,228,562,362,25,16,475,114,1092,736,1014,896,91,1516,93,210,12,432,88,411,353,638,480,418,1087,45,818,9,606,568,286,507,139,1281,1709,228,510,218,484,498,559,948,88,207,968,84,142,364,44,15,542,133,363,299,753,212,313,277,589,628,821,1481,2,59,18,125,644,324,38,704,559,387,72,36,15,231,647,57,202,1140,311,1125,538,611,192,459,1,598,310,211,406,1868,624,1126,373,369,202,373,1309,903,554,202,259,1174,254,436,997,39,513,811,28,948,434,428,426,419,167,320,748,145,447,735,1128,440,232,211,481,332,591,4,325,875,45,834,269,527,361,603,488,1071,166,1734,326,241,1434,899,738,225,240,1407,6,1197,743,850,25,136,241";
    let mut crabs: Vec<u32> = input.split(',').map(|n| n.parse().unwrap()).collect();
    crabs.sort();

    let min_cost_pt1 = cost_pt1(&crabs, crabs[crabs.len() / 2]);
    let min_cost_pt2 = cost_pt2(&crabs, crabs.iter().sum::<u32>()/crabs.len() as u32);

    println!("min cost 1 {:?}", min_cost_pt1);
    println!("min cost 2 {:?}", min_cost_pt2);

    Ok(())
}

fn cost_pt1(crabs: &[u32], align: u32) -> u32 {
    crabs
        .iter()
        .map(|n| (*n as i32 - align as i32).abs() as u32)
        .sum()
}

fn cost_pt2(crabs: &[u32], align: u32) -> u32 {
    crabs
        .iter()
        .map(|n| {
            let c = (*n as i32 - align as i32).abs() as u32;
            (c * (c + 1)) / 2
        })
        .sum()
}
