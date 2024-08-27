# docker build --build-arg todo-rocket -t app  .
docker build --platform linux/amd64 --build-arg pkg=back-end -t app  .