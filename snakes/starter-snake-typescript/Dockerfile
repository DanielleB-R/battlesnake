FROM node:16.10.0-alpine3.14

WORKDIR /usr/src/app

COPY . .

RUN npm install \
  && npm run build

EXPOSE 8080

CMD ["node", "build/index.js"]