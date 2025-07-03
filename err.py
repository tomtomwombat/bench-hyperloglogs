from math import log10, log2, log
import matplotlib.pyplot as plt 
import csv
from matplotlib import colormaps
from matplotlib.ticker import ScalarFormatter
from matplotlib.ticker import FuncFormatter

plt.rcParams['font.size'] = 20

viridis = colormaps['viridis']
magma = colormaps['magma']


min_size = 12
max_size = 22
color_step =  0 if max_size - min_size == 0 else 1 / (max_size - min_size)
filters = [('hyperloglockless (%d bytes)' % (2**i), magma((i - min_size) * color_step)) for i in range(min_size, max_size + 1)]


filters = [('hyperloglogplus (Precision = 16)', magma(0.001)), ('hyperloglockless (Precision = 16)', viridis(2 / 4))]


fig, ax = plt.subplots()

def custom_format(yy, _):
    if yy >= 1:
        return f"{int(yy)}"
    else:
        return f"{yy:.3f}".rstrip("0").rstrip(".")

for i, (name, color) in enumerate(filters):
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
        
        ax.plot(x, avg_y, color=color, label=name, linewidth=2.5)
        ax.fill_between(x, max_y, min_y, color = color, alpha = 0.15)
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

plt.xlim(left=10**6)
plt.xlim(right=10**10)
plt.ylim(bottom=0.01)
plt.ylim(top=250)
plt.gca().yaxis.set_major_formatter(ScalarFormatter())
plt.gca().yaxis.get_major_formatter().set_scientific(False)
plt.gca().yaxis.set_major_formatter(FuncFormatter(custom_format))

plt.grid()
plt.legend(loc='upper left')
plt.show()

