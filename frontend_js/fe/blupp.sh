#!/bin/zsh

cd dist
for JS in $(ls *.js); do
  if [ "$(grep -c "^class" $JS)" -ge 1 ]; then
    echo "${JS} starts with 'class"
    echo "removing old index.js"
    rm -vf index.js
    echo "removing old index.js.map"
    rm -vf index.js.map
    echo "replacing old filename ${JS} with  index.js in file ${JS}"
    MAP="${JS}.map"
    echo "searching for '${MAP}'"
    cat $JS | sed -e "s/$JS/index.js.map/g" >index.js
    echo "renaming ${MAP} to index.js.map"
    mv -v $MAP index.js.map
    echo "removing original file ${JS}"
    rm -rf $JS
    echo "replacing ${JS} with 'index.js' in index.html"
    mv index.html index.html.orig
    cat index.html.orig | sed -e "s/$JS/index.js/g" > index.html
    rm -rf index.html.orig
  else
    echo "${JS} starts not with 'class'"
    echo "removing file ${JS}"
    rm -vf $JS
    MAP="${JS}.map"
    echo "removing file ${MAP}"
    rm -vf $MAP
  fi
done

cd ..

# cat index.cb7a650d.js |  sed  -e 's/index.cb7a650d.js.map/index.js.map/g' > index.js
