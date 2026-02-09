import json
import os
import glob
import matplotlib.pyplot as plt
from collections import defaultdict


import matplotlib.colors as mcolors
from matplotlib import colormaps
plt.rcParams['font.size'] = 18



def swap_color(c, i, j):
    sparse_color = mcolors.to_rgb(cm[0])
    rgb = list(mcolors.to_rgb(c))
    rgb[i], rgb[j] = rgb[j], rgb[i]
    return tuple(rgb)

cm = [colormaps['Set2'](i / 8) for i in range(8)]


filters = {
    'hyperloglogplus::HyperLogLogPlus': cm[1],
    'hyperloglogplus::HyperLogLogPF':  cm[2],
    'cardinality_estimator::CardinalityEstimator': cm[3], 
    'probabilistic_collections::HyperLogLog':  cm[4],
    'amadeus_streaming::HyperLogLog': cm[5],
    'apache_datafusion::HyperLogLog': swap_color(cm[5], 1, 2),
    'hyperloglockless::HyperLogLog': cm[0],
    'hyperloglockless::HyperLogLogPlus': swap_color(cm[0], 1, 2),
    # ('hyperloglockless (Old)', cm[6], 1, lw),
}

def fix_name(s):
    crate, ds = s.split('__')
    ds = ds \
    .replace('hyperloglog', 'HyperLogLog')\
    .replace('plus', 'Plus') \
    .replace('cardinalityestimator', 'CardinalityEstimator')\
    .replace('pf', 'PF')
    return crate + "::" + ds
    

# --- CONFIGURATION ---
# Map your crate names to specific hex codes or standard colors
# If a crate isn't here, it defaults to gray.
COLORS = {
    "hyperloglockless": "#1f77b4",        # Nice Blue
    "apache_hll": "#ff7f0e",         # Safety Orange
    "my_experimental_crate": "#2ca02c" # Forest Green
}

DEFAULT_COLOR = "gray"
BENCHMARK_GROUP = "Fill"
# ---------------------

def plot_criterion_results():
    results = defaultdict(list)
    
    # Target Criterion JSON estimates
    path_pattern = f"target/criterion/{BENCHMARK_GROUP}/*/base/estimates.json"
    
    for file_path in glob.glob(path_pattern):
        # Folder structure: target/criterion/Fill/items-<crate>-<num>/...
        folder_name = file_path.split(os.sep)[-3]
        parts = folder_name.split('-')
        
        if len(parts) < 3:
            continue
        try:
            crate_name = parts[2]
            num_items = int(parts[1])
        except:
            crate_name = parts[1]
            num_items = int(parts[2])
        
        with open(file_path, 'r') as f:
            data = json.load(f)
            mean_ms = data['mean']['point_estimate'] / 1_000
            results[crate_name].append((num_items, mean_ms))

    if not results:
        print(f"No results found for group '{BENCHMARK_GROUP}'. Check your target directory.")
        return

    plt.figure(figsize=(12, 7))

    for crate_name, values in results.items():
        values.sort()
        x, y = zip(*values)
        
        # Determine color: check dict, else use default
        line_color = COLORS.get(crate_name, DEFAULT_COLOR)
        x, y = zip(*[(xi, yi) for (xi, yi) in zip(x, y) if 64 < xi < 2**20])
        label = fix_name(crate_name)
        color = filters[label]
        plt.plot(
            x, y, 
            marker='o', 
            markersize=4,
            label=label, 
            color=color,
            linewidth=2.5,
            alpha=1.0
        )

    # Styling for log-scale benchmarks
    plt.xscale('log', base=2)
    plt.yscale('log')

    plt.xlabel('Number of Items', fontsize=18)
    plt.ylabel('Mean Execution Time (μs)', fontsize=18)
    plt.title(f'HyperLogLog Performance (Lower is Better): Insert then Count', fontsize=18, fontweight='bold')
    
    plt.legend(loc='lower right')
    plt.grid(True, which="both", ls="--", alpha=0.3)
    
    plt.tight_layout()
    #output_file = '..\HLLGraphs\\fill.png'
    #plt.savefig(output_file, dpi=300)
    #print(f"Plot saved to {output_file}")
    plt.show()

if __name__ == "__main__":
    plot_criterion_results()