install:
	python -m pip install -Ue .[dev]

.venv:
	python -m venv .venv
	source .venv/bin/activate && make install
	echo 'run `source .venv/bin/activate` to activate virtualenv'

venv: .venv

test:
	python -m unittest -v PACKAGE_NAME
	python -m mypy -p PACKAGE_NAME

lint:
	python -m flake8 PACKAGE_NAME
	python -m ufmt check PACKAGE_NAME

format:
	python -m ufmt format PACKAGE_NAME

release: lint test clean
	flit publish

clean:
	rm -rf .mypy_cache build dist html *.egg-info

distclean: clean
	rm -rf .venv

init:
	@python init.py
