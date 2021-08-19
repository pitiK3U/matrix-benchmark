#!/bin/bash

COUNT=100

data=bench.data
bin=./a.out

CFLAGS="-march=native -mtune=native -flto"
RUSTFLAGS="-C lto -C target-cpu=native"

if [ -e $data ]; then rm $data; fi

touch $data

for file in *.rs
do
    rustc -O $file -o $bin
	sum=0
	for _ in $(seq 1 ${COUNT})
	do
		time=$($bin | tail -n 1 | cut -f1 -d' ')
		sum=$((sum + time))
	done
	echo "rustc-$file $((sum/COUNT))" >> $data
done

for file in *.c
do
    gcc -O2 ${CFLAGS} $file -o $bin
	sum=0
	for _ in $(seq 1 ${COUNT})
	do
		time=$($bin | tail -n 1 | cut -f1 -d' ')
		sum=$((sum + time))
	done
	echo "gcc-$file $((sum/COUNT))" >> $data

    clang -O2 ${CFLAGS} $file -o $bin
	sum=0
	for _ in $(seq 1 ${COUNT})
	do
		time=$($bin | tail -n 1 | cut -f1 -d' ')
		sum=$((sum + time))
	done
	echo "clang-$file $((sum/COUNT))" >> $data
done

for file in *.cpp
do
    g++ -O2 ${CFLAGS} $file -o $bin
	sum=0
	for _ in $(seq 1 ${COUNT})
	do
		time=$($bin | tail -n 1 | cut -f1 -d' ')
		sum=$((sum + time))
	done
	echo "g++-$file $((sum/COUNT))" >> $data

    clang++ -O2 ${CFLAGS} $file -o $bin
	sum=0
	for _ in $(seq 1 ${COUNT})
	do
		time=$($bin | tail -n 1 | cut -f1 -d' ')
		sum=$((sum + time))
	done
	echo "clang++-$file $((sum/COUNT))" >> $data
done

rm $bin

sort -n -k2 -o $data $data

