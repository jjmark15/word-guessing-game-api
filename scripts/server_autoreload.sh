#!/usr/bin/env bash

cargo watch -i tests/ -d 5 -cx 'run --bin word-guessing-game-api'