# Rosetta Stone: Standard Library Usage — Python
# Demonstrates: json, statistics, random, regex, time

import json
import statistics
import random
import re
from datetime import datetime, timedelta
import time

# ===== JSON =====
print("=== JSON ===")

json_str = '{"name": "Alice", "scores": [95, 87, 92], "active": true}'
data = json.loads(json_str)
print(f"Name: {data['name']}")
print(f"First score: {data['scores'][0]}")

output = {"result": "success", "count": 3, "items": ["apple", "banana"]}
print(json.dumps(output))

nested = json.loads('{"users": [{"id": 1, "name": "Bob"}, {"id": 2, "name": "Carol"}]}')
for user in nested["users"]:
    print(f"  User {user['id']}: {user['name']}")

# ===== Statistics =====
print("\n=== Statistics ===")

scores = [88, 92, 75, 95, 80, 85, 90, 78]
print(f"Mean:   {statistics.mean(scores):.2f}")
print(f"Median: {statistics.median(scores)}")
print(f"Stdev:  {statistics.stdev(scores):.2f}")
print(f"Min:    {min(scores)}")
print(f"Max:    {max(scores)}")

# ===== Random =====
print("\n=== Random ===")

dice = random.randint(1, 6)
print(f"Dice roll: {dice}")

colors = ["red", "green", "blue", "yellow"]
picked = random.choice(colors)
print(f"Picked color: {picked}")

val = random.random()
print(f"Random float: {val:.4f}")

sample = random.sample(range(1, 50), 5)
print(f"Sample: {sorted(sample)}")

# ===== Regex =====
print("\n=== Regex ===")

text = "Contact support@example.com or sales@example.com"
emails = re.findall(r'[\w.]+@[\w.]+', text)
print(f"Emails: {emails}")

phone = "555-123-4567"
if re.match(r'\d{3}-\d{3}-\d{4}', phone):
    print(f"Valid phone: {phone}")

censored = re.sub(r'[\w.]+@[\w.]+', '[REDACTED]', text)
print(f"Censored: {censored}")

# ===== Time =====
print("\n=== Time ===")

now = datetime.now()
print(f"Current time: {now}")
print(f"Year: {now.year}, Month: {now.month}, Day: {now.day}")

timestamp = time.time()
print(f"Unix timestamp: {timestamp:.0f}")

# --- Summary ---
# Python stdlib:
#   json.loads(str), json.dumps(obj)
#   statistics.mean(), .median(), .stdev()
#   random.randint(), .choice(), .random(), .sample()
#   re.findall(), .match(), .sub()
#   datetime.now(), time.time()
