#!/bin/bash

./stop-api.sh

if ! [ -d "$HOME/waterlogged-prototype/dockerised-rust-api/" ]; then
    echo 'docker app directory does not exist!'
    exit 1
fi

cd $HOME/waterlogged-prototype/dockerised-rust-api/
/root/.cargo/bin/cargo build --release
if [ $? != 0 ]; then
    echo "cargo  build failed, check the server logs!"
    exit 1
fi

# If you don't pipe stderr/out to null then github's CI will never exit!
./target/release/api-example > /dev/null 2>&1 &

# Capture PID of last executed command with `$!`
# and use it to generate a kill script
echo "kill $!" > ../stop-api.sh

chmod +x ../stop-api.sh
