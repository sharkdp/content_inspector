input=text_UTF-8.txt

iconv -f "UTF-8" -t "UTF-16" "$input" > "text_UTF-16LE.txt"
