use crate::parser::parser_data::ASTNode;

pub fn is_binary(node: &ASTNode) -> bool {
    node.node_type == "+"
        || node.node_type == "+="
        || node.node_type == "-"
        || node.node_type == "-="
        || node.node_type == "*"
        || node.node_type == "*="
        || node.node_type == "/"
        || node.node_type == "/="
        || node.node_type == "%"
        || node.node_type == "%="
        || node.node_type == "+"
        || node.node_type == "<"
        || node.node_type == ">"
        || node.node_type == "<="
        || node.node_type == ">="
        || node.node_type == "="
        || node.node_type == "=="
        || node.node_type == "!="
        || node.node_type == "&&"
        || node.node_type == "||"
}

pub fn is_unary(node: &ASTNode) -> bool {
    node.node_type == "u-" || node.node_type == "!"
}
