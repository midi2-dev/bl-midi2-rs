#!/bin/bash

if [ "$#" -ne 2 ]; then
  echo "Usage: $0 <old version> <new version>"
  exit 1
fi

OLD_VERSION=$1
NEW_VERSION=$2
ERROR_CODE=0

sed -I "" "s/version = \"$OLD_VERSION\"/version = \"$NEW_VERSION\"/" midi2/Cargo.toml || ERROR_CODE=2
sed -I "" "s/version = \"$OLD_VERSION\"/version = \"$NEW_VERSION\"/" midi2_proc/Cargo.toml || ERROR_CODE=2
sed -I "" "s/midi2 = { version = \"$OLD_VERSION\"/midi2 = { version = \"$NEW_VERSION\"/" README.md || ERROR_CODE=2

if [ "$ERROR_CODE" -ne 0 ]; then
  echo "Error: An error while replacing version strings."
  exit $ERROR_CODE
fi

echo "Version bumped from $OLD_VERSION to $NEW_VERSION"
