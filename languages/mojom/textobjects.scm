; Copyright 2026 Dora Lee

(struct_declaration) @class.around
(struct_body) @class.inside

(union_declaration) @class.around
(union_body) @class.inside

(interface_declaration) @class.around
(interface_body) @class.inside

(enum_declaration) @class.around
(enum_body) @class.inside

(method_declaration) @function.around
(parameter_list) @function.inside

(comment)+ @comment.around
