# OpenAI Proxy

## Description
REST API server, works as proxy to OpenAI API server. Takes from config.toml list of OpenAI API keys and base URL which target to OpenAI API server (destination) and forward all requests from client to destination OpenAI server, wait for response and return it back to client. Using OpenAI API keys from list preloaded form config.toml file. If response from destination server has error with http code 429, then next OpenAI API key from list became current.
Access logic. Functionality to verify permission based on authorization header with bearer token. List of valid access keys are stored in the settings file. If client's request provide correct access key in the authorization header or in settings file not have access keys list, then request will be processed without restriction, according proxy logic, else client's requests will be rejected in client get 401 not authorized error. 
Server must be incapsulated into the Docker, user provides to docker config.toml and directory for log file.

## Technology stack 
- **Rust** language
- **Axum** http server 

## Docker image
rutmir/openai-proxy:latest