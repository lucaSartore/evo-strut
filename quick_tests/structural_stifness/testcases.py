from __future__ import annotations

import numpy as np
from custom_types import *

def load_line_beam() -> tuple[Graph, StiffnessResult]:
    graph = Graph()
    graph.add_node(Node(1, Point(0.0, 0.0)))
    graph.add_node(Node(2, Point(1.0, 0.0)))
    graph.add_node(Node(3, Point(2.0, 0.0)))
    graph.add_adj(1, 2)
    graph.add_adj(2, 3)

    inertia = {
        1: np.array([[1.0, 0.0], [0.0, 0.5]]),
        2: np.array([[1.5, 0.2], [0.2, 1.2]]),
        3: np.array([[1.0, 0.0], [0.0, 0.7]]),
    }
    return graph, inertia


def load_triangle_frame() -> tuple[Graph, StiffnessResult]:
    graph = Graph()
    graph.add_node(Node(1, Point(0.0, 0.0)))
    graph.add_node(Node(2, Point(1.0, 0.0)))
    graph.add_node(Node(3, Point(0.5, 0.86)))
    graph.add_adj(1, 2)
    graph.add_adj(2, 3)
    graph.add_adj(3, 1)

    inertia = {
        1: np.array([[0.9, -0.1], [-0.1, 0.9]]),
        2: np.array([[1.2, 0.0], [0.0, 0.8]]),
        3: np.array([[1.0, 0.3], [0.3, 1.4]]),
    }
    return graph, inertia


def load_square_frame() -> tuple[Graph, StiffnessResult]:
    graph = Graph()
    graph.add_node(Node(1, Point(0.0, 0.0)))
    graph.add_node(Node(2, Point(1.0, 0.0)))
    graph.add_node(Node(3, Point(1.0, 1.0)))
    graph.add_node(Node(4, Point(0.0, 1.0)))
    graph.add_adj(1, 2)
    graph.add_adj(2, 3)
    graph.add_adj(3, 4)
    graph.add_adj(4, 1)
    graph.add_adj(1, 3)

    inertia = {
        1: np.array([[1.1, 0.1], [0.1, 0.9]]),
        2: np.array([[1.0, 0.0], [0.0, 1.1]]),
        3: np.array([[1.2, -0.2], [-0.2, 1.2]]),
        4: np.array([[0.9, 0.0], [0.0, 1.0]]),
    }
    return graph, inertia


def load_star_hub() -> tuple[Graph, StiffnessResult]:
    graph = Graph()
    graph.add_node(Node(1, Point(0.0, 0.0)))
    graph.add_node(Node(2, Point(0.0, 1.0)))
    graph.add_node(Node(3, Point(1.0, 0.0)))
    graph.add_node(Node(4, Point(0.0, -1.0)))
    graph.add_node(Node(5, Point(-1.0, 0.0)))
    graph.add_adj(1, 2)
    graph.add_adj(1, 3)
    graph.add_adj(1, 4)
    graph.add_adj(1, 5)

    inertia = {
        1: np.array([[1.6, 0.0], [0.0, 1.6]]),
        2: np.array([[0.8, 0.1], [0.1, 0.7]]),
        3: np.array([[0.7, -0.1], [-0.1, 0.8]]),
        4: np.array([[0.9, 0.0], [0.0, 0.6]]),
        5: np.array([[0.95, 0.05], [0.05, 0.85]]),
    }
    return graph, inertia
