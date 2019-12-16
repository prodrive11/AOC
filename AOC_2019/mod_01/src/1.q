// q src/1.q
p:{[p;r;dur]
 -1 "P[",p,"] Res: ",.Q.s1 r;
 -1 "P[",p,"] Duration: ", string dur; }; print1:p["1"]; print2:p["2"];
 
t0:.z.p;
v: "J"$ read0 `:input/1;
r:sum floor[v%3]-2;
print1[r;.z.p-t0];


t0:.z.p;
r:sum (+/') 1_'-1_'{floor[x%3]-2}\'[{x>0};v];
print2[r;.z.p-t0];


/

P[1] Res: 3353880
P[1] Duration: 0D00:00:00.000225000
P[2] Res: 5027950
P[2] Duration: 0D00:00:00.002232000
