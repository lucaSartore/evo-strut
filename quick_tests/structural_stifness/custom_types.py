from __future__ import annotations

import numpy as np
type NodeId = int

class Point:
    def __init__(self, x: float, y: float) -> None:
        self.x = x
        self.y = y

class Node:
    def __eq__(self, value: object, /) -> bool:
        if type(value) != Node:
            return False
        return self.id == value.id

    def __init__(self, id: NodeId, position: Point, ground_node: bool = False):
        self.id = id
        self.adj: list[Node] = []
        self.position = position
        self.ground_node = ground_node

# A graph is a 2D structure of nodes connected by some junctures
class Graph:
    def __init__(self):
        self.nodes: dict[NodeId, Node] = {}

    def add_node(self, node: Node):
        self.nodes[node.id] = node

    def add_adj(self, a: NodeId, b: NodeId):
        self.nodes[a].adj.append(self.nodes[b])
        self.nodes[b].adj.append(self.nodes[a])


# two by two matrix on the form
# [[ kxx, kxy ]
#  [ kyx, kyy ]]
# were unit of measure is N/m
type Stiffness = np.ndarray

# associate a stiffness measure to every node
type StiffnessResult = dict[NodeId, Stiffness]

