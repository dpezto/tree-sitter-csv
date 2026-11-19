(text) @string
(number) @number
(float) @float
(boolean) @boolean
(comment) @comment

((date) @type.builtin
  (#set! priority 110))

; CsvViewComment uses Neovim's default extmark priority of 4096.
((comment (date) @type.builtin)
  (#set! priority 4097))

"," @punctuation.delimiter
