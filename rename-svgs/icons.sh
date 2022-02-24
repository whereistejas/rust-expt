mkdir ./icons
cd ./icons

cat some-source-file-in-rust | while read line
do
	content=$(echo $line | sed 's/^.*r##"//' | sed 's/"##;$//');
	filename=$(echo $line | cut -d" " -f3 | sed 's/:$//');
	echo $content > $filename.svg
done

cd -
