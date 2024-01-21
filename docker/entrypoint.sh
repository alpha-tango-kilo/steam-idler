#!/bin/sh

echo "Updating steamcmd"
steamcmd +quit

echo
echo "Running steam-idler $*"
steam-idler "$@"
