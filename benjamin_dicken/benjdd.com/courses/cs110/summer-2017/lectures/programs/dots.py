from turtle import *

setup(width=400, height=400, startx=50, starty=50)

up()
goto(-50, 50)
down()

dot_distance = 25
width = 5
height = 7

penup()

for y in range(height):
    for i in range(width):
        dot()
        forward(dot_distance)
    backward(dot_distance * width)
    right(90)
    forward(dot_distance)
    left(90)

done()
