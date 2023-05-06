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
