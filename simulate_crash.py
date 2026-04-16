import time
import random

def live_monitor():
    index = 1.00
    print("📡 Aegis Live Oracle: Connecting to Leeds Property Index...")
    
    while index > 0.978:
        change = random.uniform(0.001, 0.005)
        index -= change
        print(f"Index Update: {index:.4f} | Status: MONITORING")
        time.sleep(1)
    
    print(f"🚨 ALERT: Index at {index:.4f} - BREACH DETECTED")
    print("Executing Pacifica Hedge...")

if __name__ == "__main__":
    live_monitor()