#!/bin/bash

if [ -z "$1" ]; then
  echo "Usage: $0 <PAT>"
  exit 1
fi

PAT=$1

git config --global user.email "ben_leadbetter@hotmail.com "
git config --global user.name "GitHub Actions"
git remote set-url origin https://x-access-token:${PAT}@github.com/midi2-dev/bl-midi2-rs
