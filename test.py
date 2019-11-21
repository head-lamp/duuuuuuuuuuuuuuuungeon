import numpy as np
from scipy.sparse import csgraph, csr_matrix
from scipy.spatial import Delaunay
from scipy.sparse.csgraph import minimum_spanning_tree
coords = [(-61, 125),
 (52, 16),
 (2, -58),
 (-55, 25),
 (-76, -49),
 (55, -71),
 (79, 59),
 (-102, -107),
 (158, 101),
 (21, 48),
 (32, 75),
 (111, 25),
 (119, -66),
 (25, -32),
 (72, 76),
 (49, 131),
 (-36, -53),
 (7, -115),
 (65, 44),
 (-76, 63)]

points = np.array(coords)
tri = Delaunay(points)
print(tri.simplices)
# mtx = csr_matrix(tri)

# i need to follow this to figure it out 
# https://www.geeksforgeeks.org/kruskals-minimum-spanning-tree-algorithm-greedy-algo-2/
#Tcsr = minimum_spanning_tree(tri)

# tri_edges = points[tri.simplices]
# print(tri_edges)
