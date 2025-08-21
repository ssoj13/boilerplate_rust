import subprocess
import time
import sys
from pynput.keyboard import Key, Controller

keyboard = Controller()
proc = subprocess.Popen(['./target/release/egui_opengl_app'], stderr=subprocess.PIPE)
time.sleep(2)
keyboard.press(Key.esc)
keyboard.release(Key.esc)
time.sleep(1)
proc.wait()
print(f'Exit code: {proc.returncode}')
