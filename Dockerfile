FROM steamcmd/steamcmd:alpine
COPY --chmod 111 target/release/steam-idler /usr/bin/steam-idler
ENTRYPOINT ["steam-idler"]
