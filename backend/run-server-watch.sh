#!/bin/sh

systemfd --no-pid -s http::1111 -- cargo watch -x run
