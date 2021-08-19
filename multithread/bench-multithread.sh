#!/bin/bash

data=bench-multithread.data

if [ -e $data ]; then rm $data; fi

touch $data

bin=./a.out
rustc -O rust-threadpool.rs -o $bin

threads=1
for iteration in $(seq 1 14)
do
    time=$($bin $threads | tail -n 1)
    echo "$threads $time" >> $data
    threads=$(($threads * 2))
done

rm $bin

# cat $data | sort -n -k2 >> $data.tmp
# mv $data.tmp $data

