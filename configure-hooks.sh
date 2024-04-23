#!/bin/bash

install() {
    pre-commit install --install-hooks
    pre-commit install --install-hooks -t commit-msg
    git config commit.template COMMIT_MESSAGE_TEMPLATE
}

uninstall() {
    pre-commit uninstall
    pre-commit uninstall -t commit-msg
    git config --unset commit.template
}

case "$1" in
    install)
        install
        ;;
    uninstall)
        uninstall
        ;;
    *)
        echo "Usage: $0 {install|uninstall}"
        exit 1
        ;;
esac
