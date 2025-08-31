from turtle import *

setup(width=400, height=400, startx=50, starty=50)

up()
goto(-100, 0)
down()

color('red', 'yellow')
begin_fill()
for i in range(0, 36):
    forward(200)
    left(170)
end_fill()
done()
