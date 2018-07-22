# Copy the angular app
FROM tiangolo/node-frontend:10 as builder
WORKDIR /app
COPY frontend/package*.json /app/
RUN npm install
COPY frontend/ /app/
ARG configuration=production
ARG enable_proxy_pass=
ARG backend_base_url=http://127.0.0.1:8000

# Configure the backend_base_url
RUN test -z "$enable_proxy_pass" && echo "{ \"backend_base_url\":\"$backend_base_url\" }" > src/assets/config.json || echo "{ \"backend_base_url\":\"\" }" > src/assets/config.json
# Build the angular app
RUN npm run build --aot -- --output-path=./dist/out --configuration $configuration

# Setup the webserver
FROM nginx:alpine
ARG enable_proxy_pass=
ARG backend_endpoint=
COPY --from=builder /app/dist/out /usr/share/nginx/html
COPY nginx_no_proxy_pass.conf nginx_with_proxy_pass.conf /tmp/
RUN sed "s/\$backend_endpoint/$backend_endpoint/g" /tmp/nginx_with_proxy_pass.conf > /tmp/new_nginx_with_proxy_pass.conf && \
    test -z "$enable_proxy_pass" && cp -f /tmp/nginx_no_proxy_pass.conf /etc/nginx/conf.d/default.conf || cp -f /tmp/new_nginx_with_proxy_pass.conf /etc/nginx/conf.d/default.conf 

# RUN cat /usr/share/nginx/html/assets/config.json
# RUN cat /etc/nginx/conf.d/default.conf
