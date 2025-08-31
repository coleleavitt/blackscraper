void setup() {
  size(500, 200);
  textSize(50);
  frameRate(1);
}

void draw() {
  background(100);
  // Generate a random number
  float randomNumber = random(0, 10000);
  // Convert the random number to int to get rid of the decimal places
  int randInt = int(randomNumber);
  // display the number
  text("random: " + randInt, 10, 100);
}