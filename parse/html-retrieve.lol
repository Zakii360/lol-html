```lol
lol format

retriever HTMLRetriever

version 1.0

source

    HTMLStreamParser

nodes

    Document

    Element

    Text

    Comment

    Doctype

selectors

    *

    tag

    id

    class

    attribute

    descendant

    child

    adjacent

    sibling

    group

attributes

    exists

    equals

    contains

    starts_with

    ends_with

    includes

operations

    find(selector)

    find_all(selector)

    first(selector)

    last(selector)

    next()

    previous()

    parent()

    children()

    descendants()

    ancestors()

    siblings()

    attributes()

    attribute(name)

    has_attribute(name)

    has_class(name)

    has_id(name)

    tag_name()

    text()

    html()

    inner_html()

    outer_html()

    remove()

    replace(content)

    append(content)

    prepend(content)

    before(content)

    after(content)

callbacks

    on_document

    on_start_tag

    on_end_tag

    on_text

    on_comment

    on_doctype

    on_element

    on_match

    on_finish

filters

    tag_equals

    id_equals

    class_equals

    attribute_equals

    attribute_exists

    text_contains

    visible_only

events

    Match

    Skip

    Continue

    Stop

stack

    push_element

    pop_element

    current_element

    current_parent

    current_depth

cache

    selector_cache

    attribute_cache

    node_cache

stream

    incremental

    forward_only

    event_driven

memory

    retain_active_stack

    release_completed_nodes

    zero_copy_strings

errors

    invalid_selector

    invalid_attribute

    unexpected_end

    malformed_html

    unsupported_selector

configuration

    case_sensitive false

    decode_entities true

    preserve_comments true

    preserve_whitespace true

    allow_malformed_html true

output

    node

    node_list

    event

    stream

end
```
