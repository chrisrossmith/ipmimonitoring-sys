bindgen wrapper.h -o bindings.rs -- -I./freeipmi/libipmimonitoring/
cat bindings_headers.rs bindings.rs > src/lib.rs

