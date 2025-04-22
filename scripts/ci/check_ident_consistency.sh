
grep -R --line-number "HashMap<String, Value>" src | tee /dev/stderr && exit 1

exit 0
