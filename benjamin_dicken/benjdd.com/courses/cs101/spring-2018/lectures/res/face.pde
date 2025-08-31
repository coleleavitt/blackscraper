void drawAxisLines() {
  stroke(255, 0, 0);
  line(-1000, 0, 0, 1000, 0, 0);
  stroke(0, 255, 0);
  line(0, -1000, 0, 0, 1000, 0);
  stroke(0, 0, 255);
  line(0, 0, -1000, 0, 0, 1000);
  stroke(0);
}

void setup() {
  size(600,600,P3D);
  frameRate(24);
}

void draw() {
  background(100);
  drawAxisLines();
  
  translate(100, 100, -200);
  
  fill(150, 100, 0);
  box(200);
  fill(100, 200, 200);
  translate(-50, -50, 100);
  box(50);
  translate(100, 0, 0);
  box(50);
  fill(200, 200, 0);
  translate(-50, 100, 0);
  box(150, 50, 50);
}