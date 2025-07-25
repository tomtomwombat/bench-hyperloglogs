from math import log10, log2, log
import matplotlib.pyplot as plt 
import csv
from matplotlib import colormaps
from matplotlib.ticker import ScalarFormatter
from matplotlib.ticker import FuncFormatter

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

cm = [colormaps['Set2'](i / 8) for i in range(8)]
filters = [
    ('hyperloglogplus (HyperLogLogPlus)',  cm[1], alpha, lw),
    ('hyperloglogplus (HyperLogLog)',  cm[2], alpha, lw),
    ('cardinality-estimator',  cm[3], alpha, lw),
    ('probabilistic-collections',  cm[4], alpha, lw),
    ('amadeus-streaming', cm[5], alpha, lw),
    ('hyperloglockless', cm[0], 1, lw),
]

fig, ax = plt.subplots()

def custom_format(yy, _):
    if yy >= 1:
        return f"{int(yy)}"
    else:
        return f"{yy:.3f}".rstrip("0").rstrip(".")

for i, (name, color, aa, lw) in enumerate(filters):
    file_name = 'Acc/%s.csv' % name
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

plt.title('HyperLogLog Error (Lower is Better)')
'''
# Size comparison

plt.xlim(left=min(x), right=max(x))
plt.ylim(bottom=0.01)
plt.gca().yaxis.set_major_formatter(ScalarFormatter())
plt.gca().yaxis.get_major_formatter().set_scientific(False)
plt.gca().yaxis.set_major_formatter(FuncFormatter(custom_format))
'''

# Crate Comparison

plt.xlim(left=150)
plt.xlim(right=max(x))
plt.ylim(bottom=0.0005)
plt.ylim(top=100)
plt.gca().yaxis.set_major_formatter(ScalarFormatter())
plt.gca().yaxis.get_major_formatter().set_scientific(False)
plt.gca().yaxis.set_major_formatter(FuncFormatter(custom_format))

handles,labels = ax.get_legend_handles_labels()

handles = [handles.pop()] + handles
labels = [labels.pop()] + labels

plt.grid()
# https://stackoverflow.com/questions/67033128/matplotlib-order-of-legend-entries
plt.legend(handles,labels,loc='upper left')
plt.show()

