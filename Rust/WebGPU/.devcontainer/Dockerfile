FROM rust:latest

ARG USER_NAME=utakata
ARG USER_ID=1000
ARG GROUP_ID=1000

# 環境変数の設定
ENV DEBIAN_FRONTEND=noninteractive

RUN apt update -y \
    && apt upgrade -y \
    && apt install git sudo -y \ 
    && apt clean 

RUN groupadd -g ${GROUP_ID} ${USER_NAME} \ 
    && useradd -m -u ${USER_ID} -g ${USER_NAME} ${USER_NAME} \
    && echo ${USER_NAME} ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/${USER_NAME}\
    && chmod 0440 /etc/sudoers.d/${USER_NAME} 

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh \
    && rustup target add wasm32-unknown-unknown \
    && cargo install --locked trunk

# デフォルトシェルをbashに設定
RUN chsh -s /bin/bash ${USER_NAME} \
    && echo 'export PS1="\e[01;32m\u\e[m:\e[01;34m\w\e[m\$ "' >> /home/${USER_NAME}/.bashrc

USER ${USER_NAME}
RUN sudo chmod -R 777 /usr/local/cargo