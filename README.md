# ruff-api

Experimental Python API for Ruff

[![version](https://img.shields.io/pypi/v/ruff-api.svg)](https://pypi.org/project/ruff-api)
[![license](https://img.shields.io/pypi/l/ruff-api.svg)](https://github.com/amyreese/ruff-api/blob/main/LICENSE)


NOTE: This is project is highly experimental and the API is likely to change.
Pin your dependencies accordingly.


Install
-------

```shell-session
$ pip install ruff-api
```


Usage
-----

```py
import ruff_api
```

Format the contents of a file in memory:

```py
code = ruff_api.format_string(filename, code)
```

Sort imports in memory:

```py
code = ruff_api.isort_string(filename, code)
```


License
-------

ruff-api is copyright Amethyst Reese, and licensed under the MIT license.
