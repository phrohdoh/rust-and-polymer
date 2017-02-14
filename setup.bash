#!/bin/bash

log() {
    echo "[setup.bash] $1"
}

err() {
    log "Error: $1"
    exit 1
}

[[ -f ".env" ]] && err "File '.env' exists which implies that you have already setup this project!"

require() {
    type "$1" &>/dev/null || err "$2" && log "$1: ok"
}

PWD="$(pwd)"

[[ -f "$PWD/setup.bash" ]] || err "setup.bash must be ran from the project's root."

require cargo "\`cargo\` is missing! Please visit https://www.rust-lang.org/en-US/install.html"
require diesel "\`diesel\` is missing! Run \`cargo install diesel_cli\` to install it."
require npm "\`npm\` is missing! Please visit https://nodejs.org/en/download/"
require bower "\`bower\` is missing! Please visit https://bower.io/"

log "Copying example.env to .env" ; cp example.env .env

cd ./backend

log "Running \`diesel setup\`" ; diesel setup
log "Running \`diesel migration run\`" ; diesel migration run
log "Running \`cargo build\`" ; cargo build --quiet

cd ../frontend

log "Running \`bower install\`" ; bower install

[[ $? -eq 0 ]] && log "** Everything is ready to go! **" || err "Something failed, please report this at https://github.com/Phrohdoh/rust-and-polymer/issues"
