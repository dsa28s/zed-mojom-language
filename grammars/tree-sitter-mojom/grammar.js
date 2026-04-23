// Copyright 2026 Dora Lee

module.exports = grammar({
  name: "mojom",

  extras: ($) => [/\s/, $.comment],

  word: ($) => $.identifier,

  rules: {
    source_file: ($) => repeat($._declaration),

    _declaration: ($) =>
      choice(
        $.module_declaration,
        $.import_declaration,
        $.struct_declaration,
        $.union_declaration,
        $.interface_declaration,
        $.enum_declaration,
        $.const_declaration,
      ),

    module_declaration: ($) =>
      seq(
        optional($.attribute_list),
        "module",
        field("name", $.qualified_identifier),
        ";",
      ),

    import_declaration: ($) =>
      seq(optional($.attribute_list), "import", field("path", $.string), ";"),

    struct_declaration: ($) =>
      seq(
        optional($.attribute_list),
        "struct",
        field("name", $.identifier),
        optional(field("body", $.struct_body)),
        ";",
      ),

    struct_body: ($) =>
      seq(
        "{",
        repeat(choice($.const_declaration, $.enum_declaration, $.field_declaration)),
        "}",
      ),

    field_declaration: ($) =>
      seq(
        optional($.attribute_list),
        field("type", $.type_spec),
        field("name", $.identifier),
        optional($.ordinal),
        optional($.default_value),
        ";",
      ),

    union_declaration: ($) =>
      seq(
        optional($.attribute_list),
        "union",
        field("name", $.identifier),
        field("body", $.union_body),
        ";",
      ),

    union_body: ($) => seq("{", repeat($.union_field_declaration), "}"),

    union_field_declaration: ($) =>
      seq(
        optional($.attribute_list),
        field("type", $.type_spec),
        field("name", $.identifier),
        optional($.ordinal),
        ";",
      ),

    interface_declaration: ($) =>
      seq(
        optional($.attribute_list),
        "interface",
        field("name", $.identifier),
        field("body", $.interface_body),
        ";",
      ),

    interface_body: ($) =>
      seq(
        "{",
        repeat(choice($.const_declaration, $.enum_declaration, $.method_declaration)),
        "}",
      ),

    method_declaration: ($) =>
      seq(
        optional($.attribute_list),
        field("name", $.identifier),
        optional($.ordinal),
        field("parameters", $.parameter_list),
        optional($.response),
        ";",
      ),

    parameter_list: ($) =>
      seq(
        "(",
        optional(seq($.parameter, repeat(seq(",", $.parameter)), optional(","))),
        ")",
      ),

    parameter: ($) =>
      seq(
        optional($.attribute_list),
        field("type", $.type_spec),
        field("name", $.identifier),
        optional($.ordinal),
      ),

    response: ($) => seq("=>", $.parameter_list),

    const_declaration: ($) =>
      seq(
        optional($.attribute_list),
        "const",
        field("type", $.type_spec),
        field("name", $.identifier),
        "=",
        field("value", $.constant),
        ";",
      ),

    enum_declaration: ($) =>
      seq(
        optional($.attribute_list),
        "enum",
        field("name", $.identifier),
        optional(field("body", $.enum_body)),
        ";",
      ),

    enum_body: ($) =>
      seq(
        "{",
        optional(seq($.enum_value, repeat(seq(",", $.enum_value)), optional(","))),
        "}",
      ),

    enum_value: ($) =>
      seq(
        optional($.attribute_list),
        field("name", $.identifier),
        optional(seq("=", choice($.integer, $.qualified_identifier))),
      ),

    attribute_list: ($) =>
      seq(
        "[",
        optional(seq($.attribute, repeat(seq(",", $.attribute)), optional(","))),
        "]",
      ),

    attribute: ($) =>
      seq(
        field("name", $.identifier),
        optional(seq("=", choice($.literal, $.qualified_identifier))),
      ),

    type_spec: ($) => prec.right(seq($._type_name, optional("?"))),

    _type_name: ($) =>
      choice(
        $.array_type,
        $.map_type,
        $.handle_type,
        $.pending_associated_remote_type,
        $.pending_associated_receiver_type,
        $.pending_remote_type,
        $.pending_receiver_type,
        $.associated_interface_request_type,
        $.interface_request_type,
        $.associated_type,
        $.primitive_type,
        $.qualified_identifier,
      ),

    array_type: ($) =>
      seq(
        "array",
        "<",
        field("element", $.type_spec),
        optional(seq(",", field("length", $.integer))),
        ">",
      ),

    map_type: ($) =>
      seq(
        "map",
        "<",
        field("key", choice($.primitive_type, $.qualified_identifier)),
        ",",
        field("value", $.type_spec),
        ">",
      ),

    handle_type: ($) =>
      seq("handle", optional(seq("<", $.specific_handle_type, ">"))),

    pending_remote_type: ($) =>
      seq("pending_remote", "<", field("interface", $.qualified_identifier), ">"),

    pending_receiver_type: ($) =>
      seq("pending_receiver", "<", field("interface", $.qualified_identifier), ">"),

    pending_associated_remote_type: ($) =>
      seq(
        "pending_associated_remote",
        "<",
        field("interface", $.qualified_identifier),
        ">",
      ),

    pending_associated_receiver_type: ($) =>
      seq(
        "pending_associated_receiver",
        "<",
        field("interface", $.qualified_identifier),
        ">",
      ),

    associated_interface_request_type: ($) =>
      seq("associated", $.qualified_identifier, "&"),

    associated_type: ($) => seq("associated", $.qualified_identifier),

    interface_request_type: ($) => seq($.qualified_identifier, "&"),

    primitive_type: () =>
      choice(
        "bool",
        "int8",
        "uint8",
        "int16",
        "uint16",
        "int32",
        "uint32",
        "int64",
        "uint64",
        "float",
        "double",
        "string",
      ),

    specific_handle_type: () =>
      choice(
        "message_pipe",
        "shared_buffer",
        "data_pipe_consumer",
        "data_pipe_producer",
        "platform",
      ),

    default_value: ($) => seq("=", $.constant),

    constant: ($) => choice($.literal, $.qualified_identifier),

    literal: ($) => choice($.number, $.boolean, $.default_literal, $.string),

    boolean: () => choice("true", "false"),

    default_literal: () => "default",

    ordinal: ($) => seq("@", field("value", $.ordinal_value)),

    ordinal_value: () => token(/[0-9]+/),

    integer: () => token(seq(optional(choice("+", "-")), choice(/0[xX][0-9a-fA-F]+/, /0|[1-9][0-9]*/))),

    number: () =>
      token(
        seq(
          optional(choice("+", "-")),
          choice(
            /0[xX][0-9a-fA-F]+/,
            /([0-9]+\.[0-9]*|\.[0-9]+)([eE][+-]?[0-9]+)?/,
            /[0-9]+([eE][+-]?[0-9]+)?/,
          ),
        ),
      ),

    string: ($) =>
      seq(
        '"',
        repeat(choice(token.immediate(/[^"\\\n]+/), $.escape_sequence)),
        '"',
      ),

    escape_sequence: () => token.immediate(seq("\\", /["\\/bfnrt]/)),

    qualified_identifier: ($) => seq($.identifier, repeat(seq(".", $.identifier))),

    identifier: () => /[A-Za-z_][A-Za-z0-9_]*/,

    comment: () =>
      token(
        choice(
          seq("//", /[^\n]*/),
          seq("/*", /[^*]*\*+([^/*][^*]*\*+)*/, "/"),
        ),
      ),
  },
});
