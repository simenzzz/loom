FROM python:3.12-slim AS build
COPY generate.py /generate.py
RUN python3 /generate.py --out /site

FROM nginx:1.27-alpine
COPY nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=build /site /usr/share/nginx/html
