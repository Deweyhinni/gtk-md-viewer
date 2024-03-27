# gtk-md-viewer
a really simple and kinda bad markdown viewer made with gtk-rs and webkit2gtk  
its basically just a web browser in a gtk window and a markdown to html converter lmao

## system dependancies
honestly i dont really remember but atleast something like
- gtk-devel
- webkit2gtk4.1-devel

## installation
just build it in release mode and symbolically link it to somewhere in path like ```$HOME/.local/bin/```  
you can also have the link be called something else like mdview to shorten it 
``` sh
cargo build --release
ln -s <full/path/to>/target/release/markdown-viewer <full path to somewhere in $PATH>
```

## use
### dark mode default:
``` sh
markdown-viewer <file>
```
### light mode:
``` sh
markdown-viewer <file> -l
```
