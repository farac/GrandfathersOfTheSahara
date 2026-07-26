edit *SCENE:
    cd godot && godot {{ SCENE }}

run *SCENE:
    cd godot && godot {{ SCENE }}

debug *SCENE:
    cd godot && godot -d {{ SCENE }}

dev *SCENE:
    #!/bin/bash

    cd godot
    godot -d {{ SCENE }}&
    godot_pid=$!
    cd ..

    inotifywait --include "(.*\.so|.*\.tscn)" -e create,modify,move,delete -r -m . | \
    while read line; do
        trap 'break && kill -9 $godot_pid && exit' INT
        if [[ -n $godot_pid ]]; then
            kill -9 $godot_pid >> /dev/null
        fi
        cd godot
        godot -d {{ SCENE }}&
        godot_pid=$!
        cd ..
    done

build *FLAG:
    cd rust && bacon build {{ FLAG }}

check:
    cd rust && bacon clippy

test:
    cd rust && bacon test

format:
    cd rust && cargo fmt

# These files only run inside Godot, so `cargo test` can't cover them.
# Skip them here. Coverage is checked on the rest.
cov_ignore := 'game\.rs|components/|entities\.rs|deck\.rs|/player|/tile\.rs|treasure\.rs|scenes\.rs|ui\.rs|util\.rs|input\.rs|flags\.rs|loader|lib\.rs'

coverage:
    cd rust && cargo llvm-cov --ignore-filename-regex '{{ cov_ignore }}'

coverage-gate:
    cd rust && cargo llvm-cov --summary-only --fail-under-lines 90 --ignore-filename-regex '{{ cov_ignore }}'

release:
    godot --headless --path . --export-release release GrandfathersOfTheSahara
