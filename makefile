install:
	python -m pip install -e .[dev]

version:
	python -m scripts.ruff_version

.venv:
	python -m venv .venv
	source .venv/bin/activate && make install
	echo 'run `source .venv/bin/activate` to activate virtualenv'

venv: .venv

test:
	python -m pytest --verbose
	python -m mypy -p ruff_api

lint:
	python -m flake8 ruff_api
	python -m ufmt check ruff_api
	python scripts/validate_formatting.py

format:
	python -m ufmt format ruff_api

release: test lint
	@echo "\nPush tags to github and let CI handle it!\n"
	@exit 1

clean:
	rm -rf .mypy_cache build dist html *.egg-info ruff_api/*.so

distclean: clean
	rm -rf .venv
