# load-tester
A command-line utility for load testing of web apps.

### USAGE:
    load-tester [OPTIONS] --clients <clients> --host <host> --number <number> --scheme <scheme> --time <time>

### FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

### OPTIONS:
    -b, --body <body>             Request body (path to *.json file)
    -c, --clients <clients>       Total no of clients
        --headers <headers>...    List of Request header (space separated) (format: "key=value")
    -h, --host <host>             Host URL of WebApp
    -m, --method <method>         Request method [default: get]  [possible values: get, post, put, delete]
    -n, --number <number>         Number of requests to be sent by each client (Not required if -t, --time is used)
    -p, --paths <paths>...        List of URL paths (space separated) [default: /]
    -s, --scheme <scheme>         Scheme [possible values: http, https]
    -t, --time <time>             Total time (seconds) (Not required if -n, --number is used)