#! /bin/tcsh -f

# set path = (/sbin /bin /usr/sbin /usr/bin /usr/local/sbin /usr/local/bin $HOME/bin $HOME/.cargo/bin)

cargo r bb/results/forced_immediate 100
cargo r bb/results/voluntary_immediate 100
cargo r bb/results/voluntary_delay 100