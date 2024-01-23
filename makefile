dev:
	python -m maturin develop -E dev

install:
	python -m pip install -e .[dev]

.venv:
	python -m venv .venv
	source .venv/bin/activate && make install
	echo 'run `source .venv/bin/activate` to activate virtualenv'

venv: .venv

test:
	# python -m unittest -v ruff_api
	python -m mypy -p ruff_api

lint:
	python -m flake8 ruff_api
	python -m ufmt check ruff_api

format:
	python -m ufmt format ruff_api

release: lint test clean
	flit publish

clean:
	rm -rf .mypy_cache build dist html *.egg-info ruff_api/*.so

distclean: clean
	rm -rf .venv
