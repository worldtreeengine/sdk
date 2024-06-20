export type ExpressionAtom = string | number;
export type ExpressionOperation = [string, ...Expression[]];
export type Expression = ExpressionAtom | ExpressionOperation;
