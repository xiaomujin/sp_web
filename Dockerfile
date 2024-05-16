FROM rust:latest
LABEL authors="Nekoko"
ENV RUSTUP_DIST_SERVER="https://rsproxy.cn"
ENV RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup"

#RUN echo "">/etc/apt/sources.list
#RUN echo  "deb http://mirrors.tuna.tsinghua.edu.cn/debian/ buster main contrib non-free" >/etc/apt/sources.list
#RUN echo  "deb http://mirrors.tuna.tsinghua.edu.cn/debian/ buster-updates main contrib non-free" >>/etc/apt/sources.list
#RUN echo  "deb http://mirrors.tuna.tsinghua.edu.cn/debian-security buster/updates main contrib non-free" >>/etc/apt/sources.list
#RUN echo  "deb http://mirrors.tuna.tsinghua.edu.cn/debian/ bookworm main contrib non-free" >>/etc/apt/sources.list
#RUN echo  "deb http://mirrors.tuna.tsinghua.edu.cn/debian/ bookworm-updates main contrib non-free" >>/etc/apt/sources.list
#RUN echo  "deb http://mirrors.tuna.tsinghua.edu.cn/debian-security bookworm-security main contrib non-free" >>/etc/apt/sources.list
RUN sed -i "s@deb.debian.org@mirrors.tuna.tsinghua.edu.cn@g" /etc/apt/sources.list.d/debian.sources
WORKDIR /app

COPY ./ .

VOLUME /app

RUN apt update
RUN apt install -y musl-tools musl-dev
RUN apt install -y pkg-config libssl-dev
RUN rustup default nightly
RUN rustup target add x86_64-unknown-linux-musl

#RUN cargo +nightly build --target x86_64-unknown-linux-musl --release
#RUN cargo +nightly build --target x86_64-unknown-linux-gnu --release
