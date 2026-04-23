; Copyright 2026 Dora Lee
;
; Licensed under the Apache License, Version 2.0 (the "License");
; you may not use this file except in compliance with the License.
; You may obtain a copy of the License at
;
;     http://www.apache.org/licenses/LICENSE-2.0
;
; Unless required by applicable law or agreed to in writing, software
; distributed under the License is distributed on an "AS IS" BASIS,
; WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
; See the License for the specific language governing permissions and
; limitations under the License.

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
