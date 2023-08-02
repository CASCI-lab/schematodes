import schematodes as sc

TruthType = list[tuple[list[int], list[list[int]]]]


def confirm(tss: list[sc.TwoSymbolSchemata], truth: TruthType):
    success = True
    messages: list[str] = []
    coverage = [0] * len(tss)
    for rep, bubbles in truth:
        truth_found = False
        for i, c in enumerate(tss):
            if rep in c.redescribed_schema:
                output_bubbles: set[tuple[int, ...]] = set(
                    tuple(b) for b in c.bubble_indices
                )
                true_bubbles: set[tuple[int, ...]] = set(tuple(b) for b in bubbles)
                if output_bubbles == true_bubbles:
                    truth_found = True
                    coverage[i] += 1
                    # not breaking because we want to test multiple coverage

        if not truth_found:
            success = False
            messages.append(f"true value {rep=}, {bubbles=} not found in output")

    for i, x in enumerate(coverage):
        orbit, bubbles = tss[i].redescribed_schema, tss[i].bubble_indices
        if x == 0:
            success = False
            messages.append(
                f"output value {orbit=}, {bubbles=} not covered by true values"
            )
        if x > 1:
            success = False
            messages.append(
                f"output value {orbit=}, {bubbles=} covered by multiple true values"
            )

    assert success, messages


def test_simple_case() -> None:  # explicit None to get type checks in body
    tss = sc.schemer(
        [
            [1, 2],  # f1'
            [2, 1],  # f2'
        ]
    )
    truth: TruthType = [
        ([2, 1], [[0, 1]]),
    ]
    confirm(tss, truth)


def test_no_symmetry() -> None:
    tss = sc.schemer(
        [
            [1, 1, 0, 0],  # f1'
            [0, 0, 1, 1],  # f2'
        ]
    )
    truth: TruthType = [
        ([1, 1, 0, 0], []),
        ([0, 0, 1, 1], []),
    ]
    confirm(tss, truth)


def test_pair_swaps() -> None:  # explicit None to get type checks in body
    tss = sc.schemer(
        [
            [0, 2, 0, 2],
            [2, 0, 0, 2],
            [0, 2, 2, 0],
            [2, 0, 2, 0],
            [1, 1, 2, 2],
        ]
    )
    truth: TruthType = [
        ([0, 2, 0, 2], [[0, 1], [2, 3]]),
        ([1, 1, 2, 2], []),
    ]
    confirm(tss, truth)


def test_full_symmetry() -> None:
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
    truth: TruthType = [
        ([0, 0, 1, 1, 2, 2], [[0, 3, 4, 5]]),
    ]
    confirm(tss, truth)


def test_overlapping_symmetry() -> None:
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
    truth: TruthType = [
        ([1, 2, 1, 0, 2, 0, 2], [[3, 4], [5, 6]]),
        ([1, 0, 2, 0, 2, 0, 2], [[1, 2], [3, 4], [5, 6]]),
        ([1, 2, 0, 0, 2, 0, 2], [[0, 1], [3, 4], [5, 6]]),
    ]
    confirm(tss, truth)


def test_transitivity_nontransference() -> None:
    tss = sc.schemer(
        [
            [2, 1, 0],
            [0, 2, 1],
            [1, 2, 0],
            [0, 1, 2],
        ]
    )
    tss2 = sc.schemer(
        [
            [2, 1, 0],
            [0, 1, 2],
            [1, 2, 0],
            [0, 2, 1],
        ]
    )
    truth: TruthType = [
        ([1, 2, 0], [[0, 1]]),
        ([0, 1, 2], [[0, 2]]),
        ([0, 1, 2], [[1, 2]]),
        ([0, 2, 1], [[0, 2]]),
    ]
    confirm(tss, truth)
    confirm(tss2, truth)


def test_more_symbols() -> None:
    tss = sc.schemer(
        [
            [0, 4, 2, 3],
            [0, 4, 3, 2],
            [4, 0, 2, 3],
            [4, 0, 3, 2],
        ]
    )
    truth: TruthType = [
        ([0, 4, 2, 3], [[0, 1], [2, 3]]),
    ]
    confirm(tss, truth)
