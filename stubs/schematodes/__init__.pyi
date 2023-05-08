class TwoSymbolSchemata:
    """TwoSymbolSchema"""

    def __init__(self) -> None:
        self.redescribed_schema: list[list[int]] = ...
        self.bubble_indices: list[list[int]] = ...

def schemer(one_symbol_schema: list[list[int]]) -> list[TwoSymbolSchemata]:
    """Sums two integers and returns a string using rust pyO3 bindings

    Parameters
    ----------
    one_symbol_schema: list[list[int]]
        A list of one-symbol schemata, where each element is a list of integers between 0 and 2 (inclusive).
        A 1 or 0 represents an "ON" or "OFF" state, respectively, while a 2 represents a wildcard.

    Returns
    -------
    list[TwoSymbolSchemata]
        A list of TwoSymbolSchemata objects representing the compressed schema.
    """
    ...
