The tool with the UI is available [online](https://timkraeuter.com/bpmn-analyzer-js/) but can also be run locally using docker or the provided binary.

The tool will be available at [http://localhost:8080](http://localhost:8080).

In the top right corner, one can change between the provided example BPMN models, which match the models shown in the "Fixable Soundness Checking" section.
Alternatively, one can upload custom models using the buttons in the bottom left.

# Docker
```bash
docker pull tkra/rust_bpmn_analyzer
docker run -p 8080:8080 tkra/rust_bpmn_analyzer
```

# Binary

**Windows**
```bash
./bpmnanalyzer.exe -p 8080
```

**Print help on Windows**
```bash
./bpmnanalyzer.exe -h
```

**Linux**
```bash
./bpmnanalyzer -p 8080
```

**Print help on Linux**
```bash
./bpmnanalyzer -h
```