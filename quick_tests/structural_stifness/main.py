from evaluation.bottom_up_evaluator import BottomUpEvaluator
from evaluation.dag_evaluator import DagEvaluator
from evaluation.interface import Evaluator
from evaluation.util import calculate_accuracy, calculate_beam_stiffness
from evaluation.weighted_dag_evaluator import WeightedDagEvaluator
from testcases import *
from visualize import Visualizer
from evaluation.anastruct_evaluator import AnastructEvaluator
from collections import defaultdict

def main():

    import numpy as np
    struct = load_lines();
    settings = Settings(12_000, 9_000)
    results = AnastructEvaluator.evaluate(struct, settings)
    m = np.linalg.inv(results[2])
    calculated = calculate_beam_stiffness(struct.nodes[1].position, struct.nodes[2].position, settings)
    print(m)
    print(calculated)

    return
    settings = Settings(100, 10)

    # Set visualize to True to see the plots
    visualize = True 
    
    # Added "color" as the third parameter in the tuple
    evaluators: list[tuple[str, type[Evaluator], str]] = [
        ("bottom_up", BottomUpEvaluator, "blue"),
        ("dag", DagEvaluator, "green"),
        ("weighted_dag", WeightedDagEvaluator, "orange")
    ]
    
    graph_to_evaluate = [
        ("struct_A", load_struct_A()),
        ("struct_B", load_struct_B()),
        ("struct_C", load_struct_C()),
        ("struct_D", load_struct_D()),
        ("struct_E", load_struct_E()),
        ("struct_F", load_struct_F())
    ]

    struct_accuracies = defaultdict(list)
    evaluator_accuracies = defaultdict(list)

    for (graph_name, graph) in graph_to_evaluate:
        ground_truth = AnastructEvaluator.evaluate(graph, settings)
        
        # Initialize visualizer once per graph to overlay all strategies
        v_combined = None
        if visualize:
            v_combined = Visualizer(graph)
            v_combined.add_stiffness_visualization(ground_truth, "red", "ground_truth")

        for eval_name, evaluator, color in evaluators:
            stiffness = evaluator.evaluate(graph, settings)

            accuracy = calculate_accuracy(stiffness, ground_truth)
            struct_accuracies[graph_name].append(accuracy)
            evaluator_accuracies[eval_name].append(accuracy)

            accuracy_str = "%.2f" % (accuracy*100) + "%"
            print(f"Accuracy of {eval_name} for graph {graph_name} is {accuracy_str}")
            
            if visualize:
                # Add this specific evaluator's stiffness to the combined plot
                v_combined.add_stiffness_visualization(stiffness, color, f"{eval_name} ({accuracy_str})")

        # After evaluating all strategies for this graph, show the combined plot
        if visualize:
            v_combined.plot(f"Comparison - {graph_name}")

    print("\n" + "="*30)
    print("AVERAGE ACCURACY SUMMARY")
    print("="*30)

    print("\n--- Per Struct (across all evaluators) ---")
    for name, scores in struct_accuracies.items():
        avg = (sum(scores) / len(scores)) * 100
        print(f"{name.ljust(18)}: {avg:>6.2f}%")

    print("\n--- Per Evaluator (across all structs) ---")
    for name, scores in evaluator_accuracies.items():
        avg = (sum(scores) / len(scores)) * 100
        print(f"{name.ljust(18)}: {avg:>6.2f}%")
    print("="*30)

if __name__ == "__main__":
    main()
