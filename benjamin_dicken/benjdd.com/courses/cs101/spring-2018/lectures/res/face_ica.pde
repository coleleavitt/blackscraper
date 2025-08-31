void setup () {
  size(700, 700);
}

void draw() {
  // call drawFace here
}

/*
 * Function to draw a 200x200 square face
 * The first parameter is the X position
 * The second parameter is the Y position
 */
void drawFace(int faceX, int faceY) {
  // head
  fill(130, 100, 80);
  rect(faceX + 0, faceY + 0, 200, 200);
  // eyes
  fill(100, 240, 100);
  ellipse(faceX + 50, faceY + 50, 40, 40);
  ellipse(faceX + 150, faceY + 50, 40, 40);
  // mouth
  fill(200, 100, 100);
  ellipse(faceX + 100, faceY + 150, 100, 30);
  // nose
  fill(80, 60, 40);
  triangle(faceX + 100, faceY + 120,
  faceX + 70, faceY + 120,
  faceX + 100, faceY + 50);
}