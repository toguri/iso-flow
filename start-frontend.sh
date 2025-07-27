#!/bin/bash
cd frontend
./gradlew jsBrowserDevelopmentRun --continuous &
echo "Frontend started at http://localhost:8080"
echo "PID: $!"