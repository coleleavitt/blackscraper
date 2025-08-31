#Lab 7 solutions

#Prob 1
def reverse_string(s):
    if len(s) == 0:
        return ""   #fixed by returning empty string
    return s[-1] + reverse_string(s[:-1])

'''
Problem 1, Step 2. Write out the arguments and return values
for the call reverse_string("beak"):

beak
bea
be
b
""
""
b
eb
aeb
kaeb


'''

#Prob 2
def count_occurrences(alist, value):

    if len(alist) == 0:
        return 0
    
    if alist[0] == value:
        return 1 + count_occurrences(alist[1:], value)
    else:
        return count_occurrences(alist[1:], value)

#Prob 3
def times_pos(alist):
    return times_pos_helper(alist,0)

def times_pos_helper(alist, pos):
    if len(alist) == 0:
        return []
    else:
        return [alist[0] * pos] + times_pos_helper(alist[1:], pos + 1)

print(reverse_string("pumpkin"))
print(times_pos([2,4,6,8,10]))
print(count_occurrences([2, 8, 2, 6, 2, 9], 2))
print(count_occurrences([2, 8, 2, 6, 2, 9], 5))


#Lab 7 starter code
#Prob 4
#Node class
class Node:
    def __init__(self, value):
        self._value = value
        self._inner_list = LinkedList()
        self._next = None
    
    def value(self):
        return self._value

    def get_inner_list(self):
        return self._inner_list
    
    def next(self):
        return self._next

    def __str__(self): 
       if self._next is None:
           ending = ""
       else:
           ending = " -> "
       if self._inner_list.is_empty():
           return str(self._value) + " [empty llist] " + ending
       else:
           return str(self._value) + " [" + str(self._inner_list) + "] " + ending

        
#Linked list class
class LinkedList:
    def __init__(self):
        self._head = None

    def is_empty(self):
        return self._head == None

    # add a node to the head of the list
    def add(self, node):
        node._next = self._head
        self._head = node

    
    def __str__(self):
        string = 'List -> '
        curr_node = self._head
        while curr_node != None:
            string += str(curr_node)
            curr_node = curr_node.next()
        return string

def main():

    # create the outer list
    my_ll = LinkedList()

    # create a node
    n = Node(4)

    # add a node to n's inner list
    n.get_inner_list().add(Node(2))

    # add n to the outer list
    my_ll.add(n)

    print(my_ll)

    # create another node
    n = Node(8)

    # add a node to n's inner list
    n.get_inner_list().add(Node(3))

    # add n to the outer list
    my_ll.add(n)

    print(my_ll)

    # Step 4 (a)
    # create another node n with value 7
    n = Node(7)
    
    # Step 4 (b)
    # add a node to its inner list with value 5
    n.get_inner_list().add(Node(5))

    # Step 4 (c)
    # add n to the outer list
    my_ll.add(n)

    # Step 4 (d)
    print(my_ll)

main()
