'''
Lab 10 starter code

Node and Linked classes for Problem 4
'''
class Node:
    def __init__(self, value):
        self._value = value
        self._next = None
    
    def __str__(self):
        return str(self._value) + " ->  "
    
    def value(self):
        return self._value
    
    def set_value(self, value):
        self._value = value

    def set_next(self, node):
        self._next = node
    
    def next(self):
        return self._next

    def str(self):
        return str(node._value) + " ->  "
        
class LinkedList:
    def __init__(self):
        self._head = None

    def is_empty(self):
        return self._head == None

    def get_head(self):
        return self._head

    # add a node to the head of the list
    def add(self, node):
        node._next = self._head
        self._head = node

    # Problem 4, Step 3
    # make_even: if the LL has an odd number of elements
    #            remove the last one to make it even
    def make_even(self):
        #Your code goes here
        pass #remove this line when you define your code

    def __str__(self):
        string = 'LL -> '
        curr_node = self._head
        while curr_node != None:
            string += str(curr_node)
            curr_node = curr_node.next()
        #string += ']'
        return string

'''
Binary Tree class for Problem 5

'''
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

    def insert_right(self, value):
        if self._right == None:
            self._right = BinaryTree(value)
        else:
            t = BinaryTree(value)
            t._right = self._right
            self._right = t

    def insert_left(self, value):
        if self._left == None:
            self._left = BinaryTree(value)
        else:
            t = BinaryTree(value)
            t._left = self._left
            self._left = t

    def __str__(self):
        if self == None:
            return 'None'
        else:
            return "({:d} {} {})".format(self._value
                , str(self._left), str(self._right))

'''
Problem 5 
'''
# post_order - prints the post-order traversal of the tree bt
#              each value printed on a separate line
def post_order(bt):
    # Your code goes here 
    pass #remove this line when you define your code


'''
Problem 6 
'''
# post_order_v2 - returns a Python list of the values in the tree bt
#                the values are in a post-order traversal of the tree
def post_order_v2(bt):
    # Your code goes here
    pass #remove this line when you define your code


'''
Functions used to test Problems 4 (LL) and 5 & 6 (Trees)

'''

def test_LL_code():

    print("make_even: test empty LL")
    ll = LinkedList()
    print(str(ll))
    ll.make_even()
    print(str(ll))

    print("make_even: test LL with one element")
    ll.add(Node(8))
    print(str(ll))
    ll.make_even()
    print(str(ll))

    print("make_even: test LL with odd number of elements")
    ll = LinkedList()
    ll.add(Node(8))
    ll.add(Node(7))
    ll.add(Node(6))
    ll.add(Node(2))
    ll.add(Node(9))
    print(str(ll))
    ll.make_even()
    print(str(ll))

    print("make_even: test LL with odd number of elements")
    ll.make_even()
    print(str(ll))
    print()

def test_tree_code():
    # make the tree in the diagram
    tree = BinaryTree(6)
    tree.insert_left(3)
    tree.left().insert_left(7)
    tree.left().insert_right(10)

    tree.insert_right(5)
    tree.right().insert_left(23)
    print("post-order traversals will be tested on this tree:")
    print(tree)

    # test version 1
    print("Version 1")
    post_order(tree)

    # test version 2
    print("Version 2")
    print(post_orderv2(tree))

def main():
    print("Testing")
    #Uncomment the lines below to test your code
    #test_LL_code()
    #test_tree_code()

main()
