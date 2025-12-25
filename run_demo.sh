#!/bin/bash

echo "Shutting down any process on port 8081..."

# Find and kill any process using port 8081
if lsof -ti:8081 > /dev/null 2>&1; then
    lsof -ti:8081 | xargs kill -9
    echo "Process on port 8081 terminated"
else
    echo "No process found on port 8081"
fi

echo "Starting demo application..."
# cd demo && cargo r