from tokiopool import TokioPoolExecutor
import time


def foo(x):
    time.sleep(5)
    print(x)


with TokioPoolExecutor(max_workers=1) as e:
    for i in range(100):
        e.submit(foo, str(i), b="B")
    time.sleep(60)
