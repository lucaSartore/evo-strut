import argparse
import glob
import json
import os
import matplotlib.pyplot as plt


def process_json_file(file_path, log_scale, interactive):
    # Get file name without extension
    base_name = os.path.splitext(os.path.basename(file_path))[0]

    # Load and parse the JSON data
    try:
        with open(file_path, "text_mode" if False else "r") as f:
            data = json.load(f)
    except (json.JSONDecodeError, IOError) as e:
        print(f"Skipping {file_path}: Error reading file. {e}")
        return

    # Extract required fields safely
    cost_log = data.get("cost_log", {})
    final_cost = cost_log.get("final_cost", "N/A")
    total_time = cost_log.get("total_running_time_seconds", "N/A")
    running_costs = cost_log.get("running_costs", [])

    if not running_costs:
        print(f"Skipping {file_path}: No 'running_costs' data found.")
        return

    # Extract X (generation) and Y (cost) values
    generations = [item.get("generation") for item in running_costs]
    costs = [item.get("cost") for item in running_costs]

    # Setup the plot
    plt.figure(figsize=(10, 6))
    plt.plot(generations, costs, marker="o", linestyle="-", color="b")

    # Dynamic styling
    title = f"{base_name} | Final Cost: {final_cost} | Time: {total_time:.2f}s" if isinstance(total_time, (int, float)) else f"{base_name} | Final Cost: {final_cost} | Time: {total_time}"
    plt.title(title, fontsize=12, fontweight="bold")
    plt.xlabel("Generation")
    plt.ylabel("Cost (Log Scale)" if log_scale else "Cost")
    plt.grid(True, which="both", linestyle="--", linewidth=0.5)

    if log_scale:
        plt.yscale("log")

    # Decide whether to show or save
    if interactive:
        print(f"Displaying interactive plot for {base_name}...")
        plt.show()
    else:
        # Determine output directory (same level as optimization_logs)
        # file_path is: test_name/optimization_logs/filename.json
        # We want to save to: test_name/filename.png
        dir_name = os.path.dirname(os.path.dirname(file_path))

        suffix = "_log" if log_scale else ""
        output_filename = f"{base_name}{suffix}.png"
        output_path = os.path.join(dir_name, output_filename)

        plt.savefig(output_path, bbox_inches="tight")
        plt.close()  # Free memory
        print(f"Saved plot to: {output_path}")


def main():
    parser = argparse.ArgumentParser(
        description="Plot optimization logs from JSON files."
    )
    parser.add_argument(
        "test_name",
        type=str,
        help="Directory name/pattern containing 'optimization_logs/'",
    )
    parser.add_argument(
        "-l",
        "--log",
        action="store_true",
        help="Set the Y-axis (cost) to a logarithmic scale.",
    )
    parser.add_argument(
        "-i",
        "--interactive",
        action="store_true",
        help="Display the plots interactively instead of saving them.",
    )

    args = parser.parse_args()

    # Construct the glob pattern to find all JSON files
    # E.g., "my_test/optimization_logs/*.json" or "test_*/optimization_logs/*.json"
    search_pattern = os.path.join(
        args.test_name, "optimization_logs", "*.json"
    )
    json_files = glob.glob(search_pattern)

    if not json_files:
        print(f"No JSON files found matching pattern: {search_pattern}")
        return

    print(f"Found {len(json_files)} file(s) to process...")

    for file_path in json_files:
        process_json_file(file_path, args.log, args.interactive)


if __name__ == "__main__":
    main()
