import time

start = time.perf_counter()

def fib(n):
    if n==1 or n==2:
        return 1
    return fib(n-1)+fib(n-2)
 
print(fib(42))

end = time.perf_counter()

t = end - start

print("Time spend:" + str(t) + "s")
