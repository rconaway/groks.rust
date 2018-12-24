Because of our conversation about Rust at demo on 12/13/2018, I spent some time looking 
into ways we can share Rust knowledge.  For me, it's very hard to just write documentation 
without going overboard.  It's much easier to write it as a side-effect of trying things 
while I learn.

Here are some tools I discovered:

## Unit Tests in CLion with Rust Plugin
<https://www.jetbrains.com/help/clion/rust-support.html>

This is the only solution that automatically annotates the types in valid code.  For learning,
this is a necessity.  If we decide to publish with any of the other tools I will personally end
up in a two-step process.  Learn in CLion, then transfer to the other tools to publish.
 
* ++ easily accessible to Labs
* -- not as useful outside Pillar
* ++ fully interactive
* ++ syntax highlighting
* ++ type annotations (a killer feature, IMO)
* -- not easily published to web
* -- not as easy to relate examples
* -- verbal documentation limited to simple text
* ++ supports debugging
* ++ no extra/alternative syntax
 
## Jupyter with Rust Kernel
<https://github.com/google/evcxr/tree/master/evcxr_jupyter>

I've got a lot of hope for Jupyter, but in practice there are always some edge cases that
hurt you.  I haven't tried it with Rust, yet, but lack of type annotations and debugging is a show-stopper
for my purposes.

* -- requires Jupyter and Rust Kernel which may not be installed
* -- Rust kernel may not be as refined as others
* ++ fulliy interactive
* ++ syntax highlighting
* -- more fragile (to unsafe code, for example) than CLion 
* -- no type annotations
* ++ output is easily published to web, although you can't edit it there
* ++ document oriented - easy to tie examples together with markdown
* -- slight changes to Rust syntax
* ++ not specific to Rust, although you can't mix languages in a single workbook

## Rust Playground
<https://play.rust-lang.org/>

<https://github.com/integer32llc/rust-playground>

Something to keep an eye on.  We might be able to add the RLS (below) to get type annotations and
contribute.  That's likely to be a feature someone else might add. 

* ++ Save/load from gists
* ++ Open source, so we can run a local copy if we want
* -- Oriented toward single examples, not documents 
* -- syntax highlighting
* -- no type annotations
* ++ easily accessible from web
* ++ "official"
* -- slight changes to Rust syntax


## mdbook
<https://github.com/rust-lang-nursery/mdBook>

This looks awesome if you're writing a book.

* ++ used by Rust documentation.  For example: <https://doc.rust-lang.org/rust-by-example/hello.html> 
* -- not interactive, oriented toward publishing
* ++ open source, could be a nice basis for something custom
 
 
## Rust Doc Comments (in CLion)
<https://doc.rust-lang.org/1.30.0/book/first-edition/documentation.html>

Doc comments are useful and we need to work with them.  In practice, I keep going
back to standard unit tests in test modules.  We need some collective experience, then discussion.

* -- no type annotations
* -- no code reformatting
* ++ "official"
* ++ easy to seque into formal tests
* -- more oriented toward publishing than learning
* ++ interactive

## Rust Language Server (RLS)
<https://github.com/rust-lang/rls>

Potentially part of a solution.  This might be a way to add type annotations to any of the
other tools.   However, the CLion Rust plugin is not using this - they are rolling
their own.  This may indicate that things like type annotation are hard to get out of RLS.



