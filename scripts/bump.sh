#!/usr/bin/env bash

RELEASE_TYPE=""
if [ "$1" = "patch" ]; then
  RELEASE_TYPE="patch"
elif [ "$1" = "minor" ]; then
  RELEASE_TYPE="minor"
elif [ "$1" = "major" ]; then
  RELEASE_TYPE="major"
else
  echo "Please provide a valid bump type"
  exit 1
fi

echo "Bump caido-convert"
cargo bump "$RELEASE_TYPE" -k caido-convert

echo "Bump convert"
cargo bump "$RELEASE_TYPE" -k convert

echo "Commit changes"
git add .
git commit -m "Bump version"
git push
