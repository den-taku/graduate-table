#! /bin/tcsh -f

# set path = (/sbin /bin /usr/sbin /usr/bin /usr/local/sbin /usr/local/bin $HOME/bin $HOME/.cargo/bin)

# for setting up
mkdir -p results
mkdir -p results/voluntary_immediate
mkdir -p results/voluntary_delay
mkdir -p results/forced_immediate
mkdir -p results/forced_delay
mkdir -p results/dfs

# conditions
set TESTSIZE = 100
set TIME_LIMIT = 1800000

# for simulation setting
set TESTS = (\
"3_3_20_10_10_x100"\
"3_3_20_10_30_x100"\
"3_3_30_13_10_x100"\
"3_3_30_13_30_x100"\
"3_3_40_15_10_x100"\
"3_3_40_15_30_x100"\
\
"4_3_20_10_10_x100"\
"4_3_20_10_30_x100"\
"4_3_30_13_10_x100"\
"4_3_30_13_30_x100"\
"4_3_40_15_10_x100"\
"4_3_40_15_30_x100"\
\
"4_4_20_10_10_x100"\
"4_4_20_10_30_x100"\
"4_4_30_13_10_x100"\
"4_4_30_13_30_x100"\
"4_4_40_15_10_x100"\
"4_4_40_15_30_x100"\
\
"5_3_20_10_10_x100"\
"5_3_20_10_30_x100"\
"5_3_30_13_10_x100"\
"5_3_30_13_30_x100"\
"5_3_40_15_10_x100"\
"5_3_40_15_30_x100"\
\
"6_3_20_10_10_x100"\
"6_3_20_10_30_x100"\
"6_3_30_13_10_x100"\
"6_3_30_13_30_x100"\
"6_3_40_15_10_x100"\
"6_3_40_15_30_x100"\
\
"6_4_40_15_10_x100"\
"6_4_40_15_30_x100"\
"6_4_50_20_10_x100"\
"6_4_50_20_30_x100"\
"6_4_60_25_10_x100"\
"6_4_60_25_30_x100"\
"6_5_70_30_10_x100"\
"6_5_70_30_30_x100"\
)

set i = 1
set TESTS_LEN = 0
while ( $i <= $#TESTS )
    # i番目のテストが実行される
    set i0 = 1
    @ i0 = $i * 2 - $i0
    set i1 = 0
    @ i1 = $i * 2
    set INSTANCE = $TESTS[$i0]
    set SIZE = $TESTS[$i1]
    echo $SIZE > config

    # 解く種類を指定

    mkdir -p results/forced_immediate/$INSTANCE
    mkdir -p results/forced_delay/$TESTS[$i]
    # mkdir -p results/voluntary_immediate/$TESTS[$i]
    # mkdir -p results/voluntary_delay/$TESTS[$i]

    # cargo build --release

    set TESTCASE = 0
    while ( $TESTCASE < $TESTSIZE )
        # TESTCASE番目のインスタンスが解かれる

        python3 solver.py 1 1 $TESTS[$i] $TESTCASE $TIME_LIMIT
        python3 solver.py 1 2 $TESTS[$i] $TESTCASE $TIME_LIMIT
        # ./target/release/bin VoluntaryImmediate $TESTS[$i] $TESTCASE $TIME_LIMIT
        # python3 solver.py 2 2 $TESTS[$i] $TESTCASE $TIME_LIMIT
        @ TESTCASE = $TESTCASE + 1
    end
    cd graduate-bencher
    cargo run --release ../results/forced_immediate/$INSTANCE $TESTSIZE
    cargo run --release ../results/forced_delay/$INSTANCE $TESTSIZE
    # cargo run --release ../results/voluntary_delay/$INSTANCE $TESTSIZE
    cd ..
    cd graduate-checker
    cargo run --release ../results/forced_immediate/$INSTANCE $TESTSIZE
    cargo run --release ../results/forced_delay/$INSTANCE $TESTSIZE
    # cargo run --release ../results/voluntary_immediate/$INSTANCE $TESTSIZE
    # cargo run --release ../results/voluntary_delay/$INSTANCE $TESTSIZE
    cd ..
    @ i = $i + 1
end