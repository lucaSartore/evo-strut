from custom_types import *
import matplotlib.pyplot as plt
from matplotlib.patches import Ellipse
import numpy as np

class Visualizer:
    # add a graph that should be visualized as a set of points
    # with the link connected by some edges
    def __init__(self, graph: Graph) -> None:
        self.graph = graph
        self._stiffness_layers: list[tuple[StiffnessResult, str, str]] = []
        pass

    # add the stiffness of each node visualized as an ellipse
    def add_stiffness_visualization(self, stiffness: StiffnessResult, color: str, label: str):
        self._stiffness_layers.append((stiffness, color, label))

    # plot everything
    def plot(self, title: str):
        fig, ax = plt.subplots()

        # draw edges
        drawn_edges = set()
        for node in self.graph.nodes.values():
            x0, y0 = node.position.x, node.position.y
            for neighbor in node.adj:
                edge = tuple(sorted((node.id, neighbor.id)))
                if edge in drawn_edges:
                    continue
                drawn_edges.add(edge)
                x1, y1 = neighbor.position.x, neighbor.position.y
                ax.plot([x0, x1], [y0, y1], color="black", linewidth=1, zorder=1)

        # draw nodes
        xs = [node.position.x for node in self.graph.nodes.values()]
        ys = [node.position.y for node in self.graph.nodes.values()]
        ax.scatter(xs, ys, color="black", s=30, zorder=3)

        # draw stiffness ellipses
        for stiffness_result, color, label in self._stiffness_layers:
            first_patch = True
            for node_id, stiffness in stiffness_result.items():
                if node_id not in self.graph.nodes:
                    continue
                node = self.graph.nodes[node_id]
                if stiffness.shape != (2, 2):
                    raise ValueError("Stiffness matrix must be a 2x2 array")

                matrix = np.asarray(stiffness, dtype=float)
                if not np.allclose(matrix, matrix.T, atol=1e-8):
                    matrix = 0.5 * (matrix + matrix.T)

                eigenvalues, eigenvectors = np.linalg.eigh(matrix)
                eigenvalues = np.maximum(eigenvalues, 0.0)
                widths = 2.0 * np.sqrt(eigenvalues)

                if np.all(widths == 0):
                    continue

                angle = np.degrees(np.arctan2(eigenvectors[1, 0], eigenvectors[0, 0]))
                ellipse = Ellipse(
                    (node.position.x, node.position.y),
                    width=widths[0],
                    height=widths[1],
                    angle=angle,
                    edgecolor=color,
                    facecolor=color,
                    alpha=0.3,
                    linewidth=1.5,
                    label=label if first_patch else None,
                    zorder=2,
                )
                ax.add_patch(ellipse)
                first_patch = False

        ax.set_title(title)
        ax.set_aspect("equal", adjustable="datalim")
        ax.set_xlabel("x")
        ax.set_ylabel("y")
        if self._stiffness_layers:
            ax.legend()
        plt.tight_layout()
        plt.show()
