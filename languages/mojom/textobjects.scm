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
