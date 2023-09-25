# 使用官方 Rust 镜像作为基础镜像
FROM rust:1.72 as builder

# 设置工作目录
WORKDIR /usr/src/app

# 复制项目文件到工作目录
COPY . .

# 构建项目
RUN cargo build --release

# 创建一个新的镜像
FROM debian:buster-slim
# 使用一个包含 GLIBC 2.33 的基础镜像
FROM frolvlad/alpine-glibc

# 设置工作目录
WORKDIR /usr/src/app

# 从之前的构建镜像中拷贝构建的可执行文件到当前镜像
COPY --from=builder /usr/src/app/target/release/webservice .
COPY --from=builder /usr/src/app/.env .

# 暴露端口
EXPOSE 8080

# 运行可执行文件
CMD ["./webservice"]
