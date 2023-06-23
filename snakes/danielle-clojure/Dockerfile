# Pick a good base image for your language stack
FROM clojure:temurin-20-lein-alpine

# set the app directory that your stack expects
WORKDIR /usr/src/app

# copy in your source files
COPY . .

# run any compilation steps your stack needs
RUN lein ring uberjar && cp target/battlesnake-*-standalone.jar battlesnake.jar

# port 8080 is set by default in your snake's vars.yaml
EXPOSE 8080
ENV PORT=8080

# set the command to start your API server
CMD java -jar battlesnake.jar
