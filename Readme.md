# elm-ui-project
Quickly create boilerplate code for elm ui projects

```
USAGE:
    elm-ui-project <project>

	FLAGS:
	    -h, --help       Prints help information
		-V, --version    Prints version information

			ARGS:
			    <project>    

```
Elm-ui-project crates the app skeleton as well as a makefile.

## Makefile usage

```
make dev       # create a dev build under dist directory
make build     # create a optimized production build
make devserver # start elm-live devserver (requires elm-live)
make minify    # minify out.js using uglifyjs (requires uglifyjs)
```

`out.js` file is automatically hashed after every change and `{{elmout}}.js` in `index.html` is automatically set to the new value.

## Important variables

Makefile uses several variables to provide a nicer control over build/development process

- OUTFILE
	- Output file path
	- Default: dist/out.js
- ELMPRODFLAGS
	- elm make flags for prod builds
	- Default: --optimize
- ELMDEVFLAGS
	- elm make flags for dev builds
	- Default: --debug
- DEVPORT
	- elm-live port
	- Default: 8888
- DEVHOST
	- elm-live host
	- Default: 0.0.0.0


#### Makefile examples
```
	make devserver ELMDEVFLAGS= # start devserver dont pass default --debug flag
	make build && make minify   # optimize and minify and hash the output file
	make clean                  # delete dist directory
```
