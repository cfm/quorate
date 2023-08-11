#!/bin/sh
# Translate Heroku's $PORT to Rocket's $ROCKET_PORT.
export ROCKET_PORT=$PORT
/usr/local/bin/proxy-solver-api
