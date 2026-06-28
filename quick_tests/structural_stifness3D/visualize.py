from custom_types import *
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D
from mpl_toolkits.mplot3d.art3d import Poly3DCollection
import numpy as np

class Visualizer:
    # add a graph that should be visualized as a set of points
    # with the link connected by some edges
    def __init__(self, graph: Graph, ellipse_scale = 10) -> None:
        self.graph = graph
        self._stiffness_layers: list[tuple[StiffnessResult, str, str]] = []
        self._ellipse_scale = ellipse_scale
        pass

    # add the stiffness of each node visualized as an ellipse
    def add_stiffness_visualization(self, stiffness: StiffnessResult, color: str, label: str):
        self._stiffness_layers.append((stiffness, color, label))

    # plot everything
    def plot(self, title: str):
        fig = plt.figure()
        ax = fig.add_subplot(111, projection='3d')

        # draw edges
        drawn_edges = set()
        for node in self.graph.nodes.values():
            x0, y0, z0 = node.position.x, node.position.y, node.position.z
            for neighbor in node.adj:
                edge = tuple(sorted((node.id, neighbor.id)))
                if edge in drawn_edges:
                    continue
                drawn_edges.add(edge)
                x1, y1, z1 = neighbor.position.x, neighbor.position.y, neighbor.position.z
                ax.plot([x0, x1], [y0, y1], [z0, z1], color="black", linewidth=1, zorder=1)

        # draw nodes
        xs = [node.position.x for node in self.graph.nodes.values()]
        ys = [node.position.y for node in self.graph.nodes.values()]
        zs = [node.position.z for node in self.graph.nodes.values()]
        ax.scatter(xs, ys, zs, c="black", s=30, zorder=3)

        # draw stiffness ellipsoids
        for stiffness_result, color, label in self._stiffness_layers:
            first_patch = True  # Track the first patch to apply the label once per layer
            for node_id, stiffness in stiffness_result.items():
                if node_id not in self.graph.nodes:
                    continue
                if self.graph.nodes[node_id].ground_node:
                    continue
                node = self.graph.nodes[node_id]

                if stiffness.shape != (6, 6):
                    raise ValueError("Stiffness matrix must be a 6x6 array")
                # Extract the 3x3 submatrix for x, y, z displacements
                matrix = np.asarray(stiffness, dtype=float)[0:3, 0:3]
                # Invert the matrix to show compliance [m/N] rather than stiffness [N/m]
                matrix = np.linalg.inv(matrix)
                if not np.allclose(matrix, matrix.T, atol=1e-8):
                    matrix = 0.5 * (matrix + matrix.T)

                eigenvalues, eigenvectors = np.linalg.eigh(matrix * self._ellipse_scale)
                eigenvalues = np.maximum(eigenvalues, 0.0)
                radii = 2.0 * np.sqrt(eigenvalues)

                if np.all(radii == 0):
                    continue

                # Create ellipsoid data
                u = np.linspace(0, 2 * np.pi, 25)
                v = np.linspace(0, np.pi, 25)
                x = radii[0] * np.outer(np.cos(u), np.sin(v))
                y = radii[1] * np.outer(np.sin(u), np.sin(v))
                z = radii[2] * np.outer(np.ones_like(u), np.cos(v))

                # Rotate ellipsoid to align with eigenvectors
                ellipsoid = np.array([x.flatten(), y.flatten(), z.flatten()])
                rotated_ellipsoid = eigenvectors @ ellipsoid
                x_rot, y_rot, z_rot = rotated_ellipsoid.reshape(3, *x.shape)

                # Translate ellipsoid to node position
                x_rot += node.position.x
                y_rot += node.position.y
                z_rot += node.position.z

                # Plot ellipsoid
                # Only pass the label to the first surface patch drawn for this layer
                patch_label = label if first_patch else None
                
                ax.plot_surface(
                    x_rot, y_rot, z_rot,
                    rstride=4, cstride=4, color=color, alpha=0.3, linewidth=0, zorder=2,
                    label=patch_label
                )
                
                if first_patch:
                    first_patch = False

        ax.set_title(title)
        ax.set_xlabel("x")
        ax.set_ylabel("y")
        ax.set_zlabel("z")
        ax.set_box_aspect([1, 1, 1])  # Equal aspect ratio
        
        # Display the legend containing the labeled layer items
        ax.legend()
        
        plt.tight_layout()
        plt.show()
