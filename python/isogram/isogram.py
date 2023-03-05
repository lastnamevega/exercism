def is_isogram(string: str) -> bool:
    sanitized = string.translate(str.maketrans('', '', '- ')).lower()

    return len(sanitized) == len(set(sanitized))
