from __future__ import annotations

import numpy as np
from custom_types import *

def load_struct_A() -> Graph:
    graph = Graph()
    graph.add_node(Node(1, Point(0.0, 0.0), True))
    graph.add_node(Node(2, Point(1.0, 0.0), True))
    graph.add_node(Node(3, Point(1.0, 1.0)))
    graph.add_node(Node(4, Point(0.0, 2.0)))
    graph.add_node(Node(5, Point(1.0, 2.0)))
    graph.add_node(Node(6, Point(0.0, 3.0)))
    graph.add_adj(1, 4)
    graph.add_adj(2, 3)
    graph.add_adj(3, 4)
    graph.add_adj(3, 5)
    graph.add_adj(4, 6)
    graph.add_adj(5, 6)

    return graph


def load_struct_B() -> Graph:
    graph = Graph()
    graph.add_node(Node(1, Point(0.0, 0.0), True))
    graph.add_node(Node(2, Point(1.0, 0.0), True))
    graph.add_node(Node(3, Point(0.0, 1.0)))
    graph.add_node(Node(4, Point(1.0, 2.0)))
    graph.add_node(Node(5, Point(0.0, 3.0)))
    graph.add_adj(1, 3)
    graph.add_adj(2, 3)
    graph.add_adj(2, 4)
    graph.add_adj(3, 4)
    graph.add_adj(3, 5)
    graph.add_adj(4, 5)

    return graph

def load_struct_C() -> Graph:
    graph = Graph()
    graph.add_node(Node(1, Point(0.0, 0.0), True))
    graph.add_node(Node(2, Point(1.0, 0.0), True))
    graph.add_node(Node(3, Point(1.0, 1.0)))
    graph.add_node(Node(4, Point(0.0, 2.0)))
    graph.add_node(Node(5, Point(1.0, 3.0)))
    graph.add_node(Node(6, Point(0.0, 4.0)))
    graph.add_adj(1, 4)
    graph.add_adj(2, 3)
    graph.add_adj(3, 4)
    graph.add_adj(3, 5)
    graph.add_adj(4, 5)
    graph.add_adj(4, 6)

    return graph

def load_struct_D() -> Graph:
    graph = Graph()
    graph.add_node(Node(1, Point(0.0, 0.0), True))
    graph.add_node(Node(2, Point(1.0, 0.0), True))
    graph.add_node(Node(3, Point(2.0, 0.0), True))
    graph.add_node(Node(4, Point(2.0, 2.0), False))
    graph.add_node(Node(5, Point(1.0, 1.0), False))
    graph.add_node(Node(6, Point(0.0, 2.0), False))
    graph.add_node(Node(7, Point(0.0, 4.0), False))
    graph.add_node(Node(8, Point(1.0, 3.0), False))
    graph.add_node(Node(9, Point(2.0, 4.0), False))

    graph.add_adj(1, 6)
    graph.add_adj(2, 5)
    graph.add_adj(3, 4)
    graph.add_adj(5, 4)
    graph.add_adj(5, 6)
    graph.add_adj(5, 8)
    graph.add_adj(6, 7)
    graph.add_adj(4, 9)
    graph.add_adj(8, 7)
    graph.add_adj(8, 9)

    return graph


def load_struct_E() -> Graph:
    graph = Graph()
    graph.add_node(Node(1, Point(0.0, 0.0), True))
    graph.add_node(Node(2, Point(1.0, 0.0), True))
    graph.add_node(Node(3, Point(2.0, 1.0)))

    graph.add_adj(1, 3)
    graph.add_adj(2, 3)

    return graph
