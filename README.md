# hydrake

This is a trivial program that acts a redis client daemon, intended to run in a Kubernetes pod
with a redis instance, inserting a set of fixed values periodically.

Edit the `src/main.rs` to hard code the static values. This may be useful for some situations where
we want to hydrate a redis with non-customer controlled data like product codes.

#### minimal container build 

The included Dockerfile uses the `FROM ekidd/rust-musl-builder AS build` to compile with cargo and then we copy the dependencies into a `FROM scratch` empty container. The resulting OCI image has no shell, nothing but the dependencies for the web server.

