from __future__ import annotations

import numpy as np
from custom_types import *

def load_struct_A() -> Graph:
    graph = Graph()
    graph.add_node(Node(1, Point(0.0, 0.0, 0.0), True))
    graph.add_node(Node(2, Point(1.0, 0.0, 0.0), True))
    graph.add_node(Node(3, Point(1.0, 1.0, 0.0)))
    graph.add_node(Node(4, Point(0.0, 2.0, 0.0)))
    graph.add_node(Node(5, Point(1.0, 2.0, 0.0)))
    graph.add_node(Node(6, Point(0.0, 3.0, 0.0)))
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
    graph.add_node(Node(3, Point(0.0, 1.0)))
    graph.add_node(Node(4, Point(1.0, 1.0)))
    graph.add_node(Node(5, Point(0.0, 2.0)))
    graph.add_node(Node(6, Point(1.0, 2.0)))
    graph.add_node(Node(7, Point(0.0, 3.0)))
    graph.add_node(Node(8, Point(1.0, 3.0)))
    graph.add_node(Node(9, Point(0.0, 4.0)))
    graph.add_node(Node(10, Point(1.0, 4.0)))
    graph.add_node(Node(11, Point(0.0, 5.0)))
    graph.add_node(Node(12, Point(1.0, 5.0)))
    graph.add_node(Node(13, Point(0.0, 6.0)))
    graph.add_node(Node(14, Point(1.0, 6.0)))
    graph.add_node(Node(15, Point(0.0, 7.0)))
    graph.add_node(Node(16, Point(1.0, 7.0)))
    graph.add_node(Node(17, Point(0.0, 8.0)))
    graph.add_node(Node(18, Point(1.0, 8.0)))

    graph.add_adj(1, 3)
    graph.add_adj(2, 4)
    graph.add_adj(3, 4)
    graph.add_adj(3, 5)
    graph.add_adj(4, 6)
    graph.add_adj(5, 6)
    graph.add_adj(5, 7)
    graph.add_adj(6, 8)
    graph.add_adj(7, 8)
    graph.add_adj(7, 9)
    graph.add_adj(8, 10)
    graph.add_adj(9, 10)
    graph.add_adj(9, 11)
    graph.add_adj(10, 12)
    graph.add_adj(11, 12)
    graph.add_adj(11, 13)
    graph.add_adj(12, 14)
    graph.add_adj(13, 14)
    graph.add_adj(13, 15)
    graph.add_adj(14, 16)
    graph.add_adj(15, 16)
    graph.add_adj(15, 17)
    graph.add_adj(16, 18)
    graph.add_adj(17, 18)

    return graph

def load_struct_F() -> Graph:
    graph = Graph()
    graph.add_node(Node(1, Point(0.0, 0.0), True))
    graph.add_node(Node(2, Point(1.0, 0.0), True))
    graph.add_node(Node(3, Point(0.0, 1.0)))
    graph.add_node(Node(4, Point(1.0, 1.0)))
    graph.add_node(Node(5, Point(0.0, 2.0)))
    graph.add_node(Node(6, Point(1.0, 2.0)))
    graph.add_node(Node(7, Point(0.0, 3.0)))
    graph.add_node(Node(8, Point(1.0, 3.0)))
    graph.add_node(Node(9, Point(0.0, 4.0)))
    graph.add_node(Node(10, Point(1.0, 4.0)))
    graph.add_node(Node(11, Point(0.0, 5.0)))
    graph.add_node(Node(12, Point(1.0, 5.0)))
    graph.add_node(Node(13, Point(0.0, 6.0)))
    graph.add_node(Node(14, Point(1.0, 6.0)))
    graph.add_node(Node(15, Point(0.0, 7.0)))
    graph.add_node(Node(16, Point(1.0, 7.0)))
    graph.add_node(Node(17, Point(0.0, 8.0)))
    graph.add_node(Node(18, Point(1.0, 8.0)))

    graph.add_adj(1, 3)
    graph.add_adj(2, 4)
    graph.add_adj(3, 4)
    graph.add_adj(1, 4)
    graph.add_adj(2, 3)

    graph.add_adj(3, 5)
    graph.add_adj(4, 6)
    graph.add_adj(5, 6)
    graph.add_adj(3, 6)
    graph.add_adj(4, 5)

    graph.add_adj(5, 7)
    graph.add_adj(6, 8)
    graph.add_adj(7, 8)
    graph.add_adj(5, 8)
    graph.add_adj(6, 7)

    graph.add_adj(7, 9)
    graph.add_adj(8, 10)
    graph.add_adj(9, 10)
    graph.add_adj(7, 10)
    graph.add_adj(8, 9)

    graph.add_adj(9, 11)
    graph.add_adj(10, 12)
    graph.add_adj(11, 12)
    graph.add_adj(9, 12)
    graph.add_adj(10, 11)

    graph.add_adj(11, 13)
    graph.add_adj(12, 14)
    graph.add_adj(13, 14)
    graph.add_adj(11, 14)
    graph.add_adj(12, 13)

    graph.add_adj(13, 15)
    graph.add_adj(14, 16)
    graph.add_adj(15, 16)
    graph.add_adj(13, 16)
    graph.add_adj(14, 15)

    graph.add_adj(15, 17)
    graph.add_adj(16, 18)
    graph.add_adj(17, 18)
    graph.add_adj(15, 18)
    graph.add_adj(16, 17)

    return graph

def load_triangle() -> Graph:
    graph = Graph()
    graph.add_node(Node(1, Point(0.0, 0.0), True))
    graph.add_node(Node(2, Point(1.0, 0.0), True))
    graph.add_node(Node(3, Point(1.0, 1.0)))

    graph.add_adj(1, 3)
    graph.add_adj(2, 3)
    # graph.add_adj(1, 2)

    return graph


def load_lines() -> Graph:
    graph = Graph()
    graph.add_node(Node(1, Point(0.0, 0.0, 0.0), True))
    graph.add_node(Node(2, Point(1.0, 0.0, 0.0)))
    # graph.add_node(Node(3, Point(1.0, 1.0, 0.0)))
    # graph.add_node(Node(4, Point(0.0, 1.0, 0.0)))
    # graph.add_node(Node(5, Point(-1.0, 1.0, 0.0)))
    # graph.add_node(Node(6, Point(-1.0, 0.0, 0.0)))
    # graph.add_node(Node(7, Point(-1.0, -1.0, 0.0)))
    # graph.add_node(Node(8, Point(0.0, -1.0, 0.0)))
    # graph.add_node(Node(9, Point(1.0, -1.0, 0.0)))
    # graph.add_node(Node(10, Point(1.0, -1.0, 1.0)))
    # graph.add_node(Node(11, Point(0.0, 0.0, 1.0)))

    graph.add_adj(1, 2)
    # graph.add_adj(1, 3)
    # graph.add_adj(1, 4)
    # graph.add_adj(1, 5)
    # graph.add_adj(1, 6)
    # graph.add_adj(1, 7)
    # graph.add_adj(1, 8)
    # graph.add_adj(1, 9)
    # graph.add_adj(1, 10)
    # graph.add_adj(1, 11)

    return graph


def load_pillars() -> Graph:
    graph = Graph()
    graph.add_node(Node(1, Point(0.0, 0.0, 0.0), True))
    graph.add_node(Node(2, Point(0.0, 0.0, 1.0)))
    graph.add_node(Node(3, Point(0.0, 0.0, 2.0)))
    graph.add_node(Node(4, Point(0.0, 0.0, 3.0)))

    graph.add_node(Node(5, Point(1.0, 0.0, 0.0), True))
    graph.add_node(Node(6, Point(2.0, 1.0, 1.0)))
    graph.add_node(Node(7, Point(1.0, 2.0, 2.0)))
    graph.add_node(Node(8, Point(1.0, 1.0, 3.0)))

    graph.add_adj(1, 2)
    graph.add_adj(2, 3)
    graph.add_adj(3, 4)
    
    graph.add_adj(5, 6)
    graph.add_adj(6, 7)
    graph.add_adj(7, 8)

    return graph

def load_horizontal_beam() -> Graph:
    graph = Graph()
    graph.add_node(Node(1, Point(0.0, 0.0), True))
    graph.add_node(Node(2, Point(1.0, 0.0)))
    graph.add_node(Node(3, Point(2.0, 0.0)))
    graph.add_node(Node(4, Point(3.0, 0.0)))

    graph.add_adj(1, 2)
    graph.add_adj(2, 3)
    graph.add_adj(3, 4)
    
    return graph

