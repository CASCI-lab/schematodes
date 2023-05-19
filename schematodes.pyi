class TwoSymbolSchemata:
    """TwoSymbolSchema

    Fields
    ------
    redescribed_schema: list[list[int]]
        The list of redescribed one-symbol schema, each represented as a list of 0s, 1s, and 2s.
    bubble_indices: list[list[int]]
        The list of bubble group indices. Each entry is a list of indices that can be arbitrarily permuted without affecting the redescribed schema set.
    signature: list[tuple[int,int,int]]
        The number of 0s, 1s, and 2s in each redescribed schemata.

    """

    def __init__(self) -> None:
        self.redescribed_schema: list[list[int]] = ...
        self.bubble_indices: list[list[int]] = ...
        self.signature: list[tuple[int, int, int]] = ...

def schemer(
    one_symbol_schema: list[list[int]],
    max_symbol: int | None = None,
) -> list[TwoSymbolSchemata]:
    """Redescribe a list of one-symbol schema as a list of two-symbol schema.

    Parameters
    ----------
    one_symbol_schema: list[list[int]]
        A list of one-symbol schemata, where each element is a list of integers between 0 and 2 (inclusive).
        A 1 or 0 represents an "ON" or "OFF" state, respectively, while a 2 represents a wildcard (#).
    max_symbol: int | None
        The largest symbol that should be considered possible when calculating signature. If None, the largest observed symbol is used.

    Returns
    -------
    list[TwoSymbolSchemata]
        A list of TwoSymbolSchemata objects representing the compressed schema.
    """
    ...
