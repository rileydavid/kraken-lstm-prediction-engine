####################################################################################################
## Builder
####################################################################################################
FROM node:20-alpine as build-step

RUN mkdir -p /app

WORKDIR /app

COPY package.json /app
COPY angular.json /app
COPY . /app
COPY /nginx/nginx.conf /app/nginx.conf

RUN npm install --legacy-peer-deps  
RUN npm install -g @angular/cli@17.0.7
RUN npm install -g @angular-devkit/build-angular@17.0.7

RUN ng build --configuration=production --output-path=/dist/frontend/browser --verbose
####################################################################################################
## Final image
####################################################################################################
FROM nginx:1.25.2-alpine

COPY --from=build-step /app/dist/frontend/browser/browser/ /usr/share/nginx/html
COPY --from=build-step /app/nginx.conf /etc/nginx/conf.d/default.conf

CMD [ "nginx", "-g", "daemon off;"]
