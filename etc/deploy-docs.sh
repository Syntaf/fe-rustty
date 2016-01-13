#!/bin/bash

set -ex

rev=$(git rev-parse --short HEAD)

cd target/doc
git init
git config user.name "Grant Mercer"
git config user.email "gmercer015@gmail.com"

git remote add upstream "https://${GITHUB_TOKEN}@github.com/${TRAVIS_REPO_SLUG}.git"
git fetch upstream
git reset upstream/gh-pages

echo "<meta http-equiv=refresh content=0;url=ruik/index.html>" > index.html

touch .

git add -A
git commit -m "rebuild pages at ${rev}"
git push upstream HEAD:gh-pages
