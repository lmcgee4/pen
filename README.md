# **pen**

**pen** is an easy-to-use tool for effortlessly managing and switching between virtual environments with specific Python versions.

## Features

- Create virtual environments with specific Python versions.
- Activate and deactivate virtual environments seamlessly.
- Simplified command interface for ease of use.

## Installation

TODO

## Usage

`pen {activate|deactivate|create} [options]`

### Commands

- `activate`:
  Activate the virtual environment.

  `pen activate`

- `deactivate`:
  Deactivate the virtual environment.

  `pen deactivate`

- `create --pyversion=VERSION`:
  Create a new virtual environment with the specified Python version.

  `pen create --pyversion=3.11.9`

### Options

- `-h`, `--help`:
  Show the help message.

## Example

```sh
# Create a virtual environment with Python version 3.11.9 in the current directory
pen create --pyversion=3.11.9

# Activate the virtual environment in the current directory
pen activate

# Deactivate the virtual environment in the current directory
pen deactivate
```


## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub, even for the smallest bug or the smallest idea.

## License

This project is licensed under the `MIT` License. See the LICENSE file for details.
