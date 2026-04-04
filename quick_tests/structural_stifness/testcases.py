from __future__ import annotations

import numpy as np
from custom_types import *

def load_struct_A() -> tuple[Graph, StiffnessResult]:
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

    inertia = {
        1: np.array([[0.0, 0.0], [0.0, 0.0]]),
        2: np.array([[0.0, 0.0], [0.0, 0.0]]),
        3: np.array([[0.1, 0.0], [0.0, 0.1]]),
        4: np.array([[0.1, 0.0], [0.0, 0.1]]),
        5: np.array([[0.1, 0.0], [0.0, 0.1]]),
        6: np.array([[0.1, 0.0], [0.0, 0.1]]),
    }
    return graph, inertia


def load_struct_B() -> tuple[Graph, StiffnessResult]:
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

    inertia = {
        1: np.array([[0.0, 0.0], [0.0, 0.0]]),
        2: np.array([[0.0, 0.0], [0.0, 0.0]]),
        3: np.array([[0.1, 0.0], [0.0, 0.1]]),
        4: np.array([[0.1, 0.0], [0.0, 0.1]]),
        5: np.array([[0.1, 0.0], [0.0, 0.1]]),
    }
    return graph, inertia

def load_struct_C() -> tuple[Graph, StiffnessResult]:
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

    inertia = {
        1: np.array([[0.0, 0.0], [0.0, 0.0]]),
        2: np.array([[0.0, 0.0], [0.0, 0.0]]),
        3: np.array([[0.1, 0.0], [0.0, 0.1]]),
        4: np.array([[0.1, 0.0], [0.0, 0.1]]),
        5: np.array([[0.1, 0.0], [0.0, 0.1]]),
        6: np.array([[0.1, 0.0], [0.0, 0.1]]),
    }
    return graph, inertia

def load_struct_D() -> tuple[Graph, StiffnessResult]:
    graph = Graph()
    graph.add_node(Node(1, Point(0.0, 0.0), True))
    graph.add_node(Node(2, Point(1.0, 0.0), True))
    graph.add_node(Node(3, Point(2.0, 0.0), True))
    graph.add_node(Node(4, Point(2.0, 2.0), True))
    graph.add_node(Node(5, Point(1.0, 1.0), True))
    graph.add_node(Node(6, Point(0.0, 2.0), True))
    graph.add_node(Node(7, Point(0.0, 4.0), True))
    graph.add_node(Node(8, Point(1.0, 3.0), True))
    graph.add_node(Node(9, Point(2.0, 4.0), True))

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

    inertia = {
        1: np.array([[0.0, 0.0], [0.0, 0.0]]),
        2: np.array([[0.0, 0.0], [0.0, 0.0]]),
        3: np.array([[0.0, 0.0], [0.0, 0.0]]),
        4: np.array([[0.1, 0.0], [0.0, 0.1]]),
        5: np.array([[0.1, 0.0], [0.0, 0.1]]),
        6: np.array([[0.1, 0.0], [0.0, 0.1]]),
        7: np.array([[0.1, 0.0], [0.0, 0.1]]),
        8: np.array([[0.1, 0.0], [0.0, 0.1]]),
        9: np.array([[0.1, 0.0], [0.0, 0.1]]),
    }
    return graph, inertia
