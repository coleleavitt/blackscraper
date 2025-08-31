/**
 * Author: Benjamin Dicken
 * Description: A program that searches through a string.
 */

void setup() {
  size(700, 300);
  textSize(40);
}

// The content that we will be searching through
String content = "The quick brown fox\njumped over the lazy dog\non a cloudy day";
// This will store the input that the user types
String input = "";
// Will be "yes" or "no" depending on if the search term was found 
String found = "?";

void draw() {
  
  // Do the search.
  // Reset the found variable to "no" at beginning.
  // Change to "yes" if the string is found while looping.
  found = "no";
  for(int i = 0; i <= content.length() - input.length(); i=i+1) {
    String compare = content.substring(i, i+input.length());
    if (compare.equals(input)) {
      found = "yes";
    }
  }
  
  // Draw the messages to the canvas
  background(100);
  fill(0, 255, 255);
  text("Search: " + input, 10, 50);
  text("Found: " + found, 400, 50);
  fill(200, 255, 200);
  text(content, 10, 130);
}

/** 
 * Save all of the characters that the user types into a string.
 * If the user types ENTER ('\n') then clear the input string.
 */
void keyPressed() {
  if (key == '\n') {
    input = "";
  } else {
    input = input + key;
  }
}