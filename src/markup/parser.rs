#[derive(Debug, Clone)]
pub enum Node {
    Element { name: String, children: Vec<Node> },
    Text(String),
}

#[derive(Debug, Clone)]
pub struct Document {
    pub roots: Vec<Node>,
}

pub fn parse_markup(input: &str) -> Document {
    let mut roots = Vec::new();
    let mut stack: Vec<(String, Vec<Node>)> = Vec::new();

    for token in tokenize(input) {
        match token {
            Token::Open(tag) => stack.push((tag, Vec::new())),
            Token::Close(tag) => {
                if let Some((name, children)) = stack.pop() {
                    let node = Node::Element { name, children };
                    if let Some((_, parent_children)) = stack.last_mut() {
                        parent_children.push(node);
                    } else {
                        roots.push(node);
                    }
                } else {
                    roots.push(Node::Element {
                        name: tag,
                        children: Vec::new(),
                    });
                }
            }
            Token::Text(text) => {
                if let Some((_, children)) = stack.last_mut() {
                    children.push(Node::Text(text));
                } else {
                    roots.push(Node::Text(text));
                }
            }
        }
    }

    while let Some((name, children)) = stack.pop() {
        roots.push(Node::Element { name, children });
    }

    Document { roots }
}

#[derive(Debug, Clone)]
enum Token {
    Open(String),
    Close(String),
    Text(String),
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut buf = String::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '<' {
            flush_text(&mut buf, &mut tokens);
            let mut tag = String::new();
            while let Some(&next) = chars.peek() {
                chars.next();
                if next == '>' {
                    break;
                }
                tag.push(next);
            }
            let tag = tag.trim().to_string();
            if let Some(stripped) = tag.strip_prefix('/') {
                tokens.push(Token::Close(stripped.trim().to_string()));
            } else if !tag.is_empty() {
                tokens.push(Token::Open(tag));
            }
        } else {
            buf.push(ch);
        }
    }

    flush_text(&mut buf, &mut tokens);
    tokens
}

fn flush_text(buf: &mut String, tokens: &mut Vec<Token>) {
    let text = buf.trim();
    if !text.is_empty() {
        tokens.push(Token::Text(text.to_string()));
    }
    buf.clear();
}
