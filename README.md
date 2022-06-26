# soup
A simple programming language, built from scratch in Rust

## Language Specification

### Grammar

```
start                   : {globaldeclarations}
                        ;

literal                 : INTLIT
                        | STRLIT
                        | TRUE
                        | FALSE
                        ;

type                    : BOOLEAN
                        | INT
                        ;

globaldeclarations      : [globaldeclaration]+
                        ;

globaldeclaration       : variabledeclaration
                        | functiondeclaration
                        | mainfunctiondeclaration
                        ;

variabledeclaration     : type identifier SEMICOLON
                        ;

identifier              : ID
                        ;

functiondeclaration     : functionheader block
                        ;

functionheader          : FUNC functiondeclarator RETURNS [type | VOID]
                        ;

functiondeclarator      : identifier OPENPAR {formalparameterlist} CLOSEPAR
                        ;

formalparameterlist     : formalparameter [COMMA formalparameter]*
                        ;

formalparameter         : type identifier
                        ;

mainfunctiondeclaration : FUNC mainfunctiondeclarator RETURNS VOID block
                        ;

mainfunctiondeclarator  : MAIN OPENPAR CLOSEPAR
                        ;

block                   : OPENBRACE {blockstatements} CLOSEBRACE
                        ;

blockstatements         : [blockstatement]+
                        ;

blockstatement          : variabledeclaration
                        | statement
                        ;

statement               : block
                        | SEMICOLON
                        | statementexpression SEMICOLON
                        | BREAK SEMICOLON
                        | RETURN expression SEMICOLON
                        | RETURN SEMICOLON
                        | IF expression statement
                        | IF expression statement ELSE statement
                        | WHILE expression statement
                        ;

statementexpression     : assignment
                        | functioninvocation
                        ;

primary                 : literal
                        | OPENPAR expression CLOSEPAR
                        | functioninvocation
                        ;

argumentlist            : expression
                        | argumentlist COMMA expression
                        ;

functioninvocation      : identifier OPENPAR argumentlist CLOSEPAR
                        | identifier OPENPAR CLOSEPAR
                        ;

postfixexpression       : primary
                        | identifier
                        ;

unaryexpression         : MINUS unaryexpression
                        | NOT unaryexpression
                        | postfixexpression
                        ;




multiplicativeexpression: unaryexpression multiplicativerhs
                        ;

multiplicativerhs	: MULT unaryexpression multiplicativerhs
			| DIV unaryexpression multiplicativerhs
			| MOD unaryexpression multiplicativerhs
			| POWER INTLIT multiplicativerhs
			| /* nothing */
			;




additiveexpression      : multiplicativeexpression additiverhs
                        ;

additiverhs		: PLUS multiplicativeexpression additiverhs
			| MINUS multiplicativeexpression additiverhs
			| /* nothing */
			;




relationalexpression    : additiveexpression relationalrhs
                        ;

relationalrhs		: LT additiveexpression relationalrhs
			| GT additiveexpression relationalrhs
			| LEQ additiveexpression relationalrhs
			| GEQ additiveexpression relationalrhs
			| /* nothing */
			;




equalityexpression      : relationalexpression equalityrhs
                        ;

equalityrhs		: EQ relationalexpression equalityrhs
			| NEQ relationalexpression equalityrhs
			| /* nothing */
			;




conditionalandexpression: equalityexpression conditionalandrhs
                        ;

conditionalandrhs	: {AND equalityexpression conditionalandrhs}
			;




conditionalorexpression : conditionalandexpression conditionalorrhs
                        ;

conditionalorrhs	: {OR conditionalandexpression conditionalorrhs}
			;




assignmentexpression    : conditionalorexpression
                        | assignment
                        ;

assignment              : identifier ASSIGN assignmentexpression
			| identifier PLUSEQ INTLIT
			| identifier MINUSEQ INTLIT
			| identifier MULTEQ INTLIT
			| identifier DIVEQ INTLIT
			| identifier MODEQ INTLIT
			| identifier POWEREQ INTLIT
                        ;

expression              : assignmentexpression
                        ;
```
