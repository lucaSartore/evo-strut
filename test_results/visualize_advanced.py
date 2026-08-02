#!/usr/bin/env python3
"""
Advanced visualization for optimization logs.

Set A – 45 charts (15 test cases × 3 sub-phases):
  All individual run cost curves overlaid with different colors,
  plus a bold average line. Y log-scale, X normalized to [0, 1].

Set B – 15 charts (5 test numbers × 3 sub-phases):
  One line per test type (A, C, D) showing the per-type average
  from Set A, plus a grand-average line.

Outputs written to charts_A/ and charts_B/ next to this script.
"""

import glob
import json
import os

import matplotlib
import numpy as np

matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib import colormaps

TESTS_DIR = os.path.dirname(os.path.abspath(__file__))
TEST_NUMBERS = [1, 2, 3, 4, 5]
TEST_TYPES = ["A", "C", "D"]
SUBPHASES = [
    "contact_points_grouping",
    "contact_points_optimization",
    "support_structure_optimization",
]

GRID = np.linspace(0, 1, 500)

AVG_COLOR = "#E63946"
AVG_LW = 2.5
AVG_ZORDER = 10

TYPE_COLORS = {"A": "#4895EF", "C": "#2DC653", "D": "#F77F00"}
TYPE_NAMES = {"A": "Armadillo", "C": "Candlestick", "D": "Dragon"}


def get_files(test_name, subphase):
    base = os.path.join(TESTS_DIR, test_name, "optimization_logs")
    if subphase == "contact_points_grouping":
        pattern = os.path.join(base, "contact_points_grouping.json")
    elif subphase == "contact_points_optimization":
        pattern = os.path.join(base, "contact_points_optimization_area_*.json")
    else:
        pattern = os.path.join(base, "support_structure_optimization_group_*.json")
    return sorted(glob.glob(pattern))


def load_xy(path):
    with open(path) as f:
        d = json.load(f)
    rc = d.get("cost_log", {}).get("running_costs", [])
    if not rc:
        return None, None
    gens = np.array([r["generation"] for r in rc], dtype=float)
    costs = np.array([r["cost"] for r in rc], dtype=float)
    x = np.array([0.0]) if len(gens) < 2 else (gens - gens[0]) / (gens[-1] - gens[0])
    return x, costs


def interp_to_grid(x, y):
    return np.interp(GRID, x, y)


def compute_average(files):
    curves = [interp_to_grid(x, y) for f in files for x, y in [load_xy(f)] if x is not None]
    return np.mean(curves, axis=0) if curves else None


def run_colors(n):
    if n <= 10:
        cmap = colormaps["tab10"]
        return [cmap(i / 10) for i in range(n)]
    cmap = colormaps["plasma"]
    return [cmap(i / (n - 1)) for i in range(n)]


def subphase_label(sp):
    return sp.replace("_", " ").title()


def apply_common_style(ax, title):
    ax.set_yscale("log")
    ax.set_xlim(0, 1)
    ax.set_xlabel("Normalized iteration", fontsize=12)
    ax.set_ylabel("Cost (log scale)", fontsize=12)
    ax.set_title(title, fontsize=13, fontweight="bold")
    ax.grid(True, which="both", linestyle="--", linewidth=0.4, alpha=0.5)
    ax.legend(fontsize=11)


# ── Set A ────────────────────────────────────────────────────────────────────

out_a = os.path.join(TESTS_DIR, "charts_A")
os.makedirs(out_a, exist_ok=True)

for n in TEST_NUMBERS:
    for t in TEST_TYPES:
        test_name = f"ES{n}{t}"
        for sp in SUBPHASES:
            files = get_files(test_name, sp)
            if not files:
                print(f"[A] SKIP {test_name}/{sp}: no files found")
                continue

            colors = run_colors(len(files))
            fig, ax = plt.subplots(figsize=(10, 6))

            curves = []
            for i, f in enumerate(files):
                x, y = load_xy(f)
                if x is None:
                    continue
                ax.plot(x, y, color=colors[i], alpha=0.45, linewidth=0.7)
                curves.append(interp_to_grid(x, y))

            if curves:
                avg = np.mean(curves, axis=0)
                ax.plot(GRID, avg, color=AVG_COLOR, linewidth=AVG_LW,
                        label="Average", zorder=AVG_ZORDER)

            apply_common_style(ax, f"{test_name}  ·  {subphase_label(sp)}")
            fname = f"{test_name}_{sp}.png"
            fig.savefig(os.path.join(out_a, fname), dpi=150, bbox_inches="tight")
            plt.close(fig)
            print(f"[A] {fname}")

# ── Set B ────────────────────────────────────────────────────────────────────
# 9 charts: one per (test type, sub-phase).
# Each chart has 5 lines (ES1–ES5 averages) + grand average.

out_b = os.path.join(TESTS_DIR, "charts_B")
os.makedirs(out_b, exist_ok=True)

NUMBER_COLORS = colormaps["tab10"]

for t in TEST_TYPES:
    for sp in SUBPHASES:
        fig, ax = plt.subplots(figsize=(10, 6))

        number_avgs = []
        for i, n in enumerate(TEST_NUMBERS):
            avg = compute_average(get_files(f"ES{n}{t}", sp))
            if avg is None:
                continue
            ax.plot(GRID, avg, color=NUMBER_COLORS(i / 10), linewidth=1.8,
                    alpha=0.45, label=f"Test {n}")
            number_avgs.append(avg)

        if number_avgs:
            grand = np.mean(number_avgs, axis=0)
            ax.plot(GRID, grand, color=AVG_COLOR, linewidth=AVG_LW,
                    label="Average", zorder=AVG_ZORDER)

        apply_common_style(
            ax, f"Testcase {TYPE_NAMES[t]}  ·  {subphase_label(sp)}"
        )
        fname = f"ES_{TYPE_NAMES[t]}_{sp}.png"
        fig.savefig(os.path.join(out_b, fname), dpi=150, bbox_inches="tight")
        plt.close(fig)
        print(f"[B] {fname}")
