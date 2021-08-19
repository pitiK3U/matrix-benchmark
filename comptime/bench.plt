set title "Matrix multiplication benchmark"
set xlabel "Program"
set ylabel "Time (ns)"
#set term x11
set grid
set boxwidth 1
set style data histogram
set style fill solid 1.0 border

plot 'bench.data' using 2:xticlabels(1) t '', \
    "" u ($0):2:(sprintf("%d ns", $2)) with labels offset char 0,1 t ''

pause -1 "Hit any key to continue"
