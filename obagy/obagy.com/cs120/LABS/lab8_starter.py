#Lab 8 starter code
#Prob 4
class BinaryTree:
    def __init__(self,value):
        self._value = value
        self._left = None
        self._right = None

    def value(self):
        return self._value
    
    def left(self):
        return self._left
    
    def right(self):
        return self._right

    def set_left(self, bt):
        self._left = bt

    def set_right(self, bt):
        self._right = bt

    def __str__(self):
        if self == None:
            return 'None'
        else:
            return "({:d} {} {})".format(self._value
                , str(self._left), str(self._right))

def insert(tree, value):
    if tree == None:
        return BinaryTree(value)
    #assumes no duplicates
    if value < tree.value():
        bt = insert(tree.left(), value)
        tree.set_left(bt)
    elif value > tree.value():
        bt = insert(tree.right(), value)
        tree.set_right(bt)
    return tree

def main():

    # Below is the call to create the root node
    # of the binary search tree from problem 4:
    t = insert(None, 8)
    print(t)

    # Step 3: make the calls to insert for the
    #         remaining values: 10, 5, 20, 4, and 9
    #         print the tree after each call to insert

main()
