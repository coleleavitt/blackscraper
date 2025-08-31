#CSC 120: Lab 5 Solutions

#Prob 1       
def slice_and_reverse(input_string):
    if len(input_string) < 5:
        return "Input string is too short."
    
    first_three = input_string[:3]
    last_two = input_string[-2:]
    return (first_three + last_two)[::-1]

#Prob 2
def every_other(alist):
    new_list = []
    for i in range(0,len(alist),2):
        new_list.append(alist[i])
    return new_list

#Prob 3
def print_diag_pairs(grid):
    for i in range(len(grid)):
        if i <= (len(grid) - 2):
            print(grid[i][i], grid[i][i+1])
        else:
            print(grid[i][i])

#Prob 4 a) lists, sets, dictionaries
#       b) a string
#       c) a memory address (it represents where the object is in memory)
#       d) the return values are:
#          True
#          False
#          None


def find_sentence(sentence):
  if sentence[0].isupper() and sentence[-1] == ".":
    return True
  if sentence[0].islower() or sentence[-1] != ".":
    return False

#Prob 5
def count_words(filename):
    file = open(filename)
    num_words = 0
    for line in file:
        if line[0] == "#":
            continue
        sline = line.split()
        for word in sline:
            if len(word) > 2:
                num_words += 1
    return num_words

#Prob 6
class Rectangle:
    def __init__(self, w, h):
        self._width = w
        self._height = h

    def get_width(self):
        return self._width

    def get_height(self):
        return self._height

    def __str__(self):
        return "Rectangle({}, {})".format(self._width, self._height)

    def __eq__(self, other):
        return  self._width == other.get_width() and \
                        self._height == other.get_height()

grid = [[ 2, 6, 3, 4 ],
        [ 8, 9, 5, 12],
        [ 6, 4, 2, 7 ],
        [ 9, 5, 3, 10]]

print_diag_pairs(grid)
print(slice_and_reverse('abcdefxy'))
print(every_other(["sun","moons","stars","planets","asteroids"]))
print(count_words("poem.txt"))
print(find_sentence("The cat is cute."))
print(find_sentence("dog"))
print(find_sentence("123."))

r1 = Rectangle(3,5)
r2 = Rectangle(8,7)
print(r1)
print(r2)
print(r1 == r2)

