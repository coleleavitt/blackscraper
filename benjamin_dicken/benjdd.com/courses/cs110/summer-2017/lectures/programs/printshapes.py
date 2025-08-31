# Draws the top half of an an egg figure.
def egg_top():
    print("  ______")
    print(" /      \\")
    print("/        \\")
    
# Draws the bottom half of an egg figure.
def egg_bottom():
    print("\\        /")
    print(" \\______/")
    
# Draws a complete egg figure.
def egg():
    egg_top()
    egg_bottom()
    print()

# Draws a teacup figure.
def tea_cup():
    egg_bottom()
    line()
    print()
    
# Draws a stop sign figure.
def stop_sign():
    egg_top()
    print("|  STOP  |")
    egg_bottom()
    print()

# Draws a figure that looks sort of like a hat.
def hat():
    egg_top()
    line()
    
# Draws a line of dashes.
def line():
    print("+--------+")

def main():
    egg()
    tea_cup()
    stop_sign()
    hat()
    
main()