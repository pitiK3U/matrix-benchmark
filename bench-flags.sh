#!/bin/bash

data=flags.csv
bin=./a.out
COUNT=100

if [ -e $data ]; then rm $data; fi

touch $data
printf "%28s , %38s , %6s\n" "lang-run/comp" "flags" "time (ns)" >> $data

for file in *.zig
do
	flags=("-O ReleaseSafe" "-O ReleaseFast" "-O ReleaseSmall")
	for flag in "${flags[@]}"
	do
		zig build-exe $flag -femit-bin=$bin $file
		sum=0
		for _ in $(seq 1 $COUNT)
		do
			time=$($bin 2>&1 | tail -n 1 | cut -f1 -d' ')
			sum=$((sum + time))
		done
		printf "%28s , %38s , %6d\n" "zig-$file" "$flag" "$((sum/$COUNT))" >> $data
	done
done

for file in *.cpp
do
	flags=("" "-mtune=native" "-march=native" "-flto" "-mavx2" "-ftree-vectorize -ftree-slp-vectorize")
	for flag in "${flags[@]}"
	do
		g++ -O2 -std=c++20 $flag "$file" -o $bin
		sum=0
		for _ in $(seq 1 $COUNT)
		do
			time=$($bin | tail -n 1 | cut -f1 -d' ')
			sum=$((sum + time))
		done
		printf "%28s , %38s , %6d\n" "g++-$file" "$flag" "$((sum/$COUNT))" >> $data

		clang++ -O2 -std=c++20 $flag $file -o $bin
		sum=0
		for _ in $(seq 1 $COUNT)
		do
			time=$($bin | tail -n 1 | cut -f1 -d' ')
			sum=$((sum + time))
		done
		printf "%28s , %38s , %6d\n" "clang++-$file" "$flag" "$((sum/$COUNT))" >> $data
	done
done

for file in *.rs
do
	flags=("" "-C target-cpu=native" "-C lto" "-C target-cpu=generic" "-C target-feature=avx2" "-C panic=abort" "-C target-cpu=haswell")
	for flag in "${flags[@]}"
	do
		rustc -O $flag -o $bin $file
		sum=0
		for _ in $(seq 1 $COUNT)
		do
			time=$($bin | tail -n 1 | cut -f1 -d' ')
			sum=$((sum + time))
		done
		printf "%28s , %38s , %6d\n" "rustc-$file" "$flag" "$((sum/$COUNT))" >> $data
	done
done


rm $bin

sort -t, -k3 -n $data | tee $data.tmp
mv $data.tmp $data

