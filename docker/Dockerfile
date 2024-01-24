FROM rust:slim-bookworm as builder
WORKDIR /usr/src/steam-idler
COPY . .
RUN cargo build  \
    --features docker \
    --release

# Potential images (smallest to largest):
# https://hub.docker.com/r/mlinarik/steam/tags
# https://hub.docker.com/r/tianon/steam/tags
# https://hub.docker.com/r/josh5/steam-headless/tags
FROM steamcmd/steamcmd:debian-bookworm
COPY --from=builder /usr/src/steam-idler/target/release/steam-idler /usr/bin/steam-idler

# TODO: mount this from a volume instead of copying at build-time
# https://github.com/steamcmd/docker/issues/48
# https://gaming.stackexchange.com/questions/240073/where-is-the-steam-authorization-file
COPY secrets/ssfn* /

COPY --chown=root:root --chmod=755 docker/libsteam_api.so /usr/local/lib
COPY docker/entrypoint.sh /

ENTRYPOINT ["/entrypoint.sh", "480", "1m"]
