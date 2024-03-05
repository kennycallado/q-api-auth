FROM scratch

# --build-arg PACKAGE_NAME=${package_name}
ARG PACKAGE_NAME="q-app-manager" 
ARG TARGET="x86_64-unknown-linux-musl"

COPY ./target/${TARGET}/release/${PACKAGE_NAME} /bin/rocket
COPY ./Rocket.toml /

# WORKDIR /
CMD [ "rocket" ]
