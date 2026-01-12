import time
import sys
import os
import requests

try:
    from watchdog.observers import Observer
    from watchdog.events import FileSystemEventHandler
except ImportError:
    print('Please install watchdog: pip install watchdog')
    sys.exit(1)

class ChangeHandler(FileSystemEventHandler):
    def on_modified(self, event):
        if event.src_path.endswith('.rs'):
            print(f'Change detected in {event.src_path}')
            # TODO: Implement robust parsing logic here to extract template parts.
            # This requires parsing the Rust file, finding the html! macro,
            # and extracting the static strings between dynamic expressions.
            
            # Example payload:
            # payload = {
            #     'id': f'{event.src_path}:42:10', # Must match macro expansion location
            #     'parts': ['<div>', '</div>']
            # }
            # try:
            #     requests.post('http://localhost:3000/_azumi/update_template', json=payload)
            #     print('Update sent!')
            # except Exception as e:
            #     print(f'Failed to send update: {e}')

if __name__ == '__main__':
    path = sys.argv[1] if len(sys.argv) > 1 else '.'
    event_handler = ChangeHandler()
    observer = Observer()
    observer.schedule(event_handler, path, recursive=True)
    observer.start()
    print(f'Watching {path} for changes...')
    print('Note: This is a reference watcher. You need to implement the parsing logic.')
    try:
        while True:
            time.sleep(1)
    except KeyboardInterrupt:
        observer.stop()
    observer.join()
