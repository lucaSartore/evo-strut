from types import NoneType
import numpy as np
from evaluation.bottom_up_evaluator import BottomUpEvaluator
from evaluation.dag_evaluator import DagEvaluator
from evaluation.interface import Evaluator
from evaluation.util import (
    calculate_root_mean_squared_relative_error,
    calculate_pearson_correlation,
    calculate_spearman_correlation,
    filter_stiffness_components
)
from evaluation.weighted_dag_evaluator import WeightedDagEvaluator
from testcases import *
from visualize import Visualizer
from evaluation.opensees_evaluator import OpenSeesEvaluator
from collections import defaultdict

def main():
    settings = Settings()

    # Set visualize to True to see the plots
    visualize = True

    # decide whether to filter the stiffness matrix before evaluation
    # if the matrix is filter only xx and yy components are considered when
    # evaluating accuracy, otherwise the entire matrix is kept
    filter = True
    
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

    # Store metrics for all evaluations
    struct_metrics = defaultdict(lambda: defaultdict(dict))
    evaluator_metrics = defaultdict(lambda: defaultdict(dict))

    for (graph_name, graph) in graph_to_evaluate:
        ground_truth = OpenSeesEvaluator.evaluate(graph, settings)
        
        # Initialize visualizer once per graph to overlay all strategies
        v_combined = None
        if visualize:
            v_combined = Visualizer(graph)
            v_combined.add_stiffness_visualization(ground_truth, "red", "ground_truth")

        print(f"\n{'='*80}")
        print(f"Evaluating {graph_name}")
        print(f"{'='*80}")

        for eval_name, evaluator, color in evaluators:
            stiffness = evaluator.evaluate(graph, settings)

            if v_combined != None:
                # Add this specific evaluator's stiffness to the combined plot
                v_combined.add_stiffness_visualization(stiffness, color, f"{eval_name}")

            if filter:
                ground_truth_values = filter_stiffness_components(ground_truth)
                approximated_values = filter_stiffness_components(stiffness)
            else:
                ground_truth_values = ground_truth
                approximated_values = stiffness
            # Calculate metrics
            rmsre = calculate_root_mean_squared_relative_error(approximated_values, ground_truth_values)
            pearson = calculate_spearman_correlation(approximated_values, ground_truth_values)
            spearman = calculate_pearson_correlation(approximated_values, ground_truth_values)
            
            # Store metrics
            struct_metrics[graph_name][eval_name] = {
                'rmsre': rmsre,
                'pearson': pearson,
                'spearman': spearman
            }
            evaluator_metrics[eval_name][graph_name] = {
                'rmsre': rmsre,
                'pearson': pearson,
                'spearman': spearman
            }

            # Print metrics for this evaluation
            print(f"\n  {eval_name}:")
            print(f"    ├─ rmsre:              {rmsre:>8.2f}%")
            print(f"    ├─ Pearson Corr:     {pearson:>8.4f}")
            print(f"    └─ Spearman Corr:    {spearman:>8.4f}")
            
        # After evaluating all strategies for this graph, show the combined plot
        if v_combined != None:
            v_combined.plot(f"Comparison - {graph_name}")

    # Print summary tables
    print(f"\n\n{'='*80}")
    print("METRICS SUMMARY - PER STRUCTURE")
    print(f"{'='*80}\n")
    
    for graph_name in sorted(struct_metrics.keys()):
        print(f"{graph_name}:")
        print(f"{'':3}{'Evaluator':<18}{'Rmsre':<12}{'Pearson':<12}{'Spearman':<12}")
        print(f"{'':3}{'-'*54}")
        
        for eval_name in sorted(struct_metrics[graph_name].keys()):
            metrics = struct_metrics[graph_name][eval_name]
            print(f"{'':3}{eval_name:<18}{metrics['rmsre']:<12.2f}{metrics['pearson']:<12.4f}{metrics['spearman']:<12.4f}")
        print()

    print(f"{'='*80}")
    print("METRICS SUMMARY - PER EVALUATOR")
    print(f"{'='*80}\n")
    
    for eval_name in sorted(evaluator_metrics.keys()):
        print(f"{eval_name}:")
        print(f"{'':3}{'Structure':<18}{'MAPE':<12}{'Pearson':<12}{'Spearman':<12}")
        print(f"{'':3}{'-'*54}")
        
        for graph_name in sorted(evaluator_metrics[eval_name].keys()):
            metrics = evaluator_metrics[eval_name][graph_name]
            print(f"{'':3}{graph_name:<18}{metrics['rmsre']:<12.2f}{metrics['pearson']:<12.4f}{metrics['spearman']:<12.4f}")
        
        # Print averages for this evaluator
        rmsre_avg = np.mean([evaluator_metrics[eval_name][g]['rmsre'] for g in evaluator_metrics[eval_name]])
        pearson_avg = np.mean([evaluator_metrics[eval_name][g]['pearson'] for g in evaluator_metrics[eval_name]])
        spearman_avg = np.mean([evaluator_metrics[eval_name][g]['spearman'] for g in evaluator_metrics[eval_name]])
        
        print(f"{'':3}{'-'*54}")
        print(f"{'':3}{'AVERAGE':<18}{rmsre_avg:<12.2f}{pearson_avg:<12.4f}{spearman_avg:<12.4f}")
        print()

    print(f"{'='*80}")
    print("OVERALL AVERAGES")
    print(f"{'='*80}")
    
    all_rmsre = []
    all_pearson = []
    all_spearman = []
    
    for eval_name in evaluator_metrics:
        for graph_name in evaluator_metrics[eval_name]:
            all_rmsre.append(evaluator_metrics[eval_name][graph_name]['rmsre'])
            all_pearson.append(evaluator_metrics[eval_name][graph_name]['pearson'])
            all_spearman.append(evaluator_metrics[eval_name][graph_name]['spearman'])
    
    print(f"{'rmsre':<25} {np.mean(all_rmsre):>8.2f}%")
    print(f"{'Pearson Correlation:':<25} {np.mean(all_pearson):>8.4f}")
    print(f"{'Spearman Correlation:':<25} {np.mean(all_spearman):>8.4f}")
    print(f"{'='*80}\n")

if __name__ == "__main__":
    main()
