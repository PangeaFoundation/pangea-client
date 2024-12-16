# Pangea Client

`pangea-client` is a set of libraries for different languages to interface with [Pangea API](https://docs.pangea.foundation/).

When using `node` the data is delivered in Arrow or JSON, developers are required to provide their own types.

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

Before proceeding, make sure to have node installed **globally**.
Easiest way is to install `nvm`, and use that to install the `node` version listed in `.nvmrc`. See the following:

- [`nvm`](https://github.com/nvm-sh/nvm/blob/master/README.md)
- [`node.js` ](https://nodejs.org/en/learn/getting-started/introduction-to-nodejs)

<br>

After node is installed, `cd` to the directory where you cloned the repo (`node` sub-folder for megarepo) and run:

```sh
npm i
```

<br>

## Running the Examples

To run any file in the `examples` folder, use the template

```sh
npm run example:${EXAMPLE_FILE_NAME}
```

Like so:

```sh
npm run example:arrow-blocks
npm run example:jsonstream-blocks
npm run example:jsonstream-status
...
```

<br>

NOTE: When adding a new example file, make sure to add the corresponding `package.json` script task to be able to run it.
