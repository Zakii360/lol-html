lol format

parser HTMLStreamParser

version 1.0

stream
    input bytes
    output events

events

    StartTag
        name
        attributes
        self_closing

    EndTag
        name

    Text
        value

    Comment
        value

    Doctype
        value

    EOF

states

    Data

    TagOpen

    EndTagOpen

    TagName

    BeforeAttributeName

    AttributeName

    AfterAttributeName

    BeforeAttributeValue

    AttributeValueDoubleQuoted

    AttributeValueSingleQuoted

    AttributeValueUnquoted

    AfterAttributeValue

    SelfClosingStartTag

    Comment

    Doctype

transitions

    Data
        "<" -> TagOpen
        text -> emit(Text)

    TagOpen
        "/" -> EndTagOpen
        letter -> TagName
        "!" -> Comment

    TagName
        whitespace -> BeforeAttributeName
        "/" -> SelfClosingStartTag
        ">" -> emit(StartTag)

...
