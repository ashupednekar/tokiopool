from tokiopool import TokioPoolExecutor

with TokioPoolExecutor(max_workers=10) as e:
    e.submit(lambda x: x, "a", b="B")
