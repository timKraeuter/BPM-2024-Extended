Run the tool including the UI using the following command:

**Windows**
```bash
./bpmnanalyzer.exe -p 8080
```

**Linux**
```bash
./bpmnanalyzer -p 8080
```

The tool will be available at [http://localhost:8080](http://localhost:8080) and use the port `8081` to host the model checking service.
The served web application is located in the folder `./public`.

This binary of our tool runs the UI while the other binaries are just CLI applications.

**Print help on Windows**
```bash
./bpmnanalyzer.exe -h
```

**Print help on Linux**
```bash
./bpmnanalyzer -h
```