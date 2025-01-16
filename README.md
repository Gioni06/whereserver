# Whereserver
**Whereserver** is a command-line utility designed to help you quickly find the PID and URL of a development server running on your local machine. It’s the perfect tool for developers who frequently manage multiple projects and servers.

## Why?

Managing development servers can be tedious, especially when:
	•	You forget which port your server is running on.
	•	A server is running in the background, making it difficult to locate the PID to terminate it.

Whereserver simplifies this process by instantly displaying the relevant information for all active development servers.

## Install

```bash
sh install.sh
```

## Use

```bash
$ whereserver
| Port | PID   | URL                     |
|------|-------|-------------------------|
|   80 | 2410  | http://127.0.0.1:80     |
| 5174 | 37259 | http://127.0.0.1:5174   |
```
