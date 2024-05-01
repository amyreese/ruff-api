from typing import List

class FormatOptions:
    def __init__(
        self,
        target_version: str | None = None,
        line_width: int | None = None,
        preview: bool = False,
    ): ...

def format_string(
    path: str, source: str, options: FormatOptions | None = None
) -> str: ...

class SortOptions:
    def __init__(
        self,
        first_party_modules: List[str] | None = None,
        standard_library_modules: List[str] | None = None,
    ): ...

def isort_string(path: str, source: str, options: SortOptions | None = None) -> str: ...
