import collections
import random
import sys
import time

import requests

counts = collections.defaultdict(collections.Counter)
start = time.time()
paths = ("/a", "/b", "/c")

try:
    while True:
        path = random.choice(paths)
        resp = requests.get(f"http://localhost:3000{path}")
        print(path, resp.status_code, file=sys.stderr)
        counts[path][resp.status_code] += 1
        time.sleep(0.01)
except KeyboardInterrupt:
    pass

seconds = int(time.time() - start)
# minutes = max(1, round(seconds / 60))
minutes = seconds / 60
print("took", seconds, "seconds")

total = sum((sum(c.values()) for c in counts.values()))
print("total:", total, "requests")

for path in paths:
    print()
    path_counts = counts[path]
    path_total = sum(path_counts.values())
    print(path)
    print("total:", path_total, "requests")
    for status_code, count in sorted(path_counts.items()):
        rps = count / seconds
        rpm = count / minutes
        print(f"{status_code}: {count} responses ({100 * count / path_total:.1f}%, {rps:.1f} rps, {rpm:.1f} rpm)")
