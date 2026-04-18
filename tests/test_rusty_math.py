from rusty_math import Vec3

a = Vec3(1, 0, 0)
b = Vec3(0, 1, 0)

print(a)                  # Vec3(1, 0, 0)
print(a.dot(b))           # 0.0  (perpendicular)
print(a.cross(b))         # Vec3(0, 0, 1)  (z-axis)
print(a.magnitude())      # 1.0
print(a.normalize())      # Vec3(1, 0, 0)  (already unit)
print(a.angle(b))         # 1.5707...  (π/2 radians)

print(a + b)              # Vec3(1, 1, 0)
print(a - b)              # Vec3(1, -1, 0)
print(a * 3)              # Vec3(3, 0, 0)

try:
    Vec3(0, 0, 0).normalize()
except ValueError as e:
    print(f"ValueError: {e}")