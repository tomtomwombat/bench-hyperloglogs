import os
import glob
import matplotlib.pyplot as plt
from math import log10
from matplotlib import colormaps

plt.rcParams['font.size'] = 20
cm = [colormaps['Set2'](i / 8) for i in range(8)]

def read_csv(path):
    xs = []
    ys = []
    with open(path, "r") as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            x, y = line.split(",")
            xs.append(int(x))
            ys.append(int(y))
    return xs, ys


def main():
    csv_files = glob.glob("*.csv")

    if not csv_files:
        print("No CSV files found in current directory.")
        return

    plt.figure(figsize=(10, 6))

    for file in sorted(csv_files, key=lambda x: 'hyperloglockless' in x):
        xs, ys = read_csv(file)
        label = os.path.splitext(os.path.basename(file))[0]
        label = label.replace('_', ':')
        if 'hyperloglockless' in label:
            color = cm[0]
        else:
            color = 'black'
        # ys = [log10(yys +) for yys in ys]
        plt.plot(xs, ys, label=label, color=color, linewidth=3.5)

    plt.xlabel("Number of Items Inserted")
    plt.ylabel("Memory Usage (bytes)")
    plt.title("HLL Memory Growth Comparison (Precision = 14)")
    plt.grid(True, linestyle="--", alpha=0.5)
    plt.legend()
    plt.xlim(left=-10)
    plt.xlim(left=-10)
    plt.xlim(right=8000)
    plt.ylim(bottom=-10)
    # plt.tight_layout()
    plt.show()


if __name__ == "__main__":
    main()
