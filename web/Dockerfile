FROM node:16.15-bullseye

ENV LANG=c.UTF-8
ENV TZ=Asia/Tokyo

RUN curl https://deb.nodesource.com/setup_12.x | bash
RUN curl https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add -
RUN echo "deb https://dl.yarnpkg.com/debian/ stable main" | tee /etc/apt/sources.list.d/yarn.list
RUN apt-get update && apt-get install -y yarn

RUN pwd

RUN mkdir web
WORKDIR /web

COPY . /web

RUN yarn install

