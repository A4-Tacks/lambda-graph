setl foldmethod=syntax
syn clear
syn region datasCase matchgroup=datasCaseDelimiter start=/^\..*/ end=/^\ze\./ fold

hi link datasCaseDelimiter  String
hi link datasCase           Normal
