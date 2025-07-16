setl foldmethod=syntax
syn clear
syn region datasCase start=/^\..*/ end=/^\ze\./ contains=datasCasePrefix fold
syn match  datasCasePrefix /^\..*/ contained nextgroup=datasGraph skipnl
syn region datasGraph start=/\ze/ end=/^\ze\./ contained contains=datasGraphFill
syn match  datasGraphFill /x\+/ contained

hi link datasCasePrefix     String
hi link datasCase           Normal
hi      datasGraphFill      ctermfg=255 ctermbg=255
