// use parser::node::*;
// use parser::statements::*;
// use parser::expressions::*;

// use parser::program::*;
// use evalute::object::*;

// pub fn eval(node: Node) -> Box<Object> {
//   match node.node_type {
//     ExpressionStatement => {
//       Box::new(eval(node))
//     },
//     IntegerLiteral => {
//       Box::new(Integer {
//         // TBD error handling
//         value: node.value.parse::<i64>().unwrap_or(0)
//       })
//     },
//     Program
//   }
// }


// pub fn eval_program(mut program: Program) {
//   for statement in program.statements.into_iter() {
//     // eval_statement(statement);
//   }
// }

// func Eval(node ast.Node) object.Object {
//   switch node := node.(type) {
//     // 文
//     case *ast.Program:
//       return evalStatements(node.Statements)
//     case *ast.ExpressionStatement:
//       return Eval(node.Expression)
//     // 式
//     case *ast.IntegerLiteral:
//       return &object.Integer{Value: node.Value}
//   }
//   return nil
// }

// func evalStatements(stmts []ast.Statement) object.Object {
//   var result object.Object
//   for _, statement := range stmts {
//     result = Eval(statement)
//   }
//   return result
// }