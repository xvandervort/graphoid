"""Actor Pattern — Python equivalent of actors.gr

Python has no built-in actor system. This requires manual
implementation with threads and queues, or a library like Pykka.
"""

import threading
import queue
import time


class Actor:
    """Base actor class — Python has no built-in equivalent."""

    def __init__(self):
        self.mailbox = queue.Queue(maxsize=256)
        self.running = True
        self._thread = threading.Thread(target=self._loop, daemon=True)
        self._thread.start()

    def _loop(self):
        while self.running:
            try:
                envelope = self.mailbox.get(timeout=0.1)
                msg = envelope["msg"]
                reply_q = envelope.get("reply")
                result = self.on_message(msg)
                if reply_q is not None:
                    reply_q.put(result)
            except queue.Empty:
                pass
            except Exception as e:
                print(f"Actor crash: {e}")

    def on_message(self, msg):
        raise NotImplementedError

    def send(self, msg):
        self.mailbox.put({"msg": msg})

    def request(self, msg):
        reply_q = queue.Queue(maxsize=1)
        self.mailbox.put({"msg": msg, "reply": reply_q})
        return reply_q.get(timeout=5)

    def close(self):
        self.running = False
        self._thread.join(timeout=1)


# --- Counter actor ---
class Counter(Actor):
    def __init__(self, initial=0):
        self.count = initial
        super().__init__()

    def on_message(self, msg):
        if msg == "increment":
            self.count += 1
            return None
        elif msg == "get":
            return self.count
        return None


# --- Calculator actor ---
class Calculator(Actor):
    def on_message(self, msg):
        if isinstance(msg, dict):
            op = msg.get("op")
            a = msg.get("a", 0)
            b = msg.get("b", 0)
            if op == "add":
                return a + b
            elif op == "multiply":
                return a * b
        return "unknown"


# --- Usage ---
print("=== Counter ===")
c = Counter()
c.send("increment")
c.send("increment")
c.send("increment")
time.sleep(0.1)  # Python needs time for messages to process
result = c.request("get")
print(f"Count: {result}")  # 3
c.close()

print("\n=== Counter with initial state ===")
c2 = Counter(initial=100)
c2.send("increment")
time.sleep(0.1)
result = c2.request("get")
print(f"Count: {result}")  # 101
c2.close()

print("\n=== Calculator ===")
calc = Calculator()
result = calc.request({"op": "add", "a": 3, "b": 4})
print(f"3 + 4 = {result}")  # 7
result = calc.request({"op": "multiply", "a": 5, "b": 6})
print(f"5 * 6 = {result}")  # 30
calc.close()

# Python supervision would require yet more manual implementation.
# Libraries like Pykka or Thespian provide this, but nothing built-in.

print("\nActor demo complete!")
