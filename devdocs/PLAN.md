# clb

clb - Rust port/reimplementation of a subset of ~/p/gh/clipboard/ (aka "cb")

## libs used
- clap (conventional usage)
- objc, etc. (if useful)
- arboard (if useful)

## non-goals

- multiple/virtual clipboards (for now); clipboard 0 is clb's default.
- any interaction with *nix "primary selection" (for now)
- recreating cb's terminal output coloration
  - clb's output is unstyled and 7-bit ascii (for now)
- recreating cb's use of non-7-bit ascii for terminal output styling

## cli subcommands

```bash
copy,    cp, c  Copy items into a clipboard. # PLAN-copy.md
paste,   p      Paste items from a clipboard. # PLAN-paste.md
move,    mv     Treat the last copy as a cut, then paste. # PLAN-move.md (new feature)
clear,   clr    Clear a clipboard. # PLAN-clear.md
cut,     ct     Cut items into a clipboard. # PLAN-cut.md
edit,    ed     Edit the contents of a clipboard as text. # PLAN-edit.md (temporarily converts the clipboard contents to text if possible, otherwise displays an error message. canceled edits (e.g. :cq) leave the clipboard alone)
info,    in     Show info about a clipboard. # PLAN-info.md 
status,  st     Check the status of a clipboard. # PLAN-info.md (status and info will be combined into info)
show,    sh     Show the contents of a clipboard. # PLAN-show.md (text only for now; chafa/icat planned.)
config,  cfg    Show the configuration of clb. # PLAN-config.md
completion      Generate shell completion scripts for clb. # PLAN-completion.md (new feature)
import,  imp    Import a clipboard from a file. # DEFERRED.md
export,  ex     Export a clipboard to a file. # DEFERRED.md
history, hs     Show the history of a clipboard. # DEFERRED.md
ignore,  ig     Ignore types of content in a clipboard. # DEFERRED.md
search,  sr     Search for items in a clipboard. # DEFERRED.md
undo,    u      Placeholder: Not implemented yet # NOT-PLANNED.md
redo,    r      Placeholder: Not implemented yet # NOT-PLANNED.md
swap,    sw     Swap the contents of multiple clipboards. # NOT-PLANNED.md
load,    ld     Load a clipboard into another clipboard. # NOT-PLANNED.md
remove,  rm     Remove items from a clipboard. # NOT-PLANNED.md
add,     ad     Add items to a clipboard. # NOT-PLANNED.md
note,    nt     Add a note to a clipboard. # NOT-PLANNED.md
script,  sc     Set a script to run for this clipboard. # NOT-PLANNED.md
share,   shr    Share a clipboard with others. # NOT-PLANNED.md
```
