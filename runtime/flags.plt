set datafile separator ','

set style data histogram
set style histogram rowstacked
set style fill solid 0.5 border 01

set key autotitle columnhead

set xtics rotate 90

plot 'flags.csv' u 3:xticlabels(sprintf("%s/%s", stringcolumn(1), stringcolumn(2)))

pause -1
