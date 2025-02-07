#!/bin/bash

if [ "$#" -ne 2 ]; then
  echo "Usage: $0 <old version> <new version>"
  exit 1
fi

OLD_VERSION=$1
NEW_VERSION=$2
ERROR_CODE=0

if [[ "$OSTYPE" == "darwin"* ]]; then
  SED_CMD=("sed" "-i" "")
else
  SED_CMD=("sed" "-i")
fi

cat .github/scripts/files-with-current-version-string | xargs -I % "${SED_CMD[@]}" s/$OLD_VERSION/$NEW_VERSION/g % || ERROR_CODE=2

if [ "$ERROR_CODE" -ne 0 ]; then
  echo "Error: An error while replacing version strings."
  exit $ERROR_CODE
fi

echo "Version bumped from $OLD_VERSION to $NEW_VERSION"
