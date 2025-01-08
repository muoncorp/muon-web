#!/bin/sh

kill $(ps ax | grep muon-web-server  | grep opt | head -n 1 | awk '{ print $1 }')
echo muon-web-server stopped.