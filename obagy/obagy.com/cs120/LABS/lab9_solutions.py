#CSC 120: Lab 9 answers

#Prob 1    
def is_palindrome(astr):
    # Normalize the string by removing spaces and converting to lowercase
    normalized_s = ''.join(astr.split()).lower()
    
    # Create a stack and push all characters onto it
    stack = Stack()
    for char in normalized_s:
        stack.push(char)

    # Check if the string reads the same forward and backward
    for char in normalized_s:
        if char != stack.pop():
            return False
    
    return True


#Prob 2
def flatten_to_string(alist):
    if not alist:
        return ""

    # Check if the first element is a list
    if type(alist[0]) == list:
        return flatten_to_string(alist[0]) + flatten_to_string(alist[1:])  
    else:
        return str(alist[0]) + flatten_to_string(alist[1:]) 

#Prob 3
#   a) First In, First Out
#   b) Last In, First Out
#   c) False
#   d) False

#Prob 4
def remove_value(self, value):
    if self._head == None:
        return None
    current = self._head

    #remove first node if its value 
    #attribute equals value
    if current._value == value:
        self._head = current._next
        # be sure to set the _next attribute to None, otherwise
        # the node still refers to the next node in the LL
        current._next = None       
        return

    prev = current
    current = current._next
    while current != None:
        if current._value == value:
            prev._next = current._next
            # same comment as above
            current._next = None       
            return
        prev = current
        current = current._next 

#Prob 5
def tree_sum(tree):
    if tree == None:
        return 0
    if tree.value() % 2 == 1:
        return tree.value() + tree_sum(tree.left()) + tree_sum(tree.right())
    else:
        return tree_sum(tree.left()) + tree_sum(tree.right())
               
#Prob 6
"""

           3
          / \
        6    5
       /       \
      4         8
               / \
              9   2

"""


#Example calls to solutions
#Prob 1 - needs the Stack class
class Stack:
    def __init__(self):
        self.items = []

    def push(self, item):
        self.items.append(item)

    def pop(self):
        if not self.is_empty():
            return self.items.pop()
        return None

    def is_empty(self):
        return len(self.items) == 0

print(is_palindrome("A man a plan a canal Panama"))   # Output: True
print(is_palindrome("Radar"))                         # Output: True
print(is_palindrome("hello"))                         # Output: False

#Prob 2
nested_list = [1, ['a', 'b'], 2, ['c', [3, 4]], 'd']
result = flatten_to_string(nested_list)
print(result)                                         # Output: "1ab2c34d"

#Prob 4 Needs the LinkedList class
class LinkedList:
    def __init__(self):
        self._head = None

    def add(self,new):
        new._next = self._head
        self._head = new

    def remove_value(self, value):
        if self._head == None:
            return None
        current = self._head

        #remove first node if its value 
        #attribute equals value
        if current._value == value:
            self._head = current._next
            current._next = None
            return

        prev = current
        current = current._next
        while current != None:
            if current._value == value:
                prev._next = current._next
                current._next = None
                return
            prev = current
            current = current._next 
            
    def __str__(self):
        string = 'LList -> '
        current = self._head
        while current != None:
            string += str(current)
            current = current._next
        return string
        
class Node:
    def __init__(self,value):
        self._value = value
        self._next = None

    def __str__(self):
        if self._next == None:
            nxt = "None"
        else:
            nxt = "->"
        return " |" + str(self._value) + "|:" + nxt


# make a linked list with four elements
my_ll = LinkedList()
my_ll.add(Node(2))
my_ll.add(Node(4))
my_ll.add(Node(4))
my_ll.add(Node(6))

#print the linked list
print(my_ll)

my_ll.remove_value(4)
print("removed 4")
print(my_ll)
print("removed non-existant value")
my_ll.remove_value(200)
print(my_ll)
my_ll.remove_value(6)
print("removed 6")
print(my_ll)
my_ll.remove_value(2)
print("removed 2")
print(my_ll)

ll2 = LinkedList()
print(ll2)
print("remove from empty list ")
ll2.remove_value(200)
print("now add 4")
ll2.add(Node(4))
print(ll2)
print("now remove 4")
ll2.remove_value(4)
print(ll2)



