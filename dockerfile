# <3 sid
from debian:sid

workdir /app

# get whatever apt packages we need
run apt-get update && apt-get install -y aptitude && aptitude install -y \
  rustc \
  git \
  libsdl2-dev \
  libsdl2-image-dev \
  libsdl2-ttf-dev \
  vim

run useradd -ms /bin/bash kewluser
user kewluser
workdir /home/kewluser
run git clone https://github.com/kendase3/hddvd
workdir /home/kewluser/hddvd
run cargo build
