cat './.mds-list' | while read line
do
    testseq="zh.md"
    if [[ $line =~ $testseq || "$line" == "" ]]; then
        echo "skip $line"
    else
        source_readme="./source/readme.md"
        lowline=`echo "$line" | awk '{print tolower($0)}'`
        zh=${line//source\//}
        dir=$(dirname $zh)
        # source/readme.md => en.md
        if [[ $lowline == $source_readme ]];then
        filename="en.md"
        else 
        filename=$(basename $zh)
        fi
        echo "$line >> $dir/$filename"
        mkdir -p $dir && cp $line "$_/$filename"
    fi
done