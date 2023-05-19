import schematodes as sc

tss = sc.schemer(
    [
        [0, 2, 0, 2],
        [2, 0, 0, 2],
        [0, 2, 2, 0],
        [2, 0, 2, 0],
        [1, 1, 2, 2],
    ]
)
for c in tss:
    print("===========================")
    print(f"{c.redescribed_schema} ----- \n {c.bubble_indices}")

print()
print("-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-")
print()

tss = sc.schemer(
    [
        [2, 0, 1, 2, 0, 1],  # f1'
        [2, 0, 1, 2, 1, 0],  # f2'
        [2, 0, 1, 0, 2, 1],  # f3'
        [2, 0, 1, 0, 1, 2],  # f4'
        [2, 0, 1, 1, 2, 0],  # f5'
        [2, 0, 1, 1, 0, 2],  # f6'
        [0, 0, 1, 2, 2, 1],  # f7'
        [0, 0, 1, 2, 1, 2],  # f8'
        [0, 0, 1, 1, 2, 2],  # f9'
        [1, 0, 1, 2, 2, 0],  # f10'
        [1, 0, 1, 2, 0, 2],  # f11'
        [1, 0, 1, 0, 2, 2],  # f12'
    ]
)
for c in tss:
    print("===========================")
    print(f"{c.redescribed_schema} ----- \n {c.bubble_indices}")

print()
print("-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-")
print()

tss = sc.schemer(
    [
        [1, 0, 2, 2, 0, 2, 0],  # f1'
        [1, 0, 2, 2, 0, 0, 2],  # f2'
        [1, 0, 2, 0, 2, 2, 0],  # f3'
        [1, 0, 2, 0, 2, 0, 2],  # f4'
        [1, 2, 0, 2, 0, 2, 0],  # f5'
        [1, 2, 0, 2, 0, 0, 2],  # f6'
        [1, 2, 0, 0, 2, 2, 0],  # f7'
        [1, 2, 0, 0, 2, 0, 2],  # f8'
        [1, 2, 1, 2, 0, 2, 0],  # f9'
        [1, 2, 1, 2, 0, 0, 2],  # f10'
        [1, 2, 1, 0, 2, 2, 0],  # f11'
        [1, 2, 1, 0, 2, 0, 2],  # f12'
        [2, 1, 0, 2, 0, 2, 0],  # f13'
        [2, 1, 0, 2, 0, 0, 2],  # f14'
        [2, 1, 0, 0, 2, 2, 0],  # f15'
        [2, 1, 0, 0, 2, 0, 2],  # f16'
    ]
)
for c in tss:
    print("===========================")
    print(f"{c.redescribed_schema} ----- \n {c.bubble_indices}")

print()
print("-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-")
print()

tss = sc.schemer(
    [
        [1, 2, 0, 0, 1],  # f1'
        [1, 0, 2, 0, 1],  # f2'
        [1, 2, 0, 1, 0],  # f3'
        [2, 1, 0, 0, 1],  # f4'
        [2, 1, 0, 1, 0],  # f5'
    ]
)
for c in tss:
    print("===========================")
    print(f"{c.redescribed_schema} ----- \n {c.bubble_indices}")

print()
print("-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-")
print()

tss = sc.schemer(
    [
        [1, 2, 0, 0, 1],  # f1'
        [1, 0, 2, 0, 1],  # f2'
        [1, 2, 0, 1, 0],  # f3'
        # [2, 1, 0, 0, 1],  # f4'
        [2, 1, 0, 1, 0],  # f5'
    ]
)
for c in tss:
    print("===========================")
    print(f"{c.redescribed_schema} ----- \n {c.bubble_indices}")

print()
print("-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-")
print()

tss = sc.schemer(
    [
        [1, 2],  # f1'
        [2, 1],  # f2'
    ]
)
for c in tss:
    print("===========================")
    print(f"{c.redescribed_schema} ----- \n {c.bubble_indices}")

print()
print("-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-")
print()

tss = sc.schemer(
    [
        [1, 1, 0, 0],  # f1'
        [0, 0, 1, 1],  # f2'
    ]
)
for c in tss:
    print("===========================")
    print(f"{c.redescribed_schema} ----- \n {c.bubble_indices}")

print()
print("-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-")
print()

tss = sc.schemer(
    [
        [1, 0, 0, 1],  # f1
        [0, 1, 1, 1],  # f2
        [1, 0, 1, 1],  # f3
    ]
)
for c in tss:
    print("===========================")
    print(f"{c.redescribed_schema} ----- \n {c.bubble_indices}")
print()
print("-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-")
print()

tss = sc.schemer(
    [
        [0, 1, 1, 1],  # f1'
        [1, 0, 2, 1],  # f2'
    ]
)
for c in tss:
    print("===========================")
    print(f"{c.redescribed_schema} ----- \n {c.bubble_indices}")

print()
print("-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-")
print()
tss = sc.schemer(
    [
        [2, 1, 0],
        [0, 1, 2],
        [1, 2, 0],
        [0, 2, 1],
    ]
)
for c in tss:
    print("===========================")
    print(f"{c.redescribed_schema} ----- \n {c.bubble_indices}")

print()
print("-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-")
print()
tss = sc.schemer(
    [
        [2, 1, 0],
        [0, 2, 1],
        [1, 2, 0],
        [0, 1, 2],
    ]
)
for c in tss:
    print("===========================")
    print(f"{c.redescribed_schema} ----- \n {c.bubble_indices}")


print()
print("-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-")
print()
tss = sc.schemer(
    [
        [0, 4, 2, 3],
        [0, 4, 3, 2],
        [4, 0, 2, 3],
        [4, 0, 3, 2],
    ]
)
for c in tss:
    print("===========================")
    print(f"{c.redescribed_schema} ----- \n {c.bubble_indices}")

print()
print("-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-")
print()
tss = sc.schemer(
    [
        [0, 4, 2, 3],
        [0, 4, 3, 2],
        [4, 0, 2, 3],
        [4, 0, 3, 2],
    ],
    max_symbol=4,
)
for c in tss:
    print("===========================")
    print(f"{c.redescribed_schema} ----- \n {c.bubble_indices}")
