SRCS:=ruff_api
EXTRAS:=dev

ifeq ($(OS),Windows_NT)
	ACTIVATE:=.venv/Scripts/activate
else
	ACTIVATE:=.venv/bin/activate
endif

UV:=$(shell uv --version)
ifdef UV
	VENV:=uv venv
	PIP:=uv pip
else
	VENV:=python -m venv
	PIP:=python -m pip
endif

.venv:
	$(VENV) .venv

venv: .venv
	source $(ACTIVATE) && make install
	echo 'run `source $(ACTIVATE)` to use virtualenv'

install:
	$(PIP) install -Ue .[$(EXTRAS)]

version:
	python -m scripts.ruff_version

test:
	python -m pytest --verbose
	python -m mypy -p ruff_api

lint:
	cargo clippy
	python -m flake8 ruff_api
	python -m ufmt check ruff_api
	python scripts/ruff_version.py
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
