import random

tokens = [ "T", "V", "\n", "C" ]
weights = [ 1, 1, 2, 1 ]

data = random.choices(tokens, weights = weights, k = 1000000)

with open("data_long.txt", "w+") as f:
    f.write("".join(data));
