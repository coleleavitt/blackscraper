from turtle import *

speed(20)

for i in range(180):
    forward(100)
    right(30)
    forward(20)
    left(60)
    forward(50)
    right(30)

    penup()
    setposition(0, 0)
    pendown()

    right(2)

done()
