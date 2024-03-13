The tool with the UI is available [online](https://bpm-2024.whitefield-c9fed487.northeurope.azurecontainerapps.io/) but can also be run locally using the following command:

**Windows**
```bash
./bpmnanalyzer.exe -p 8080
```

**Linux**
```bash
./bpmnanalyzer -p 8080
```

For the final version, we will provide the same docker image used to deploy the tool [online](https://bpm-2024.whitefield-c9fed487.northeurope.azurecontainerapps.io/).
A docker image to run the tool is available [here](https://hub.docker.com/TBD).

The tool will be available at [http://localhost:8080](http://localhost:8080).
The served web application is located in the folder `./public`.

In the top right corner, one can change between the provided example BPMN models, which match the models shown in the "Fixable Soundness Checking" section.
Alternatively, one can upload custom models using the buttons in the bottom left.

This binary of our tool runs the UI, while the other binaries are just CLI applications.

**Print help on Windows**
```bash
./bpmnanalyzer.exe -h
```

**Print help on Linux**
```bash
./bpmnanalyzer -h
```