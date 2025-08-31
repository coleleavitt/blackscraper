
box_width = 6
box_height = 1
    
def print_generic_box():
    print("-" * box_width)
    nw = box_width - 2
    mid = "|" + " " * nw + "|\n"
    print(mid * box_height, end="")
    print("-" * box_width)
    
print_generic_box()

box_width = 9
box_height = 2
print_generic_box()

box_width = 13
box_height = 3
print_generic_box()
