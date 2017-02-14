# Polyrust

Combining [Polymer](http://polymer-project.org/) and [Rust](http://rust-lang.org/) (via [Rocket](https://rocket.rs/) and [Diesel](http://diesel.rs))

### Setup

```
# from the project root
$ ./setup.bash
```

### Running the backend

```
# from the project root
$ cd backend ; cargo run
```

### Running the frontend

```
# from the project root
$ cd frontend ; polymer serve
```

### Viewing the UI

You will need to run your browser with CORS disabled until I properly respond to CORS requests.

I do this on macOS with this function in my shell:

```bash
google-chrome-no-cors() {
    open -a "Google Chrome" --args --disable-web-security --user-data-dir
}
```

To use this open a bash instance and run:

```
$ google-chrome-no-cors
```

Which will launch Google Chrome in an insecure mode so I don't have to muck with CORS yet.

Now you can play around with the frontend by navigating to `localhost:8080` (or whichever port `polymer serve` picks).