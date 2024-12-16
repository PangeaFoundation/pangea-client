# Pangea Client

`pangea-client` is a set of libraries for different languages to interface with [Pangea API](https://docs.pangea.foundation/).

When using `python` the data is delivered in Arrow or JSON, developers are required to provide their own types.

<br>

## Getting started

Access to the API via `pangea-client` requires credentials, please [apply for access](https://pangea.foundation/get-access) first.

Once credentials are issued, they will need to be set in the environment variables.

The easiest way to use these credentials is to create a `.env` file in the project root folder and populate it like so:

```sh
PANGEA_USERNAME=xxxxx
PANGEA_PASSWORD=xxxxx
```

<br>

## Examples

Checkout the github repository separately for example code.

<br>

## Setup

Before proceeding, poetry needs to be installed **globally**.

```sh
curl -sSL https://install.python-poetry.org | python3 -
```

<br>

⚠️ WARNING ⚠️

**_DO NOT_** install using pip, **_only_** via the above script.

<br>

After poetry is installed, `cd` to the directory where you cloned the repo (python sub-folder for megarepo) and run:

```sh
poetry install
```

<br>

## Running the Examples

```sh
poetry run python3 examples/arrow-blocks.py
poetry run python3 examples/jsonstream-status.py
poetry run python3 examples/jsonstream-blocks.py
```

or setup a venv shell with poetry first:

```sh
poetry shell
```

then you can run

```sh
python3 examples/arrow-blocks.py
```
