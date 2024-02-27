#!/usr/bin/env python

"""Generate the scalability plot for the article."""

import argparse
import json
from scipy import stats
import matplotlib.pyplot as plt

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("file", help="JSON file with benchmark results")

args = parser.parse_args()

with open(args.file) as f:
    results = json.load(f)["results"]

all_means = [b["mean"] for b in results]
blocks = ['5x1', '10x1', '15x1', '16x1', '17x1', '20x1', '5x3', '5x5', '3x10', '3x20']

plt.plot(blocks, all_means, label="average runtime")

plt.xlabel("Number of branches x Length of branches")
plt.ylabel("Average runtime in s")
plt.legend()
plt.savefig('./parallel-branches-benchmark.pdf')
plt.show()

