# Benchmark: Degrees of Parallelism

This folder contains the models with growing **parallel branches** used in the paper.

## Generating/Unpacking models

We provide the models used in the paper **and** our CLI-application to generate these models.

**Model generation help:**
```bash
java -jar BPMNModelGenerator-1.0-all.jar -h
```
**Model generation:**
```bash
java -jar BPMNModelGenerator-1.0-all.jar -path="./models" -length-of-branches=20 -number-of-branches=20
```
Generates 400 models with 1-20 parallel branches of length 1-20 in the `./models` directory.
The source code for the CLI-application is available at [TBD after review](TODO).

The precomputed models are located in the zip `./models/models.zip`.

## Benchmark

**Windows:**
```bash
cd ../..
hyperfine.exe -V
```

**Linux:**
```bash
cd ../..
hyperfine -V
```

## Results

The benchmark results are found in the `results.json` file.