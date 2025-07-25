import json
import os
import matplotlib.pyplot as plt
from matplotlib import colormaps
import numpy as np

plt.rcParams['font.size'] = 20
# plt.style.use('dark_background')
cm = [colormaps['Set1'](i / 9) for i in range(9)]
cm = [colormaps['Dark2'](i / 8) for i in range(8)]
cm = [colormaps['Set2'](i / 8) for i in range(8)]
def add_labels(x, y):
    for i in range(len(x)):
        plt.text(i, y[i], round(y[i], 2), ha='center')

# Function to add value labels above the bars
def autolabel(rects, ax):
    """Attach a text label above each bar in *rects*, displaying its height."""
    for rect in rects:
        height = rect.get_height()
        
        if height > 1000.0:        
            formatted_height = f'{(height / 1000.0):.0f}' + ' µs'
        else:
            formatted_height = f'{height:.1f}'
        ax.annotate(formatted_height,  # The text to display
                    xy=(rect.get_x() + rect.get_width() / 2, height),  # Position of the value label
                    xytext=(0, 1),  # 3 points vertical offset
                    textcoords="offset points",  # Relative positioning
                    ha='center',  # Horizontal alignment
                    va='bottom', # Vertical alignment
                    fontsize=12)  


def plot(operation, labels, data, log=False, width=0.15):
    x = np.arange(len(labels))  # Numerical positions for the groups

    fig, ax = plt.subplots()

    alpha = 0.35
    rects = [
        ax.bar(x + (i - len(data) / 2)*width, val, width, label=name, color=color) for i,(name,val,color) in enumerate(data)
    ]

    # Add labels, title, and customize x-axis ticks
    ax.set_ylabel('%s Time (ns)' % operation) 
    ax.set_title('HyperLogLog %s Time (Precision = 16)' % operation)
    ax.set_xticks(x)
    ax.set_xticklabels(labels)
    ax.legend(loc='upper left')
    if log: ax.set_yscale('log')
    # Add labels to the bars
    [autolabel(r, ax) for r in rects]

    # plt.ylim(top=60000)
    plt.show()


plot(
    'Multi-Threaded Insert',
    ['1 Thread', '2 Threads', '4 Thread', '8 Thread', '16 Threads'],
    [
        
        ('hyperloglockless::AtomicHyperLogLog', [7.6101934, 6.9030741, 4.2476726, 4.5348077, 2.6094058], cm[0]),
        ('RwLock<hyperloglockless::HyperLogLog>', [6.9717907, 42.570497, 148.361534, 203.432196, 147.147022], 'black'),
    ],
    width=0.35,
)

plot(
    'Multi-Threaded Insert',
    ['1 Thread (Criterion)', '1 Thread', '4 Threads', '16 Threads'][1:],
    [
        ('hyperloglockless::AtomicHyperLogLog', [1.4551, 7.703992, 4.400486, 3.309353][1:], cm[0]),
        ('hyperloglogplus::HyperLogLogPlus', [5.0512, 12.307561, 126.704604,  131.721376][1:],  cm[1]),
        ('hyperloglogplus::HyperLogLogPF', [4.1132, 13.469236, 109.974119, 128.691293][1:], cm[2]),
        ('cardinality_estimator::CardinalityEstimator', [3.3578, 19.772476, 188.866262, 209.342089][1:], cm[3]),
        ('probabilistic_collections::HyperLogLog', [1.4374, 8.973813, 139.558334, 231.348534][1:], cm[4]),
        ('amadeus_streaming::HyperLogLog', [9.6482, 17.690664, 183.625877, 325.404944][1:], cm[5]),
    ]
)

plot(
    'Count',
    ['1 Thread (Criterion)', '1 Thread', '4 Threads', '16 Threads'][1:],
    [
        ('hyperloglockless', [0.42224, 2.871251, 0.74436, 0.359037][1:], cm[0]),
        ('hyperloglogplus (HyperLogLog)', [41.803 * 1000, 42299.154, 43008.95, 43340.585][1:], cm[1]),
        ('hyperloglogplus (HyperLogLogPlus)', [41.418 * 1000, 41753.129, 42403.368, 42605.169][1:], cm[2]),
        ('cardinality-estimator', [11.758, 18.066991, 46.720138, 65.857357][1:], cm[3]),
        ('probabilistic-collections', [794.08 * 1000, 42395.06, 203848.351, 70098.573][1:], cm[4]),
        ('amadeus-streaming', [3.6594, 5.837332, 47.158641, 65.650805][1:], cm[5]),
    ],
    # log=True,
)
