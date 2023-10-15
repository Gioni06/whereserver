# Whereserver
Whereserver is a command line utility to find the PID and URL of a development server that's running on your machine.

I wrote this utility because I often forget which port a development server is running on.
Also sometimes a server is running in some background process, and I have to manually find the PID of the process and kill it.

```bash
sh install.sh
```

```bash
$ whereserver
| Port | PID   | URL                     |
|------|-------|-------------------------|
|   80 | 2410  | http://127.0.0.1:80     |
| 5174 | 37259 | http://127.0.0.1:5174   |
```