FROM debian:bookworm-slim AS prod

ENV TINI_VERSION v0.19.0
ADD https://github.com/krallin/tini/releases/download/${TINI_VERSION}/tini-static /tini
RUN chmod +x /tini

ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL
ARG API_TEST_HELPER_HOST
ENV API_TEST_HELPER_HOST=$API_TEST_HELPER_HOST
ARG API_TEST_HELPER_PORT
ENV API_TEST_HELPER_PORT=$API_TEST_HELPER_PORT
ARG APP_AUTH_SECRET
ENV APP_AUTH_SECRET=$APP_AUTH_SECRET

COPY --chmod=755 ./backend/target/release/api-test-helper /
ENTRYPOINT ["/tini", "--"]
CMD ["./api-test-helper"]
