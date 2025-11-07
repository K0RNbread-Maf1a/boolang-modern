grammar BooModern;

// Parser Rules
compilationUnit
    : namespaceDeclaration? importDirective* typeDeclaration* EOF
    ;

namespaceDeclaration
    : 'namespace' qualifiedIdentifier NEWLINE+ block?
    ;

importDirective
    : 'import' qualifiedIdentifier ('as' identifier)? NEWLINE+
    | 'from' qualifiedIdentifier 'import' importList NEWLINE+
    ;

importList
    : '*'
    | identifier (',' identifier)*
    ;

typeDeclaration
    : classDeclaration
    | interfaceDeclaration
    | enumDeclaration
    | structDeclaration
    ;

classDeclaration
    : attributes? modifiers? 'class' identifier typeParameters? baseClasses? NEWLINE+ INDENT classMember* DEDENT
    ;

interfaceDeclaration
    : attributes? modifiers? 'interface' identifier typeParameters? baseInterfaces? NEWLINE+ INDENT interfaceMember* DEDENT
    ;

enumDeclaration
    : attributes? modifiers? 'enum' identifier NEWLINE+ INDENT enumMember (',' NEWLINE+ enumMember)* DEDENT
    ;

structDeclaration
    : attributes? modifiers? 'struct' identifier NEWLINE+ INDENT structMember* DEDENT
    ;

classMember
    : fieldDeclaration
    | propertyDeclaration
    | methodDeclaration
    | constructorDeclaration
    | eventDeclaration
    ;

methodDeclaration
    : attributes? modifiers? 'def' identifier typeParameters? '(' parameterList? ')' (':' type)? NEWLINE+ block
    ;

constructorDeclaration
    : attributes? modifiers? 'def' 'constructor' '(' parameterList? ')' NEWLINE+ block
    ;

propertyDeclaration
    : attributes? modifiers? identifier ':' type (NEWLINE+ INDENT propertyAccessor+ DEDENT)?
    ;

propertyAccessor
    : 'get' ':' NEWLINE+ block
    | 'set' ':' NEWLINE+ block
    ;

fieldDeclaration
    : attributes? modifiers? identifier ':' type ('=' expression)? NEWLINE+
    ;

parameterList
    : parameter (',' parameter)*
    ;

parameter
    : identifier ':' type ('=' expression)?
    ;

block
    : INDENT statement+ DEDENT
    | simpleStatement
    ;

statement
    : simpleStatement
    | ifStatement
    | whileStatement
    | forStatement
    | tryStatement
    | returnStatement
    | yieldStatement
    | breakStatement
    | continueStatement
    | raiseStatement
    ;

simpleStatement
    : expressionStatement NEWLINE+
    | assignmentStatement NEWLINE+
    | variableDeclaration NEWLINE+
    ;

variableDeclaration
    : identifier ':' type ('=' expression)?
    | identifier '=' expression
    ;

assignmentStatement
    : leftHandSide assignmentOperator expression
    ;

assignmentOperator
    : '=' | '+=' | '-=' | '*=' | '/=' | '%=' | '&=' | '|=' | '^='
    ;

leftHandSide
    : identifier
    | memberAccess
    | indexAccess
    ;

ifStatement
    : 'if' expression ':' NEWLINE+ block elifStatement* elseStatement?
    ;

elifStatement
    : 'elif' expression ':' NEWLINE+ block
    ;

elseStatement
    : 'else' ':' NEWLINE+ block
    ;

whileStatement
    : 'while' expression ':' NEWLINE+ block
    ;

forStatement
    : 'for' identifier 'in' expression ':' NEWLINE+ block
    ;

tryStatement
    : 'try' ':' NEWLINE+ block exceptClause+ finallyClause?
    | 'try' ':' NEWLINE+ block finallyClause
    ;

exceptClause
    : 'except' (identifier ':' type)? ':' NEWLINE+ block
    ;

finallyClause
    : 'finally' ':' NEWLINE+ block
    ;

returnStatement
    : 'return' expression?
    ;

yieldStatement
    : 'yield' expression?
    ;

breakStatement
    : 'break'
    ;

continueStatement
    : 'continue'
    ;

raiseStatement
    : 'raise' expression?
    ;

expressionStatement
    : expression
    ;

expression
    : lambdaExpression
    | conditionalExpression
    ;

lambdaExpression
    : 'lambda' '(' parameterList? ')' ':' expression
    | '{' parameterList? '|' blockBody '}'
    ;

conditionalExpression
    : logicalOrExpression ('if' logicalOrExpression 'else' expression)?
    ;

logicalOrExpression
    : logicalAndExpression ('or' logicalAndExpression)*
    ;

logicalAndExpression
    : equalityExpression ('and' equalityExpression)*
    ;

equalityExpression
    : relationalExpression (('==' | '!=') relationalExpression)*
    ;

relationalExpression
    : additiveExpression (('<' | '>' | '<=' | '>=') additiveExpression)*
    ;

additiveExpression
    : multiplicativeExpression (('+' | '-') multiplicativeExpression)*
    ;

multiplicativeExpression
    : unaryExpression (('*' | '/' | '%') unaryExpression)*
    ;

unaryExpression
    : ('not' | '-' | '+' | '~')? postfixExpression
    ;

postfixExpression
    : primaryExpression
    ( memberAccess
    | methodCall
    | indexAccess
    | castExpression
    )*
    ;

primaryExpression
    : literal
    | identifier
    | '(' expression ')'
    | arrayLiteral
    | hashLiteral
    | 'self'
    | 'super'
    ;

memberAccess
    : '.' identifier
    ;

methodCall
    : '(' argumentList? ')'
    ;

indexAccess
    : '[' expression ']'
    ;

castExpression
    : 'as' type
    ;

argumentList
    : argument (',' argument)*
    ;

argument
    : (identifier ':')? expression
    ;

arrayLiteral
    : '[' (expression (',' expression)*)? ']'
    ;

hashLiteral
    : '{' (hashEntry (',' hashEntry)*)? '}'
    ;

hashEntry
    : expression ':' expression
    ;

literal
    : INTEGER_LITERAL
    | FLOAT_LITERAL
    | STRING_LITERAL
    | CHAR_LITERAL
    | BOOLEAN_LITERAL
    | NULL_LITERAL
    ;

type
    : simpleType ('[' ']')* ('?')?
    ;

simpleType
    : qualifiedIdentifier typeArguments?
    | primitiveType
    ;

primitiveType
    : 'int' | 'long' | 'short' | 'byte'
    | 'float' | 'double' | 'decimal'
    | 'bool' | 'char' | 'string'
    | 'object' | 'void'
    ;

typeArguments
    : '[' type (',' type)* ']'
    | '(' type (',' type)* ')'
    ;

typeParameters
    : '[' typeParameter (',' typeParameter)* ']'
    ;

typeParameter
    : identifier ('(' typeConstraint ')')?
    ;

typeConstraint
    : 'of' type
    ;

baseClasses
    : '(' type (',' type)* ')'
    ;

baseInterfaces
    : '(' type (',' type)* ')'
    ;

attributes
    : '[' attribute (',' attribute)* ']' NEWLINE+
    ;

attribute
    : qualifiedIdentifier ('(' argumentList? ')')?
    ;

modifiers
    : modifier+
    ;

modifier
    : 'public' | 'private' | 'protected' | 'internal'
    | 'static' | 'final' | 'abstract' | 'virtual'
    | 'override' | 'async' | 'partial'
    ;

qualifiedIdentifier
    : identifier ('.' identifier)*
    ;

identifier
    : IDENTIFIER
    ;

blockBody
    : statement+
    ;

interfaceMember
    : methodSignature
    | propertySignature
    ;

methodSignature
    : 'def' identifier '(' parameterList? ')' (':' type)? NEWLINE+
    ;

propertySignature
    : identifier ':' type NEWLINE+
    ;

structMember
    : fieldDeclaration
    | methodDeclaration
    ;

enumMember
    : identifier ('=' INTEGER_LITERAL)?
    ;

// Lexer Rules
BOOLEAN_LITERAL : 'true' | 'false' ;
NULL_LITERAL : 'null' ;

INTEGER_LITERAL
    : [0-9]+
    | '0x' [0-9a-fA-F]+
    | '0b' [01]+
    ;

FLOAT_LITERAL
    : [0-9]+ '.' [0-9]+ ([eE] [+-]? [0-9]+)?
    | [0-9]+ [eE] [+-]? [0-9]+
    ;

STRING_LITERAL
    : '"' (~["\\\r\n] | '\\' .)* '"'
    | '\'' (~['\\\r\n] | '\\' .)* '\''
    | '"""' .*? '"""'
    ;

CHAR_LITERAL
    : '\'' (~['\\\r\n] | '\\' .) '\''
    ;

IDENTIFIER
    : [a-zA-Z_][a-zA-Z0-9_]*
    ;

NEWLINE
    : '\r'? '\n'
    ;

WS
    : [ \t]+ -> channel(HIDDEN)
    ;

COMMENT
    : '//' ~[\r\n]* -> channel(HIDDEN)
    | '/*' .*? '*/' -> channel(HIDDEN)
    | '#' ~[\r\n]* -> channel(HIDDEN)
    ;

// Indentation tokens (to be handled by custom lexer)
INDENT : '<INDENT>' ;
DEDENT : '<DEDENT>' ;
