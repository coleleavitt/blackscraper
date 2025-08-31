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
}

