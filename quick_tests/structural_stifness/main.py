from evaluation.bottom_up_evaluator import BottomUpEvaluator
from evaluation.dag_evaluator import DagEvaluator
from evaluation.interface import Evaluator
from evaluation.util import calculate_accuracy
from testcases import *
from visualize import Visualizer
from evaluation.anastruct_evaluator import AnastructEvaluator
from collections import defaultdict

def main():
    settings = Settings(100, 10)

    visualize = True
    evaluators: list[type[Evaluator]] = [
        BottomUpEvaluator,
        DagEvaluator
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

    # Data structures to track accuracies
    # Key: Name, Value: List of accuracy floats
    struct_accuracies = defaultdict(list)
    evaluator_accuracies = defaultdict(list)

    for (graph_name, graph) in graph_to_evaluate:
        ground_truth = AnastructEvaluator.evaluate(graph, settings)
        
        for evaluator in evaluators:
            eval_name = evaluator.__name__
            stiffness = evaluator.evaluate(graph, settings)

            accuracy = calculate_accuracy(stiffness, ground_truth)
            
            # Store results for averaging later
            struct_accuracies[graph_name].append(accuracy)
            evaluator_accuracies[eval_name].append(accuracy)

            accuracy_str = "%.2f" % (accuracy*100) + "%"
            print(f"Accuracy of {eval_name} for graph {graph_name} is {accuracy_str}")
            
            if visualize:
                v = Visualizer(graph)
                v.add_stiffness_visualization(ground_truth, "red", "ground_truth")
                v.add_stiffness_visualization(stiffness, "green", eval_name)
                v.plot(f"{eval_name} - graph {graph_name} - accuracy={accuracy_str}")

    print("\n" + "="*30)
    print("AVERAGE ACCURACY SUMMARY")
    print("="*30)

    # 1. Print average accuracy per Struct
    print("\n--- Per Struct (across all evaluators) ---")
    for name, scores in struct_accuracies.items():
        avg = (sum(scores) / len(scores)) * 100
        print(f"{name.ljust(18)}: {avg:>6.2f}%")

    # 2. Print average accuracy per Evaluator
    print("\n--- Per Evaluator (across all structs) ---")
    for name, scores in evaluator_accuracies.items():
        avg = (sum(scores) / len(scores)) * 100
        print(f"{name.ljust(18)}: {avg:>6.2f}%")
    print("="*30)

if __name__ == "__main__":
    main()
