ELM_FILES=$(shell find src/ -type f -name "*.elm")
PUBFOLDERS=$(shell find public/ -type d -not -path public/)
ELMENV=DEVELOPMENT
OUTFILE=dist/out.js
ELMPRODFLAGS=--optimize
ELMDEVFLAGS=--debug
DEVPORT=8888
DEVHOST=0.0.0.0
.Phony:build minify dev devserver devindex
devserver:clean devindex
	mkdir -p dist
	elm-live src/Main.elm --port=${DEVPORT} --host=${DEVHOST} -d dist -- --output=${OUTFILE} ${ELMDEVFLAGS}

dev:clean dist/index.html
	echo "Dev build complete "
build:clean
	${MAKE} ELMENV=PRODUCTION dist/index.html
	echo "Production build complete please run make minify for minification"
${OUTFILE}:${ELM_FILES}
ifeq (${ELMENV},PRODUCTION)
	elm make ${ELMPRODFLAGS} --output $@ src/Main.elm
else
	elm make ${ELMDEVFLAGS} --output $@ src/Main.elm
endif
	

minify:hash
	uglifyjs ${OUTFILE} --compress 'pure_funcs="F2,F3,F4,F5,F6,F7,F8,F9,A2,A3,A4,A5,A6,A7,A8,A9",pure_getters,keep_fargs=false,unsafe_comps,unsafe' | uglifyjs --mangle --output=${OUTFILE}

hash:${OUTFILE}
	sh .scripts/filehash.sh $< > $@
	cp -v $< "dist/`sh .scripts/filehash.sh $<`.js"

dist/index.html:public/index.html hash dist/config.js pubfolders
	sed  's/{{elmout}}/${shell cat hash}/' $< > $@
dist/config.js:config.dev.js config.prod.js
ifeq (${ELMENV},PRODUCTION)
	cp config.prod.js $@
else
	cp config.dev.js $@
endif

pubfolders:
	cp -r ${PUBFOLDERS} dist/
devindex:public/index.html dist/config.js pubfolders
	sed  's/{{elmout}}/out/' $< > dist/index.html

clean:
	-rm -r dist 2>/dev/null
	mkdir -p dist
