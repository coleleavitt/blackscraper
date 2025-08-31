void setup () {
  size(400, 400);
}

void draw() {
  drawHouse();
}

void drawHouse() {
  strokeWeight(2);
  fill(50, 200, 255);
  triangle(200, 100, 100, 150, 300, 150);
  rect(100, 150, 200, 100);
  rect(150, 175, 50, 75);
}