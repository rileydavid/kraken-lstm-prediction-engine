# Stage 1
FROM node:20-alpine as build-step

RUN mkdir -p /app

WORKDIR /app

COPY package.json /app

RUN npm install --force

COPY . /app

RUN npm run build

# Stage 2
FROM nginx:1.25.2-alpine

COPY --from=build-step /app/dist/frontend/browser /usr/share/nginx/html

CMD [ "nginx", "-g", "daemon off;"]
