cat './.mds-list' | while read line
do
    if [ "$line" != "" ]; then
        zh=${line//source\//}
        dir=$(dirname $zh)
        filename=$(basename $zh)
        echo $dir + $filename
        mkdir -p $dir && cp $line $_
    fi
done