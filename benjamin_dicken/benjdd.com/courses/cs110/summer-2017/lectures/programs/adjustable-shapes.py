def get_rect(width, height):
    line = '#' * width + '\n'
    rect = line * height
    return rect

def get_triangle(width):
    index = 1
    triangle = ''
    while index <= width:
        triangle = triangle + '*' * index + '\n'
        index = index + 1
    return triangle

def get_tree(width):
    index = 1
    tree = ''
    while index <= width:
        a = width - index
        b = width - a 
        tree += ' ' * int(a/2) + '^' * b + '\n'
        index = index + 2
    return tree

def main():
    r_a = get_rect(7, 10)
    t_a = get_tree(7)
    print(t_a + r_a)
    
    # Could also do this instead:
    # print(get_rect(7, 10) + get_tree(7))

    r_b = get_rect(5,9)
    t_b = get_triangle(5)
    print(t_b + r_b)   
    
    # Could also do this instead:
    # print(get_rect(5, 9) + get_triangle(5))
    
main()