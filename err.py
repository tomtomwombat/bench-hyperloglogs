from math import log10, log2, log
import matplotlib.pyplot as plt 
import csv
from matplotlib.ticker import ScalarFormatter
from matplotlib.ticker import FuncFormatter
import matplotlib.colors as mcolors
from matplotlib import colormaps
plt.rcParams['font.size'] = 20

viridis = colormaps['viridis']
magma = colormaps['magma']
# plt.style.use('dark_background')
'''
min_size = 12
max_size = 22
color_step =  0 if max_size - min_size == 0 else 1 / (max_size - min_size)
filters = [('hyperloglockless (%d bytes)' % (2**i), magma((i - min_size) * color_step)) for i in range(min_size, max_size + 1)]
'''

alpha = 1
lw = 3.5


def swap_color(c, i, j):
    sparse_color = mcolors.to_rgb(cm[0])
    rgb = list(mcolors.to_rgb(c))
    rgb[i], rgb[j] = rgb[j], rgb[i]
    return tuple(rgb)

cm = [colormaps['Set2'](i / 8) for i in range(8)]

filters = [
    ('hyperloglogplus::HyperLogLogPlus',  cm[1], alpha, lw),
    ('hyperloglogplus::HyperLogLogPF',  cm[2], alpha, lw),
    ('cardinality_estimator::CardinalityEstimator',  cm[3], alpha, lw),
    ('probabilistic_collections::HyperLogLog',  cm[4], alpha, lw),
    ('amadeus_streaming::HyperLogLog', cm[5], alpha, lw),
    ('apache_datafusion::HyperLogLog', swap_color(cm[5], 1, 2), alpha, lw),
    ('hyperloglockless::HyperLogLog', cm[0], 1, lw),
    ('hyperloglockless::HyperLogLogPlus', swap_color(cm[0], 1, 2), 1, lw),
    # ('hyperloglockless (Old)', cm[6], 1, lw),
]

fig, ax = plt.subplots()

def custom_format(yy, _):
    if yy >= 1:
        return f"{int(yy)}"
    else:
        return f"{yy:.5f}".rstrip("0").rstrip(".")

for i, (name, color, aa, lw) in enumerate(filters):
    file_name = ('Acc/%s.csv' % name).replace('::', '__')
    print(file_name)
    with open(file_name, 'r') as csvfile:
        data = []
        rows = csv.reader(csvfile, delimiter = ',')
        for row in rows:
            if row[1] == 'NaN':
                continue
            num_items = int(row[0])
            avg_y = float(row[1])*100.0
            min_y = float(row[2])*100.0
            max_y = float(row[3])*100.0

            data.append((num_items, avg_y, min_y, max_y))

        x,avg_y,min_y,max_y = zip(*data)
        
        ax.plot(x, avg_y, color=color, label=name, linewidth=lw, alpha=aa)
        #ax.fill_between(x, max_y, min_y, color = color, alpha = aa*0.15)
        ax.set_yscale('log')
        ax.set_xscale('log')

plt.xlabel('True Number of Distinct Elements') 
plt.ylabel('Error %') 

plt.title('HyperLogLog Error, Lower is Better (Precision = 14)')

# Crate Comparison

plt.xlim(left=150)
plt.xlim(right=max(x))
plt.ylim(bottom=0.000005)
plt.ylim(top=125)
plt.gca().yaxis.set_major_formatter(ScalarFormatter())
plt.gca().yaxis.get_major_formatter().set_scientific(False)
plt.gca().yaxis.set_major_formatter(FuncFormatter(custom_format))

handles, labels = ax.get_legend_handles_labels()

# make hyperlogloglockless appear on top
handles = [handles.pop(), handles.pop()] + handles
labels = [labels.pop(), labels.pop()] + labels

plt.grid()
# https://stackoverflow.com/questions/67033128/matplotlib-order-of-legend-entries
plt.legend(handles, labels, loc='lower right')
plt.show()

