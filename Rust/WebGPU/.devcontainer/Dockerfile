FROM rust:1.81.0

ARG USER_NAME=utakata
ARG USER_ID=1000
ARG GROUP_ID=1000

RUN groupadd -g ${GROUP_ID} ${USER_NAME} && \
    useradd -m -u ${USER_ID} -g ${USER_NAME} ${USER_NAME}

RUN apt update -y \
    && apt upgrade -y \
    && apt install git -y \ 
    && apt clean 

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh \
    && rustup target add wasm32-unknown-unknown \
    && cargo install --locked trunk

# デフォルトシェルをbashに設定
RUN chsh -s /bin/bash ${USER_NAME}

USER ${USER_NAME}