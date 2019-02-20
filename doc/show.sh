#!/bin/sh

pandoc $1 -s -t man | /usr/bin/man -l -
