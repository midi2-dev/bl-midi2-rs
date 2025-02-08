#!/bin/bash

sudo apt-get update
sudo apt-get install -y git-flow

git fetch origin
git switch --track origin/main
git switch --track origin/develop

git config --global user.email "ben_leadbetter@hotmail.com"
git config --global user.name "CI"

git config gitflow.branch.master main
git config gitflow.branch.develop develop
git config gitflow.prefix.feature feature/
git config gitflow.prefix.bugfix bugfix/
git config gitflow.prefix.release release/
git config gitflow.prefix.hotfix hotfix/
git config gitflow.prefix.support support/
git config gitflow.prefix.versiontag ""
git config gitflow.path.hooks ""
