#!/usr/bin/env bash
## Author: Prasanna V. Loganathar

run() {
    init
    build
    manifest
}

init() {
    if [ -z "$__INIT" ]; then 
        __INIT="1"
    else
        return 0
    fi
    trap "echo '> script: incomplete termination requested'" TERM   
    set -e
    # script dir
    local dir=$( dirname "${BASH_SOURCE[0]}" )
    pushd . > /dev/null
    cd ${dir}
    setup_vars
}

setup_vars() {
    # PKG_BASE_NAME="$(basename $(pwd))"
    PKG_BASE_NAME="main"
    BUILD_TARGET="${BUILD_TARGET:-${PKG_BASE_NAME}}"
    BUILD_MODE="${BUILD_MODE:-release}"
    BUILD_TARGET_EXE="./target/${BUILD_MODE}/${BUILD_TARGET}.exe"
    MANIFEST_PATH="./src/manifest.xml"
}

build() {
    init
    echo "> build: start"
    cargo build --${BUILD_MODE}
    echo "> build: done"
}

manifest() {
    init
    if [ ! -f "$BUILD_TARGET_EXE" ]; then
        echo "> target not found: $BUILD_TARGET_EXE"
        exit
    fi
    echo "> embed manifest: start"
    local CMD="mt -nologo -manifest $MANIFEST_PATH -outputresource:$BUILD_TARGET_EXE;#1"
    local VCCMD="C:\\Program Files (x86)\\Microsoft Visual Studio\\2017\\Community\\VC\\Auxiliary\\Build\\vcvars64.bat"
    if [ ! $(which mt &> /dev/null) ] ; then
        cmd /V /C "set _VCCMD=$VCCMD & dir & "'!_VCCMD!'" & $CMD"
    else
        $CMD
    fi
    echo "> embed manifest: done"
}

if [ -z "$@" ]; then
    run
else 
    "$@"
fi;
popd > /dev/null