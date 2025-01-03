#!/bin/sh

# Install certbot
#emerge app-crypt/certbot app-crypt/certbot-nginx

# Must be run as 'root' user
certbot --nginx -d muon.co