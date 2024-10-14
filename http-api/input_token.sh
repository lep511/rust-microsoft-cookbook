#!/bin/bash
read -sp "Enter your REPLICATE_API_TOKEN: " token
echo
echo "export REPLICATE_API_TOKEN=$token" >> ~/.bashrc
source ~/.bashrc
