ELM_FILES=$(shell find src/ -type f -name "*.elm")

dist/out.js:${ELM_FILES}
 elm make --optimize --output $@ src/Main.elm

.Phony:dev-server hash
hash:dist/out.js
 sh .scripts/filehash.sh $< > $@
 cp -v $< "dist/`sh .scripts/filehash.sh $<`.js"

dist/index.html:public/index.html hash
 sed  's/{{elmout}}/${shell cat hash}/' $< > $@
#  
# dev-server:src/Main.elm
#  -rm -r dist
#  elm-live $< -- --output dist/out.js

