#!/usr/bin/env bash

# Define variables
PEN_DIR="$HOME/.pen"
BASHRC="$HOME/.bashrc"
PEN_SCRIPT_URL="https://raw.githubusercontent.com/azomDev/pen/main/files/main.sh"
UPDATER_SCRIPT_URL="https://raw.githubusercontent.com/azomDev/pen/main/files/updater.sh"
VERSION_TXT_URL="https://raw.githubusercontent.com/azomDev/pen/main/files/version.txt"
PEN_EXECUTABLE_URL="https://raw.githubusercontent.com/azomDev/pen/main/files/core"

# Check if the .pen directory exists, if yes, exit
if [ -d "$PEN_DIR" ]; then
    echo "Directory $PEN_DIR already exists. Exiting."
    exit 1
fi

# Check if .bashrc file exists, if no, exit
if [ ! -f "$BASHRC" ]; then
    echo "File $BASHRC does not exist. Exiting."
    exit 1
fi

# Create .pen directory in the home of the user
mkdir -p "$PEN_DIR"

# Curl the main.sh script and core executable from GitHub and put them in the .pen directory
curl -o "$PEN_DIR/main.sh" "$PEN_SCRIPT_URL"
curl -o "$PEN_DIR/updater.sh" "$UPDATER_SCRIPT_URL"
curl -o "$PEN_DIR/version.txt" "$VERSION_TXT_URL"
curl -L -o "$PEN_DIR/core" "$PEN_EXECUTABLE_URL"

# Make the core executable
chmod +x "$PEN_DIR/core"

# Create pythonVersions directory inside .pen
mkdir -p "$PEN_DIR/pythonVersions"

# Add alias to the bashrc file
if ! grep -q "alias pen=" "$BASHRC"; then
    {
        echo -e '\n# pen'
        echo 'alias pen=". $HOME/.pen/main.sh"'
    } >>"$BASHRC"
    echo "Alias for pen added to $BASHRC"
else
    echo "Alias for pen already exists in $BASHRC"
fi
echo "Installation complete. Please restart your terminal session or run 'source ~/.bashrc' to apply the changes."
