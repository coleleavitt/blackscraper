#Lab 8 solutions

#Prob 1
def first_matches_last(slist):
    if slist == []:
        return []
    else:
        s = slist[0]
        if s[0] == s[-1]:
            return [slist[0]] + first_matches_last(slist[1:])
        else:
            return first_matches_last(slist[1:])

#Prob 2
def display(alist):
    if alist == []:
        return
    print("-" * len(alist[0]))
    helper(alist)
    print("-" * len(alist[0]))

def helper(alist):
    if alist == []:
        return
    else:
        print(alist[0])
        helper(alist[1:])

#Prob 3
def expand(my_str, size):
    s_len = len(my_str)
    return my_str * (size//s_len) + my_str[:size % s_len]


print(first_matches_last(["abba", "nope", "kook", "hello", "bob"]))
display(["   *   ", "  ***  "," ***** ","*******"])          
print(expand("ab",7))
print(expand("cat",7))
print(expand("boat",7))


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

#Calls to make a tree
# Below is the call to create the root node
# of the binary search tree from problem 4:
t = insert(None, 8)
print(t)
# make the calls to insert for the
# remaining values: 10, 5, 20, 4
# print the tree after each call to insert
insert(t,10)
print(t)
insert(t,5)
print(t) 
insert(t,20)
print(t)
insert(t,4)
print(t)
insert(t,9)
print(t)
