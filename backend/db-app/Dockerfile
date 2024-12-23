FROM debian:bookworm-slim AS prod

ENV TINI_VERSION v0.19.0
ADD https://github.com/krallin/tini/releases/download/${TINI_VERSION}/tini-static /tini
RUN chmod +x /tini

ARG EXEC_ENV
ENV EXEC_ENV=$EXEC_ENV
ARG DB_NAME
ENV DB_NAME=$DB_NAME
ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL

WORKDIR /usr/src/backend/db-app/

COPY backend/db-app/migrations ./migrations/
COPY --chmod=755 ./backend/target/release/db-app ./

ENTRYPOINT ["/tini", "--"]
CMD [ "./db-app" ]
