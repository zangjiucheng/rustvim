# RustVim - Features & TODO

## Current Features

### Editor Modes
- [x] Normal Mode
- [x] Insert Mode
- [x] Visual Mode (character-wise)
- [x] Visual Line Mode (`V`)
- [x] Visual Block Mode (`Ctrl+V`)
- [x] Command Mode (`:w`, `:q`, etc.)
- [x] Search Mode (`/`, `?`)

### Text Editing
- [x] Character insertion/deletion
- [x] Line deletion (`dd`)
- [x] Character delete (`x`)
- [x] Word operations (`dw`, `de`, `db`)
- [x] Line operations (`0`, `$`, `^`)
- [x] Insert new lines (`o`, `O`)
- [x] Case change commands (`~`)

### Undo/Redo System
- [x] Composite undo for insert mode sessions
- [x] Full undo/redo stack (1000 levels)
- [x] Block delete undo support

### Yank/Paste System
- [x] Single and multi-line yank (`yy`, `yw`, `y$`)
- [x] Register system
- [x] Paste after/before (`p`, `P`)

### Search
- [x] Forward/backward search
- [x] Search highlighting
- [x] Search wrapping
- [x] Repeat search (`n`, `N`)

### Multi-Buffer Support
- [x] Multiple buffers
- [x] Buffer switching (`:bn`, `:bp`, `:b`)
- [x] Buffer listing (`:ls`)

### Configuration
- [x] TOML-based config (`~/.rustvimrc`)
- [x] Runtime `:set` commands
- [x] Tab size, line numbers, word wrap, auto-indent

### Plugin System
- [x] Native Rust plugin architecture
- [x] Built-in plugins: word count, sort, reverse, uniq, hello, time, status, charfreq

### Syntax Highlighting
- [x] Tree-sitter based
- [x] Languages: Rust, JavaScript, Python, C, JSON, Markdown
- [x] Auto language detection

### Keymap System
- [x] Configurable key bindings per mode
- [x] Multi-key sequences (`gg`, `dd`)
- [x] Operator-pending mode
- [x] Count prefixes (`5j`)

### Terminal Interface
- [x] Raw mode handling
- [x] Cursor shapes (block/underline/bar)
- [x] Bell and flash feedback
- [x] Escape sequence parsing

---

## Missing Features

### High Priority
- [x] Change operator (`c`) - delete and enter insert mode
- [x] Text objects (`iw`, `aw`, `i"`, `a"`, `i(`, `a(`)
- [x] Find character motions (`f`, `F`, `t`, `T`)
- [x] Repeat find (`;`, `,`)
- [ ] Match bracket motion (`%`)
- [ ] Shift operators (`<`, `>`)
- [ ] Marks (`m{a-z}`, `'a`)
- [ ] Jump list navigation
- [ ] Visual mode operators (apply `d`, `y` to selection)

### Medium Priority
- [ ] Replace Mode (`R`)
- [ ] Sentence motions (`(`, `)`)
- [ ] Paragraph motions (`{`, `}`)
- [ ] Screen line motions (`H`, `M`, `L`)
- [ ] Macro recording (`q{a-z}`)
- [ ] Command-line completion
- [ ] Filename completion
- [ ] Incremental search
- [ ] Regex search support
- [ ] Buffer confirmation (prompt before closing unsaved)

### Lower Priority
- [ ] Window splitting (`:split`, `:vsplit`)
- [ ] Tab pages (`:tabnew`, `:tabnext`)
- [ ] Text folding
- [ ] Spell checking
- [ ] Netrw (file explorer)
- [ ] Vimdiff mode
- [ ] Abbreviations
- [ ] Auto-commands
- [ ] Quickfix window

---

## Performance Improvements

- [ ] Incremental Tree-sitter parsing (currently re-parses on every keystroke)
- [ ] Partial screen refresh (only redraw changed portions)
- [ ] Large file handling optimization (>10MB files)

---

## UX Enhancements

- [ ] Cursor line highlighting
- [ ] Better error messages (Vim-like error codes)
- [ ] Virtual edit (cursor beyond line end)
- [ ] Mouse support
- [ ] Better status messages
- [ ] Popup menu for completion

---

## Code Quality

- [ ] More integration tests
- [ ] Better API documentation
- [ ] Error handling improvements
- [ ] Further modular component separation
