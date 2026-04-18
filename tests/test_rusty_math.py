import rusty_math

print(rusty_math.add(3, 4))
print(rusty_math.subtract(10, 3))
print(rusty_math.multiply(6, 7))
print(rusty_math.divide(10, 4))

try:
    rusty_math.divide(1, 0)
except ZeroDivisionError as e:
    print(f"ZeroDivisionError: {e}")