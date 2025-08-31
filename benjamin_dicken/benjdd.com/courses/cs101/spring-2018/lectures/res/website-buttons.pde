void setup() {
  size(300, 300);
  frameRate(10);
  background(100);
  textSize(24);
}

void draw() {
  fill(255, 100, 25);
  rect(50, 25, 200, 50);
  fill(255, 255, 0);
  rect(50, 125, 200, 50);
  fill(0, 255, 255);
  rect(50, 225, 200, 50);
  fill(0);
  text("Amazon", 110, 60);
  text("Google", 115, 160);
  text("ESPN", 130, 260);
  if(mousePressed) {
    fill(0, 0, 0, 150);
    if (mouseY < 75) {
      if (mouseY > 25) {
        if (mouseX > 50) {
          if (mouseX < 250) {
            link("http://www.amazon.com"); 
            rect(50, 25, 200, 50);
          }
        }
      }
    } else if (mouseY < 175) {
      if (mouseY > 125) {
        if (mouseX > 50) {
          if (mouseX < 250) {
            link("http://www.google.com"); 
            rect(50, 125, 200, 50);
          }
        }
      }
    } else if (mouseY < 275) {
      if (mouseY > 225) {
        if (mouseX > 50) {
          if (mouseX < 250) {
            link("http://www.espn.com");  
            rect(50, 225, 200, 50);
          }
        }
      }
    }
  }
}