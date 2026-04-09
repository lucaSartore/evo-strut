from __future__ import annotations

from dataclasses import dataclass
import numpy as np
type NodeId = int


@dataclass
class Settings:
    area: float = 4
    e_mod: float = 3000
    g_mod: float = 1000
    jxx: float = 6
    iy: float = 3
    iz: float = 3


class Point:
    def __init__(self, x: float, y: float, z: float) -> None:
        self.x = x
        self.y = y
        self.z = z

    def as_list(self) -> list[float]:
        return [self.x, self.y, self.z]

    def __add__(self, other: Point) -> Point:
        return Point(self.x + other.x, self.y + other.y, self.z + other.z)

    def __sub__(self, other: Point) -> Point:
        return Point(self.x - other.x, self.y - other.y, self.z + other.z)

    def abs(self) -> float:
        return (self.x ** 2 + self.y ** 2 + self.z ** 2) ** 0.5

    @staticmethod
    def distance(a: Point, b: Point) -> float:
        return (a-b).abs()

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


# 6 by 6 matrix of the stiffness of each component
type Stiffness = np.ndarray

# associate a stiffness measure to every node
type StiffnessResult = dict[NodeId, Stiffness]

