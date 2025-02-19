# Build Stage
FROM ghcr.io/evanrichter/cargo-fuzz:latest as builder

## Add source code to the build stage.
ADD . /src
WORKDIR /src

RUN mkdir /testsuite && \
    sh -c "find tests -name '*.txt' -exec cp {} /testsuite/ \;" \
    sh -c "find tests -name '*.csv' -exec cp {} /testsuite/ \;"

RUN echo building instrumented harnesses && \
    bash -c "pushd fuzz && cargo +nightly -Z sparse-registry fuzz build && popd" && \
    mv fuzz/target/x86_64-unknown-linux-gnu/release/collapse_fuzz /collapse_fuzz && \
    echo done

RUN echo building non-instrumented harnesses && \
    export RUSTFLAGS="--cfg fuzzing -Clink-dead-code -Cdebug-assertions -C codegen-units=1" && \
    bash -c "pushd fuzz && cargo +nightly -Z sparse-registry build --release && popd" && \
    mv fuzz/target/release/collapse_fuzz /collapse_fuzz_no_inst && \
    echo done

# Package Stage
FROM rustlang/rust:nightly

COPY --from=builder /collapse_fuzz /collapse_fuzz_no_inst /
COPY --from=builder /testsuite/  /testsuite/
