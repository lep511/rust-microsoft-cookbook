#!/bin/bash
read -sp "Enter your GEMINI_API_KEY: " token
echo
echo "export GEMINI_API_KEY=$token" >> ~/.bashrc
source ~/.bashrc
