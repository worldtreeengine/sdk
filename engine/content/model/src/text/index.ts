export interface ParagraphTextNode {
    p: Text;
}

export interface ItalicTextNode {
    i: Text;
}

export interface BoldTextNode {
    b: Text;
}

export interface AnchorTextNode {
    a: Text,
    href: string,
}

export type TextNode = ParagraphTextNode | ItalicTextNode | BoldTextNode | AnchorTextNode | string;

export type Text = TextNode[];
