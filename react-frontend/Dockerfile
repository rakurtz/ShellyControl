FROM node:bookworm-slim as builder
WORKDIR /app
COPY /package.json .
RUN npm install --loglevel verbose
COPY /.env .
COPY /src /app/src/
COPY /public /app/public/
RUN npm run build
# debugging
# RUN echo "builder: ls /app/build" && ls -l /app/build

FROM node:bookworm-slim as runner
RUN npm install -g serve
WORKDIR /app
COPY /.env .
COPY --from=builder /app/build /app/build/
# debugging
# RUN echo "runner: ls /app/build" && ls -l /app/build

EXPOSE 4000
CMD ["serve",  "-s", "build",  "-l", "4000"]

