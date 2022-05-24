#!/bin/bash

# Constants
EXAMPLE_CONFIG_FILE='pecan.example.toml'
CONFIG_FILE='pecan.toml'
SECRET_KEY_ENTRY='secret_key'

# Generate the new file
if [ ! -f "$CONFIG_FILE" ]; then
  echo "Creating a new config file. You may need to edit this text file to suit your needs."
  cp "$EXAMPLE_CONFIG_FILE" "$CONFIG_FILE"
fi

# Add the secret key to the file for release mode if it's not already present
if ! grep -q -m 1 "$SECRET_KEY_ENTRY" "$CONFIG_FILE"; then
  echo "Generating a new secret key with OpenSSL and adding it to your config."

  SECRET_KEY=$(openssl rand -base64 32)

  echo "" >> "$CONFIG_FILE" &&
  echo "" >> "$CONFIG_FILE" &&
  echo "# This is the secret key, used for encrypting authentication data." >> "$CONFIG_FILE" &&
  echo "# Don't touch it, or all logged-in users will be logged out. Don't share it either." >> "$CONFIG_FILE" &&
  echo "[release]" >> "$CONFIG_FILE" &&
  echo "$SECRET_KEY_ENTRY = \"$SECRET_KEY\"" >> "$CONFIG_FILE"
fi
