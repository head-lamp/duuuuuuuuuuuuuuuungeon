import math
import random
import sys
from pprint import pprint 
import uuid

from scipy.spatial import Delaunay
import numpy as np
import matplotlib.pyplot as plt

#import pyglet 

max_height = 5000
max_width = 5000

#size = width, height = 320, 240

#black = 255, 255, 0
#window = pyglet.window.Window()

cells = 150

class Room:
    def __init__(self, position, width, height, grid):
        self.position = position
        self.x = position[0]
        self.y = position[1]
        self.width = width
        self.height = height
        self.neighbors = list()
        self.isMainRoom = False
        # might need a unqiue id for the graph, idk
        self.id = uuid.uuid4()

    @property
    def left(self):
        return self.x - (self.width/2)

    @property
    def right(self):
        return self.x + (self.width/2)

    @property
    def top(self):
        return self.y + (self.height/2)

    @property
    def bottom(self):
        return self.y - (self.height/2)

    # unused, get distance between two rooms
    #def getDistanceBetweenRoom(self, room):
    #    return math.sqrt((room.x - self.x) ** 2 + (room.y - self.y)**2)

    # push the room in the opposite direction
    def repulse(self, room):
        dx = (self.x - room.x)
        dy = (self.y - room.y)
        self.x += dx + random.randint(-10, 10) * random.randint(-1, -1)
        self.y += dy + random.randint(-10, 10) * random.randint(-1, -1)

    # check if the room is colliding
    def isRoomOverlap(self, room):
        if (self.left > room.right or room.left > self.right):
            return False
        if (self.top < room.bottom or room.top < self.bottom):
            return False
        return True

    # unused, and probably kind of useless
    #def isPointInsideRoom(self, x, y, room):
    #    if (x > room.left) and (x < room.right) and (y < room.top) and (y > room.bottom):
    #        return True
    #    return False

class Dungeon:
    def __init__(self, radius, numOfRooms, maxWidth, maxHeight, grid=None):
        self.rooms = None
        self.maxWidth = maxWidth
        self.maxHeight = maxHeight
        self.generate(radius, numOfRooms)

    @property
    def mainRooms(self):
        return filter(lambda room: room.isMainRoom, self.rooms)

    def generate(self, radius, numOfRooms):
        self.placeRooms(radius, numOfRooms)
        self.pickMainRooms()
        self.generateGraph()
        plot(self)

    # place the rooms and scatter them out
    def placeRooms(self, radius, numOfRooms):
        rooms = []
        for x in range(numOfRooms):
            pos = getRandomPointInCircle(radius)
            random.randint(1,self.maxWidth)
            w = random.randint(1,self.maxWidth)
            h = random.randint(1,self.maxHeight)
            rooms.append(Room(pos, w, h, 4))
        self.rooms = rooms

        # scatter rooms around, check if there are collisions
        # if there are collisions go ahead and scatter em again
        count = 0
        done = False
        while not done:
            #while(collisions != 0):
            count += 1
            done = True
            collisions = self.scatterRooms(self.rooms)
            if collisions > 0:
                done = False
            print(collisions)

        print("coords")
        pprint(list(map(lambda room: (room.x, room.y), rooms)))
        return rooms

    def scatterRooms(self,rooms):
        print("scattering")
        collisions = 0
        for i in rooms:
            for j in rooms:
                if i != j and j.isRoomOverlap(i):
                    collisions += 1
                    j.repulse(i)

        return collisions

    def pickMainRooms(self):
        numOfRooms = len(self.rooms)
        avgW = sum([room.width for room in self.rooms]) / numOfRooms
        avgH = sum([room.height for room in self.rooms]) / numOfRooms
        for room in self.rooms:
            if room.width > avgW and room.height > avgH:
                room.isMainRoom = True
        print("main room coords")
        pprint([room.position for room in self.mainRooms])

    def generateGraph(self):
        points = np.array([room.position for room in self.mainRooms])
        tri = Delaunay(points)
        tri_edges = points[tri.simplices]
        graph = {node.position:{'id': node.id, 'edges':set()} for node in self.mainRooms}
        edges = []
        for edge_group in tri_edges:
            print('edge groups')
            print(edge_group)
            for edge in edge_group:
                print('edge')
                node = graph[tuple(edge)]
                node['edges'].update( (tuple(x) for x in tuple(edge_group)))
        pprint(graph)


def roundm(n, m): return math.floor(((n + m - 1) / m)) * m

def getRandomPointInCircle(radius, tile_size=4):
    t = 2 * math.pi * random.random()
    u = random.random() + random.random()
    r = 0
    if u > 1:
        r = 2 - u
    else:
        r = u
    return roundm(radius * r * math.cos(t), tile_size), \
         roundm(radius * r * math.sin(t), tile_size)

def plot(dungeon):
    points = np.array([ room.position for room in dungeon.mainRooms])
    plt.scatter([x[0] for x in points], [y[1] for y in points])
    #plt.show()

def main():
    numOfRooms = 20
    radius = 80
    
    dungeon = Dungeon(radius, numOfRooms, 64, 64)

if __name__ == '__main__':
    main()
