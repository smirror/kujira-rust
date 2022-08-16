#/bin/bash
input=$1
output=${input%.*}

rustc $input
./$output
rm $output
