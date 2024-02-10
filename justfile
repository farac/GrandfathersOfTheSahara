edit *SCENE:
    cd godot && godot {{SCENE}}

run *SCENE:
    cd godot && godot {{SCENE}}

debug *SCENE:
    cd godot && godot -d {{SCENE}}

dev *SCENE:
    #!/bin/bash
    
    cd godot
    godot -d {{SCENE}}&
    godot_pid=$!
    cd ..

    inotifywait --include "(.*\.so|.*\.tscn)" -e create,modify,move,delete -r -m . | \
    while read line; do
        trap 'break && kill -9 $godot_pid && exit' INT
        if [[ -n $godot_pid ]]; then
            kill -9 $godot_pid >> /dev/null
        fi
        cd godot
        godot -d {{SCENE}}&
        godot_pid=$!
        cd ..
    done

export:
    godot --headless --path . --export-release release GrandfathersOfTheSahara
