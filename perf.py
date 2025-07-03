import json
import os
import matplotlib.pyplot as plt
from matplotlib import colormaps
import numpy as np

plt.rcParams['font.size'] = 20

viridis = colormaps['viridis']
magma = colormaps['magma']

def add_labels(x, y):
    for i in range(len(x)):
        plt.text(i, y[i], round(y[i], 2), ha='center')


# Data for the two groups

locked = [4.08, 12.30, 39.87, 130.34, 175.83, 131.19]
lockless = [2.07, 4.70, 4.83, 3.21, 3.42,  1.99]

labels = ['1 Thread (Criterion)', '1 Thread', '2 Threads', '4 Threads', '8 Threads', '16 Threads']  # Labels for the groups

x = np.arange(len(labels))  # Numerical positions for the groups
width = 0.35  # Width of the bars

fig, ax = plt.subplots()

rects1 = ax.bar(x - width/2, lockless, width, label='hyperloglockless', color=viridis(0.5))
rects2 = ax.bar(x + width/2, locked, width, label='hyperloglogplus', color=magma(0))

# Add labels, title, and customize x-axis ticks
ax.set_ylabel('Insert Speed (ns)') 
ax.set_title('HyperLogLog Insert Latency (Lower is Better)')
ax.set_xticks(x)
ax.set_xticklabels(labels)
ax.legend(loc='upper left')


# Function to add value labels above the bars
def autolabel(rects):
    """Attach a text label above each bar in *rects*, displaying its height."""
    for rect in rects:
        height = rect.get_height()
        formatted_height = f'{height:.2f}'
        ax.annotate(formatted_height,  # The text to display
                    xy=(rect.get_x() + rect.get_width() / 2, height),  # Position of the value label
                    xytext=(0, 3),  # 3 points vertical offset
                    textcoords="offset points",  # Relative positioning
                    ha='center',  # Horizontal alignment
                    va='bottom')  # Vertical alignment

# Add labels to the bars
autolabel(rects1)
autolabel(rects2)

plt.show()

