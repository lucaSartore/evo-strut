from evaluation.bottom_up_evaluator import BottomUpEvaluator
from evaluation.interface import Evaluator
from evaluation.util import calculate_accuracy
from testcases import *
from visualize import Visualizer
from evaluation.anastruct_evaluator import AnastructEvaluator

def main():
    settings = Settings(100, 10)

    visualize = True
    evaluators: list[type[Evaluator]] = [
        BottomUpEvaluator
    ]
    graph_to_evaluate = [
        ("triangle", load_triangle()),
        ("lines", load_lines()),
        ("pillars", load_pillars()),
        ("horizontal_beam", load_horizontal_beam()),
        ("struct_A", load_struct_A()),
        ("struct_B", load_struct_B()),
        ("struct_C", load_struct_C()),
        ("struct_D", load_struct_D())
    ]
    graph = load_struct_D()

    for (graph_name, graph) in graph_to_evaluate:
        ground_truth = AnastructEvaluator.evaluate(graph, settings)
        for evaluator in evaluators:
            stiffness = evaluator.evaluate(graph, settings)

            accuracy = calculate_accuracy(stiffness, ground_truth)
            accuracy_str = "%.2f" % (accuracy*100) + "%"
            print(f"accuracy of {evaluator.__name__} for graph {graph_name} is {accuracy_str}")
            if visualize:
                v = Visualizer(graph)
                v.add_stiffness_visualization(ground_truth, "red", "ground_truth")
                v.add_stiffness_visualization(stiffness, "green", evaluator.__name__)
                v.plot(f"{evaluator.__name__} - graph {graph_name} - accuracy={accuracy_str}")

if __name__ == "__main__":
    main()
