var buf:array [0..280000] of byte;
i,j,k,len,kol:longint;
f:file of byte;
txt:text;
str:string;
bt:array [0..7] of byte=(8,10,9,11,1,7,1,7);

function num(c:longint):string;
var tmp:string;
ii,cc:longint;
begin
tmp:='0000';
cc:=c;
for ii:=4 downto 1 do begin
 tmp[ii]:=chr(ord('0')+cc mod 10);
 cc:=cc div 10;
end;
num:=tmp;
end;

begin
assign(f,'RAVEN.DAT');
reset(f);
BlockRead(f,buf[0],280000,len);
close(f);
kol:=len div $15A;
for i:=0 to kol-1 do begin
 assign(txt,num(i)+'.dat');
 rewrite(txt);
 write(txt,buf[i*$15A],' ',buf[i*$15A+1]);
 close(txt);
 assign(txt,num(i)+'.txt');
 rewrite(txt);
 for j:=0 to 7 do begin
  str:='';
  for k:=1 to buf[i*$15A+j*$2B+2] do begin
   buf[i*$15A+j*$2B+k+2]:=buf[i*$15A+j*$2B+k+2]-bt[j];
   str:=str+chr(buf[i*$15A+j*$2B+k+2]);
  end;
  writeln(txt,str);
 end;
 close(txt);
end;
end.