#!/bin/sh

apt-get update

# Use sudo if system has sudo (docker doesn't by default)
opt_sudo=""
if which sudo >/dev/null ; then
    opt_sudo="sudo"
fi


# This command installs the development headers for FLTK on debian
xargs ${opt_sudo} apt-get -qq --no-install-recommends install <debian-fltk-dev-headers.txt

# This command installs the runtime deps for FLTK on debian (they should already be installed after installing dev-headers)
xargs ${opt_sudo} apt-get -qq --no-install-recommends install <debian-fltk-rt-deps.txt

# Other deps
xargs ${opt_sudo} apt-get -qq --no-install-recommends install <other-deps.txt
