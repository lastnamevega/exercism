def distance(strand_a: str, strand_b: str) -> int:
    if len(strand_a) != len(strand_b):
        raise ValueError('Strand lengths must be equal')

    return sum(a != b for a, b in zip(strand_a, strand_b))
