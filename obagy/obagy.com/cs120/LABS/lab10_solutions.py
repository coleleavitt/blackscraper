'''
Problem 1

special method
  - In Python, special methods are methods whose names start with double
    underscores. These methods allow you to customize the behavior of the
    class objects so that they work with built-in Python operations and functions.
    For example, if you define __eq__() for a class, you can then use the
    "==" operator on objects of the class.
  
abstract data type (also name at least two examples)
  - An abstract data type (ADT) describes a set of data values and associated 
    operations that are specified independent of any particular implementation
    Two examples are a Stack and a Linked List.
    (Note: any built-in data type in Python in an ADT.)

a recursive function
  - A recursive function is a function that is defined in terms of itself.

binary search tree
  - A binary search tree is a binary tree that has the property that for every
    node in the tree, the values in the nodes in the left-subtree are less than
    the node's value, and the values in the nodes in the right-subtree are greater
    than the node's value.

black box testing
  - Writing tests according to a specification to determine if the program meets
    the requirements outlined in the specification. (You do not have access to the
    source code.)

white box testing
  - Writing tests to test a small unit of code, such as a single method or function.
    You have access to the code and you write function or method calls with various
    arguments to test the code. 

edge case
  - A test case that is on the outer range of valid values. This usually means the 
    smallest values that would not cause an error, or the largest values that would
    not cause an error.

In the Python method astr.split(), if no argument is given, the method 
splits the string astr on whitespace. Define whitespace.
  - Whitespace is a space, a tab, or a newline.

Problem 2
a) O(n)
   To determine the index of an element, the method has to find the element in the list.
   Since in the worst case it could be the last element of the list, the method has
   to potentially look at n elements, where n is length of the list.
b) alist = [2,4,6,8,10]
   alist.index(10)

Problem 3
O(n)
O(n)
The code in b) is sequenial and we use the big-O property that if the code is sequential, we use the max of the complexitites.

'''

'''
Node and Linked classes for Problem 4
Step 1 a): Ask if the linked list is emtpy, has one element, or has many elements.
Step 1 b): Yes, you need a loop. You must determine if the list has an even or odd number
           of elements.
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

    # Problem 4
    # make_even: if the LL has an odd number of elements
    #            remove the last one to make it even
    def make_even(self):
        if self._head == None:   #the list is empty
            return None
        curr = self._head
        if curr._next == None:   #the list has only one element
            self._head = None
            return None

        count = 2
        prev = curr             
        curr = curr._next
        while curr._next != None:
            prev = curr
            curr = curr._next
            count += 1

        # remove last node if count is odd
        if count % 2 == 1:
            prev._next = None       


    def __str__(self):
        string = 'LL -> '
        curr_node = self._head
        while curr_node != None:
            string += str(curr_node)
            curr_node = curr_node.next()
        #string += ']'
        return string

Step 5: What is the complexity of your make_even()?
 O(n) - There is one loop with a constant amount of work in the loop body.`
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
Problem 5 code solutions
'''
# post_order - prints the post-order traversal of the tree bt
#              each value printed on a separate line
def post_order(bt):
    if bt == None:
        return
    else:
        post_order(bt.left())
        post_order(bt.right())
        print(bt.value())


# post_order - returns a Python list of the values in the tree bt
#              the values are in a post-order traversal of the tree
def post_orderv2(bt):
    if bt == None:
        return []
    else:
        return post_orderv2(bt.left()) + post_orderv2(bt.right()) + [bt.value()]


'''
Functions used to test Problems 3 (LL) and 4 (Trees)

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
    test_LL_code()
    test_tree_code()
main()
