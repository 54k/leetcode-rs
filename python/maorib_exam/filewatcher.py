import time
import sys
import zlib
from os import path

from watchdog.observers import Observer
from watchdog.events import FileSystemEventHandler

monitor_stop = False

class EventHandler(FileSystemEventHandler):
    def __init__(self, fname):
        self.fname = fname
        with open(self.fname, 'rb') as fp:
            self.hash = zlib.crc32(fp.read())

    def on_any_event(self, event):
        global monitor_stop
        print("Got event: %s" % event)
        if event.event_type == "deleted":
            monitor_stop = True
        elif event.event_type == "modified":
            hash = ""
            if path.exists(self.fname):
                with open(self.fname, 'rb') as fp:
                    if fp.name != self.fname:
                        print(f"File name changed from {self.fname} to {fp.name}")
                    hash = zlib.crc32(fp.read())
                if self.hash != hash:
                    print(f"Hash mismatch was {self.hash}, become {hash}")
                self.hash = hash
            else:
                self.hash = None

def start_and_wait(fname):
    print(f"Start watching file {fname}")
    event_handler = EventHandler(fname)
    global monitor_stop
    monitor_stop = False
    observer = Observer()
    observer.schedule(event_handler, fname, recursive=True)
    try:
        observer.start()
        while not monitor_stop:
            time.sleep(1)
        observer.stop()
    except KeyboardInterrupt:
        observer.stop()
        exit(0)
    finally:
        observer.join()
        print(f"End watching file {fname}")

if __name__ == "__main__":
    fname = sys.argv[1]
    if not fname:
        raise Exception("Please specify file name to watch as an argument")

    while True:
        while not path.exists(fname):
            print(f"Awaiting file {fname} to be created")
            time.sleep(5)
        start_and_wait(fname)
