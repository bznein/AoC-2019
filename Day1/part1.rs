fn main() {
    let input = "104451
112406
109733
86460
53795
116181
124973
86893
142967
77371
81449
61038
67074
138470
80850
106182
104458
139358
137806
60516
72879
92775
68968
51371
50001
113500
61705
127042
52989
142698
116254
128519
85282
88955
105966
85309
85182
135414
126973
88140
105968
102361
54599
87378
133774
72266
102915
140436
103312
71966
105082
124225
106179
108271
124969
93752
138578
89071
149579
98460
98780
54179
142225
120878
96915
136992
98383
123828
65254
79860
100411
143105
73999
109390
119817
141457
140983
120983
142747
110296
132048
129606
67404
120221
148298
72329
133164
146765
85752
130554
127331
139180
89050
110535
84393
127362
143205
140756
147071
133740";
    let s: Vec<String> = input.split_whitespace().map(String::from).collect();
    let vals : Vec<i32> = s.iter().map(|x| x.parse::<i32>().unwrap()/3-2).collect();
    println!("{:?}",vals.iter().sum::<i32>());
}
