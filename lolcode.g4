grammar project1grammar;

// LEXER
HAI            : '#HAI';
KTHXBYE        : '#KTHXBYE';
OBTW           : '#OBTW';
TLDR           : '#TLDR';
OIC            : '#OIC';
MKAY           : '#MKAY';

MAEK_HEAD      : '#MAEK HEAD';
GIMMEH_TITLE   : '#GIMMEH TITLE';
MAEK_PARAGRAF  : '#MAEK PARAGRAF';
GIMMEH_BOLD    : '#GIMMEH BOLD';
GIMMEH_ITALICS : '#GIMMEH ITALICS';
MAEK_LIST      : '#MAEK LIST';
GIMMEH_ITEM    : '#GIMMEH ITEM';
GIMMEH_SOUNDZ  : '#GIMMEH SOUNDZ';
GIMMEH_VIDZ    : '#GIMMEH VIDZ';
I_HAZ          : '#I HAZ';
IT_IZ          : '#IT IZ';
LEMME_SEE      : '#LEMME SEE';
GIMMEH_NEWLINE : '#GIMMEH NEWLINE';

ID
  : ('A'..'Z' | 'a'..'z' | '_')
    ('A'..'Z' | 'a'..'z' | '0'..'9' | '_')*
  ;

WORD
  : (~('#' | '\r' | '\n' | '\t'))+
  ;


WS
  : ('\r'? '\n')+ { $channel = HIDDEN; }
  | ('\t' | '\r')+ { $channel = HIDDEN; }
  ;

//Parser
program          : HAI content KTHXBYE EOF ;
content          : section* ;
section          : comment | head | paragraph | list | sound | video
                 | variable_define | variable_use | newline | text ;

comment          : OBTW text TLDR ;
head             : MAEK_HEAD title OIC ;
title            : GIMMEH_TITLE text MKAY ;
paragraph        : MAEK_PARAGRAF paragraph_content OIC ;
paragraph_content: (text | bold | italics | list | newline)* ;
bold             : GIMMEH_BOLD text MKAY ;
italics          : GIMMEH_ITALICS text MKAY ;
list             : MAEK_LIST item+ OIC ;
item             : GIMMEH_ITEM text MKAY ;
sound            : GIMMEH_SOUNDZ text MKAY ;
video            : GIMMEH_VIDZ text MKAY ;
variable_define  : I_HAZ ID IT_IZ text MKAY ;
variable_use     : LEMME_SEE ID MKAY ;
newline          : GIMMEH_NEWLINE ;
text             : WORD+ ;
