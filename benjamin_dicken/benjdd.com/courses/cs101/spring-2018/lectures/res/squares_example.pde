void setup  () { size(200, 200) ; }
void   draw ( ) {
background(  100, 200, 250);
fill(0, 0, 255);
if (mousePressed) {
for (int i =       0; i < 10   ; i += 1) {
rect(i*20, 20, 15, 15);
if (mouseX > 100) {
fill(100, 200, 255);
}  
}
if (    mouseButton == RIGHT) {
background(0 , 0    ,  0  );
strokeWeight(7);
}
fill(255, 0, 0);
}
rect(50, 50,  100, 100  ) ;
}