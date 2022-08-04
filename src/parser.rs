use std::{
    io::{self, Error, ErrorKind},
    rc::Rc,
};

use crate::{
    ast::{
        Assign, Binary, Block, Call, Expr, Expression, Function, Grouping, If, Literal, Logical,
        Print, Stmt, Unary, Var, Variable, While,
    },
    error::{self, parse_error},
    token::{DataType, Token, TokenType},
};

pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &[Token]) -> Parser {
        Parser { tokens, current: 0 }
    }
    /// Parses the tokens and returns the AST.
    pub fn parse(&mut self) -> Result<Vec<Rc<dyn Stmt>>, Error> {
        let mut statements = Vec::<Rc<dyn Stmt>>::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        Ok(statements)
    }
    /// Returns any type of expression. This is the main entry point of the precedence tree for expressions.
    fn expression(&mut self) -> Result<Rc<dyn Expr>, Error> {
        self.assignment()
    }
    /// Returns a statement that's either a function- var declaration or any other statement
    /// which the statment function can return.
    /// It matches if the current token is a function (fun), if it is it will return a function declaration.
    /// If not it checks if it's a var declaration.
    /// Otherwise it will return a statement by calling the statement function.
    fn declaration(&mut self) -> Result<Rc<dyn Stmt>, Error> {
        if self.matches(&[TokenType::Fun]) {
            return self.function("function");
        } else if self.matches(&[TokenType::Var]) {
            return self.var_declaration();
        }
        match self.statement() {
            Ok(stmt) => Ok(stmt),
            Err(err) => {
                self.synchronize();
                Err(err)
            }
        }
    }
    /// Checks what type of statement we are dealing with and calls the corresponding function that statement.
    /// This is done by checking the current token type.
    /// This means that because the Token type and therefor the statement type
    /// we're dealing with is checked in this function
    /// and not in any of the other functions called below.
    fn statement(&mut self) -> Result<Rc<dyn Stmt>, Error> {
        if self.matches(&[TokenType::For]) {
            return self.for_statement();
        } else if self.matches(&[TokenType::If]) {
            self.if_statement()
        } else if self.matches(&[TokenType::Print]) {
            self.print_statement()
        } else if self.matches(&[TokenType::While]) {
            self.while_statement()
        } else if self.matches(&[TokenType::LeftBrace]) {
            Ok(Rc::new(Block::new(self.block()?)))
        // If we don't match any of the above, we're dealing with a regular statement.
        // Which just means a single expression ending with a semicolon.
        } else {
            self.expression_statement()
        }
    }
    /// Consumes the current Token and checks if it's an open paranthesis.
    /// The parser advances and with the next token we grab the initializer expression.
    /// We match the current character and if it's a semicolon we pur None in the initializer.
    /// If this is not the case the next check is to see if it's a Var. As a for loop can initialize a variable,
    /// within the paranthesis.
    /// If this isn't the case either we call an expression statement.
    /// The parser has advanced and next we check for the condition.
    /// If the current token is a semicolon we set the condition to None.
    /// Else we call an expression. After this the next token should be a semicolon, else we throw an error.
    /// The parser has advanced and next we check for the increment.
    /// If it's a right paranthesis we set the increment to None.
    /// Else we call an expression.
    /// The parser has advanced again and we make sure the next token is a right paranthesis.
    /// The body for the for loop is than grabbed by calling for a statement.
    /// If the increment value is not None we create a block Object with e vector of statements.
    /// In this vector is the increment value and the body. This created object will be assigned to the body.
    /// Next if the condition is not None we create a While object which takes in the condition and the body.
    /// This will be assigned to the body.
    /// Else we create a While object with the body and the condition while be a datatype set to true.
    /// Lastly if the initializer is not None we create a Block object which takes in a vector of statements,
    /// in the vec we put the initializer and the body. Otherwise this is ignored
    /// In the end the body is returned.
    /// The reason why While objects are created and not a 'For' object is because they contain almost
    /// the same logic.
    fn for_statement(&mut self) -> Result<Rc<dyn Stmt>, Error> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;
        let initializer = if self.matches(&[TokenType::Semicolon]) {
            None
        } else if self.matches(&[TokenType::Var]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition = if self.matches(&[TokenType::Semicolon]) {
            None
        } else {
            Some(self.expression()?)
        };
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;

        let increment = if self.matches(&[TokenType::RightParen]) {
            None
        } else {
            Some(self.expression()?)
        };

        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;
        let mut body = self.statement()?;

        if let Some(inc) = increment {
            let vec = Rc::new(vec![body, Rc::new(Expression::new(inc))]);
            body = Rc::new(Block::new(vec));
        }

        if let Some(c) = condition {
            body = Rc::new(While::new(c, body));
        } else {
            body = Rc::new(While::new(
                Rc::new(Literal::new(Some(DataType::Bool(true)))),
                body,
            ));
        };

        if let Some(init) = initializer {
            let vec = Rc::new(vec![init, body]);
            body = Rc::new(Block::new(vec));
        }

        Ok(body)
    }
    /// Checks if the current token is an open parenthesis by using the consume function.
    /// The parser advances and the expression is grabbed.
    /// Next it checks if the next token is a closing parenthesis. As an if statements condition needs to
    /// be between parenthesis.
    /// The parser has advanced again and the next expression is grabbed which is the then_branch.
    /// This is the statement that is executed if the condition is true.
    /// The parser has advanced again, if the current token is now an else branch
    /// statement is grabbed. Recursively more else if's and an else statement can be added this way.
    /// Eventually a new If object is created using the condition expression the then_branch and an optional
    /// else_branch.
    fn if_statement(&mut self) -> Result<Rc<dyn Stmt>, Error> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;

        let then_branch = self.statement()?;
        let mut else_branch: Option<Rc<dyn Stmt>> = None;
        if self.matches(&[TokenType::Else]) {
            else_branch = Some(match self.statement() {
                Ok(e) => e,
                Err(_) => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "Expect statement after 'else'.",
                    ));
                }
            });
        }
        Ok(Rc::new(If::new(condition, then_branch, else_branch)))
    }
    /// Grabs the expression, the parser advances and via the consume function it's checked
    /// if the next token is a semicolon to finish the statement.
    /// A Print object is created using the expression and returned.
    fn print_statement(&mut self) -> Result<Rc<dyn Stmt>, Error> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Rc::new(Print::new(value)))
    }
    /// Consumes the current token in the parser which should be the name of the variable.
    /// Next it checks if the next token is an = token. If it is, it grabs the exprssion of the next token
    /// and applies it as the variables value.
    /// Before creating and returning the Var object, consume is called, this is to check if the next token is a semicolon.
    /// If it is not, it will throw an error, as all statements should end with a semicolon (;).
    fn var_declaration(&mut self) -> Result<Rc<dyn Stmt>, Error> {
        // Variable names are lexed as Identifier tokens.
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?;
        let initializer = if self.matches(&[TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;
        Ok(Rc::new(Var::new(name.dup(), initializer)))
    }
    /// Calls the consume function to check for a left paren. If it is not, it will throw an error.
    /// A While statements condition should be between parantheses.
    /// After that the parser has advanced and we call the expression function
    /// to get the condition of the While statement.
    /// After that the parser has advanced again and with the consume function we can check
    /// if the condition is closed off with a closing parenthesis. If it is not, it will throw an error.
    /// The parser has advanced again and we call the statement function to get the body of the While statement.
    /// A statement can be a block or a single expression. Example: while (true) { ... } or while (true) print "hi";
    /// The body and condition can be used to then make a While object.
    fn while_statement(&mut self) -> Result<Rc<dyn Stmt>, Error> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after condition.")?;
        let body = self.statement()?;
        Ok(Rc::new(While::new(condition, body)))
    }
    /// An expression statement is an expression made into a statement by ending it with a semicolon.
    /// The function grabs the expression by going down the precendence tree for expressions.
    /// It checks with consume if the next Token is a semicolon (;) and if it is the expression is used to make
    /// an Expression Statement object.
    fn expression_statement(&mut self) -> Result<Rc<dyn Stmt>, Error> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
        Ok(Rc::new(Expression::new(expr)))
    }
    /// Grabs the current token which is the name of the function. In the assignment function which this
    /// function has been called it's already established the following tokens are part of a function.
    /// The parser advances and it will now check for parameters.
    /// This is done by looping in a do while loop style until the parser hits no more comma's ','.
    /// Function parameters are split by a comma, when it's done looking for parameters the parser consumes twice.
    /// To make sure the the function parameters are closed of by a closing parenthesis ')' and followed by a opening
    /// curly brace. As the next block is the body of the function.
    /// The block function is called to grab the functions body and the established paremeters, body and function name
    /// are put into a Function object.
    fn function(&mut self, kind: &str) -> Result<Rc<dyn Stmt>, Error> {
        let kind_error = format!("Expect {} name.", kind);
        let name = self.consume(TokenType::Identifier, kind_error.as_str())?;
        let paren_error = format!("Expect '(' after {kind} name.");
        self.consume(TokenType::LeftParen, paren_error.as_str())?;

        let mut parameters = Vec::<Token>::new();

        if !self.check(TokenType::RightParen) {
            loop {
                if parameters.len() >= 255 {
                    error::parse_error(self.peek(), "Can't have more than 255 parameters.");
                }
                parameters.push(self.consume(TokenType::Identifier, "Expect parameter name.")?);

                if !self.matches(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;
        let block_error = format!("Expect '{{' before {kind} body.");
        self.consume(TokenType::LeftBrace, block_error.as_str())?;

        let body = self.block()?;
        Ok(Rc::new(Function::new(name, Rc::new(parameters), body)))
    }

    fn block(&mut self) -> Result<Rc<Vec<Rc<dyn Stmt>>>, Error> {
        let mut statements = Vec::<Rc<dyn Stmt>>::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(Rc::new(statements))
    }
    /// Assigns a value to a variable. If the epxression is not of type 'Variable', it will return the expression.
    /// If the expression is of type 'Variable', it will assign the value to the variable.
    /// Because the value is one position furhter than the parser has advanced via recursion the next expression can be grabbed.
    /// A new Assignment object is returned with the variable's name and the value we got via recursion.
    fn assignment(&mut self) -> Result<Rc<dyn Expr>, Error> {
        let expr = self.or()?;
        if self.matches(&[TokenType::Equal]) {
            // We get the equals sign as token so we can use it for an error message.
            let equals = self.previous().dup();
            let value = self.assignment()?;
            match expr.as_any().downcast_ref::<Variable>() {
                Some(v) => return Ok(Rc::new(Assign::new(v.name.dup(), value))),
                None => return Err(self.parse_error(&equals, "Invalid assignment target.")),
            }
        }
        Ok(expr)
    }
    /// Grabs the expression by going down the precedence tree.
    /// The first function it will pass is the AND operator which is the other option for
    /// a Logical object apart from AND.
    /// If the current token is OR a Logical Object is made.
    /// On the left the expression we already grabbed. The operator OR
    /// and on the right a new expression is grabbed as the parser has advanced to the next token by this point.
    fn or(&mut self) -> Result<Rc<dyn Expr>, Error> {
        let mut expr = self.and()?;

        while self.matches(&[TokenType::Or]) {
            let operator = self.previous().dup();
            let right = self.and()?;
            expr = Rc::new(Logical::new(expr, operator, right))
        }
        Ok(expr)
    }
    /// Grabs and expression by goin down the precedence tree.
    /// Checks if the current Token is an AND operator.
    /// It will make a Logical object with the expression we already grabbed (left),
    /// The operator AND and
    /// a expression we are grabbing as the parser has advanced. (right)
    fn and(&mut self) -> Result<Rc<dyn Expr>, Error> {
        let mut expr = self.equality()?;

        while self.matches(&[TokenType::And]) {
            let operator = self.previous().dup();
            let right = self.equality()?;
            expr = Rc::new(Logical::new(expr, operator, right))
        }
        Ok(expr)
    }
    /// Grabs an expression by going down the precedence tree.
    /// If the next token is a != or == operator a new Binary object can be made.
    /// We have the expression, the operator
    /// and because the parser has advanced we grab the expression on the right side.
    fn equality(&mut self) -> Result<Rc<dyn Expr>, Error> {
        let mut expr = self.comparison();
        while self.matches(&[TokenType::Equalequal, TokenType::Bangequal]) {
            let operator = self.previous().dup();
            let right = self.comparison();
            expr = Ok(Rc::new(Binary::new(expr?, operator.clone(), right?)));
        }
        expr
    }
    /// Grabs an expression by going down the precedence tree.
    /// If the next Token in the parser is a comparison a binary object is created.
    /// We have the expression that will be on the left side, the operator
    /// and because the parser has advanced we grab the expression on the right side.
    fn comparison(&mut self) -> Result<Rc<dyn Expr>, Error> {
        let mut expr = self.term();
        let comparison_vec = vec![
            TokenType::Greater,
            TokenType::Greaterequal,
            TokenType::Less,
            TokenType::Lessequal,
        ];
        while self.matches(&comparison_vec) {
            let operator = self.previous().dup();
            let right = self.term();
            expr = Ok(Rc::new(Binary::new(expr?, operator.clone(), right?)));
        }
        expr
    }
    /// Recursively goes down the precedence tree first.
    /// If the current Token in the parser is a term (+, -) It will grab one of the two operators and
    /// Then create a new Binary object with the current expression and the operator.
    /// Similar to the factor method, the parser has passed to the next token so we can grab the right side, by calling
    /// the factor method and it will give a right side expression.
    fn term(&mut self) -> Result<Rc<dyn Expr>, Error> {
        let mut expr = self.factor();
        while self.matches(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().dup();
            let right = self.factor();
            expr = Ok(Rc::new(Binary::new(expr?, operator.clone(), right?)));
        }
        expr
    }
    /// Goes into unary method first, then factor.
    /// If the token is a unary operator, it will return a unary expression.
    /// If the token is not a unary operator, it will return the factor.
    /// If the token is not a unary operator and not a factor, it will return an error.
    /// If the token is a factor, it will return the factor by making a binary expression.
    /// The binary expression contains out of the unary expression * or / and then another unary expression.
    /// Valid examples: 1 * 3, 20 / 3. "hello" - world, would also parse,
    /// however during interpretting this will be caught as an error.
    fn factor(&mut self) -> Result<Rc<dyn Expr>, Error> {
        let mut expr = self.unary();
        while self.matches(&[TokenType::Star, TokenType::Slash]) {
            let operator = self.previous().dup();
            let right = self.unary();
            expr = Ok(Rc::new(Binary::new(expr?, operator.clone(), right?)));
        }
        expr
    }
    /// Returns the unary operator if it exists. (!, or -)
    /// If it doesn't exist, it'll return an epxression from the call method.
    /// If it does exist, it'll return an Unary expression with an operator and the expression after it
    /// example: !false, -a
    /// The reason the right hand side will not give back the same expression is because the parser has advanced already.
    fn unary(&mut self) -> Result<Rc<dyn Expr>, Error> {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().dup();
            let right = self.unary();
            return Ok(Rc::new(Unary::new(operator.clone(), right?)));
        }
        self.call()
    }
    /// This function returns a Call object and is used to get the arguments of a function call.
    /// The function starts by checking if the current token in the parser isn't a right paranthesis.
    /// If this is the case a function call would look like this: doSomething(). Meaning no parameters are in the function.
    /// If this is the case when the Call object is made an empty vec of arguments is passed.
    /// The callee parameter which is also used for the Call object comes comes from the call method which comes from a primary function call.
    /// A call object also stores the closing paranthesis. This is done by calling self.consume.
    /// The parse will also advance furhter when this function is called.
    /// If consume is called and the current Token isn't a right parenthesis an error will be given.
    /// If the function contains arguments a loop will run. The loop is written in a do while form.
    /// This means the code in the loop will be at least run once. An argument is added to the arguments vector each time.
    /// This new argument is given from the expression function. Everytime this function is called the parser advances to the next token.
    /// If the next token isn't a comma it will break the 'do while loop' as this means this was the last argument of the function.
    /// This is because function arguments are seperated by a comma. Example: doSomething(one, two three)
    fn finish_call(&mut self, callee: Rc<dyn Expr>) -> Result<Rc<dyn Expr>, Error> {
        let mut arguments = Vec::<Rc<dyn Expr>>::new();
        if !self.check(TokenType::RightParen) {
            loop {
                // The limit of a function's argument count is now 254. It only reports an error, it doesn't return one.
                if arguments.len() >= 255 {
                    self.parse_error(self.peek(), "Can't have more than 255 arguments.");
                }
                arguments.push(self.expression()?);
                if !self.matches(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        let paren = self.consume(TokenType::RightParen, "Expect ')' after arguments.")?;
        Ok(Rc::new(Call::new(callee, paren, arguments)))
    }
    /// grabs an expression containing a datatype from primary.
    /// The parser advances and it loops unril the current token in the parser isn't a left paranthesis.
    /// While this is true the value of expr is replaced with the returned value from the finish call method, which is used to parse
    /// a functions argument list. That function takes in the callee as its parameter which is the old value of the expression, grabbed from
    /// the primary function.
    /// Whe the loop is done the new expression is returned.
    fn call(&mut self) -> Result<Rc<dyn Expr>, Error> {
        let mut expr = self.primary();
        loop {
            if self.matches(&[TokenType::LeftParen]) {
                expr = self.finish_call(expr?);
            } else {
                break;
            }
        }
        expr
    }
    /// Primary method returns a data value wrapped in an Unary object.
    /// This is the base of the expression tree you could say.
    /// It can be a literal (10, "hello world", false), a variable (input, age), a parenthesized expression (2 + 2)
    fn primary(&mut self) -> Result<Rc<dyn Expr>, Error> {
        if self.matches(&[TokenType::False]) {
            return Ok(Rc::new(Literal::new(Some(DataType::Bool(false)))));
        }
        if self.matches(&[TokenType::True]) {
            return Ok(Rc::new(Literal::new(Some(DataType::Bool(true)))));
        }
        if self.matches(&[TokenType::Nil]) {
            return Ok(Rc::new(Literal::new(None)));
        }

        if self.matches(&[TokenType::Number, TokenType::String]) {
            let data_type = self.previous();
            return Ok(Rc::new(Literal::new(Some(
                data_type.clone().literal.unwrap(),
            ))));
        }
        if self.matches(&[TokenType::Identifier]) {
            return Ok(Rc::new(Variable::new(self.previous().dup())));
        }

        if self.matches(&[TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Rc::new(Grouping::new(expr?)));
        }

        Err(self.parse_error(self.peek(), "Expect expression."))
    }
    /// Loops over the given token types in the paremeter.
    /// If the next token is one of the given token types, it will return true and advances the parser
    /// to the next token.
    fn matches(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(*token_type) {
                self.advance();
                return true;
            }
        }
        return false;
    }
    /// Gets the current Token in the parser and advances to the next one.
    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, Error> {
        if self.check(token_type) {
            Ok(self.advance().dup())
        } else {
            Err(self.parse_error(self.peek(), message))
        }
    }
    /// Reports an error and returns an Error object.
    fn parse_error(&self, token: &Token, message: &str) -> Error {
        parse_error(token, message);
        io::Error::new(ErrorKind::Other, message)
    }
    /// Tries to fix the error by advancing the parser.
    fn synchronize(&mut self) {
        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }
            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => {
                    return;
                }
                _ => {
                    self.advance();
                    ()
                }
            }
        }
    }
    /// Compares the current token with the TokenType given in the parameter.
    /// If it's the last token in the parser, it will return false.
    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek().token_type == token_type;
    }
    /// Advances to the next token in the parser.
    /// Returns the previous token.
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
    /// Checks if the current token is the last token in the parser.
    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }
    /// Returns the current token in the parser.
    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }
    /// Returns the previous token in the parser.
    fn previous(&self) -> &Token {
        if self.current == 0 {
            return &self.tokens[0];
        }
        self.tokens.get(self.current - 1).unwrap()
    }
}
