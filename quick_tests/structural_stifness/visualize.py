from .models import *
import matplotlib.pyplot as plt

class Visualizer:
    # add a graph that should be visualized as a set of points
    # with the link connected by some edges
    def __init__(self, graph: Graph) -> None:
        pass

    # add the stiffness of each node visualized as an ellipse
    def add_stiffness_visualization(self, stiffness: StiffnessResult, color: str, label: str):
        pass

    # plot everything
    def plot(self, title: str):
        pass
