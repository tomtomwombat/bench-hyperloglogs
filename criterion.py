import json
import os
import matplotlib.pyplot as plt
from matplotlib import colormaps

plt.rcParams['font.size'] = 20

cm = [colormaps['Set2'](i / 8) for i in range(8)]
filters = [
    ('hyperloglockless::HyperLogLog', cm[0]),
    ('hyperloglockless::AtomicHyperLogLog', cm[0]),
    ('hyperloglockless::HyperLogLogPlus', cm[0]),

    ('cardinality_estimator::CardinalityEstimator', 'black'),

    ('hyperloglogplus::HyperLogLogPF', 'black'),
    ('hyperloglogplus::HyperLogLogPlus', 'black'),

    ('amadeus_streaming::HyperLogLog', 'black'),
    ('probabilistic_collections::HyperLogLog', 'black'),
    
    ('apache_datafusion::HyperLogLog', 'black'),
]

filters = [(x.replace('::', '\n::').replace('_', '\n_'), y) for (x,y) in filters]

directory = r"target\criterion"

def file_to_name(file):
    for name, _ in filters:
        if name.lower().replace('\n::', '__').replace('\n_', '_') == file:
            return name

def color(for_name):
    for name, c in filters:
        if name == for_name:
            return c

def is_input(x):
    try:
        int(x)
        return True
    except:
        return False

# Function to add value labels above the bars
def autolabel(rects, ax):
    """Attach a text label above each bar in *rects*, displaying its height."""
    for rect in rects:
        height = rect.get_height()
        
        if height > 1000.0:        
            formatted_height = f'{(height / 1000.0):.0f}' + ' µs'
        else:
            formatted_height = f'{height:.2f}'
        ax.annotate(formatted_height,  # The text to display
                    xy=(rect.get_x() + rect.get_width() / 2, height),  # Position of the value label
                    xytext=(0, 1),  # 3 points vertical offset
                    textcoords="offset points",  # Relative positioning
                    ha='center',  # Horizontal alignment
                    va='bottom', # Vertical alignment
                    fontsize=14)  

def get_immediate_subdirectories(a_dir):
    return [name for name in os.listdir(a_dir) if os.path.isdir(os.path.join(a_dir, name))]

def get_non_reports(d):
     return [x for x in get_immediate_subdirectories(d) if x != 'report']

def read_data():
    result = {}

    bench_names = get_non_reports(directory)
    for bench_name in get_non_reports(directory):
        result[bench_name] = {}
        for entity in get_non_reports(directory + '\\' + bench_name):
            name = file_to_name(entity)
            if name is None: continue

            with open(directory + '\\' + bench_name + '\\' + entity + '\\' + '\\base\\estimates.json') as f:
                dic = json.load(f)
                val = float(dic['mean']['point_estimate'])
                result[bench_name][name] = val
    return result

def plot(title, data, mult=1000.0, log=False):
    fig,ax = plt.subplots(1,1, figsize=(10,10))
    b = []
    for name, color in filters:
        b.append(
            ax.bar(
                name,
                data[name] / mult,
                width=0.85, 
                color=color, 
                align='center', 
                # edgecolor = 'black', 
                # linewidth = 1.0, 
                alpha=1,
                )
            )
    names = [x for x,_ in filters]
    y = [data[x] for x,_ in filters]
    [autolabel(r, ax) for r in b] # set val above each bar
    plt.ylabel('Speed (ns)')
    ax.tick_params(axis='x', labelsize=11)
    if log: ax.set_yscale('log')
    plt.title(title)
    # ax.legend(b, names, ncol = 3, loc = 'best', framealpha = 0.1)
    # ax.legend(b, names, loc = 'upper left', framealpha = 0.1)
    plt.show()

data = read_data()
plot('HyperLogLog Insert Time (Precision = 14)', data['insert'])
plot('HyperLogLog Count Time (Precision = 14)', data['count'], mult=1.0)
plot('HyperLogLog Insert 8K & Count Time (Precision = 14)', data['fill'], mult=1.0)