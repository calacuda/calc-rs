from typing import List, Tuple, Union
from dataclasses import dataclass
from funcparserlib.lexer import make_tokenizer, TokenSpec, Token
from funcparserlib.parser import tok, Parser, many, forward_decl, finished
from os.path import expanduser, join
from pathlib import Path
import re


@dataclass
class BinaryExpr:
    op: str
    left: "Expr"
    right: "Expr"


Expr = Union[BinaryExpr, int, float]


def tokenize(s: str) -> List[Token]:
    specs = [
        TokenSpec("whitespace", r"\s+"),
        TokenSpec("float", r"[+\-]?\d+\.\d*([Ee][+\-]?\d+)*"),
        TokenSpec("int", r"[+\-]?\d+"),
        TokenSpec("op", r"(\*\*)|[+\-*/()]"),
    ]
    tokenizer = make_tokenizer(specs)
    
    return [t for t in tokenizer(s) if t.type != "whitespace"]


def parse(tokens: List[Token]) -> Expr:
    int_num = tok("int") >> int
    float_num = tok("float") >> float
    number = int_num | float_num

    expr: Parser[Token, Expr] = forward_decl()
    parenthesized = -op("(") + expr + -op(")")
    primary = number | parenthesized
    power = primary + many(op("**") + primary) >> to_expr
    term = power + many((op("*") | op("/")) + power) >> to_expr
    sum = term + many((op("+") | op("-")) + term) >> to_expr
    expr.define(sum)

    document = expr + -finished
    
    return document.parse(tokens)


def op(name: str) -> Parser[Token, str]:
    return tok("op", name)


def to_expr(args: Tuple[Expr, List[Tuple[str, Expr]]]) -> Expr:
    first, rest = args
    result = first

    for op, expr in rest:
        result = BinaryExpr(op, result, expr)

    return result


def add(op1, op2):
    if op1 is None or op2 is None:
        return None

    return op1 + op2


def sub(op1, op2):
    if op1 is None or op2 is None:
        return None

    return op1 - op2


def mul(op1, op2):
    if op1 is None or op2 is None:
        return None

    return op1 * op2


def div(op1, op2):
    if op1 is None or op2 is None:
        return None    

    if op2 != 0:
        return op1 / op2
    else:
        return None

def exp(op1, op2):
    if op1 is None or op2 is None:
        return None
    
    return op1 ** op2


switch = {"+": add, "-": sub, "*": mul, "/": div, "**": exp}


def solve_recurse(ast):
    if type(ast) in [int, float]:
        return ast

    ast.left = solve_recurse(ast.left)
    ast.right = solve_recurse(ast.right)

    if ast.left is not BinaryExpr and ast.right is not BinaryExpr:
        return switch.get(ast.op)(ast.left, ast.right)
    else:
        return ast


def _solve(ast):
    loc_ast = ast

    while type(loc_ast) is BinaryExpr:
        loc_ast = solve_recurse(loc_ast)

    return loc_ast


def parse_f(equation: str):
    equ = equation.lstrip()
       
    for regex, substitute in [
        (r"([*|+|/|-|**|\(|\)]) - (\d)", r"\1 -\2"),  # remove space between negative sign and number
        (r"([\d|A-Z|a-z])[ ]{0,1}(\()", r"\1 * \2"),  # add multiplication when there's a number next to a, open parentheses
        (r"(\))[ ]{0,1}([-]{0,1}[\d|A-Z|a-z])", r"\1 * \2"),  # add multiplication when there's a number next to a, close parentheses
        (r"(\d)\s{0,1}([A-Za-z])", r"\1 * \2"),  # adds a mulitiplication sign between number and variable
        (r"(A-Z|a-z)\s{0,1}(\d)", r"\1 * \2"),  # adds a mulitiplication sign between variable and number
        (r"[^\d]?\.\s?(\d)", r"0.\1"), # make pattern ".4" into "0.4"
    ]:
        equ = re.sub(regex, substitute, equ)
        # print(f"{equ=}")

    return equ



def solve(equation):
    # print(f"{equation = }")
    tokens = [tok.strip() for tok in re.split("(-\d+|\d+|\+|/|-|\*\*|\*|\(|\)|A-Z|a-z)", equation) if tok.strip()]
    # print(f"{tokens=}")
    equ = " ".join(tokens).replace("* *", "**").replace(" . ", ".")
    equ = parse_f(equ)

    ast = parse(tokenize(equ))

    return _solve(ast)


def prepare_f(equ: str, arg_name: str, x: int):
    return equ.replace(arg_name, str(x))


def solve_func(equ, start, end):
    xs = range(start, end+1)
    func_desc, func_body = equ.split("=")
    # print(f"{func_body=}")
    func_body = parse_f(func_body)
    # print(f"{func_body=}")
    # get func args
    func_name, func_arg = func_desc.split("(")
    func_arg = func_arg.replace(")", "").strip()[0]  # ensure that the arg is only one letter
    # print(f"{func_arg=}, {func_body=}")
    # print(f"")
    # get domain
    ys = [solve(prepare_f(func_body, func_arg, i)) for i in xs]

    return ys


def solve_funcs(equs: [str], request):
    start = request.get("min") if request.get("min") else -100
    stop = request.get("max") if request.get("max") else 100
    # xs = [i for i in range(start, stop+1)]
    answers = calc_rs.solve_funcs(equs, start, stop)
    # print(answers.get("g(x)")[1])
    
    return answers


if __name__ == "__main__":
    import sys

    print(solve(sys.stdin.read().strip()))
