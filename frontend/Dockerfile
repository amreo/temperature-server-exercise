FROM tiangolo/node-frontend:10 as builder
WORKDIR /app
COPY frontend/package*.json /app/
RUN npm install
COPY frontend/ /app/
ARG configuration=production
RUN npm run build --aot -- --output-path=./dist/out --configuration $configuration

# Stage 1, based on Nginx, to have only the compiled app, ready for production with Nginx
FROM nginx:1.15
COPY --from=builder /app/dist/out /usr/share/nginx/html
# Copy the default nginx.conf provided by tiangolo/node-frontend
COPY --from=builder /nginx.conf /etc/nginx/conf.d/default.conf

