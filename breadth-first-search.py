from collections import deque
from msilib import sequence
from pickle import PERSID

graph = {}
graph["you"] = ["alice", "bob", "claire"]
graph["bob"] = ["anuj", "peggy"]
graph["alice"] = ["peggy"]
graph["claire"] = ["thom", "jonny"]
graph["anuj"] = []
graph["peggy"] = []
graph["thom"] = []
graph["jonny"] = []

# O(V+E) V for number of vertices, E for number of edges
def beadth_first_search(name):
    search_queue = deque()
    search_queue += graph[name]
    searched = [] # keep track of which people you've searched before
    while search_queue:
        person = search_queue.popleft()
        if not person in searched:  # only search this person if you haven't searched them
            if person_is_seller(person):
                print(person + " is a mango seller!")
                return True
            else:
                search_queue += graph[person]
                searched.append(person)
        
    return False

def person_is_seller(name):
    return name[-1] == 'm'

beadth_first_search('you')