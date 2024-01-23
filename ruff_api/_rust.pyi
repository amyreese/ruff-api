class FormatOptions:
    def __init__(
        self,
        target_version: str | None = None,
        line_width: int | None = None,
        preview: bool = False,
    ): ...

def format_string(path: str, source: str, options: FormatOptions | None) -> str: ...
