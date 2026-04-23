; Copyright 2026 Dora Lee

[
  "module"
  "import"
  "const"
  "struct"
  "union"
  "interface"
  "enum"
] @keyword

[
  "array"
  "associated"
  "handle"
  "map"
  "pending_remote"
  "pending_receiver"
  "pending_associated_remote"
  "pending_associated_receiver"
] @type.builtin

(primitive_type) @type.builtin
(specific_handle_type) @type.builtin

[
  "true"
  "false"
] @boolean

(default_literal) @constant.builtin

(comment) @comment
(string) @string
(escape_sequence) @string.escape
(number) @number
(integer) @number
(ordinal) @label

(module_declaration
  name: (qualified_identifier) @title)

(struct_declaration
  name: (identifier) @type)

(union_declaration
  name: (identifier) @type)

(interface_declaration
  name: (identifier) @type)

(enum_declaration
  name: (identifier) @enum)

(const_declaration
  name: (identifier) @constant)

(enum_value
  name: (identifier) @variant)

(method_declaration
  name: (identifier) @function)

(field_declaration
  name: (identifier) @property)

(union_field_declaration
  name: (identifier) @property)

(parameter
  name: (identifier) @variable.parameter)

(attribute
  name: (identifier) @attribute)

(qualified_identifier) @type

[
  "=>"
  "="
  "&"
  "?"
] @operator

[
  ";"
  ","
  "."
  "@"
] @punctuation.delimiter

[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
  "<"
  ">"
] @punctuation.bracket
