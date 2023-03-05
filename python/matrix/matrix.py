from typing import List


class Matrix:
    def __init__(self, matrix_string: str):
        rows = matrix_string.split('\n')
        self.matrix = [list(map(int, row.split(' '))) for row in rows]

    def row(self, index: int) -> List[int]:
        return self.matrix[index - 1]

    def column(self, index: int) -> List[int]:
        return [row[index - 1] for row in self.matrix]
