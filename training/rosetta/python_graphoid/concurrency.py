"""Concurrency — Python equivalent of concurrency.gr

Python uses threading + queue for similar patterns.
Actors must be manually implemented.
"""

import threading
import queue
import time

# --- Spawn a task ---
# Python: use threading.Thread
result_queue = queue.Queue()

def worker():
    result_queue.put("hello from thread!")

t = threading.Thread(target=worker)
t.start()
t.join()
msg = result_queue.get()
print(msg)  # "hello from thread!"

# --- Channel-like communication ---
# Python: use queue.Queue (similar to buffered channel)
ch = queue.Queue()

def producer():
    for i in range(5):
        ch.put(i)
    ch.put(None)  # sentinel to signal "done" (no .close() on queues)

threading.Thread(target=producer).start()

total = 0
while True:
    val = ch.get()
    if val is None:
        break
    total += val
print(f"Sum: {total}")  # Sum: 10

# --- Actor pattern (manual implementation) ---
class Counter:
    def __init__(self):
        self.count = 0
        self.mailbox = queue.Queue()
        self.running = True
        self.thread = threading.Thread(target=self._run)
        self.thread.start()

    def _run(self):
        while self.running:
            try:
                msg, reply_q = self.mailbox.get(timeout=0.1)
                if msg == "increment":
                    self.count += 1
                    if reply_q:
                        reply_q.put(None)
                elif msg == "get":
                    if reply_q:
                        reply_q.put(self.count)
            except queue.Empty:
                pass

    def send(self, msg):
        self.mailbox.put((msg, None))

    def request(self, msg):
        reply_q = queue.Queue()
        self.mailbox.put((msg, reply_q))
        return reply_q.get()

    def close(self):
        self.running = False
        self.thread.join()


counter = Counter()
counter.send("increment")
counter.send("increment")
time.sleep(0.1)  # need to wait for processing
result = counter.request("get")
print(f"Count: {result}")  # Count: 2
counter.close()

# --- Select (manual implementation) ---
# Python has no built-in select for queues
# You'd need selectors module for sockets, or poll multiple queues

# --- Timers ---
# Python: use threading.Timer
def on_timer():
    print("Timer fired!")

timer = threading.Timer(0.1, on_timer)
timer.start()
timer.join()

# --- Signals ---
import signal
import sys

def handle_sigint(signum, frame):
    print("Got SIGINT")
    sys.exit(0)

# signal.signal(signal.SIGINT, handle_sigint)
# No channel-based signal handling in Python

print("Concurrency demo complete!")
