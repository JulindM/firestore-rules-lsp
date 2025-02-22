#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#ifdef _MSC_VER
#pragma optimize("", off)
#elif defined(__clang__)
#pragma clang optimize off
#elif defined(__GNUC__)
#pragma GCC optimize ("O0")
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 161
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 126
#define ALIAS_COUNT 1
#define TOKEN_COUNT 73
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 7
#define PRODUCTION_ID_COUNT 2

enum ts_symbol_identifiers {
  anon_sym_rules_version = 1,
  anon_sym_EQ = 2,
  anon_sym_SEMI = 3,
  anon_sym_service = 4,
  anon_sym_cloud_DOTfirestore = 5,
  sym_comment = 6,
  sym_identifier = 7,
  anon_sym_true = 8,
  anon_sym_false = 9,
  sym_number = 10,
  sym_null = 11,
  anon_sym_DQUOTE = 12,
  anon_sym_SQUOTE = 13,
  sym_unescaped_double_string_fragment = 14,
  sym_unescaped_single_string_fragment = 15,
  sym_escape_sequence = 16,
  anon_sym_DOLLAR_LPAREN = 17,
  anon_sym_RPAREN = 18,
  anon_sym_SLASH = 19,
  anon_sym_get = 20,
  anon_sym_getAfter = 21,
  anon_sym_exists = 22,
  anon_sym_existsAfter = 23,
  anon_sym_debug = 24,
  anon_sym_LPAREN = 25,
  anon_sym_COMMA = 26,
  anon_sym_duration = 27,
  anon_sym_hashing = 28,
  anon_sym_latlng = 29,
  anon_sym_math = 30,
  anon_sym_timestamp = 31,
  anon_sym_request = 32,
  anon_sym_resource = 33,
  anon_sym_LPAREN2 = 34,
  anon_sym_LBRACK = 35,
  anon_sym_RBRACK = 36,
  aux_sym_range_token1 = 37,
  anon_sym_COLON = 38,
  anon_sym_SLASHd_PLUS_SLASH = 39,
  anon_sym_DOT = 40,
  anon_sym_BANG = 41,
  anon_sym_DASH = 42,
  anon_sym_STAR = 43,
  anon_sym_PERCENT = 44,
  anon_sym_PLUS = 45,
  anon_sym_LT = 46,
  anon_sym_LT_EQ = 47,
  anon_sym_GT_EQ = 48,
  anon_sym_GT = 49,
  anon_sym_EQ_EQ = 50,
  anon_sym_BANG_EQ = 51,
  anon_sym_in = 52,
  anon_sym_AMP_AMP = 53,
  anon_sym_PIPE_PIPE = 54,
  anon_sym_QMARK = 55,
  anon_sym_let = 56,
  anon_sym_return = 57,
  anon_sym_LBRACE = 58,
  anon_sym_RBRACE = 59,
  anon_sym_function = 60,
  sym_collection_path_seg = 61,
  sym_single_path_seg = 62,
  sym_multi_path_seg = 63,
  anon_sym_read = 64,
  anon_sym_write = 65,
  anon_sym_list = 66,
  anon_sym_create = 67,
  anon_sym_update = 68,
  anon_sym_delete = 69,
  anon_sym_allow = 70,
  anon_sym_if = 71,
  anon_sym_match = 72,
  sym_source_file = 73,
  sym_rules_version_def = 74,
  sym_service_name = 75,
  sym_literal = 76,
  sym_string = 77,
  sym_path_segment = 78,
  sym_path = 79,
  sym_function_argument = 80,
  sym_namespace_reserved_function = 81,
  sym_function_calling_name = 82,
  sym_function_call = 83,
  sym_namespace_reserved_variable = 84,
  sym_variable = 85,
  sym_expr_group = 86,
  sym_list = 87,
  sym_primary = 88,
  sym_range = 89,
  sym_indexing = 90,
  sym_field_indexing = 91,
  sym_member_object = 92,
  sym_member_field = 93,
  sym_member = 94,
  sym_unary = 95,
  sym_multiplication = 96,
  sym_addition = 97,
  sym_relation = 98,
  sym_conditional_and = 99,
  sym_conditional_or = 100,
  sym_ternary = 101,
  sym_expr = 102,
  sym_variable_def = 103,
  sym_fun_return = 104,
  sym_function_body = 105,
  sym_param_list = 106,
  sym_function_def = 107,
  sym_match_path = 108,
  sym_method = 109,
  sym_rule_def = 110,
  sym_match_def = 111,
  sym_service_body = 112,
  sym_match_body = 113,
  aux_sym_string_repeat1 = 114,
  aux_sym_string_repeat2 = 115,
  aux_sym_path_repeat1 = 116,
  aux_sym_function_call_repeat1 = 117,
  aux_sym_list_repeat1 = 118,
  aux_sym_unary_repeat1 = 119,
  aux_sym_unary_repeat2 = 120,
  aux_sym_function_body_repeat1 = 121,
  aux_sym_param_list_repeat1 = 122,
  aux_sym_match_path_repeat1 = 123,
  aux_sym_rule_def_repeat1 = 124,
  aux_sym_service_body_repeat1 = 125,
  anon_alias_sym_function_name = 126,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_rules_version] = "rules_version",
  [anon_sym_EQ] = "=",
  [anon_sym_SEMI] = ";",
  [anon_sym_service] = "service",
  [anon_sym_cloud_DOTfirestore] = "cloud.firestore",
  [sym_comment] = "comment",
  [sym_identifier] = "identifier",
  [anon_sym_true] = "true",
  [anon_sym_false] = "false",
  [sym_number] = "number",
  [sym_null] = "null",
  [anon_sym_DQUOTE] = "\"",
  [anon_sym_SQUOTE] = "'",
  [sym_unescaped_double_string_fragment] = "string_fragment",
  [sym_unescaped_single_string_fragment] = "string_fragment",
  [sym_escape_sequence] = "escape_sequence",
  [anon_sym_DOLLAR_LPAREN] = "$(",
  [anon_sym_RPAREN] = ")",
  [anon_sym_SLASH] = "/",
  [anon_sym_get] = "get",
  [anon_sym_getAfter] = "getAfter",
  [anon_sym_exists] = "exists",
  [anon_sym_existsAfter] = "existsAfter",
  [anon_sym_debug] = "debug",
  [anon_sym_LPAREN] = "(",
  [anon_sym_COMMA] = ",",
  [anon_sym_duration] = "duration",
  [anon_sym_hashing] = "hashing",
  [anon_sym_latlng] = "latlng",
  [anon_sym_math] = "math",
  [anon_sym_timestamp] = "timestamp",
  [anon_sym_request] = "request",
  [anon_sym_resource] = "resource",
  [anon_sym_LPAREN2] = "(",
  [anon_sym_LBRACK] = "[",
  [anon_sym_RBRACK] = "]",
  [aux_sym_range_token1] = "range_token1",
  [anon_sym_COLON] = ":",
  [anon_sym_SLASHd_PLUS_SLASH] = "/d+/",
  [anon_sym_DOT] = ".",
  [anon_sym_BANG] = "!",
  [anon_sym_DASH] = "-",
  [anon_sym_STAR] = "*",
  [anon_sym_PERCENT] = "%",
  [anon_sym_PLUS] = "+",
  [anon_sym_LT] = "<",
  [anon_sym_LT_EQ] = "<=",
  [anon_sym_GT_EQ] = ">=",
  [anon_sym_GT] = ">",
  [anon_sym_EQ_EQ] = "==",
  [anon_sym_BANG_EQ] = "!=",
  [anon_sym_in] = "in",
  [anon_sym_AMP_AMP] = "&&",
  [anon_sym_PIPE_PIPE] = "||",
  [anon_sym_QMARK] = "\?",
  [anon_sym_let] = "let",
  [anon_sym_return] = "return",
  [anon_sym_LBRACE] = "{",
  [anon_sym_RBRACE] = "}",
  [anon_sym_function] = "function",
  [sym_collection_path_seg] = "collection_path_seg",
  [sym_single_path_seg] = "single_path_seg",
  [sym_multi_path_seg] = "multi_path_seg",
  [anon_sym_read] = "read",
  [anon_sym_write] = "write",
  [anon_sym_list] = "list",
  [anon_sym_create] = "create",
  [anon_sym_update] = "update",
  [anon_sym_delete] = "delete",
  [anon_sym_allow] = "allow",
  [anon_sym_if] = "if",
  [anon_sym_match] = "match",
  [sym_source_file] = "source_file",
  [sym_rules_version_def] = "rules_version_def",
  [sym_service_name] = "service_name",
  [sym_literal] = "literal",
  [sym_string] = "string",
  [sym_path_segment] = "path_segment",
  [sym_path] = "path",
  [sym_function_argument] = "function_argument",
  [sym_namespace_reserved_function] = "namespace_reserved_function",
  [sym_function_calling_name] = "function_calling_name",
  [sym_function_call] = "function_call",
  [sym_namespace_reserved_variable] = "namespace_reserved_variable",
  [sym_variable] = "variable",
  [sym_expr_group] = "expr_group",
  [sym_list] = "list",
  [sym_primary] = "primary",
  [sym_range] = "range",
  [sym_indexing] = "indexing",
  [sym_field_indexing] = "field_indexing",
  [sym_member_object] = "member_object",
  [sym_member_field] = "member_field",
  [sym_member] = "member",
  [sym_unary] = "unary",
  [sym_multiplication] = "multiplication",
  [sym_addition] = "addition",
  [sym_relation] = "relation",
  [sym_conditional_and] = "conditional_and",
  [sym_conditional_or] = "conditional_or",
  [sym_ternary] = "ternary",
  [sym_expr] = "expr",
  [sym_variable_def] = "variable_def",
  [sym_fun_return] = "fun_return",
  [sym_function_body] = "function_body",
  [sym_param_list] = "param_list",
  [sym_function_def] = "function_def",
  [sym_match_path] = "match_path",
  [sym_method] = "method",
  [sym_rule_def] = "rule_def",
  [sym_match_def] = "match_def",
  [sym_service_body] = "service_body",
  [sym_match_body] = "match_body",
  [aux_sym_string_repeat1] = "string_repeat1",
  [aux_sym_string_repeat2] = "string_repeat2",
  [aux_sym_path_repeat1] = "path_repeat1",
  [aux_sym_function_call_repeat1] = "function_call_repeat1",
  [aux_sym_list_repeat1] = "list_repeat1",
  [aux_sym_unary_repeat1] = "unary_repeat1",
  [aux_sym_unary_repeat2] = "unary_repeat2",
  [aux_sym_function_body_repeat1] = "function_body_repeat1",
  [aux_sym_param_list_repeat1] = "param_list_repeat1",
  [aux_sym_match_path_repeat1] = "match_path_repeat1",
  [aux_sym_rule_def_repeat1] = "rule_def_repeat1",
  [aux_sym_service_body_repeat1] = "service_body_repeat1",
  [anon_alias_sym_function_name] = "function_name",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [anon_sym_rules_version] = anon_sym_rules_version,
  [anon_sym_EQ] = anon_sym_EQ,
  [anon_sym_SEMI] = anon_sym_SEMI,
  [anon_sym_service] = anon_sym_service,
  [anon_sym_cloud_DOTfirestore] = anon_sym_cloud_DOTfirestore,
  [sym_comment] = sym_comment,
  [sym_identifier] = sym_identifier,
  [anon_sym_true] = anon_sym_true,
  [anon_sym_false] = anon_sym_false,
  [sym_number] = sym_number,
  [sym_null] = sym_null,
  [anon_sym_DQUOTE] = anon_sym_DQUOTE,
  [anon_sym_SQUOTE] = anon_sym_SQUOTE,
  [sym_unescaped_double_string_fragment] = sym_unescaped_double_string_fragment,
  [sym_unescaped_single_string_fragment] = sym_unescaped_double_string_fragment,
  [sym_escape_sequence] = sym_escape_sequence,
  [anon_sym_DOLLAR_LPAREN] = anon_sym_DOLLAR_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [anon_sym_SLASH] = anon_sym_SLASH,
  [anon_sym_get] = anon_sym_get,
  [anon_sym_getAfter] = anon_sym_getAfter,
  [anon_sym_exists] = anon_sym_exists,
  [anon_sym_existsAfter] = anon_sym_existsAfter,
  [anon_sym_debug] = anon_sym_debug,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [anon_sym_duration] = anon_sym_duration,
  [anon_sym_hashing] = anon_sym_hashing,
  [anon_sym_latlng] = anon_sym_latlng,
  [anon_sym_math] = anon_sym_math,
  [anon_sym_timestamp] = anon_sym_timestamp,
  [anon_sym_request] = anon_sym_request,
  [anon_sym_resource] = anon_sym_resource,
  [anon_sym_LPAREN2] = anon_sym_LPAREN,
  [anon_sym_LBRACK] = anon_sym_LBRACK,
  [anon_sym_RBRACK] = anon_sym_RBRACK,
  [aux_sym_range_token1] = aux_sym_range_token1,
  [anon_sym_COLON] = anon_sym_COLON,
  [anon_sym_SLASHd_PLUS_SLASH] = anon_sym_SLASHd_PLUS_SLASH,
  [anon_sym_DOT] = anon_sym_DOT,
  [anon_sym_BANG] = anon_sym_BANG,
  [anon_sym_DASH] = anon_sym_DASH,
  [anon_sym_STAR] = anon_sym_STAR,
  [anon_sym_PERCENT] = anon_sym_PERCENT,
  [anon_sym_PLUS] = anon_sym_PLUS,
  [anon_sym_LT] = anon_sym_LT,
  [anon_sym_LT_EQ] = anon_sym_LT_EQ,
  [anon_sym_GT_EQ] = anon_sym_GT_EQ,
  [anon_sym_GT] = anon_sym_GT,
  [anon_sym_EQ_EQ] = anon_sym_EQ_EQ,
  [anon_sym_BANG_EQ] = anon_sym_BANG_EQ,
  [anon_sym_in] = anon_sym_in,
  [anon_sym_AMP_AMP] = anon_sym_AMP_AMP,
  [anon_sym_PIPE_PIPE] = anon_sym_PIPE_PIPE,
  [anon_sym_QMARK] = anon_sym_QMARK,
  [anon_sym_let] = anon_sym_let,
  [anon_sym_return] = anon_sym_return,
  [anon_sym_LBRACE] = anon_sym_LBRACE,
  [anon_sym_RBRACE] = anon_sym_RBRACE,
  [anon_sym_function] = anon_sym_function,
  [sym_collection_path_seg] = sym_collection_path_seg,
  [sym_single_path_seg] = sym_single_path_seg,
  [sym_multi_path_seg] = sym_multi_path_seg,
  [anon_sym_read] = anon_sym_read,
  [anon_sym_write] = anon_sym_write,
  [anon_sym_list] = anon_sym_list,
  [anon_sym_create] = anon_sym_create,
  [anon_sym_update] = anon_sym_update,
  [anon_sym_delete] = anon_sym_delete,
  [anon_sym_allow] = anon_sym_allow,
  [anon_sym_if] = anon_sym_if,
  [anon_sym_match] = anon_sym_match,
  [sym_source_file] = sym_source_file,
  [sym_rules_version_def] = sym_rules_version_def,
  [sym_service_name] = sym_service_name,
  [sym_literal] = sym_literal,
  [sym_string] = sym_string,
  [sym_path_segment] = sym_path_segment,
  [sym_path] = sym_path,
  [sym_function_argument] = sym_function_argument,
  [sym_namespace_reserved_function] = sym_namespace_reserved_function,
  [sym_function_calling_name] = sym_function_calling_name,
  [sym_function_call] = sym_function_call,
  [sym_namespace_reserved_variable] = sym_namespace_reserved_variable,
  [sym_variable] = sym_variable,
  [sym_expr_group] = sym_expr_group,
  [sym_list] = sym_list,
  [sym_primary] = sym_primary,
  [sym_range] = sym_range,
  [sym_indexing] = sym_indexing,
  [sym_field_indexing] = sym_field_indexing,
  [sym_member_object] = sym_member_object,
  [sym_member_field] = sym_member_field,
  [sym_member] = sym_member,
  [sym_unary] = sym_unary,
  [sym_multiplication] = sym_multiplication,
  [sym_addition] = sym_addition,
  [sym_relation] = sym_relation,
  [sym_conditional_and] = sym_conditional_and,
  [sym_conditional_or] = sym_conditional_or,
  [sym_ternary] = sym_ternary,
  [sym_expr] = sym_expr,
  [sym_variable_def] = sym_variable_def,
  [sym_fun_return] = sym_fun_return,
  [sym_function_body] = sym_function_body,
  [sym_param_list] = sym_param_list,
  [sym_function_def] = sym_function_def,
  [sym_match_path] = sym_match_path,
  [sym_method] = sym_method,
  [sym_rule_def] = sym_rule_def,
  [sym_match_def] = sym_match_def,
  [sym_service_body] = sym_service_body,
  [sym_match_body] = sym_match_body,
  [aux_sym_string_repeat1] = aux_sym_string_repeat1,
  [aux_sym_string_repeat2] = aux_sym_string_repeat2,
  [aux_sym_path_repeat1] = aux_sym_path_repeat1,
  [aux_sym_function_call_repeat1] = aux_sym_function_call_repeat1,
  [aux_sym_list_repeat1] = aux_sym_list_repeat1,
  [aux_sym_unary_repeat1] = aux_sym_unary_repeat1,
  [aux_sym_unary_repeat2] = aux_sym_unary_repeat2,
  [aux_sym_function_body_repeat1] = aux_sym_function_body_repeat1,
  [aux_sym_param_list_repeat1] = aux_sym_param_list_repeat1,
  [aux_sym_match_path_repeat1] = aux_sym_match_path_repeat1,
  [aux_sym_rule_def_repeat1] = aux_sym_rule_def_repeat1,
  [aux_sym_service_body_repeat1] = aux_sym_service_body_repeat1,
  [anon_alias_sym_function_name] = anon_alias_sym_function_name,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [anon_sym_rules_version] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SEMI] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_service] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_cloud_DOTfirestore] = {
    .visible = true,
    .named = false,
  },
  [sym_comment] = {
    .visible = true,
    .named = true,
  },
  [sym_identifier] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_true] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_false] = {
    .visible = true,
    .named = false,
  },
  [sym_number] = {
    .visible = true,
    .named = true,
  },
  [sym_null] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_DQUOTE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SQUOTE] = {
    .visible = true,
    .named = false,
  },
  [sym_unescaped_double_string_fragment] = {
    .visible = true,
    .named = true,
  },
  [sym_unescaped_single_string_fragment] = {
    .visible = true,
    .named = true,
  },
  [sym_escape_sequence] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_DOLLAR_LPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SLASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_get] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_getAfter] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_exists] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_existsAfter] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_debug] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COMMA] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_duration] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_hashing] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_latlng] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_math] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_timestamp] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_request] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_resource] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LPAREN2] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACK] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_range_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_COLON] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SLASHd_PLUS_SLASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DOT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_BANG] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_STAR] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PERCENT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PLUS] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LT_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_GT_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_GT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_EQ_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_BANG_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_in] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_AMP_AMP] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PIPE_PIPE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_QMARK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_let] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_return] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_function] = {
    .visible = true,
    .named = false,
  },
  [sym_collection_path_seg] = {
    .visible = true,
    .named = true,
  },
  [sym_single_path_seg] = {
    .visible = true,
    .named = true,
  },
  [sym_multi_path_seg] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_read] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_write] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_list] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_create] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_update] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_delete] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_allow] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_if] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_match] = {
    .visible = true,
    .named = false,
  },
  [sym_source_file] = {
    .visible = true,
    .named = true,
  },
  [sym_rules_version_def] = {
    .visible = true,
    .named = true,
  },
  [sym_service_name] = {
    .visible = true,
    .named = true,
  },
  [sym_literal] = {
    .visible = true,
    .named = true,
  },
  [sym_string] = {
    .visible = true,
    .named = true,
  },
  [sym_path_segment] = {
    .visible = true,
    .named = true,
  },
  [sym_path] = {
    .visible = true,
    .named = true,
  },
  [sym_function_argument] = {
    .visible = true,
    .named = true,
  },
  [sym_namespace_reserved_function] = {
    .visible = true,
    .named = true,
  },
  [sym_function_calling_name] = {
    .visible = true,
    .named = true,
  },
  [sym_function_call] = {
    .visible = true,
    .named = true,
  },
  [sym_namespace_reserved_variable] = {
    .visible = true,
    .named = true,
  },
  [sym_variable] = {
    .visible = true,
    .named = true,
  },
  [sym_expr_group] = {
    .visible = true,
    .named = true,
  },
  [sym_list] = {
    .visible = true,
    .named = true,
  },
  [sym_primary] = {
    .visible = true,
    .named = true,
  },
  [sym_range] = {
    .visible = true,
    .named = true,
  },
  [sym_indexing] = {
    .visible = true,
    .named = true,
  },
  [sym_field_indexing] = {
    .visible = true,
    .named = true,
  },
  [sym_member_object] = {
    .visible = true,
    .named = true,
  },
  [sym_member_field] = {
    .visible = true,
    .named = true,
  },
  [sym_member] = {
    .visible = true,
    .named = true,
  },
  [sym_unary] = {
    .visible = true,
    .named = true,
  },
  [sym_multiplication] = {
    .visible = true,
    .named = true,
  },
  [sym_addition] = {
    .visible = true,
    .named = true,
  },
  [sym_relation] = {
    .visible = true,
    .named = true,
  },
  [sym_conditional_and] = {
    .visible = true,
    .named = true,
  },
  [sym_conditional_or] = {
    .visible = true,
    .named = true,
  },
  [sym_ternary] = {
    .visible = true,
    .named = true,
  },
  [sym_expr] = {
    .visible = true,
    .named = true,
  },
  [sym_variable_def] = {
    .visible = true,
    .named = true,
  },
  [sym_fun_return] = {
    .visible = true,
    .named = true,
  },
  [sym_function_body] = {
    .visible = true,
    .named = true,
  },
  [sym_param_list] = {
    .visible = true,
    .named = true,
  },
  [sym_function_def] = {
    .visible = true,
    .named = true,
  },
  [sym_match_path] = {
    .visible = true,
    .named = true,
  },
  [sym_method] = {
    .visible = true,
    .named = true,
  },
  [sym_rule_def] = {
    .visible = true,
    .named = true,
  },
  [sym_match_def] = {
    .visible = true,
    .named = true,
  },
  [sym_service_body] = {
    .visible = true,
    .named = true,
  },
  [sym_match_body] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_string_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_string_repeat2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_path_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_function_call_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_list_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_unary_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_unary_repeat2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_function_body_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_param_list_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_match_path_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_rule_def_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_service_body_repeat1] = {
    .visible = false,
    .named = false,
  },
  [anon_alias_sym_function_name] = {
    .visible = true,
    .named = false,
  },
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
  [1] = {
    [1] = anon_alias_sym_function_name,
  },
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 3,
  [4] = 4,
  [5] = 5,
  [6] = 6,
  [7] = 7,
  [8] = 8,
  [9] = 9,
  [10] = 10,
  [11] = 11,
  [12] = 12,
  [13] = 13,
  [14] = 14,
  [15] = 15,
  [16] = 16,
  [17] = 17,
  [18] = 18,
  [19] = 19,
  [20] = 20,
  [21] = 21,
  [22] = 22,
  [23] = 23,
  [24] = 24,
  [25] = 25,
  [26] = 26,
  [27] = 27,
  [28] = 28,
  [29] = 29,
  [30] = 30,
  [31] = 31,
  [32] = 32,
  [33] = 33,
  [34] = 34,
  [35] = 35,
  [36] = 36,
  [37] = 37,
  [38] = 38,
  [39] = 39,
  [40] = 40,
  [41] = 41,
  [42] = 42,
  [43] = 43,
  [44] = 44,
  [45] = 45,
  [46] = 46,
  [47] = 47,
  [48] = 48,
  [49] = 49,
  [50] = 50,
  [51] = 51,
  [52] = 52,
  [53] = 53,
  [54] = 54,
  [55] = 55,
  [56] = 56,
  [57] = 57,
  [58] = 58,
  [59] = 59,
  [60] = 60,
  [61] = 61,
  [62] = 62,
  [63] = 63,
  [64] = 64,
  [65] = 65,
  [66] = 66,
  [67] = 67,
  [68] = 68,
  [69] = 69,
  [70] = 70,
  [71] = 71,
  [72] = 72,
  [73] = 73,
  [74] = 74,
  [75] = 75,
  [76] = 76,
  [77] = 77,
  [78] = 78,
  [79] = 79,
  [80] = 80,
  [81] = 81,
  [82] = 82,
  [83] = 83,
  [84] = 84,
  [85] = 85,
  [86] = 86,
  [87] = 87,
  [88] = 88,
  [89] = 89,
  [90] = 90,
  [91] = 91,
  [92] = 92,
  [93] = 93,
  [94] = 94,
  [95] = 95,
  [96] = 96,
  [97] = 97,
  [98] = 98,
  [99] = 99,
  [100] = 100,
  [101] = 101,
  [102] = 102,
  [103] = 103,
  [104] = 104,
  [105] = 105,
  [106] = 106,
  [107] = 107,
  [108] = 108,
  [109] = 109,
  [110] = 110,
  [111] = 111,
  [112] = 112,
  [113] = 113,
  [114] = 114,
  [115] = 115,
  [116] = 116,
  [117] = 117,
  [118] = 118,
  [119] = 119,
  [120] = 120,
  [121] = 121,
  [122] = 122,
  [123] = 123,
  [124] = 124,
  [125] = 125,
  [126] = 126,
  [127] = 127,
  [128] = 128,
  [129] = 129,
  [130] = 130,
  [131] = 131,
  [132] = 132,
  [133] = 133,
  [134] = 134,
  [135] = 135,
  [136] = 136,
  [137] = 137,
  [138] = 138,
  [139] = 139,
  [140] = 140,
  [141] = 141,
  [142] = 142,
  [143] = 143,
  [144] = 144,
  [145] = 145,
  [146] = 146,
  [147] = 147,
  [148] = 148,
  [149] = 149,
  [150] = 150,
  [151] = 151,
  [152] = 152,
  [153] = 153,
  [154] = 154,
  [155] = 155,
  [156] = 156,
  [157] = 157,
  [158] = 158,
  [159] = 159,
  [160] = 160,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(165);
      ADVANCE_MAP(
        '!', 297,
        '"', 251,
        '$', 13,
        '%', 300,
        '&', 10,
        '\'', 252,
        '(', 274,
        ')', 261,
        '*', 299,
        '+', 301,
        ',', 275,
        '-', 298,
        '.', 295,
        '/', 263,
        ':', 293,
        ';', 169,
        '<', 302,
        '=', 168,
        '>', 305,
        '?', 311,
        '[', 291,
        '\\', 161,
        ']', 292,
        'a', 86,
        'c', 84,
        'd', 44,
        'e', 158,
        'f', 31,
        'g', 45,
        'h', 27,
        'i', 66,
        'l', 28,
        'm', 32,
        'n', 153,
        'r', 46,
        's', 56,
        't', 75,
        'u', 109,
        'w', 114,
        '{', 314,
        '|', 159,
        '}', 315,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(164);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(247);
      END_STATE();
    case 1:
      if (lookahead == '\n') ADVANCE(172);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead != 0) ADVANCE(1);
      END_STATE();
    case 2:
      ADVANCE_MAP(
        '!', 296,
        '"', 251,
        '\'', 252,
        '(', 290,
        ')', 261,
        ',', 275,
        '-', 298,
        '/', 262,
        '=', 167,
        '[', 291,
        ']', 292,
        'd', 181,
        'e', 241,
        'f', 173,
        'g', 182,
        'h', 174,
        'l', 175,
        'm', 177,
        'n', 238,
        'r', 183,
        't', 198,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(2);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(247);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 3:
      ADVANCE_MAP(
        '!', 23,
        '%', 300,
        '&', 10,
        '(', 274,
        ')', 261,
        '*', 299,
        '+', 301,
        ',', 275,
        '-', 298,
        '.', 295,
        '/', 262,
        ':', 293,
        ';', 169,
        '<', 302,
        '=', 168,
        '>', 305,
        '?', 311,
        '[', 291,
        ']', 292,
        'i', 94,
        '|', 159,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(5);
      END_STATE();
    case 4:
      ADVANCE_MAP(
        '!', 23,
        '%', 300,
        '&', 10,
        '(', 274,
        ')', 261,
        '*', 299,
        '+', 301,
        ',', 275,
        '-', 298,
        '.', 295,
        '/', 262,
        ':', 293,
        ';', 169,
        '<', 302,
        '=', 24,
        '>', 305,
        '?', 311,
        '[', 291,
        ']', 292,
        'i', 94,
        '|', 159,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(6);
      END_STATE();
    case 5:
      ADVANCE_MAP(
        '!', 23,
        '%', 300,
        '&', 10,
        ')', 261,
        '*', 299,
        '+', 301,
        ',', 275,
        '-', 298,
        '.', 295,
        '/', 262,
        ':', 293,
        ';', 169,
        '<', 302,
        '=', 168,
        '>', 305,
        '?', 311,
        '[', 291,
        ']', 292,
        'i', 94,
        '|', 159,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(5);
      END_STATE();
    case 6:
      ADVANCE_MAP(
        '!', 23,
        '%', 300,
        '&', 10,
        ')', 261,
        '*', 299,
        '+', 301,
        ',', 275,
        '-', 298,
        '.', 295,
        '/', 262,
        ':', 293,
        ';', 169,
        '<', 302,
        '=', 24,
        '>', 305,
        '?', 311,
        '[', 291,
        ']', 292,
        'i', 94,
        '|', 159,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(6);
      END_STATE();
    case 7:
      if (lookahead == '"') ADVANCE(251);
      if (lookahead == '/') ADVANCE(18);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(7);
      END_STATE();
    case 8:
      if (lookahead == '"') ADVANCE(251);
      if (lookahead == '/') ADVANCE(254);
      if (lookahead == '\\') ADVANCE(161);
      if (lookahead == '\n' ||
          lookahead == '\r') SKIP(7);
      if (('\t' <= lookahead && lookahead <= '\f') ||
          lookahead == ' ') ADVANCE(253);
      if (lookahead != 0) ADVANCE(255);
      END_STATE();
    case 9:
      if (lookahead == '$') ADVANCE(13);
      if (lookahead == ')') ADVANCE(261);
      if (lookahead == '/') ADVANCE(19);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(9);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 10:
      if (lookahead == '&') ADVANCE(309);
      END_STATE();
    case 11:
      if (lookahead == '\'') ADVANCE(252);
      if (lookahead == '/') ADVANCE(257);
      if (lookahead == '\\') ADVANCE(161);
      if (lookahead == '\n' ||
          lookahead == '\r') SKIP(12);
      if (('\t' <= lookahead && lookahead <= '\f') ||
          lookahead == ' ') ADVANCE(256);
      if (lookahead != 0) ADVANCE(258);
      END_STATE();
    case 12:
      if (lookahead == '\'') ADVANCE(252);
      if (lookahead == '/') ADVANCE(18);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(12);
      END_STATE();
    case 13:
      if (lookahead == '(') ADVANCE(260);
      END_STATE();
    case 14:
      if (lookahead == '*') ADVANCE(160);
      END_STATE();
    case 15:
      if (lookahead == '*') ADVANCE(14);
      END_STATE();
    case 16:
      if (lookahead == '+') ADVANCE(22);
      END_STATE();
    case 17:
      if (lookahead == '.') ADVANCE(67);
      END_STATE();
    case 18:
      if (lookahead == '/') ADVANCE(1);
      END_STATE();
    case 19:
      if (lookahead == '/') ADVANCE(1);
      if (lookahead == 'd') ADVANCE(16);
      END_STATE();
    case 20:
      if (lookahead == '/') ADVANCE(1);
      if (lookahead == '{') ADVANCE(163);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(317);
      END_STATE();
    case 21:
      ADVANCE_MAP(
        '/', 20,
        'd', 235,
        'h', 174,
        'l', 175,
        'm', 177,
        'r', 183,
        't', 199,
        '{', 314,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(21);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 22:
      if (lookahead == '/') ADVANCE(294);
      END_STATE();
    case 23:
      if (lookahead == '=') ADVANCE(307);
      END_STATE();
    case 24:
      if (lookahead == '=') ADVANCE(306);
      END_STATE();
    case 25:
      if (lookahead == '=') ADVANCE(15);
      if (lookahead == '}') ADVANCE(318);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(25);
      END_STATE();
    case 26:
      if (lookahead == '_') ADVANCE(156);
      END_STATE();
    case 27:
      if (lookahead == 'a') ADVANCE(121);
      END_STATE();
    case 28:
      if (lookahead == 'a') ADVANCE(138);
      if (lookahead == 'e') ADVANCE(132);
      if (lookahead == 'i') ADVANCE(124);
      END_STATE();
    case 29:
      if (lookahead == 'a') ADVANCE(92);
      END_STATE();
    case 30:
      if (lookahead == 'a') ADVANCE(41);
      if (lookahead == 'q') ADVANCE(154);
      if (lookahead == 's') ADVANCE(106);
      if (lookahead == 't') ADVANCE(150);
      END_STATE();
    case 31:
      if (lookahead == 'a') ADVANCE(89);
      if (lookahead == 'u') ADVANCE(95);
      END_STATE();
    case 32:
      if (lookahead == 'a') ADVANCE(133);
      END_STATE();
    case 33:
      if (lookahead == 'a') ADVANCE(139);
      END_STATE();
    case 34:
      if (lookahead == 'a') ADVANCE(142);
      END_STATE();
    case 35:
      if (lookahead == 'a') ADVANCE(144);
      END_STATE();
    case 36:
      if (lookahead == 'b') ADVANCE(148);
      if (lookahead == 'l') ADVANCE(63);
      END_STATE();
    case 37:
      if (lookahead == 'c') ADVANCE(73);
      if (lookahead == 'h') ADVANCE(282);
      END_STATE();
    case 38:
      if (lookahead == 'c') ADVANCE(53);
      END_STATE();
    case 39:
      if (lookahead == 'c') ADVANCE(54);
      END_STATE();
    case 40:
      if (lookahead == 'c') ADVANCE(146);
      END_STATE();
    case 41:
      if (lookahead == 'd') ADVANCE(320);
      END_STATE();
    case 42:
      if (lookahead == 'd') ADVANCE(17);
      END_STATE();
    case 43:
      if (lookahead == 'd') ADVANCE(35);
      END_STATE();
    case 44:
      if (lookahead == 'e') ADVANCE(36);
      if (lookahead == 'u') ADVANCE(119);
      END_STATE();
    case 45:
      if (lookahead == 'e') ADVANCE(131);
      END_STATE();
    case 46:
      if (lookahead == 'e') ADVANCE(30);
      if (lookahead == 'u') ADVANCE(90);
      END_STATE();
    case 47:
      if (lookahead == 'e') ADVANCE(243);
      END_STATE();
    case 48:
      if (lookahead == 'e') ADVANCE(245);
      END_STATE();
    case 49:
      if (lookahead == 'e') ADVANCE(321);
      END_STATE();
    case 50:
      if (lookahead == 'e') ADVANCE(323);
      END_STATE();
    case 51:
      if (lookahead == 'e') ADVANCE(325);
      END_STATE();
    case 52:
      if (lookahead == 'e') ADVANCE(324);
      END_STATE();
    case 53:
      if (lookahead == 'e') ADVANCE(170);
      END_STATE();
    case 54:
      if (lookahead == 'e') ADVANCE(288);
      END_STATE();
    case 55:
      if (lookahead == 'e') ADVANCE(171);
      END_STATE();
    case 56:
      if (lookahead == 'e') ADVANCE(111);
      END_STATE();
    case 57:
      if (lookahead == 'e') ADVANCE(122);
      END_STATE();
    case 58:
      if (lookahead == 'e') ADVANCE(112);
      END_STATE();
    case 59:
      if (lookahead == 'e') ADVANCE(116);
      END_STATE();
    case 60:
      if (lookahead == 'e') ADVANCE(113);
      END_STATE();
    case 61:
      if (lookahead == 'e') ADVANCE(34);
      END_STATE();
    case 62:
      if (lookahead == 'e') ADVANCE(127);
      END_STATE();
    case 63:
      if (lookahead == 'e') ADVANCE(143);
      END_STATE();
    case 64:
      if (lookahead == 'e') ADVANCE(128);
      END_STATE();
    case 65:
      if (lookahead == 'e') ADVANCE(129);
      END_STATE();
    case 66:
      if (lookahead == 'f') ADVANCE(327);
      if (lookahead == 'n') ADVANCE(308);
      END_STATE();
    case 67:
      if (lookahead == 'f') ADVANCE(79);
      END_STATE();
    case 68:
      if (lookahead == 'f') ADVANCE(145);
      END_STATE();
    case 69:
      if (lookahead == 'f') ADVANCE(147);
      END_STATE();
    case 70:
      if (lookahead == 'g') ADVANCE(272);
      END_STATE();
    case 71:
      if (lookahead == 'g') ADVANCE(280);
      END_STATE();
    case 72:
      if (lookahead == 'g') ADVANCE(278);
      END_STATE();
    case 73:
      if (lookahead == 'h') ADVANCE(328);
      END_STATE();
    case 74:
      if (lookahead == 'h') ADVANCE(80);
      END_STATE();
    case 75:
      if (lookahead == 'i') ADVANCE(93);
      if (lookahead == 'r') ADVANCE(152);
      END_STATE();
    case 76:
      if (lookahead == 'i') ADVANCE(38);
      END_STATE();
    case 77:
      if (lookahead == 'i') ADVANCE(104);
      END_STATE();
    case 78:
      if (lookahead == 'i') ADVANCE(141);
      END_STATE();
    case 79:
      if (lookahead == 'i') ADVANCE(120);
      END_STATE();
    case 80:
      if (lookahead == 'i') ADVANCE(101);
      END_STATE();
    case 81:
      if (lookahead == 'i') ADVANCE(126);
      END_STATE();
    case 82:
      if (lookahead == 'i') ADVANCE(105);
      END_STATE();
    case 83:
      if (lookahead == 'i') ADVANCE(107);
      END_STATE();
    case 84:
      if (lookahead == 'l') ADVANCE(103);
      if (lookahead == 'r') ADVANCE(61);
      END_STATE();
    case 85:
      if (lookahead == 'l') ADVANCE(249);
      END_STATE();
    case 86:
      if (lookahead == 'l') ADVANCE(87);
      END_STATE();
    case 87:
      if (lookahead == 'l') ADVANCE(102);
      END_STATE();
    case 88:
      if (lookahead == 'l') ADVANCE(100);
      END_STATE();
    case 89:
      if (lookahead == 'l') ADVANCE(125);
      END_STATE();
    case 90:
      if (lookahead == 'l') ADVANCE(57);
      END_STATE();
    case 91:
      if (lookahead == 'l') ADVANCE(85);
      END_STATE();
    case 92:
      if (lookahead == 'm') ADVANCE(110);
      END_STATE();
    case 93:
      if (lookahead == 'm') ADVANCE(62);
      END_STATE();
    case 94:
      if (lookahead == 'n') ADVANCE(308);
      END_STATE();
    case 95:
      if (lookahead == 'n') ADVANCE(40);
      END_STATE();
    case 96:
      if (lookahead == 'n') ADVANCE(313);
      END_STATE();
    case 97:
      if (lookahead == 'n') ADVANCE(276);
      END_STATE();
    case 98:
      if (lookahead == 'n') ADVANCE(316);
      END_STATE();
    case 99:
      if (lookahead == 'n') ADVANCE(166);
      END_STATE();
    case 100:
      if (lookahead == 'n') ADVANCE(71);
      END_STATE();
    case 101:
      if (lookahead == 'n') ADVANCE(72);
      END_STATE();
    case 102:
      if (lookahead == 'o') ADVANCE(157);
      END_STATE();
    case 103:
      if (lookahead == 'o') ADVANCE(149);
      END_STATE();
    case 104:
      if (lookahead == 'o') ADVANCE(97);
      END_STATE();
    case 105:
      if (lookahead == 'o') ADVANCE(98);
      END_STATE();
    case 106:
      if (lookahead == 'o') ADVANCE(151);
      END_STATE();
    case 107:
      if (lookahead == 'o') ADVANCE(99);
      END_STATE();
    case 108:
      if (lookahead == 'o') ADVANCE(117);
      END_STATE();
    case 109:
      if (lookahead == 'p') ADVANCE(43);
      END_STATE();
    case 110:
      if (lookahead == 'p') ADVANCE(284);
      END_STATE();
    case 111:
      if (lookahead == 'r') ADVANCE(155);
      END_STATE();
    case 112:
      if (lookahead == 'r') ADVANCE(266);
      END_STATE();
    case 113:
      if (lookahead == 'r') ADVANCE(270);
      END_STATE();
    case 114:
      if (lookahead == 'r') ADVANCE(78);
      END_STATE();
    case 115:
      if (lookahead == 'r') ADVANCE(96);
      END_STATE();
    case 116:
      if (lookahead == 'r') ADVANCE(130);
      END_STATE();
    case 117:
      if (lookahead == 'r') ADVANCE(55);
      END_STATE();
    case 118:
      if (lookahead == 'r') ADVANCE(39);
      END_STATE();
    case 119:
      if (lookahead == 'r') ADVANCE(33);
      END_STATE();
    case 120:
      if (lookahead == 'r') ADVANCE(65);
      END_STATE();
    case 121:
      if (lookahead == 's') ADVANCE(74);
      END_STATE();
    case 122:
      if (lookahead == 's') ADVANCE(26);
      END_STATE();
    case 123:
      if (lookahead == 's') ADVANCE(269);
      END_STATE();
    case 124:
      if (lookahead == 's') ADVANCE(134);
      END_STATE();
    case 125:
      if (lookahead == 's') ADVANCE(48);
      END_STATE();
    case 126:
      if (lookahead == 's') ADVANCE(140);
      END_STATE();
    case 127:
      if (lookahead == 's') ADVANCE(136);
      END_STATE();
    case 128:
      if (lookahead == 's') ADVANCE(135);
      END_STATE();
    case 129:
      if (lookahead == 's') ADVANCE(137);
      END_STATE();
    case 130:
      if (lookahead == 's') ADVANCE(83);
      END_STATE();
    case 131:
      if (lookahead == 't') ADVANCE(264);
      END_STATE();
    case 132:
      if (lookahead == 't') ADVANCE(312);
      END_STATE();
    case 133:
      if (lookahead == 't') ADVANCE(37);
      END_STATE();
    case 134:
      if (lookahead == 't') ADVANCE(322);
      END_STATE();
    case 135:
      if (lookahead == 't') ADVANCE(286);
      END_STATE();
    case 136:
      if (lookahead == 't') ADVANCE(29);
      END_STATE();
    case 137:
      if (lookahead == 't') ADVANCE(108);
      END_STATE();
    case 138:
      if (lookahead == 't') ADVANCE(88);
      END_STATE();
    case 139:
      if (lookahead == 't') ADVANCE(77);
      END_STATE();
    case 140:
      if (lookahead == 't') ADVANCE(123);
      END_STATE();
    case 141:
      if (lookahead == 't') ADVANCE(49);
      END_STATE();
    case 142:
      if (lookahead == 't') ADVANCE(50);
      END_STATE();
    case 143:
      if (lookahead == 't') ADVANCE(51);
      END_STATE();
    case 144:
      if (lookahead == 't') ADVANCE(52);
      END_STATE();
    case 145:
      if (lookahead == 't') ADVANCE(58);
      END_STATE();
    case 146:
      if (lookahead == 't') ADVANCE(82);
      END_STATE();
    case 147:
      if (lookahead == 't') ADVANCE(60);
      END_STATE();
    case 148:
      if (lookahead == 'u') ADVANCE(70);
      END_STATE();
    case 149:
      if (lookahead == 'u') ADVANCE(42);
      END_STATE();
    case 150:
      if (lookahead == 'u') ADVANCE(115);
      END_STATE();
    case 151:
      if (lookahead == 'u') ADVANCE(118);
      END_STATE();
    case 152:
      if (lookahead == 'u') ADVANCE(47);
      END_STATE();
    case 153:
      if (lookahead == 'u') ADVANCE(91);
      END_STATE();
    case 154:
      if (lookahead == 'u') ADVANCE(64);
      END_STATE();
    case 155:
      if (lookahead == 'v') ADVANCE(76);
      END_STATE();
    case 156:
      if (lookahead == 'v') ADVANCE(59);
      END_STATE();
    case 157:
      if (lookahead == 'w') ADVANCE(326);
      END_STATE();
    case 158:
      if (lookahead == 'x') ADVANCE(81);
      END_STATE();
    case 159:
      if (lookahead == '|') ADVANCE(310);
      END_STATE();
    case 160:
      if (lookahead == '}') ADVANCE(319);
      END_STATE();
    case 161:
      if (lookahead == '"' ||
          lookahead == '\'' ||
          lookahead == '?' ||
          lookahead == '\\' ||
          lookahead == '`') ADVANCE(259);
      END_STATE();
    case 162:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(248);
      END_STATE();
    case 163:
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(25);
      END_STATE();
    case 164:
      if (eof) ADVANCE(165);
      ADVANCE_MAP(
        '!', 297,
        '"', 251,
        '$', 13,
        '%', 300,
        '&', 10,
        '\'', 252,
        '(', 290,
        ')', 261,
        '*', 299,
        '+', 301,
        ',', 275,
        '-', 298,
        '.', 295,
        '/', 263,
        ':', 293,
        ';', 169,
        '<', 302,
        '=', 168,
        '>', 305,
        '?', 311,
        '[', 291,
        ']', 292,
        'a', 86,
        'c', 84,
        'd', 44,
        'e', 158,
        'f', 31,
        'g', 45,
        'h', 27,
        'i', 66,
        'l', 28,
        'm', 32,
        'n', 153,
        'r', 46,
        's', 56,
        't', 75,
        'u', 109,
        'w', 114,
        '{', 314,
        '|', 159,
        '}', 315,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(164);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(247);
      END_STATE();
    case 165:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 166:
      ACCEPT_TOKEN(anon_sym_rules_version);
      END_STATE();
    case 167:
      ACCEPT_TOKEN(anon_sym_EQ);
      END_STATE();
    case 168:
      ACCEPT_TOKEN(anon_sym_EQ);
      if (lookahead == '=') ADVANCE(306);
      END_STATE();
    case 169:
      ACCEPT_TOKEN(anon_sym_SEMI);
      END_STATE();
    case 170:
      ACCEPT_TOKEN(anon_sym_service);
      END_STATE();
    case 171:
      ACCEPT_TOKEN(anon_sym_cloud_DOTfirestore);
      END_STATE();
    case 172:
      ACCEPT_TOKEN(sym_comment);
      END_STATE();
    case 173:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'a') ADVANCE(205);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 174:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'a') ADVANCE(220);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 175:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'a') ADVANCE(230);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 176:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'a') ADVANCE(207);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 177:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'a') ADVANCE(228);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 178:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'a') ADVANCE(229);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 179:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'b') ADVANCE(236);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 180:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'c') ADVANCE(186);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 181:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(179);
      if (lookahead == 'u') ADVANCE(219);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 182:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(226);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 183:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(215);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 184:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(244);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 185:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(246);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 186:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(289);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 187:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(217);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 188:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(218);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 189:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(224);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 190:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(225);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 191:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'f') ADVANCE(233);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 192:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'f') ADVANCE(234);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 193:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'g') ADVANCE(273);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 194:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'g') ADVANCE(281);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 195:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'g') ADVANCE(279);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 196:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'h') ADVANCE(283);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 197:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'h') ADVANCE(202);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 198:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'i') ADVANCE(208);
      if (lookahead == 'r') ADVANCE(239);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 199:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'i') ADVANCE(208);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 200:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'i') ADVANCE(223);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 201:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'i') ADVANCE(212);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 202:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'i') ADVANCE(211);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 203:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'l') ADVANCE(210);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 204:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'l') ADVANCE(250);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 205:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'l') ADVANCE(222);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 206:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'l') ADVANCE(204);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 207:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'm') ADVANCE(214);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 208:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'm') ADVANCE(189);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 209:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'n') ADVANCE(277);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 210:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'n') ADVANCE(194);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 211:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'n') ADVANCE(195);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 212:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'o') ADVANCE(209);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 213:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'o') ADVANCE(237);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 214:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'p') ADVANCE(285);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 215:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'q') ADVANCE(240);
      if (lookahead == 's') ADVANCE(213);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 216:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'r') ADVANCE(180);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 217:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'r') ADVANCE(267);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 218:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'r') ADVANCE(271);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 219:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'r') ADVANCE(178);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 220:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 's') ADVANCE(197);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 221:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 's') ADVANCE(268);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 222:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 's') ADVANCE(185);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 223:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 's') ADVANCE(231);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 224:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 's') ADVANCE(232);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 225:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 's') ADVANCE(227);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 226:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(265);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 227:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(287);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 228:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(196);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 229:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(201);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 230:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(203);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 231:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(221);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 232:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(176);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 233:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(187);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 234:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(188);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 235:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'u') ADVANCE(219);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 236:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'u') ADVANCE(193);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 237:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'u') ADVANCE(216);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 238:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'u') ADVANCE(206);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 239:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'u') ADVANCE(184);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 240:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'u') ADVANCE(190);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 241:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'x') ADVANCE(200);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 242:
      ACCEPT_TOKEN(sym_identifier);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 243:
      ACCEPT_TOKEN(anon_sym_true);
      END_STATE();
    case 244:
      ACCEPT_TOKEN(anon_sym_true);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 245:
      ACCEPT_TOKEN(anon_sym_false);
      END_STATE();
    case 246:
      ACCEPT_TOKEN(anon_sym_false);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 247:
      ACCEPT_TOKEN(sym_number);
      if (lookahead == '.') ADVANCE(162);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(247);
      END_STATE();
    case 248:
      ACCEPT_TOKEN(sym_number);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(248);
      END_STATE();
    case 249:
      ACCEPT_TOKEN(sym_null);
      END_STATE();
    case 250:
      ACCEPT_TOKEN(sym_null);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 251:
      ACCEPT_TOKEN(anon_sym_DQUOTE);
      END_STATE();
    case 252:
      ACCEPT_TOKEN(anon_sym_SQUOTE);
      END_STATE();
    case 253:
      ACCEPT_TOKEN(sym_unescaped_double_string_fragment);
      if (lookahead == '/') ADVANCE(254);
      if (lookahead == '\t' ||
          lookahead == 0x0b ||
          lookahead == '\f' ||
          lookahead == ' ') ADVANCE(253);
      if (lookahead != 0 &&
          (lookahead < '\t' || '\r' < lookahead) &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(255);
      END_STATE();
    case 254:
      ACCEPT_TOKEN(sym_unescaped_double_string_fragment);
      if (lookahead == '/') ADVANCE(255);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(255);
      END_STATE();
    case 255:
      ACCEPT_TOKEN(sym_unescaped_double_string_fragment);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(255);
      END_STATE();
    case 256:
      ACCEPT_TOKEN(sym_unescaped_single_string_fragment);
      if (lookahead == '/') ADVANCE(257);
      if (lookahead == '\t' ||
          lookahead == 0x0b ||
          lookahead == '\f' ||
          lookahead == ' ') ADVANCE(256);
      if (lookahead != 0 &&
          (lookahead < '\t' || '\r' < lookahead) &&
          lookahead != '\'' &&
          lookahead != '\\') ADVANCE(258);
      END_STATE();
    case 257:
      ACCEPT_TOKEN(sym_unescaped_single_string_fragment);
      if (lookahead == '/') ADVANCE(258);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != '\'' &&
          lookahead != '\\') ADVANCE(258);
      END_STATE();
    case 258:
      ACCEPT_TOKEN(sym_unescaped_single_string_fragment);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != '\'' &&
          lookahead != '\\') ADVANCE(258);
      END_STATE();
    case 259:
      ACCEPT_TOKEN(sym_escape_sequence);
      END_STATE();
    case 260:
      ACCEPT_TOKEN(anon_sym_DOLLAR_LPAREN);
      END_STATE();
    case 261:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 262:
      ACCEPT_TOKEN(anon_sym_SLASH);
      if (lookahead == '/') ADVANCE(1);
      END_STATE();
    case 263:
      ACCEPT_TOKEN(anon_sym_SLASH);
      if (lookahead == '/') ADVANCE(1);
      if (lookahead == '{') ADVANCE(163);
      END_STATE();
    case 264:
      ACCEPT_TOKEN(anon_sym_get);
      if (lookahead == 'A') ADVANCE(68);
      END_STATE();
    case 265:
      ACCEPT_TOKEN(anon_sym_get);
      if (lookahead == 'A') ADVANCE(191);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('B' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 266:
      ACCEPT_TOKEN(anon_sym_getAfter);
      END_STATE();
    case 267:
      ACCEPT_TOKEN(anon_sym_getAfter);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 268:
      ACCEPT_TOKEN(anon_sym_exists);
      if (lookahead == 'A') ADVANCE(192);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('B' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 269:
      ACCEPT_TOKEN(anon_sym_exists);
      if (lookahead == 'A') ADVANCE(69);
      END_STATE();
    case 270:
      ACCEPT_TOKEN(anon_sym_existsAfter);
      END_STATE();
    case 271:
      ACCEPT_TOKEN(anon_sym_existsAfter);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 272:
      ACCEPT_TOKEN(anon_sym_debug);
      END_STATE();
    case 273:
      ACCEPT_TOKEN(anon_sym_debug);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 274:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 275:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 276:
      ACCEPT_TOKEN(anon_sym_duration);
      END_STATE();
    case 277:
      ACCEPT_TOKEN(anon_sym_duration);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 278:
      ACCEPT_TOKEN(anon_sym_hashing);
      END_STATE();
    case 279:
      ACCEPT_TOKEN(anon_sym_hashing);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 280:
      ACCEPT_TOKEN(anon_sym_latlng);
      END_STATE();
    case 281:
      ACCEPT_TOKEN(anon_sym_latlng);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 282:
      ACCEPT_TOKEN(anon_sym_math);
      END_STATE();
    case 283:
      ACCEPT_TOKEN(anon_sym_math);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 284:
      ACCEPT_TOKEN(anon_sym_timestamp);
      END_STATE();
    case 285:
      ACCEPT_TOKEN(anon_sym_timestamp);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 286:
      ACCEPT_TOKEN(anon_sym_request);
      END_STATE();
    case 287:
      ACCEPT_TOKEN(anon_sym_request);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 288:
      ACCEPT_TOKEN(anon_sym_resource);
      END_STATE();
    case 289:
      ACCEPT_TOKEN(anon_sym_resource);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(242);
      END_STATE();
    case 290:
      ACCEPT_TOKEN(anon_sym_LPAREN2);
      END_STATE();
    case 291:
      ACCEPT_TOKEN(anon_sym_LBRACK);
      END_STATE();
    case 292:
      ACCEPT_TOKEN(anon_sym_RBRACK);
      END_STATE();
    case 293:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 294:
      ACCEPT_TOKEN(anon_sym_SLASHd_PLUS_SLASH);
      END_STATE();
    case 295:
      ACCEPT_TOKEN(anon_sym_DOT);
      END_STATE();
    case 296:
      ACCEPT_TOKEN(anon_sym_BANG);
      END_STATE();
    case 297:
      ACCEPT_TOKEN(anon_sym_BANG);
      if (lookahead == '=') ADVANCE(307);
      END_STATE();
    case 298:
      ACCEPT_TOKEN(anon_sym_DASH);
      END_STATE();
    case 299:
      ACCEPT_TOKEN(anon_sym_STAR);
      END_STATE();
    case 300:
      ACCEPT_TOKEN(anon_sym_PERCENT);
      END_STATE();
    case 301:
      ACCEPT_TOKEN(anon_sym_PLUS);
      END_STATE();
    case 302:
      ACCEPT_TOKEN(anon_sym_LT);
      if (lookahead == '=') ADVANCE(303);
      END_STATE();
    case 303:
      ACCEPT_TOKEN(anon_sym_LT_EQ);
      END_STATE();
    case 304:
      ACCEPT_TOKEN(anon_sym_GT_EQ);
      END_STATE();
    case 305:
      ACCEPT_TOKEN(anon_sym_GT);
      if (lookahead == '=') ADVANCE(304);
      END_STATE();
    case 306:
      ACCEPT_TOKEN(anon_sym_EQ_EQ);
      END_STATE();
    case 307:
      ACCEPT_TOKEN(anon_sym_BANG_EQ);
      END_STATE();
    case 308:
      ACCEPT_TOKEN(anon_sym_in);
      END_STATE();
    case 309:
      ACCEPT_TOKEN(anon_sym_AMP_AMP);
      END_STATE();
    case 310:
      ACCEPT_TOKEN(anon_sym_PIPE_PIPE);
      END_STATE();
    case 311:
      ACCEPT_TOKEN(anon_sym_QMARK);
      END_STATE();
    case 312:
      ACCEPT_TOKEN(anon_sym_let);
      END_STATE();
    case 313:
      ACCEPT_TOKEN(anon_sym_return);
      END_STATE();
    case 314:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 315:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 316:
      ACCEPT_TOKEN(anon_sym_function);
      END_STATE();
    case 317:
      ACCEPT_TOKEN(sym_collection_path_seg);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(317);
      END_STATE();
    case 318:
      ACCEPT_TOKEN(sym_single_path_seg);
      END_STATE();
    case 319:
      ACCEPT_TOKEN(sym_multi_path_seg);
      END_STATE();
    case 320:
      ACCEPT_TOKEN(anon_sym_read);
      END_STATE();
    case 321:
      ACCEPT_TOKEN(anon_sym_write);
      END_STATE();
    case 322:
      ACCEPT_TOKEN(anon_sym_list);
      END_STATE();
    case 323:
      ACCEPT_TOKEN(anon_sym_create);
      END_STATE();
    case 324:
      ACCEPT_TOKEN(anon_sym_update);
      END_STATE();
    case 325:
      ACCEPT_TOKEN(anon_sym_delete);
      END_STATE();
    case 326:
      ACCEPT_TOKEN(anon_sym_allow);
      END_STATE();
    case 327:
      ACCEPT_TOKEN(anon_sym_if);
      END_STATE();
    case 328:
      ACCEPT_TOKEN(anon_sym_match);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 0},
  [2] = {.lex_state = 2},
  [3] = {.lex_state = 2},
  [4] = {.lex_state = 2},
  [5] = {.lex_state = 2},
  [6] = {.lex_state = 2},
  [7] = {.lex_state = 2},
  [8] = {.lex_state = 2},
  [9] = {.lex_state = 2},
  [10] = {.lex_state = 2},
  [11] = {.lex_state = 2},
  [12] = {.lex_state = 2},
  [13] = {.lex_state = 2},
  [14] = {.lex_state = 2},
  [15] = {.lex_state = 2},
  [16] = {.lex_state = 2},
  [17] = {.lex_state = 2},
  [18] = {.lex_state = 2},
  [19] = {.lex_state = 2},
  [20] = {.lex_state = 2},
  [21] = {.lex_state = 2},
  [22] = {.lex_state = 2},
  [23] = {.lex_state = 2},
  [24] = {.lex_state = 2},
  [25] = {.lex_state = 2},
  [26] = {.lex_state = 3},
  [27] = {.lex_state = 4},
  [28] = {.lex_state = 3},
  [29] = {.lex_state = 4},
  [30] = {.lex_state = 4},
  [31] = {.lex_state = 4},
  [32] = {.lex_state = 4},
  [33] = {.lex_state = 4},
  [34] = {.lex_state = 4},
  [35] = {.lex_state = 4},
  [36] = {.lex_state = 4},
  [37] = {.lex_state = 4},
  [38] = {.lex_state = 4},
  [39] = {.lex_state = 4},
  [40] = {.lex_state = 4},
  [41] = {.lex_state = 4},
  [42] = {.lex_state = 4},
  [43] = {.lex_state = 4},
  [44] = {.lex_state = 4},
  [45] = {.lex_state = 4},
  [46] = {.lex_state = 4},
  [47] = {.lex_state = 4},
  [48] = {.lex_state = 4},
  [49] = {.lex_state = 4},
  [50] = {.lex_state = 4},
  [51] = {.lex_state = 4},
  [52] = {.lex_state = 4},
  [53] = {.lex_state = 4},
  [54] = {.lex_state = 4},
  [55] = {.lex_state = 4},
  [56] = {.lex_state = 4},
  [57] = {.lex_state = 4},
  [58] = {.lex_state = 4},
  [59] = {.lex_state = 4},
  [60] = {.lex_state = 4},
  [61] = {.lex_state = 4},
  [62] = {.lex_state = 4},
  [63] = {.lex_state = 4},
  [64] = {.lex_state = 4},
  [65] = {.lex_state = 4},
  [66] = {.lex_state = 4},
  [67] = {.lex_state = 4},
  [68] = {.lex_state = 21},
  [69] = {.lex_state = 0},
  [70] = {.lex_state = 0},
  [71] = {.lex_state = 0},
  [72] = {.lex_state = 0},
  [73] = {.lex_state = 0},
  [74] = {.lex_state = 0},
  [75] = {.lex_state = 0},
  [76] = {.lex_state = 0},
  [77] = {.lex_state = 21},
  [78] = {.lex_state = 0},
  [79] = {.lex_state = 21},
  [80] = {.lex_state = 21},
  [81] = {.lex_state = 2},
  [82] = {.lex_state = 0},
  [83] = {.lex_state = 0},
  [84] = {.lex_state = 0},
  [85] = {.lex_state = 0},
  [86] = {.lex_state = 0},
  [87] = {.lex_state = 0},
  [88] = {.lex_state = 0},
  [89] = {.lex_state = 0},
  [90] = {.lex_state = 11},
  [91] = {.lex_state = 0},
  [92] = {.lex_state = 8},
  [93] = {.lex_state = 2},
  [94] = {.lex_state = 11},
  [95] = {.lex_state = 0},
  [96] = {.lex_state = 0},
  [97] = {.lex_state = 0},
  [98] = {.lex_state = 0},
  [99] = {.lex_state = 11},
  [100] = {.lex_state = 8},
  [101] = {.lex_state = 8},
  [102] = {.lex_state = 0},
  [103] = {.lex_state = 0},
  [104] = {.lex_state = 0},
  [105] = {.lex_state = 0},
  [106] = {.lex_state = 0},
  [107] = {.lex_state = 0},
  [108] = {.lex_state = 9},
  [109] = {.lex_state = 0},
  [110] = {.lex_state = 0},
  [111] = {.lex_state = 0},
  [112] = {.lex_state = 0},
  [113] = {.lex_state = 0},
  [114] = {.lex_state = 2},
  [115] = {.lex_state = 0},
  [116] = {.lex_state = 9},
  [117] = {.lex_state = 0},
  [118] = {.lex_state = 2},
  [119] = {.lex_state = 2},
  [120] = {.lex_state = 0},
  [121] = {.lex_state = 0},
  [122] = {.lex_state = 0},
  [123] = {.lex_state = 0},
  [124] = {.lex_state = 0},
  [125] = {.lex_state = 0},
  [126] = {.lex_state = 0},
  [127] = {.lex_state = 0},
  [128] = {.lex_state = 0},
  [129] = {.lex_state = 0},
  [130] = {.lex_state = 0},
  [131] = {.lex_state = 0},
  [132] = {.lex_state = 0},
  [133] = {.lex_state = 0},
  [134] = {.lex_state = 9},
  [135] = {.lex_state = 0},
  [136] = {.lex_state = 0},
  [137] = {.lex_state = 0},
  [138] = {.lex_state = 9},
  [139] = {.lex_state = 0},
  [140] = {.lex_state = 0},
  [141] = {.lex_state = 2},
  [142] = {.lex_state = 0},
  [143] = {.lex_state = 0},
  [144] = {.lex_state = 0},
  [145] = {.lex_state = 0},
  [146] = {.lex_state = 3},
  [147] = {.lex_state = 0},
  [148] = {.lex_state = 0},
  [149] = {.lex_state = 0},
  [150] = {.lex_state = 0},
  [151] = {.lex_state = 0},
  [152] = {.lex_state = 2},
  [153] = {.lex_state = 9},
  [154] = {.lex_state = 3},
  [155] = {.lex_state = 0},
  [156] = {.lex_state = 3},
  [157] = {.lex_state = 2},
  [158] = {.lex_state = 0},
  [159] = {.lex_state = 0},
  [160] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_rules_version] = ACTIONS(1),
    [anon_sym_EQ] = ACTIONS(1),
    [anon_sym_SEMI] = ACTIONS(1),
    [anon_sym_service] = ACTIONS(1),
    [anon_sym_cloud_DOTfirestore] = ACTIONS(1),
    [sym_comment] = ACTIONS(3),
    [anon_sym_true] = ACTIONS(1),
    [anon_sym_false] = ACTIONS(1),
    [sym_number] = ACTIONS(1),
    [sym_null] = ACTIONS(1),
    [anon_sym_DQUOTE] = ACTIONS(1),
    [anon_sym_SQUOTE] = ACTIONS(1),
    [sym_escape_sequence] = ACTIONS(1),
    [anon_sym_DOLLAR_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [anon_sym_SLASH] = ACTIONS(1),
    [anon_sym_get] = ACTIONS(1),
    [anon_sym_getAfter] = ACTIONS(1),
    [anon_sym_exists] = ACTIONS(1),
    [anon_sym_existsAfter] = ACTIONS(1),
    [anon_sym_debug] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
    [anon_sym_duration] = ACTIONS(1),
    [anon_sym_hashing] = ACTIONS(1),
    [anon_sym_latlng] = ACTIONS(1),
    [anon_sym_math] = ACTIONS(1),
    [anon_sym_timestamp] = ACTIONS(1),
    [anon_sym_request] = ACTIONS(1),
    [anon_sym_resource] = ACTIONS(1),
    [anon_sym_LPAREN2] = ACTIONS(1),
    [anon_sym_LBRACK] = ACTIONS(1),
    [anon_sym_RBRACK] = ACTIONS(1),
    [aux_sym_range_token1] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [anon_sym_DOT] = ACTIONS(1),
    [anon_sym_BANG] = ACTIONS(1),
    [anon_sym_DASH] = ACTIONS(1),
    [anon_sym_STAR] = ACTIONS(1),
    [anon_sym_PERCENT] = ACTIONS(1),
    [anon_sym_PLUS] = ACTIONS(1),
    [anon_sym_LT] = ACTIONS(1),
    [anon_sym_LT_EQ] = ACTIONS(1),
    [anon_sym_GT_EQ] = ACTIONS(1),
    [anon_sym_GT] = ACTIONS(1),
    [anon_sym_EQ_EQ] = ACTIONS(1),
    [anon_sym_BANG_EQ] = ACTIONS(1),
    [anon_sym_in] = ACTIONS(1),
    [anon_sym_AMP_AMP] = ACTIONS(1),
    [anon_sym_PIPE_PIPE] = ACTIONS(1),
    [anon_sym_QMARK] = ACTIONS(1),
    [anon_sym_let] = ACTIONS(1),
    [anon_sym_return] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [anon_sym_function] = ACTIONS(1),
    [sym_single_path_seg] = ACTIONS(1),
    [sym_multi_path_seg] = ACTIONS(1),
    [anon_sym_read] = ACTIONS(1),
    [anon_sym_write] = ACTIONS(1),
    [anon_sym_list] = ACTIONS(1),
    [anon_sym_create] = ACTIONS(1),
    [anon_sym_update] = ACTIONS(1),
    [anon_sym_delete] = ACTIONS(1),
    [anon_sym_allow] = ACTIONS(1),
    [anon_sym_if] = ACTIONS(1),
    [anon_sym_match] = ACTIONS(1),
  },
  [1] = {
    [sym_source_file] = STATE(144),
    [sym_rules_version_def] = STATE(122),
    [sym_service_name] = STATE(121),
    [anon_sym_rules_version] = ACTIONS(5),
    [anon_sym_service] = ACTIONS(7),
    [sym_comment] = ACTIONS(3),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 29,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      sym_identifier,
    ACTIONS(13), 1,
      sym_number,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_SQUOTE,
    ACTIONS(19), 1,
      anon_sym_RPAREN,
    ACTIONS(21), 1,
      anon_sym_SLASH,
    ACTIONS(27), 1,
      anon_sym_LPAREN2,
    ACTIONS(29), 1,
      anon_sym_LBRACK,
    ACTIONS(31), 1,
      anon_sym_BANG,
    ACTIONS(33), 1,
      anon_sym_DASH,
    STATE(12), 1,
      aux_sym_unary_repeat1,
    STATE(13), 1,
      aux_sym_unary_repeat2,
    STATE(26), 1,
      sym_namespace_reserved_variable,
    STATE(34), 1,
      sym_string,
    STATE(41), 1,
      sym_literal,
    STATE(42), 1,
      sym_primary,
    STATE(57), 1,
      sym_expr,
    STATE(93), 1,
      aux_sym_path_repeat1,
    STATE(109), 1,
      sym_function_argument,
    STATE(123), 1,
      sym_path,
    STATE(124), 1,
      sym_member_object,
    STATE(154), 1,
      sym_namespace_reserved_function,
    STATE(156), 1,
      sym_function_calling_name,
    ACTIONS(11), 3,
      anon_sym_true,
      anon_sym_false,
      sym_null,
    STATE(38), 4,
      sym_function_call,
      sym_variable,
      sym_expr_group,
      sym_list,
    ACTIONS(23), 5,
      anon_sym_get,
      anon_sym_getAfter,
      anon_sym_exists,
      anon_sym_existsAfter,
      anon_sym_debug,
    ACTIONS(25), 7,
      anon_sym_duration,
      anon_sym_hashing,
      anon_sym_latlng,
      anon_sym_math,
      anon_sym_timestamp,
      anon_sym_request,
      anon_sym_resource,
    STATE(54), 9,
      sym_indexing,
      sym_member,
      sym_unary,
      sym_multiplication,
      sym_addition,
      sym_relation,
      sym_conditional_and,
      sym_conditional_or,
      sym_ternary,
  [111] = 28,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      sym_identifier,
    ACTIONS(13), 1,
      sym_number,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_SQUOTE,
    ACTIONS(21), 1,
      anon_sym_SLASH,
    ACTIONS(27), 1,
      anon_sym_LPAREN2,
    ACTIONS(29), 1,
      anon_sym_LBRACK,
    ACTIONS(31), 1,
      anon_sym_BANG,
    ACTIONS(33), 1,
      anon_sym_DASH,
    STATE(12), 1,
      aux_sym_unary_repeat1,
    STATE(13), 1,
      aux_sym_unary_repeat2,
    STATE(26), 1,
      sym_namespace_reserved_variable,
    STATE(34), 1,
      sym_string,
    STATE(41), 1,
      sym_literal,
    STATE(42), 1,
      sym_primary,
    STATE(57), 1,
      sym_expr,
    STATE(93), 1,
      aux_sym_path_repeat1,
    STATE(123), 1,
      sym_path,
    STATE(124), 1,
      sym_member_object,
    STATE(130), 1,
      sym_function_argument,
    STATE(154), 1,
      sym_namespace_reserved_function,
    STATE(156), 1,
      sym_function_calling_name,
    ACTIONS(11), 3,
      anon_sym_true,
      anon_sym_false,
      sym_null,
    STATE(38), 4,
      sym_function_call,
      sym_variable,
      sym_expr_group,
      sym_list,
    ACTIONS(23), 5,
      anon_sym_get,
      anon_sym_getAfter,
      anon_sym_exists,
      anon_sym_existsAfter,
      anon_sym_debug,
    ACTIONS(25), 7,
      anon_sym_duration,
      anon_sym_hashing,
      anon_sym_latlng,
      anon_sym_math,
      anon_sym_timestamp,
      anon_sym_request,
      anon_sym_resource,
    STATE(54), 9,
      sym_indexing,
      sym_member,
      sym_unary,
      sym_multiplication,
      sym_addition,
      sym_relation,
      sym_conditional_and,
      sym_conditional_or,
      sym_ternary,
  [219] = 26,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      sym_identifier,
    ACTIONS(13), 1,
      sym_number,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_LPAREN2,
    ACTIONS(29), 1,
      anon_sym_LBRACK,
    ACTIONS(31), 1,
      anon_sym_BANG,
    ACTIONS(33), 1,
      anon_sym_DASH,
    ACTIONS(35), 1,
      aux_sym_range_token1,
    STATE(12), 1,
      aux_sym_unary_repeat1,
    STATE(13), 1,
      aux_sym_unary_repeat2,
    STATE(26), 1,
      sym_namespace_reserved_variable,
    STATE(34), 1,
      sym_string,
    STATE(41), 1,
      sym_literal,
    STATE(42), 1,
      sym_primary,
    STATE(60), 1,
      sym_expr,
    STATE(124), 1,
      sym_member_object,
    STATE(137), 1,
      sym_range,
    STATE(154), 1,
      sym_namespace_reserved_function,
    STATE(156), 1,
      sym_function_calling_name,
    ACTIONS(11), 3,
      anon_sym_true,
      anon_sym_false,
      sym_null,
    STATE(38), 4,
      sym_function_call,
      sym_variable,
      sym_expr_group,
      sym_list,
    ACTIONS(23), 5,
      anon_sym_get,
      anon_sym_getAfter,
      anon_sym_exists,
      anon_sym_existsAfter,
      anon_sym_debug,
    ACTIONS(25), 7,
      anon_sym_duration,
      anon_sym_hashing,
      anon_sym_latlng,
      anon_sym_math,
      anon_sym_timestamp,
      anon_sym_request,
      anon_sym_resource,
    STATE(54), 9,
      sym_indexing,
      sym_member,
      sym_unary,
      sym_multiplication,
      sym_addition,
      sym_relation,
      sym_conditional_and,
      sym_conditional_or,
      sym_ternary,
  [321] = 26,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      sym_identifier,
    ACTIONS(13), 1,
      sym_number,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_LPAREN2,
    ACTIONS(29), 1,
      anon_sym_LBRACK,
    ACTIONS(31), 1,
      anon_sym_BANG,
    ACTIONS(33), 1,
      anon_sym_DASH,
    ACTIONS(35), 1,
      aux_sym_range_token1,
    STATE(12), 1,
      aux_sym_unary_repeat1,
    STATE(13), 1,
      aux_sym_unary_repeat2,
    STATE(26), 1,
      sym_namespace_reserved_variable,
    STATE(34), 1,
      sym_string,
    STATE(41), 1,
      sym_literal,
    STATE(42), 1,
      sym_primary,
    STATE(67), 1,
      sym_expr,
    STATE(124), 1,
      sym_member_object,
    STATE(154), 1,
      sym_namespace_reserved_function,
    STATE(155), 1,
      sym_range,
    STATE(156), 1,
      sym_function_calling_name,
    ACTIONS(11), 3,
      anon_sym_true,
      anon_sym_false,
      sym_null,
    STATE(38), 4,
      sym_function_call,
      sym_variable,
      sym_expr_group,
      sym_list,
    ACTIONS(23), 5,
      anon_sym_get,
      anon_sym_getAfter,
      anon_sym_exists,
      anon_sym_existsAfter,
      anon_sym_debug,
    ACTIONS(25), 7,
      anon_sym_duration,
      anon_sym_hashing,
      anon_sym_latlng,
      anon_sym_math,
      anon_sym_timestamp,
      anon_sym_request,
      anon_sym_resource,
    STATE(54), 9,
      sym_indexing,
      sym_member,
      sym_unary,
      sym_multiplication,
      sym_addition,
      sym_relation,
      sym_conditional_and,
      sym_conditional_or,
      sym_ternary,
  [423] = 25,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      sym_identifier,
    ACTIONS(13), 1,
      sym_number,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_LPAREN2,
    ACTIONS(29), 1,
      anon_sym_LBRACK,
    ACTIONS(31), 1,
      anon_sym_BANG,
    ACTIONS(33), 1,
      anon_sym_DASH,
    ACTIONS(37), 1,
      anon_sym_RBRACK,
    STATE(12), 1,
      aux_sym_unary_repeat1,
    STATE(13), 1,
      aux_sym_unary_repeat2,
    STATE(26), 1,
      sym_namespace_reserved_variable,
    STATE(34), 1,
      sym_string,
    STATE(41), 1,
      sym_literal,
    STATE(42), 1,
      sym_primary,
    STATE(56), 1,
      sym_expr,
    STATE(124), 1,
      sym_member_object,
    STATE(154), 1,
      sym_namespace_reserved_function,
    STATE(156), 1,
      sym_function_calling_name,
    ACTIONS(11), 3,
      anon_sym_true,
      anon_sym_false,
      sym_null,
    STATE(38), 4,
      sym_function_call,
      sym_variable,
      sym_expr_group,
      sym_list,
    ACTIONS(23), 5,
      anon_sym_get,
      anon_sym_getAfter,
      anon_sym_exists,
      anon_sym_existsAfter,
      anon_sym_debug,
    ACTIONS(25), 7,
      anon_sym_duration,
      anon_sym_hashing,
      anon_sym_latlng,
      anon_sym_math,
      anon_sym_timestamp,
      anon_sym_request,
      anon_sym_resource,
    STATE(54), 9,
      sym_indexing,
      sym_member,
      sym_unary,
      sym_multiplication,
      sym_addition,
      sym_relation,
      sym_conditional_and,
      sym_conditional_or,
      sym_ternary,
  [522] = 24,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      sym_identifier,
    ACTIONS(13), 1,
      sym_number,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_LPAREN2,
    ACTIONS(29), 1,
      anon_sym_LBRACK,
    ACTIONS(31), 1,
      anon_sym_BANG,
    ACTIONS(33), 1,
      anon_sym_DASH,
    STATE(12), 1,
      aux_sym_unary_repeat1,
    STATE(13), 1,
      aux_sym_unary_repeat2,
    STATE(26), 1,
      sym_namespace_reserved_variable,
    STATE(34), 1,
      sym_string,
    STATE(41), 1,
      sym_literal,
    STATE(42), 1,
      sym_primary,
    STATE(46), 1,
      sym_expr,
    STATE(124), 1,
      sym_member_object,
    STATE(154), 1,
      sym_namespace_reserved_function,
    STATE(156), 1,
      sym_function_calling_name,
    ACTIONS(11), 3,
      anon_sym_true,
      anon_sym_false,
      sym_null,
    STATE(38), 4,
      sym_function_call,
      sym_variable,
      sym_expr_group,
      sym_list,
    ACTIONS(23), 5,
      anon_sym_get,
      anon_sym_getAfter,
      anon_sym_exists,
      anon_sym_existsAfter,
      anon_sym_debug,
    ACTIONS(25), 7,
      anon_sym_duration,
      anon_sym_hashing,
      anon_sym_latlng,
      anon_sym_math,
      anon_sym_timestamp,
      anon_sym_request,
      anon_sym_resource,
    STATE(54), 9,
      sym_indexing,
      sym_member,
      sym_unary,
      sym_multiplication,
      sym_addition,
      sym_relation,
      sym_conditional_and,
      sym_conditional_or,
      sym_ternary,
  [618] = 24,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      sym_identifier,
    ACTIONS(13), 1,
      sym_number,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_LPAREN2,
    ACTIONS(29), 1,
      anon_sym_LBRACK,
    ACTIONS(31), 1,
      anon_sym_BANG,
    ACTIONS(33), 1,
      anon_sym_DASH,
    STATE(12), 1,
      aux_sym_unary_repeat1,
    STATE(13), 1,
      aux_sym_unary_repeat2,
    STATE(26), 1,
      sym_namespace_reserved_variable,
    STATE(34), 1,
      sym_string,
    STATE(41), 1,
      sym_literal,
    STATE(42), 1,
      sym_primary,
    STATE(45), 1,
      sym_expr,
    STATE(124), 1,
      sym_member_object,
    STATE(154), 1,
      sym_namespace_reserved_function,
    STATE(156), 1,
      sym_function_calling_name,
    ACTIONS(11), 3,
      anon_sym_true,
      anon_sym_false,
      sym_null,
    STATE(38), 4,
      sym_function_call,
      sym_variable,
      sym_expr_group,
      sym_list,
    ACTIONS(23), 5,
      anon_sym_get,
      anon_sym_getAfter,
      anon_sym_exists,
      anon_sym_existsAfter,
      anon_sym_debug,
    ACTIONS(25), 7,
      anon_sym_duration,
      anon_sym_hashing,
      anon_sym_latlng,
      anon_sym_math,
      anon_sym_timestamp,
      anon_sym_request,
      anon_sym_resource,
    STATE(54), 9,
      sym_indexing,
      sym_member,
      sym_unary,
      sym_multiplication,
      sym_addition,
      sym_relation,
      sym_conditional_and,
      sym_conditional_or,
      sym_ternary,
  [714] = 24,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      sym_identifier,
    ACTIONS(13), 1,
      sym_number,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_LPAREN2,
    ACTIONS(29), 1,
      anon_sym_LBRACK,
    ACTIONS(31), 1,
      anon_sym_BANG,
    ACTIONS(33), 1,
      anon_sym_DASH,
    STATE(12), 1,
      aux_sym_unary_repeat1,
    STATE(13), 1,
      aux_sym_unary_repeat2,
    STATE(26), 1,
      sym_namespace_reserved_variable,
    STATE(34), 1,
      sym_string,
    STATE(41), 1,
      sym_literal,
    STATE(42), 1,
      sym_primary,
    STATE(63), 1,
      sym_expr,
    STATE(124), 1,
      sym_member_object,
    STATE(154), 1,
      sym_namespace_reserved_function,
    STATE(156), 1,
      sym_function_calling_name,
    ACTIONS(11), 3,
      anon_sym_true,
      anon_sym_false,
      sym_null,
    STATE(38), 4,
      sym_function_call,
      sym_variable,
      sym_expr_group,
      sym_list,
    ACTIONS(23), 5,
      anon_sym_get,
      anon_sym_getAfter,
      anon_sym_exists,
      anon_sym_existsAfter,
      anon_sym_debug,
    ACTIONS(25), 7,
      anon_sym_duration,
      anon_sym_hashing,
      anon_sym_latlng,
      anon_sym_math,
      anon_sym_timestamp,
      anon_sym_request,
      anon_sym_resource,
    STATE(54), 9,
      sym_indexing,
      sym_member,
      sym_unary,
      sym_multiplication,
      sym_addition,
      sym_relation,
      sym_conditional_and,
      sym_conditional_or,
      sym_ternary,
  [810] = 24,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      sym_identifier,
    ACTIONS(13), 1,
      sym_number,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_LPAREN2,
    ACTIONS(29), 1,
      anon_sym_LBRACK,
    ACTIONS(31), 1,
      anon_sym_BANG,
    ACTIONS(33), 1,
      anon_sym_DASH,
    STATE(12), 1,
      aux_sym_unary_repeat1,
    STATE(13), 1,
      aux_sym_unary_repeat2,
    STATE(26), 1,
      sym_namespace_reserved_variable,
    STATE(34), 1,
      sym_string,
    STATE(41), 1,
      sym_literal,
    STATE(42), 1,
      sym_primary,
    STATE(59), 1,
      sym_expr,
    STATE(124), 1,
      sym_member_object,
    STATE(154), 1,
      sym_namespace_reserved_function,
    STATE(156), 1,
      sym_function_calling_name,
    ACTIONS(11), 3,
      anon_sym_true,
      anon_sym_false,
      sym_null,
    STATE(38), 4,
      sym_function_call,
      sym_variable,
      sym_expr_group,
      sym_list,
    ACTIONS(23), 5,
      anon_sym_get,
      anon_sym_getAfter,
      anon_sym_exists,
      anon_sym_existsAfter,
      anon_sym_debug,
    ACTIONS(25), 7,
      anon_sym_duration,
      anon_sym_hashing,
      anon_sym_latlng,
      anon_sym_math,
      anon_sym_timestamp,
      anon_sym_request,
      anon_sym_resource,
    STATE(54), 9,
      sym_indexing,
      sym_member,
      sym_unary,
      sym_multiplication,
      sym_addition,
      sym_relation,
      sym_conditional_and,
      sym_conditional_or,
      sym_ternary,
  [906] = 24,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      sym_identifier,
    ACTIONS(13), 1,
      sym_number,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_LPAREN2,
    ACTIONS(29), 1,
      anon_sym_LBRACK,
    ACTIONS(31), 1,
      anon_sym_BANG,
    ACTIONS(33), 1,
      anon_sym_DASH,
    STATE(12), 1,
      aux_sym_unary_repeat1,
    STATE(13), 1,
      aux_sym_unary_repeat2,
    STATE(26), 1,
      sym_namespace_reserved_variable,
    STATE(34), 1,
      sym_string,
    STATE(41), 1,
      sym_literal,
    STATE(42), 1,
      sym_primary,
    STATE(62), 1,
      sym_expr,
    STATE(124), 1,
      sym_member_object,
    STATE(154), 1,
      sym_namespace_reserved_function,
    STATE(156), 1,
      sym_function_calling_name,
    ACTIONS(11), 3,
      anon_sym_true,
      anon_sym_false,
      sym_null,
    STATE(38), 4,
      sym_function_call,
      sym_variable,
      sym_expr_group,
      sym_list,
    ACTIONS(23), 5,
      anon_sym_get,
      anon_sym_getAfter,
      anon_sym_exists,
      anon_sym_existsAfter,
      anon_sym_debug,
    ACTIONS(25), 7,
      anon_sym_duration,
      anon_sym_hashing,
      anon_sym_latlng,
      anon_sym_math,
      anon_sym_timestamp,
      anon_sym_request,
      anon_sym_resource,
    STATE(54), 9,
      sym_indexing,
      sym_member,
      sym_unary,
      sym_multiplication,
      sym_addition,
      sym_relation,
      sym_conditional_and,
      sym_conditional_or,
      sym_ternary,
  [1002] = 24,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      sym_identifier,
    ACTIONS(13), 1,
      sym_number,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_LPAREN2,
    ACTIONS(29), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_DASH,
    ACTIONS(39), 1,
      anon_sym_BANG,
    STATE(13), 1,
      aux_sym_unary_repeat2,
    STATE(21), 1,
      aux_sym_unary_repeat1,
    STATE(26), 1,
      sym_namespace_reserved_variable,
    STATE(34), 1,
      sym_string,
    STATE(41), 1,
      sym_literal,
    STATE(42), 1,
      sym_primary,
    STATE(44), 1,
      sym_expr,
    STATE(124), 1,
      sym_member_object,
    STATE(154), 1,
      sym_namespace_reserved_function,
    STATE(156), 1,
      sym_function_calling_name,
    ACTIONS(11), 3,
      anon_sym_true,
      anon_sym_false,
      sym_null,
    STATE(38), 4,
      sym_function_call,
      sym_variable,
      sym_expr_group,
      sym_list,
    ACTIONS(23), 5,
      anon_sym_get,
      anon_sym_getAfter,
      anon_sym_exists,
      anon_sym_existsAfter,
      anon_sym_debug,
    ACTIONS(25), 7,
      anon_sym_duration,
      anon_sym_hashing,
      anon_sym_latlng,
      anon_sym_math,
      anon_sym_timestamp,
      anon_sym_request,
      anon_sym_resource,
    STATE(54), 9,
      sym_indexing,
      sym_member,
      sym_unary,
      sym_multiplication,
      sym_addition,
      sym_relation,
      sym_conditional_and,
      sym_conditional_or,
      sym_ternary,
  [1098] = 24,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      sym_identifier,
    ACTIONS(13), 1,
      sym_number,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_LPAREN2,
    ACTIONS(29), 1,
      anon_sym_LBRACK,
    ACTIONS(31), 1,
      anon_sym_BANG,
    ACTIONS(41), 1,
      anon_sym_DASH,
    STATE(12), 1,
      aux_sym_unary_repeat1,
    STATE(22), 1,
      aux_sym_unary_repeat2,
    STATE(26), 1,
      sym_namespace_reserved_variable,
    STATE(34), 1,
      sym_string,
    STATE(41), 1,
      sym_literal,
    STATE(42), 1,
      sym_primary,
    STATE(44), 1,
      sym_expr,
    STATE(124), 1,
      sym_member_object,
    STATE(154), 1,
      sym_namespace_reserved_function,
    STATE(156), 1,
      sym_function_calling_name,
    ACTIONS(11), 3,
      anon_sym_true,
      anon_sym_false,
      sym_null,
    STATE(38), 4,
      sym_function_call,
      sym_variable,
      sym_expr_group,
      sym_list,
    ACTIONS(23), 5,
      anon_sym_get,
      anon_sym_getAfter,
      anon_sym_exists,
      anon_sym_existsAfter,
      anon_sym_debug,
    ACTIONS(25), 7,
      anon_sym_duration,
      anon_sym_hashing,
      anon_sym_latlng,
      anon_sym_math,
      anon_sym_timestamp,
      anon_sym_request,
      anon_sym_resource,
    STATE(54), 9,
      sym_indexing,
      sym_member,
      sym_unary,
      sym_multiplication,
      sym_addition,
      sym_relation,
      sym_conditional_and,
      sym_conditional_or,
      sym_ternary,
  [1194] = 24,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      sym_identifier,
    ACTIONS(13), 1,
      sym_number,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_LPAREN2,
    ACTIONS(29), 1,
      anon_sym_LBRACK,
    ACTIONS(31), 1,
      anon_sym_BANG,
    ACTIONS(33), 1,
      anon_sym_DASH,
    STATE(12), 1,
      aux_sym_unary_repeat1,
    STATE(13), 1,
      aux_sym_unary_repeat2,
    STATE(26), 1,
      sym_namespace_reserved_variable,
    STATE(34), 1,
      sym_string,
    STATE(41), 1,
      sym_literal,
    STATE(42), 1,
      sym_primary,
    STATE(64), 1,
      sym_expr,
    STATE(124), 1,
      sym_member_object,
    STATE(154), 1,
      sym_namespace_reserved_function,
    STATE(156), 1,
      sym_function_calling_name,
    ACTIONS(11), 3,
      anon_sym_true,
      anon_sym_false,
      sym_null,
    STATE(38), 4,
      sym_function_call,
      sym_variable,
      sym_expr_group,
      sym_list,
    ACTIONS(23), 5,
      anon_sym_get,
      anon_sym_getAfter,
      anon_sym_exists,
      anon_sym_existsAfter,
      anon_sym_debug,
    ACTIONS(25), 7,
      anon_sym_duration,
      anon_sym_hashing,
      anon_sym_latlng,
      anon_sym_math,
      anon_sym_timestamp,
      anon_sym_request,
      anon_sym_resource,
    STATE(54), 9,
      sym_indexing,
      sym_member,
      sym_unary,
      sym_multiplication,
      sym_addition,
      sym_relation,
      sym_conditional_and,
      sym_conditional_or,
      sym_ternary,
  [1290] = 24,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      sym_identifier,
    ACTIONS(13), 1,
      sym_number,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_LPAREN2,
    ACTIONS(29), 1,
      anon_sym_LBRACK,
    ACTIONS(31), 1,
      anon_sym_BANG,
    ACTIONS(33), 1,
      anon_sym_DASH,
    STATE(12), 1,
      aux_sym_unary_repeat1,
    STATE(13), 1,
      aux_sym_unary_repeat2,
    STATE(26), 1,
      sym_namespace_reserved_variable,
    STATE(34), 1,
      sym_string,
    STATE(41), 1,
      sym_literal,
    STATE(42), 1,
      sym_primary,
    STATE(52), 1,
      sym_expr,
    STATE(124), 1,
      sym_member_object,
    STATE(154), 1,
      sym_namespace_reserved_function,
    STATE(156), 1,
      sym_function_calling_name,
    ACTIONS(11), 3,
      anon_sym_true,
      anon_sym_false,
      sym_null,
    STATE(38), 4,
      sym_function_call,
      sym_variable,
      sym_expr_group,
      sym_list,
    ACTIONS(23), 5,
      anon_sym_get,
      anon_sym_getAfter,
      anon_sym_exists,
      anon_sym_existsAfter,
      anon_sym_debug,
    ACTIONS(25), 7,
      anon_sym_duration,
      anon_sym_hashing,
      anon_sym_latlng,
      anon_sym_math,
      anon_sym_timestamp,
      anon_sym_request,
      anon_sym_resource,
    STATE(54), 9,
      sym_indexing,
      sym_member,
      sym_unary,
      sym_multiplication,
      sym_addition,
      sym_relation,
      sym_conditional_and,
      sym_conditional_or,
      sym_ternary,
  [1386] = 24,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      sym_identifier,
    ACTIONS(13), 1,
      sym_number,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_LPAREN2,
    ACTIONS(29), 1,
      anon_sym_LBRACK,
    ACTIONS(31), 1,
      anon_sym_BANG,
    ACTIONS(33), 1,
      anon_sym_DASH,
    STATE(12), 1,
      aux_sym_unary_repeat1,
    STATE(13), 1,
      aux_sym_unary_repeat2,
    STATE(26), 1,
      sym_namespace_reserved_variable,
    STATE(34), 1,
      sym_string,
    STATE(41), 1,
      sym_literal,
    STATE(42), 1,
      sym_primary,
    STATE(50), 1,
      sym_expr,
    STATE(124), 1,
      sym_member_object,
    STATE(154), 1,
      sym_namespace_reserved_function,
    STATE(156), 1,
      sym_function_calling_name,
    ACTIONS(11), 3,
      anon_sym_true,
      anon_sym_false,
      sym_null,
    STATE(38), 4,
      sym_function_call,
      sym_variable,
      sym_expr_group,
      sym_list,
    ACTIONS(23), 5,
      anon_sym_get,
      anon_sym_getAfter,
      anon_sym_exists,
      anon_sym_existsAfter,
      anon_sym_debug,
    ACTIONS(25), 7,
      anon_sym_duration,
      anon_sym_hashing,
      anon_sym_latlng,
      anon_sym_math,
      anon_sym_timestamp,
      anon_sym_request,
      anon_sym_resource,
    STATE(54), 9,
      sym_indexing,
      sym_member,
      sym_unary,
      sym_multiplication,
      sym_addition,
      sym_relation,
      sym_conditional_and,
      sym_conditional_or,
      sym_ternary,
  [1482] = 24,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      sym_identifier,
    ACTIONS(13), 1,
      sym_number,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_LPAREN2,
    ACTIONS(29), 1,
      anon_sym_LBRACK,
    ACTIONS(31), 1,
      anon_sym_BANG,
    ACTIONS(33), 1,
      anon_sym_DASH,
    STATE(12), 1,
      aux_sym_unary_repeat1,
    STATE(13), 1,
      aux_sym_unary_repeat2,
    STATE(26), 1,
      sym_namespace_reserved_variable,
    STATE(34), 1,
      sym_string,
    STATE(41), 1,
      sym_literal,
    STATE(42), 1,
      sym_primary,
    STATE(51), 1,
      sym_expr,
    STATE(124), 1,
      sym_member_object,
    STATE(154), 1,
      sym_namespace_reserved_function,
    STATE(156), 1,
      sym_function_calling_name,
    ACTIONS(11), 3,
      anon_sym_true,
      anon_sym_false,
      sym_null,
    STATE(38), 4,
      sym_function_call,
      sym_variable,
      sym_expr_group,
      sym_list,
    ACTIONS(23), 5,
      anon_sym_get,
      anon_sym_getAfter,
      anon_sym_exists,
      anon_sym_existsAfter,
      anon_sym_debug,
    ACTIONS(25), 7,
      anon_sym_duration,
      anon_sym_hashing,
      anon_sym_latlng,
      anon_sym_math,
      anon_sym_timestamp,
      anon_sym_request,
      anon_sym_resource,
    STATE(54), 9,
      sym_indexing,
      sym_member,
      sym_unary,
      sym_multiplication,
      sym_addition,
      sym_relation,
      sym_conditional_and,
      sym_conditional_or,
      sym_ternary,
  [1578] = 24,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      sym_identifier,
    ACTIONS(13), 1,
      sym_number,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_LPAREN2,
    ACTIONS(29), 1,
      anon_sym_LBRACK,
    ACTIONS(31), 1,
      anon_sym_BANG,
    ACTIONS(33), 1,
      anon_sym_DASH,
    STATE(12), 1,
      aux_sym_unary_repeat1,
    STATE(13), 1,
      aux_sym_unary_repeat2,
    STATE(26), 1,
      sym_namespace_reserved_variable,
    STATE(34), 1,
      sym_string,
    STATE(41), 1,
      sym_literal,
    STATE(42), 1,
      sym_primary,
    STATE(58), 1,
      sym_expr,
    STATE(124), 1,
      sym_member_object,
    STATE(154), 1,
      sym_namespace_reserved_function,
    STATE(156), 1,
      sym_function_calling_name,
    ACTIONS(11), 3,
      anon_sym_true,
      anon_sym_false,
      sym_null,
    STATE(38), 4,
      sym_function_call,
      sym_variable,
      sym_expr_group,
      sym_list,
    ACTIONS(23), 5,
      anon_sym_get,
      anon_sym_getAfter,
      anon_sym_exists,
      anon_sym_existsAfter,
      anon_sym_debug,
    ACTIONS(25), 7,
      anon_sym_duration,
      anon_sym_hashing,
      anon_sym_latlng,
      anon_sym_math,
      anon_sym_timestamp,
      anon_sym_request,
      anon_sym_resource,
    STATE(54), 9,
      sym_indexing,
      sym_member,
      sym_unary,
      sym_multiplication,
      sym_addition,
      sym_relation,
      sym_conditional_and,
      sym_conditional_or,
      sym_ternary,
  [1674] = 24,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      sym_identifier,
    ACTIONS(13), 1,
      sym_number,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_LPAREN2,
    ACTIONS(29), 1,
      anon_sym_LBRACK,
    ACTIONS(31), 1,
      anon_sym_BANG,
    ACTIONS(33), 1,
      anon_sym_DASH,
    STATE(12), 1,
      aux_sym_unary_repeat1,
    STATE(13), 1,
      aux_sym_unary_repeat2,
    STATE(26), 1,
      sym_namespace_reserved_variable,
    STATE(34), 1,
      sym_string,
    STATE(41), 1,
      sym_literal,
    STATE(42), 1,
      sym_primary,
    STATE(53), 1,
      sym_expr,
    STATE(124), 1,
      sym_member_object,
    STATE(154), 1,
      sym_namespace_reserved_function,
    STATE(156), 1,
      sym_function_calling_name,
    ACTIONS(11), 3,
      anon_sym_true,
      anon_sym_false,
      sym_null,
    STATE(38), 4,
      sym_function_call,
      sym_variable,
      sym_expr_group,
      sym_list,
    ACTIONS(23), 5,
      anon_sym_get,
      anon_sym_getAfter,
      anon_sym_exists,
      anon_sym_existsAfter,
      anon_sym_debug,
    ACTIONS(25), 7,
      anon_sym_duration,
      anon_sym_hashing,
      anon_sym_latlng,
      anon_sym_math,
      anon_sym_timestamp,
      anon_sym_request,
      anon_sym_resource,
    STATE(54), 9,
      sym_indexing,
      sym_member,
      sym_unary,
      sym_multiplication,
      sym_addition,
      sym_relation,
      sym_conditional_and,
      sym_conditional_or,
      sym_ternary,
  [1770] = 24,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      sym_identifier,
    ACTIONS(13), 1,
      sym_number,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_LPAREN2,
    ACTIONS(29), 1,
      anon_sym_LBRACK,
    ACTIONS(31), 1,
      anon_sym_BANG,
    ACTIONS(33), 1,
      anon_sym_DASH,
    STATE(12), 1,
      aux_sym_unary_repeat1,
    STATE(13), 1,
      aux_sym_unary_repeat2,
    STATE(26), 1,
      sym_namespace_reserved_variable,
    STATE(34), 1,
      sym_string,
    STATE(41), 1,
      sym_literal,
    STATE(42), 1,
      sym_primary,
    STATE(61), 1,
      sym_expr,
    STATE(124), 1,
      sym_member_object,
    STATE(154), 1,
      sym_namespace_reserved_function,
    STATE(156), 1,
      sym_function_calling_name,
    ACTIONS(11), 3,
      anon_sym_true,
      anon_sym_false,
      sym_null,
    STATE(38), 4,
      sym_function_call,
      sym_variable,
      sym_expr_group,
      sym_list,
    ACTIONS(23), 5,
      anon_sym_get,
      anon_sym_getAfter,
      anon_sym_exists,
      anon_sym_existsAfter,
      anon_sym_debug,
    ACTIONS(25), 7,
      anon_sym_duration,
      anon_sym_hashing,
      anon_sym_latlng,
      anon_sym_math,
      anon_sym_timestamp,
      anon_sym_request,
      anon_sym_resource,
    STATE(54), 9,
      sym_indexing,
      sym_member,
      sym_unary,
      sym_multiplication,
      sym_addition,
      sym_relation,
      sym_conditional_and,
      sym_conditional_or,
      sym_ternary,
  [1866] = 24,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      sym_identifier,
    ACTIONS(13), 1,
      sym_number,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_LPAREN2,
    ACTIONS(29), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_DASH,
    ACTIONS(39), 1,
      anon_sym_BANG,
    STATE(13), 1,
      aux_sym_unary_repeat2,
    STATE(21), 1,
      aux_sym_unary_repeat1,
    STATE(26), 1,
      sym_namespace_reserved_variable,
    STATE(34), 1,
      sym_string,
    STATE(41), 1,
      sym_literal,
    STATE(42), 1,
      sym_primary,
    STATE(44), 1,
      sym_expr,
    STATE(124), 1,
      sym_member_object,
    STATE(154), 1,
      sym_namespace_reserved_function,
    STATE(156), 1,
      sym_function_calling_name,
    ACTIONS(11), 3,
      anon_sym_true,
      anon_sym_false,
      sym_null,
    STATE(38), 4,
      sym_function_call,
      sym_variable,
      sym_expr_group,
      sym_list,
    ACTIONS(23), 5,
      anon_sym_get,
      anon_sym_getAfter,
      anon_sym_exists,
      anon_sym_existsAfter,
      anon_sym_debug,
    ACTIONS(25), 7,
      anon_sym_duration,
      anon_sym_hashing,
      anon_sym_latlng,
      anon_sym_math,
      anon_sym_timestamp,
      anon_sym_request,
      anon_sym_resource,
    STATE(54), 9,
      sym_indexing,
      sym_member,
      sym_unary,
      sym_multiplication,
      sym_addition,
      sym_relation,
      sym_conditional_and,
      sym_conditional_or,
      sym_ternary,
  [1962] = 24,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      sym_identifier,
    ACTIONS(13), 1,
      sym_number,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_LPAREN2,
    ACTIONS(29), 1,
      anon_sym_LBRACK,
    ACTIONS(31), 1,
      anon_sym_BANG,
    ACTIONS(41), 1,
      anon_sym_DASH,
    STATE(12), 1,
      aux_sym_unary_repeat1,
    STATE(22), 1,
      aux_sym_unary_repeat2,
    STATE(26), 1,
      sym_namespace_reserved_variable,
    STATE(34), 1,
      sym_string,
    STATE(41), 1,
      sym_literal,
    STATE(42), 1,
      sym_primary,
    STATE(44), 1,
      sym_expr,
    STATE(124), 1,
      sym_member_object,
    STATE(154), 1,
      sym_namespace_reserved_function,
    STATE(156), 1,
      sym_function_calling_name,
    ACTIONS(11), 3,
      anon_sym_true,
      anon_sym_false,
      sym_null,
    STATE(38), 4,
      sym_function_call,
      sym_variable,
      sym_expr_group,
      sym_list,
    ACTIONS(23), 5,
      anon_sym_get,
      anon_sym_getAfter,
      anon_sym_exists,
      anon_sym_existsAfter,
      anon_sym_debug,
    ACTIONS(25), 7,
      anon_sym_duration,
      anon_sym_hashing,
      anon_sym_latlng,
      anon_sym_math,
      anon_sym_timestamp,
      anon_sym_request,
      anon_sym_resource,
    STATE(54), 9,
      sym_indexing,
      sym_member,
      sym_unary,
      sym_multiplication,
      sym_addition,
      sym_relation,
      sym_conditional_and,
      sym_conditional_or,
      sym_ternary,
  [2058] = 24,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      sym_identifier,
    ACTIONS(13), 1,
      sym_number,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_LPAREN2,
    ACTIONS(29), 1,
      anon_sym_LBRACK,
    ACTIONS(31), 1,
      anon_sym_BANG,
    ACTIONS(33), 1,
      anon_sym_DASH,
    STATE(12), 1,
      aux_sym_unary_repeat1,
    STATE(13), 1,
      aux_sym_unary_repeat2,
    STATE(26), 1,
      sym_namespace_reserved_variable,
    STATE(34), 1,
      sym_string,
    STATE(41), 1,
      sym_literal,
    STATE(42), 1,
      sym_primary,
    STATE(66), 1,
      sym_expr,
    STATE(124), 1,
      sym_member_object,
    STATE(154), 1,
      sym_namespace_reserved_function,
    STATE(156), 1,
      sym_function_calling_name,
    ACTIONS(11), 3,
      anon_sym_true,
      anon_sym_false,
      sym_null,
    STATE(38), 4,
      sym_function_call,
      sym_variable,
      sym_expr_group,
      sym_list,
    ACTIONS(23), 5,
      anon_sym_get,
      anon_sym_getAfter,
      anon_sym_exists,
      anon_sym_existsAfter,
      anon_sym_debug,
    ACTIONS(25), 7,
      anon_sym_duration,
      anon_sym_hashing,
      anon_sym_latlng,
      anon_sym_math,
      anon_sym_timestamp,
      anon_sym_request,
      anon_sym_resource,
    STATE(54), 9,
      sym_indexing,
      sym_member,
      sym_unary,
      sym_multiplication,
      sym_addition,
      sym_relation,
      sym_conditional_and,
      sym_conditional_or,
      sym_ternary,
  [2154] = 24,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      sym_identifier,
    ACTIONS(13), 1,
      sym_number,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_LPAREN2,
    ACTIONS(29), 1,
      anon_sym_LBRACK,
    ACTIONS(31), 1,
      anon_sym_BANG,
    ACTIONS(33), 1,
      anon_sym_DASH,
    STATE(12), 1,
      aux_sym_unary_repeat1,
    STATE(13), 1,
      aux_sym_unary_repeat2,
    STATE(26), 1,
      sym_namespace_reserved_variable,
    STATE(34), 1,
      sym_string,
    STATE(41), 1,
      sym_literal,
    STATE(42), 1,
      sym_primary,
    STATE(65), 1,
      sym_expr,
    STATE(124), 1,
      sym_member_object,
    STATE(154), 1,
      sym_namespace_reserved_function,
    STATE(156), 1,
      sym_function_calling_name,
    ACTIONS(11), 3,
      anon_sym_true,
      anon_sym_false,
      sym_null,
    STATE(38), 4,
      sym_function_call,
      sym_variable,
      sym_expr_group,
      sym_list,
    ACTIONS(23), 5,
      anon_sym_get,
      anon_sym_getAfter,
      anon_sym_exists,
      anon_sym_existsAfter,
      anon_sym_debug,
    ACTIONS(25), 7,
      anon_sym_duration,
      anon_sym_hashing,
      anon_sym_latlng,
      anon_sym_math,
      anon_sym_timestamp,
      anon_sym_request,
      anon_sym_resource,
    STATE(54), 9,
      sym_indexing,
      sym_member,
      sym_unary,
      sym_multiplication,
      sym_addition,
      sym_relation,
      sym_conditional_and,
      sym_conditional_or,
      sym_ternary,
  [2250] = 20,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      sym_identifier,
    ACTIONS(13), 1,
      sym_number,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_SQUOTE,
    ACTIONS(27), 1,
      anon_sym_LPAREN2,
    ACTIONS(29), 1,
      anon_sym_LBRACK,
    STATE(26), 1,
      sym_namespace_reserved_variable,
    STATE(43), 1,
      sym_string,
    STATE(124), 1,
      sym_member_object,
    STATE(127), 1,
      sym_list,
    STATE(136), 1,
      sym_primary,
    STATE(154), 1,
      sym_namespace_reserved_function,
    STATE(156), 1,
      sym_function_calling_name,
    STATE(37), 2,
      sym_function_call,
      sym_variable,
    STATE(41), 2,
      sym_literal,
      sym_expr_group,
    STATE(49), 2,
      sym_field_indexing,
      sym_member,
    ACTIONS(11), 3,
      anon_sym_true,
      anon_sym_false,
      sym_null,
    ACTIONS(23), 5,
      anon_sym_get,
      anon_sym_getAfter,
      anon_sym_exists,
      anon_sym_existsAfter,
      anon_sym_debug,
    ACTIONS(25), 7,
      anon_sym_duration,
      anon_sym_hashing,
      anon_sym_latlng,
      anon_sym_math,
      anon_sym_timestamp,
      anon_sym_request,
      anon_sym_resource,
  [2326] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(43), 4,
      anon_sym_EQ,
      anon_sym_SLASH,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(45), 19,
      anon_sym_SEMI,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_in,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_QMARK,
  [2357] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(47), 1,
      anon_sym_LPAREN,
    ACTIONS(43), 3,
      anon_sym_SLASH,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(45), 19,
      anon_sym_SEMI,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_in,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_QMARK,
  [2390] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(49), 4,
      anon_sym_EQ,
      anon_sym_SLASH,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(51), 19,
      anon_sym_SEMI,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_in,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_QMARK,
  [2421] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(55), 3,
      anon_sym_SLASH,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(53), 19,
      anon_sym_SEMI,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_in,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_QMARK,
  [2451] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(59), 3,
      anon_sym_SLASH,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(57), 19,
      anon_sym_SEMI,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_in,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_QMARK,
  [2481] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(63), 3,
      anon_sym_SLASH,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(61), 19,
      anon_sym_SEMI,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_in,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_QMARK,
  [2511] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(67), 3,
      anon_sym_SLASH,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(65), 19,
      anon_sym_SEMI,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_in,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_QMARK,
  [2541] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(71), 3,
      anon_sym_SLASH,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(69), 19,
      anon_sym_SEMI,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_in,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_QMARK,
  [2571] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(77), 1,
      anon_sym_LBRACK,
    ACTIONS(75), 3,
      anon_sym_SLASH,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(73), 18,
      anon_sym_SEMI,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_in,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_QMARK,
  [2603] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(81), 3,
      anon_sym_SLASH,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(79), 19,
      anon_sym_SEMI,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_in,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_QMARK,
  [2633] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(85), 3,
      anon_sym_SLASH,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(83), 19,
      anon_sym_SEMI,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_in,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_QMARK,
  [2663] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(91), 1,
      anon_sym_LBRACK,
    ACTIONS(93), 1,
      anon_sym_DOT,
    ACTIONS(89), 3,
      anon_sym_SLASH,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(87), 17,
      anon_sym_SEMI,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_in,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_QMARK,
  [2697] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(77), 1,
      anon_sym_LBRACK,
    ACTIONS(95), 3,
      anon_sym_SLASH,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(93), 18,
      anon_sym_SEMI,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_in,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_QMARK,
  [2729] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(99), 3,
      anon_sym_SLASH,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(97), 19,
      anon_sym_SEMI,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_in,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_QMARK,
  [2759] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(103), 3,
      anon_sym_SLASH,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(101), 19,
      anon_sym_SEMI,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_in,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_QMARK,
  [2789] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(95), 3,
      anon_sym_SLASH,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(93), 18,
      anon_sym_SEMI,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_in,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_QMARK,
  [2818] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(109), 1,
      anon_sym_DOT,
    ACTIONS(107), 3,
      anon_sym_SLASH,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(105), 17,
      anon_sym_SEMI,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_in,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_QMARK,
  [2849] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(75), 3,
      anon_sym_SLASH,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(73), 18,
      anon_sym_SEMI,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_in,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_QMARK,
  [2878] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(113), 3,
      anon_sym_SLASH,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(111), 17,
      anon_sym_SEMI,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_in,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_QMARK,
  [2906] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(117), 1,
      anon_sym_SLASH,
    ACTIONS(127), 1,
      anon_sym_AMP_AMP,
    ACTIONS(129), 1,
      anon_sym_PIPE_PIPE,
    ACTIONS(131), 1,
      anon_sym_QMARK,
    ACTIONS(119), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(121), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(123), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(115), 5,
      anon_sym_SEMI,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
    ACTIONS(125), 5,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_in,
  [2948] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(117), 1,
      anon_sym_SLASH,
    ACTIONS(119), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(121), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(135), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(133), 13,
      anon_sym_SEMI,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_in,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_QMARK,
  [2982] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(139), 3,
      anon_sym_SLASH,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(137), 17,
      anon_sym_SEMI,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_in,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_QMARK,
  [3010] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(143), 3,
      anon_sym_SLASH,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(141), 17,
      anon_sym_SEMI,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_in,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_QMARK,
  [3038] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(89), 3,
      anon_sym_SLASH,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(87), 17,
      anon_sym_SEMI,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_in,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_QMARK,
  [3066] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(147), 3,
      anon_sym_SLASH,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(145), 17,
      anon_sym_SEMI,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_in,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_QMARK,
  [3094] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(117), 1,
      anon_sym_SLASH,
    ACTIONS(121), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(151), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(149), 15,
      anon_sym_SEMI,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DASH,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_in,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_QMARK,
  [3126] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(117), 1,
      anon_sym_SLASH,
    ACTIONS(119), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(121), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(123), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(125), 5,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_in,
    ACTIONS(153), 8,
      anon_sym_SEMI,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_QMARK,
  [3162] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(117), 1,
      anon_sym_SLASH,
    ACTIONS(127), 1,
      anon_sym_AMP_AMP,
    ACTIONS(119), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(121), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(123), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(125), 5,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_in,
    ACTIONS(155), 7,
      anon_sym_SEMI,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_PIPE_PIPE,
      anon_sym_QMARK,
  [3200] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(107), 3,
      anon_sym_SLASH,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(105), 17,
      anon_sym_SEMI,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_in,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_QMARK,
  [3228] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(159), 3,
      anon_sym_SLASH,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(157), 17,
      anon_sym_SEMI,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_in,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_QMARK,
  [3256] = 12,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(117), 1,
      anon_sym_SLASH,
    ACTIONS(127), 1,
      anon_sym_AMP_AMP,
    ACTIONS(129), 1,
      anon_sym_PIPE_PIPE,
    ACTIONS(131), 1,
      anon_sym_QMARK,
    ACTIONS(161), 1,
      anon_sym_COMMA,
    ACTIONS(163), 1,
      anon_sym_RBRACK,
    STATE(105), 1,
      aux_sym_list_repeat1,
    ACTIONS(119), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(121), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(123), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(125), 5,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_in,
  [3300] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(117), 1,
      anon_sym_SLASH,
    ACTIONS(127), 1,
      anon_sym_AMP_AMP,
    ACTIONS(129), 1,
      anon_sym_PIPE_PIPE,
    ACTIONS(131), 1,
      anon_sym_QMARK,
    ACTIONS(119), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(121), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(123), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(165), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
    ACTIONS(125), 5,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_in,
  [3339] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(117), 1,
      anon_sym_SLASH,
    ACTIONS(127), 1,
      anon_sym_AMP_AMP,
    ACTIONS(129), 1,
      anon_sym_PIPE_PIPE,
    ACTIONS(131), 1,
      anon_sym_QMARK,
    ACTIONS(119), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(121), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(123), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(167), 2,
      anon_sym_COMMA,
      anon_sym_RBRACK,
    ACTIONS(125), 5,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_in,
  [3378] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(117), 1,
      anon_sym_SLASH,
    ACTIONS(127), 1,
      anon_sym_AMP_AMP,
    ACTIONS(129), 1,
      anon_sym_PIPE_PIPE,
    ACTIONS(131), 1,
      anon_sym_QMARK,
    ACTIONS(169), 1,
      anon_sym_RPAREN,
    ACTIONS(119), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(121), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(123), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(125), 5,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_in,
  [3416] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(117), 1,
      anon_sym_SLASH,
    ACTIONS(127), 1,
      anon_sym_AMP_AMP,
    ACTIONS(129), 1,
      anon_sym_PIPE_PIPE,
    ACTIONS(131), 1,
      anon_sym_QMARK,
    ACTIONS(171), 1,
      anon_sym_RBRACK,
    ACTIONS(119), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(121), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(123), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(125), 5,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_in,
  [3454] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(117), 1,
      anon_sym_SLASH,
    ACTIONS(127), 1,
      anon_sym_AMP_AMP,
    ACTIONS(129), 1,
      anon_sym_PIPE_PIPE,
    ACTIONS(131), 1,
      anon_sym_QMARK,
    ACTIONS(173), 1,
      anon_sym_COLON,
    ACTIONS(119), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(121), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(123), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(125), 5,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_in,
  [3492] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(117), 1,
      anon_sym_SLASH,
    ACTIONS(127), 1,
      anon_sym_AMP_AMP,
    ACTIONS(129), 1,
      anon_sym_PIPE_PIPE,
    ACTIONS(131), 1,
      anon_sym_QMARK,
    ACTIONS(175), 1,
      anon_sym_SEMI,
    ACTIONS(119), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(121), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(123), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(125), 5,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_in,
  [3530] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(117), 1,
      anon_sym_SLASH,
    ACTIONS(127), 1,
      anon_sym_AMP_AMP,
    ACTIONS(129), 1,
      anon_sym_PIPE_PIPE,
    ACTIONS(131), 1,
      anon_sym_QMARK,
    ACTIONS(177), 1,
      anon_sym_SEMI,
    ACTIONS(119), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(121), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(123), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(125), 5,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_in,
  [3568] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(117), 1,
      anon_sym_SLASH,
    ACTIONS(127), 1,
      anon_sym_AMP_AMP,
    ACTIONS(129), 1,
      anon_sym_PIPE_PIPE,
    ACTIONS(131), 1,
      anon_sym_QMARK,
    ACTIONS(179), 1,
      anon_sym_SEMI,
    ACTIONS(119), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(121), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(123), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(125), 5,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_in,
  [3606] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(117), 1,
      anon_sym_SLASH,
    ACTIONS(127), 1,
      anon_sym_AMP_AMP,
    ACTIONS(129), 1,
      anon_sym_PIPE_PIPE,
    ACTIONS(131), 1,
      anon_sym_QMARK,
    ACTIONS(181), 1,
      anon_sym_RPAREN,
    ACTIONS(119), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(121), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(123), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(125), 5,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_in,
  [3644] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(117), 1,
      anon_sym_SLASH,
    ACTIONS(127), 1,
      anon_sym_AMP_AMP,
    ACTIONS(129), 1,
      anon_sym_PIPE_PIPE,
    ACTIONS(131), 1,
      anon_sym_QMARK,
    ACTIONS(183), 1,
      anon_sym_SEMI,
    ACTIONS(119), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(121), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(123), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(125), 5,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_in,
  [3682] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(117), 1,
      anon_sym_SLASH,
    ACTIONS(127), 1,
      anon_sym_AMP_AMP,
    ACTIONS(129), 1,
      anon_sym_PIPE_PIPE,
    ACTIONS(131), 1,
      anon_sym_QMARK,
    ACTIONS(185), 1,
      anon_sym_RBRACK,
    ACTIONS(119), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(121), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(123), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(125), 5,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_in,
  [3720] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(187), 1,
      sym_identifier,
    STATE(26), 1,
      sym_namespace_reserved_variable,
    STATE(157), 1,
      sym_variable,
    ACTIONS(25), 7,
      anon_sym_duration,
      anon_sym_hashing,
      anon_sym_latlng,
      anon_sym_math,
      anon_sym_timestamp,
      anon_sym_request,
      anon_sym_resource,
  [3742] = 3,
    ACTIONS(3), 1,
      sym_comment,
    STATE(91), 1,
      sym_method,
    ACTIONS(189), 7,
      anon_sym_get,
      anon_sym_read,
      anon_sym_write,
      anon_sym_list,
      anon_sym_create,
      anon_sym_update,
      anon_sym_delete,
  [3758] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(191), 1,
      anon_sym_RBRACE,
    ACTIONS(193), 1,
      anon_sym_function,
    ACTIONS(196), 1,
      anon_sym_allow,
    ACTIONS(199), 1,
      anon_sym_match,
    STATE(70), 4,
      sym_function_def,
      sym_rule_def,
      sym_match_def,
      aux_sym_service_body_repeat1,
  [3780] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(202), 1,
      anon_sym_RBRACE,
    ACTIONS(204), 1,
      anon_sym_function,
    ACTIONS(206), 1,
      anon_sym_allow,
    ACTIONS(208), 1,
      anon_sym_match,
    STATE(70), 4,
      sym_function_def,
      sym_rule_def,
      sym_match_def,
      aux_sym_service_body_repeat1,
  [3802] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(204), 1,
      anon_sym_function,
    ACTIONS(206), 1,
      anon_sym_allow,
    ACTIONS(208), 1,
      anon_sym_match,
    ACTIONS(210), 1,
      anon_sym_RBRACE,
    STATE(71), 4,
      sym_function_def,
      sym_rule_def,
      sym_match_def,
      aux_sym_service_body_repeat1,
  [3824] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(204), 1,
      anon_sym_function,
    ACTIONS(206), 1,
      anon_sym_allow,
    ACTIONS(208), 1,
      anon_sym_match,
    ACTIONS(212), 1,
      anon_sym_RBRACE,
    STATE(74), 4,
      sym_function_def,
      sym_rule_def,
      sym_match_def,
      aux_sym_service_body_repeat1,
  [3846] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(204), 1,
      anon_sym_function,
    ACTIONS(206), 1,
      anon_sym_allow,
    ACTIONS(208), 1,
      anon_sym_match,
    ACTIONS(214), 1,
      anon_sym_RBRACE,
    STATE(70), 4,
      sym_function_def,
      sym_rule_def,
      sym_match_def,
      aux_sym_service_body_repeat1,
  [3868] = 3,
    ACTIONS(3), 1,
      sym_comment,
    STATE(111), 1,
      sym_method,
    ACTIONS(189), 7,
      anon_sym_get,
      anon_sym_read,
      anon_sym_write,
      anon_sym_list,
      anon_sym_create,
      anon_sym_update,
      anon_sym_delete,
  [3884] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(216), 1,
      anon_sym_let,
    ACTIONS(218), 1,
      anon_sym_return,
    STATE(142), 1,
      sym_fun_return,
    STATE(78), 2,
      sym_variable_def,
      aux_sym_function_body_repeat1,
  [3901] = 4,
    ACTIONS(3), 1,
      sym_comment,
    STATE(79), 1,
      aux_sym_match_path_repeat1,
    STATE(128), 1,
      sym_match_path,
    ACTIONS(220), 3,
      sym_collection_path_seg,
      sym_single_path_seg,
      sym_multi_path_seg,
  [3916] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(216), 1,
      anon_sym_let,
    ACTIONS(218), 1,
      anon_sym_return,
    STATE(133), 1,
      sym_fun_return,
    STATE(84), 2,
      sym_variable_def,
      aux_sym_function_body_repeat1,
  [3933] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(222), 1,
      anon_sym_LBRACE,
    STATE(80), 1,
      aux_sym_match_path_repeat1,
    ACTIONS(224), 3,
      sym_collection_path_seg,
      sym_single_path_seg,
      sym_multi_path_seg,
  [3948] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(226), 1,
      anon_sym_LBRACE,
    STATE(80), 1,
      aux_sym_match_path_repeat1,
    ACTIONS(228), 3,
      sym_collection_path_seg,
      sym_single_path_seg,
      sym_multi_path_seg,
  [3963] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(233), 1,
      anon_sym_SLASH,
    STATE(81), 1,
      aux_sym_path_repeat1,
    ACTIONS(231), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [3977] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(238), 1,
      anon_sym_COMMA,
    STATE(82), 1,
      aux_sym_rule_def_repeat1,
    ACTIONS(236), 2,
      anon_sym_SEMI,
      anon_sym_COLON,
  [3991] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(241), 4,
      anon_sym_RBRACE,
      anon_sym_function,
      anon_sym_allow,
      anon_sym_match,
  [4001] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(243), 1,
      anon_sym_let,
    ACTIONS(246), 1,
      anon_sym_return,
    STATE(84), 2,
      sym_variable_def,
      aux_sym_function_body_repeat1,
  [4015] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(248), 4,
      anon_sym_RBRACE,
      anon_sym_function,
      anon_sym_allow,
      anon_sym_match,
  [4025] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(250), 4,
      anon_sym_RBRACE,
      anon_sym_function,
      anon_sym_allow,
      anon_sym_match,
  [4035] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(252), 4,
      anon_sym_RBRACE,
      anon_sym_function,
      anon_sym_allow,
      anon_sym_match,
  [4045] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(254), 4,
      anon_sym_RBRACE,
      anon_sym_function,
      anon_sym_allow,
      anon_sym_match,
  [4055] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(256), 1,
      anon_sym_SEMI,
    ACTIONS(258), 1,
      anon_sym_COMMA,
    ACTIONS(260), 1,
      anon_sym_COLON,
    STATE(82), 1,
      aux_sym_rule_def_repeat1,
  [4071] = 4,
    ACTIONS(262), 1,
      sym_comment,
    ACTIONS(264), 1,
      anon_sym_SQUOTE,
    STATE(94), 1,
      aux_sym_string_repeat2,
    ACTIONS(266), 2,
      sym_unescaped_single_string_fragment,
      sym_escape_sequence,
  [4085] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(258), 1,
      anon_sym_COMMA,
    ACTIONS(268), 1,
      anon_sym_SEMI,
    ACTIONS(270), 1,
      anon_sym_COLON,
    STATE(89), 1,
      aux_sym_rule_def_repeat1,
  [4101] = 4,
    ACTIONS(262), 1,
      sym_comment,
    ACTIONS(272), 1,
      anon_sym_DQUOTE,
    STATE(92), 1,
      aux_sym_string_repeat1,
    ACTIONS(274), 2,
      sym_unescaped_double_string_fragment,
      sym_escape_sequence,
  [4115] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(21), 1,
      anon_sym_SLASH,
    STATE(81), 1,
      aux_sym_path_repeat1,
    ACTIONS(277), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [4129] = 4,
    ACTIONS(262), 1,
      sym_comment,
    ACTIONS(279), 1,
      anon_sym_SQUOTE,
    STATE(99), 1,
      aux_sym_string_repeat2,
    ACTIONS(281), 2,
      sym_unescaped_single_string_fragment,
      sym_escape_sequence,
  [4143] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(283), 4,
      anon_sym_RBRACE,
      anon_sym_function,
      anon_sym_allow,
      anon_sym_match,
  [4153] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(285), 4,
      anon_sym_RBRACE,
      anon_sym_function,
      anon_sym_allow,
      anon_sym_match,
  [4163] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(287), 4,
      anon_sym_RBRACE,
      anon_sym_function,
      anon_sym_allow,
      anon_sym_match,
  [4173] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(289), 4,
      anon_sym_RBRACE,
      anon_sym_function,
      anon_sym_allow,
      anon_sym_match,
  [4183] = 4,
    ACTIONS(262), 1,
      sym_comment,
    ACTIONS(291), 1,
      anon_sym_SQUOTE,
    STATE(99), 1,
      aux_sym_string_repeat2,
    ACTIONS(293), 2,
      sym_unescaped_single_string_fragment,
      sym_escape_sequence,
  [4197] = 4,
    ACTIONS(262), 1,
      sym_comment,
    ACTIONS(279), 1,
      anon_sym_DQUOTE,
    STATE(92), 1,
      aux_sym_string_repeat1,
    ACTIONS(296), 2,
      sym_unescaped_double_string_fragment,
      sym_escape_sequence,
  [4211] = 4,
    ACTIONS(262), 1,
      sym_comment,
    ACTIONS(264), 1,
      anon_sym_DQUOTE,
    STATE(100), 1,
      aux_sym_string_repeat1,
    ACTIONS(298), 2,
      sym_unescaped_double_string_fragment,
      sym_escape_sequence,
  [4225] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(300), 4,
      anon_sym_RBRACE,
      anon_sym_function,
      anon_sym_allow,
      anon_sym_match,
  [4235] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(302), 4,
      anon_sym_RBRACE,
      anon_sym_function,
      anon_sym_allow,
      anon_sym_match,
  [4245] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(304), 1,
      anon_sym_RPAREN,
    ACTIONS(306), 1,
      anon_sym_COMMA,
    STATE(104), 1,
      aux_sym_function_call_repeat1,
  [4258] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(161), 1,
      anon_sym_COMMA,
    ACTIONS(309), 1,
      anon_sym_RBRACK,
    STATE(115), 1,
      aux_sym_list_repeat1,
  [4271] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(311), 1,
      anon_sym_RPAREN,
    ACTIONS(313), 1,
      anon_sym_COMMA,
    STATE(113), 1,
      aux_sym_param_list_repeat1,
  [4284] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(15), 1,
      anon_sym_DQUOTE,
    ACTIONS(17), 1,
      anon_sym_SQUOTE,
    STATE(147), 1,
      sym_string,
  [4297] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(315), 1,
      sym_identifier,
    ACTIONS(317), 1,
      anon_sym_DOLLAR_LPAREN,
    STATE(118), 1,
      sym_path_segment,
  [4310] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(319), 1,
      anon_sym_RPAREN,
    ACTIONS(321), 1,
      anon_sym_COMMA,
    STATE(117), 1,
      aux_sym_function_call_repeat1,
  [4323] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(323), 1,
      anon_sym_RPAREN,
    ACTIONS(325), 1,
      anon_sym_COMMA,
    STATE(110), 1,
      aux_sym_param_list_repeat1,
  [4336] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(236), 3,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_COLON,
  [4345] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(328), 3,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_COLON,
  [4354] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(313), 1,
      anon_sym_COMMA,
    ACTIONS(330), 1,
      anon_sym_RPAREN,
    STATE(110), 1,
      aux_sym_param_list_repeat1,
  [4367] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(334), 1,
      anon_sym_SLASH,
    ACTIONS(332), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [4378] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(167), 1,
      anon_sym_RBRACK,
    ACTIONS(336), 1,
      anon_sym_COMMA,
    STATE(115), 1,
      aux_sym_list_repeat1,
  [4391] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(339), 1,
      sym_identifier,
    ACTIONS(341), 1,
      anon_sym_RPAREN,
    STATE(140), 1,
      sym_param_list,
  [4404] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(321), 1,
      anon_sym_COMMA,
    ACTIONS(343), 1,
      anon_sym_RPAREN,
    STATE(104), 1,
      aux_sym_function_call_repeat1,
  [4417] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(345), 1,
      anon_sym_SLASH,
    ACTIONS(231), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [4428] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(349), 1,
      anon_sym_SLASH,
    ACTIONS(347), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [4439] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(351), 1,
      anon_sym_LBRACE,
    STATE(148), 1,
      sym_service_body,
  [4449] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(351), 1,
      anon_sym_LBRACE,
    STATE(159), 1,
      sym_service_body,
  [4459] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(7), 1,
      anon_sym_service,
    STATE(120), 1,
      sym_service_name,
  [4469] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(165), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [4477] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(353), 1,
      anon_sym_DOT,
    STATE(47), 1,
      sym_member_field,
  [4487] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(355), 1,
      anon_sym_LBRACE,
    STATE(103), 1,
      sym_function_body,
  [4497] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(323), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [4505] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(91), 1,
      anon_sym_LBRACK,
    ACTIONS(93), 1,
      anon_sym_DOT,
  [4515] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(357), 1,
      anon_sym_LBRACE,
    STATE(97), 1,
      sym_match_body,
  [4525] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(355), 1,
      anon_sym_LBRACE,
    STATE(88), 1,
      sym_function_body,
  [4535] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(304), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [4543] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(359), 2,
      anon_sym_let,
      anon_sym_return,
  [4551] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(361), 1,
      anon_sym_RBRACE,
  [4558] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(363), 1,
      anon_sym_RBRACE,
  [4565] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(365), 1,
      sym_identifier,
  [4572] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(367), 1,
      anon_sym_COLON,
  [4579] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(109), 1,
      anon_sym_DOT,
  [4586] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(171), 1,
      anon_sym_RBRACK,
  [4593] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(369), 1,
      anon_sym_SLASHd_PLUS_SLASH,
  [4600] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(371), 1,
      anon_sym_cloud_DOTfirestore,
  [4607] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(373), 1,
      anon_sym_RPAREN,
  [4614] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(375), 1,
      anon_sym_EQ,
  [4621] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(377), 1,
      anon_sym_RBRACE,
  [4628] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(379), 1,
      ts_builtin_sym_end,
  [4635] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(381), 1,
      ts_builtin_sym_end,
  [4642] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(383), 1,
      anon_sym_service,
  [4649] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(385), 1,
      anon_sym_LPAREN,
  [4656] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(387), 1,
      anon_sym_SEMI,
  [4663] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(389), 1,
      ts_builtin_sym_end,
  [4670] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(391), 1,
      ts_builtin_sym_end,
  [4677] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(393), 1,
      anon_sym_RBRACK,
  [4684] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(395), 1,
      anon_sym_if,
  [4691] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(397), 1,
      anon_sym_LPAREN2,
  [4698] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(399), 1,
      sym_identifier,
  [4705] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(47), 1,
      anon_sym_LPAREN,
  [4712] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(185), 1,
      anon_sym_RBRACK,
  [4719] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(401), 1,
      anon_sym_LPAREN,
  [4726] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(403), 1,
      anon_sym_EQ,
  [4733] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(405), 1,
      anon_sym_LBRACE,
  [4740] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(407), 1,
      ts_builtin_sym_end,
  [4747] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(409), 1,
      anon_sym_if,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 111,
  [SMALL_STATE(4)] = 219,
  [SMALL_STATE(5)] = 321,
  [SMALL_STATE(6)] = 423,
  [SMALL_STATE(7)] = 522,
  [SMALL_STATE(8)] = 618,
  [SMALL_STATE(9)] = 714,
  [SMALL_STATE(10)] = 810,
  [SMALL_STATE(11)] = 906,
  [SMALL_STATE(12)] = 1002,
  [SMALL_STATE(13)] = 1098,
  [SMALL_STATE(14)] = 1194,
  [SMALL_STATE(15)] = 1290,
  [SMALL_STATE(16)] = 1386,
  [SMALL_STATE(17)] = 1482,
  [SMALL_STATE(18)] = 1578,
  [SMALL_STATE(19)] = 1674,
  [SMALL_STATE(20)] = 1770,
  [SMALL_STATE(21)] = 1866,
  [SMALL_STATE(22)] = 1962,
  [SMALL_STATE(23)] = 2058,
  [SMALL_STATE(24)] = 2154,
  [SMALL_STATE(25)] = 2250,
  [SMALL_STATE(26)] = 2326,
  [SMALL_STATE(27)] = 2357,
  [SMALL_STATE(28)] = 2390,
  [SMALL_STATE(29)] = 2421,
  [SMALL_STATE(30)] = 2451,
  [SMALL_STATE(31)] = 2481,
  [SMALL_STATE(32)] = 2511,
  [SMALL_STATE(33)] = 2541,
  [SMALL_STATE(34)] = 2571,
  [SMALL_STATE(35)] = 2603,
  [SMALL_STATE(36)] = 2633,
  [SMALL_STATE(37)] = 2663,
  [SMALL_STATE(38)] = 2697,
  [SMALL_STATE(39)] = 2729,
  [SMALL_STATE(40)] = 2759,
  [SMALL_STATE(41)] = 2789,
  [SMALL_STATE(42)] = 2818,
  [SMALL_STATE(43)] = 2849,
  [SMALL_STATE(44)] = 2878,
  [SMALL_STATE(45)] = 2906,
  [SMALL_STATE(46)] = 2948,
  [SMALL_STATE(47)] = 2982,
  [SMALL_STATE(48)] = 3010,
  [SMALL_STATE(49)] = 3038,
  [SMALL_STATE(50)] = 3066,
  [SMALL_STATE(51)] = 3094,
  [SMALL_STATE(52)] = 3126,
  [SMALL_STATE(53)] = 3162,
  [SMALL_STATE(54)] = 3200,
  [SMALL_STATE(55)] = 3228,
  [SMALL_STATE(56)] = 3256,
  [SMALL_STATE(57)] = 3300,
  [SMALL_STATE(58)] = 3339,
  [SMALL_STATE(59)] = 3378,
  [SMALL_STATE(60)] = 3416,
  [SMALL_STATE(61)] = 3454,
  [SMALL_STATE(62)] = 3492,
  [SMALL_STATE(63)] = 3530,
  [SMALL_STATE(64)] = 3568,
  [SMALL_STATE(65)] = 3606,
  [SMALL_STATE(66)] = 3644,
  [SMALL_STATE(67)] = 3682,
  [SMALL_STATE(68)] = 3720,
  [SMALL_STATE(69)] = 3742,
  [SMALL_STATE(70)] = 3758,
  [SMALL_STATE(71)] = 3780,
  [SMALL_STATE(72)] = 3802,
  [SMALL_STATE(73)] = 3824,
  [SMALL_STATE(74)] = 3846,
  [SMALL_STATE(75)] = 3868,
  [SMALL_STATE(76)] = 3884,
  [SMALL_STATE(77)] = 3901,
  [SMALL_STATE(78)] = 3916,
  [SMALL_STATE(79)] = 3933,
  [SMALL_STATE(80)] = 3948,
  [SMALL_STATE(81)] = 3963,
  [SMALL_STATE(82)] = 3977,
  [SMALL_STATE(83)] = 3991,
  [SMALL_STATE(84)] = 4001,
  [SMALL_STATE(85)] = 4015,
  [SMALL_STATE(86)] = 4025,
  [SMALL_STATE(87)] = 4035,
  [SMALL_STATE(88)] = 4045,
  [SMALL_STATE(89)] = 4055,
  [SMALL_STATE(90)] = 4071,
  [SMALL_STATE(91)] = 4085,
  [SMALL_STATE(92)] = 4101,
  [SMALL_STATE(93)] = 4115,
  [SMALL_STATE(94)] = 4129,
  [SMALL_STATE(95)] = 4143,
  [SMALL_STATE(96)] = 4153,
  [SMALL_STATE(97)] = 4163,
  [SMALL_STATE(98)] = 4173,
  [SMALL_STATE(99)] = 4183,
  [SMALL_STATE(100)] = 4197,
  [SMALL_STATE(101)] = 4211,
  [SMALL_STATE(102)] = 4225,
  [SMALL_STATE(103)] = 4235,
  [SMALL_STATE(104)] = 4245,
  [SMALL_STATE(105)] = 4258,
  [SMALL_STATE(106)] = 4271,
  [SMALL_STATE(107)] = 4284,
  [SMALL_STATE(108)] = 4297,
  [SMALL_STATE(109)] = 4310,
  [SMALL_STATE(110)] = 4323,
  [SMALL_STATE(111)] = 4336,
  [SMALL_STATE(112)] = 4345,
  [SMALL_STATE(113)] = 4354,
  [SMALL_STATE(114)] = 4367,
  [SMALL_STATE(115)] = 4378,
  [SMALL_STATE(116)] = 4391,
  [SMALL_STATE(117)] = 4404,
  [SMALL_STATE(118)] = 4417,
  [SMALL_STATE(119)] = 4428,
  [SMALL_STATE(120)] = 4439,
  [SMALL_STATE(121)] = 4449,
  [SMALL_STATE(122)] = 4459,
  [SMALL_STATE(123)] = 4469,
  [SMALL_STATE(124)] = 4477,
  [SMALL_STATE(125)] = 4487,
  [SMALL_STATE(126)] = 4497,
  [SMALL_STATE(127)] = 4505,
  [SMALL_STATE(128)] = 4515,
  [SMALL_STATE(129)] = 4525,
  [SMALL_STATE(130)] = 4535,
  [SMALL_STATE(131)] = 4543,
  [SMALL_STATE(132)] = 4551,
  [SMALL_STATE(133)] = 4558,
  [SMALL_STATE(134)] = 4565,
  [SMALL_STATE(135)] = 4572,
  [SMALL_STATE(136)] = 4579,
  [SMALL_STATE(137)] = 4586,
  [SMALL_STATE(138)] = 4593,
  [SMALL_STATE(139)] = 4600,
  [SMALL_STATE(140)] = 4607,
  [SMALL_STATE(141)] = 4614,
  [SMALL_STATE(142)] = 4621,
  [SMALL_STATE(143)] = 4628,
  [SMALL_STATE(144)] = 4635,
  [SMALL_STATE(145)] = 4642,
  [SMALL_STATE(146)] = 4649,
  [SMALL_STATE(147)] = 4656,
  [SMALL_STATE(148)] = 4663,
  [SMALL_STATE(149)] = 4670,
  [SMALL_STATE(150)] = 4677,
  [SMALL_STATE(151)] = 4684,
  [SMALL_STATE(152)] = 4691,
  [SMALL_STATE(153)] = 4698,
  [SMALL_STATE(154)] = 4705,
  [SMALL_STATE(155)] = 4712,
  [SMALL_STATE(156)] = 4719,
  [SMALL_STATE(157)] = 4726,
  [SMALL_STATE(158)] = 4733,
  [SMALL_STATE(159)] = 4740,
  [SMALL_STATE(160)] = 4747,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(141),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(139),
  [9] = {.entry = {.count = 1, .reusable = false}}, SHIFT(27),
  [11] = {.entry = {.count = 1, .reusable = false}}, SHIFT(43),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(43),
  [15] = {.entry = {.count = 1, .reusable = true}}, SHIFT(101),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(90),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [21] = {.entry = {.count = 1, .reusable = false}}, SHIFT(108),
  [23] = {.entry = {.count = 1, .reusable = false}}, SHIFT(146),
  [25] = {.entry = {.count = 1, .reusable = false}}, SHIFT(28),
  [27] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [29] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [31] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [33] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [35] = {.entry = {.count = 1, .reusable = false}}, SHIFT(135),
  [37] = {.entry = {.count = 1, .reusable = true}}, SHIFT(29),
  [39] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [41] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [43] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_variable, 1, 0, 0),
  [45] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_variable, 1, 0, 0),
  [47] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_calling_name, 1, 0, 0),
  [49] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_namespace_reserved_variable, 1, 0, 0),
  [51] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_namespace_reserved_variable, 1, 0, 0),
  [53] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_list, 2, 0, 0),
  [55] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_list, 2, 0, 0),
  [57] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string, 3, 0, 0),
  [59] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string, 3, 0, 0),
  [61] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 3, 0, 0),
  [63] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 3, 0, 0),
  [65] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_list, 4, 0, 0),
  [67] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_list, 4, 0, 0),
  [69] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_expr_group, 3, 0, 0),
  [71] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_expr_group, 3, 0, 0),
  [73] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_literal, 1, 0, 0),
  [75] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_literal, 1, 0, 0),
  [77] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [79] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_list, 3, 0, 0),
  [81] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_list, 3, 0, 0),
  [83] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 4, 0, 0),
  [85] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 4, 0, 0),
  [87] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_member_field, 2, 0, 0),
  [89] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_member_field, 2, 0, 0),
  [91] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [93] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_primary, 1, 0, 0),
  [95] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_primary, 1, 0, 0),
  [97] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string, 2, 0, 0),
  [99] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string, 2, 0, 0),
  [101] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 5, 0, 0),
  [103] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 5, 0, 0),
  [105] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_expr, 1, 0, 0),
  [107] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_expr, 1, 0, 0),
  [109] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_member_object, 1, 0, 0),
  [111] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_unary, 2, 0, 0),
  [113] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_unary, 2, 0, 0),
  [115] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ternary, 5, 0, 0),
  [117] = {.entry = {.count = 1, .reusable = false}}, SHIFT(16),
  [119] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [121] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [123] = {.entry = {.count = 1, .reusable = false}}, SHIFT(7),
  [125] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [127] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [129] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [131] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [133] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_relation, 3, 0, 0),
  [135] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_relation, 3, 0, 0),
  [137] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_member, 2, 0, 0),
  [139] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_member, 2, 0, 0),
  [141] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_field_indexing, 4, 0, 0),
  [143] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_field_indexing, 4, 0, 0),
  [145] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_multiplication, 3, 0, 0),
  [147] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_multiplication, 3, 0, 0),
  [149] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_addition, 3, 0, 0),
  [151] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_addition, 3, 0, 0),
  [153] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_conditional_and, 3, 0, 0),
  [155] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_conditional_or, 3, 0, 0),
  [157] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_indexing, 4, 0, 0),
  [159] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_indexing, 4, 0, 0),
  [161] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [163] = {.entry = {.count = 1, .reusable = true}}, SHIFT(35),
  [165] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_argument, 1, 0, 0),
  [167] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_list_repeat1, 2, 0, 0),
  [169] = {.entry = {.count = 1, .reusable = true}}, SHIFT(33),
  [171] = {.entry = {.count = 1, .reusable = true}}, SHIFT(55),
  [173] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [175] = {.entry = {.count = 1, .reusable = true}}, SHIFT(85),
  [177] = {.entry = {.count = 1, .reusable = true}}, SHIFT(98),
  [179] = {.entry = {.count = 1, .reusable = true}}, SHIFT(132),
  [181] = {.entry = {.count = 1, .reusable = true}}, SHIFT(119),
  [183] = {.entry = {.count = 1, .reusable = true}}, SHIFT(131),
  [185] = {.entry = {.count = 1, .reusable = true}}, SHIFT(48),
  [187] = {.entry = {.count = 1, .reusable = false}}, SHIFT(26),
  [189] = {.entry = {.count = 1, .reusable = true}}, SHIFT(112),
  [191] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_service_body_repeat1, 2, 0, 0),
  [193] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_service_body_repeat1, 2, 0, 0), SHIFT_REPEAT(153),
  [196] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_service_body_repeat1, 2, 0, 0), SHIFT_REPEAT(69),
  [199] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_service_body_repeat1, 2, 0, 0), SHIFT_REPEAT(77),
  [202] = {.entry = {.count = 1, .reusable = true}}, SHIFT(96),
  [204] = {.entry = {.count = 1, .reusable = true}}, SHIFT(153),
  [206] = {.entry = {.count = 1, .reusable = true}}, SHIFT(69),
  [208] = {.entry = {.count = 1, .reusable = true}}, SHIFT(77),
  [210] = {.entry = {.count = 1, .reusable = true}}, SHIFT(102),
  [212] = {.entry = {.count = 1, .reusable = true}}, SHIFT(149),
  [214] = {.entry = {.count = 1, .reusable = true}}, SHIFT(143),
  [216] = {.entry = {.count = 1, .reusable = true}}, SHIFT(68),
  [218] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [220] = {.entry = {.count = 1, .reusable = true}}, SHIFT(79),
  [222] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_match_path, 1, 0, 0),
  [224] = {.entry = {.count = 1, .reusable = true}}, SHIFT(80),
  [226] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_match_path_repeat1, 2, 0, 0),
  [228] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_match_path_repeat1, 2, 0, 0), SHIFT_REPEAT(80),
  [231] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_path_repeat1, 2, 0, 0),
  [233] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_path_repeat1, 2, 0, 0), SHIFT_REPEAT(108),
  [236] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_rule_def_repeat1, 2, 0, 0),
  [238] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_rule_def_repeat1, 2, 0, 0), SHIFT_REPEAT(75),
  [241] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_body, 3, 0, 0),
  [243] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_function_body_repeat1, 2, 0, 0), SHIFT_REPEAT(68),
  [246] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_function_body_repeat1, 2, 0, 0),
  [248] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_rule_def, 7, 0, 0),
  [250] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_rule_def, 3, 0, 0),
  [252] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_body, 4, 0, 0),
  [254] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_def, 6, 0, 1),
  [256] = {.entry = {.count = 1, .reusable = true}}, SHIFT(95),
  [258] = {.entry = {.count = 1, .reusable = true}}, SHIFT(75),
  [260] = {.entry = {.count = 1, .reusable = true}}, SHIFT(151),
  [262] = {.entry = {.count = 1, .reusable = false}}, SHIFT_EXTRA(),
  [264] = {.entry = {.count = 1, .reusable = false}}, SHIFT(39),
  [266] = {.entry = {.count = 1, .reusable = true}}, SHIFT(94),
  [268] = {.entry = {.count = 1, .reusable = true}}, SHIFT(86),
  [270] = {.entry = {.count = 1, .reusable = true}}, SHIFT(160),
  [272] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_string_repeat1, 2, 0, 0),
  [274] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_string_repeat1, 2, 0, 0), SHIFT_REPEAT(92),
  [277] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_path, 1, 0, 0),
  [279] = {.entry = {.count = 1, .reusable = false}}, SHIFT(30),
  [281] = {.entry = {.count = 1, .reusable = true}}, SHIFT(99),
  [283] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_rule_def, 4, 0, 0),
  [285] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_match_body, 3, 0, 0),
  [287] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_match_def, 3, 0, 0),
  [289] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_rule_def, 6, 0, 0),
  [291] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_string_repeat2, 2, 0, 0),
  [293] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_string_repeat2, 2, 0, 0), SHIFT_REPEAT(99),
  [296] = {.entry = {.count = 1, .reusable = true}}, SHIFT(92),
  [298] = {.entry = {.count = 1, .reusable = true}}, SHIFT(100),
  [300] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_match_body, 2, 0, 0),
  [302] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_def, 5, 0, 1),
  [304] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_function_call_repeat1, 2, 0, 0),
  [306] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_function_call_repeat1, 2, 0, 0), SHIFT_REPEAT(3),
  [309] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [311] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_param_list, 1, 0, 0),
  [313] = {.entry = {.count = 1, .reusable = true}}, SHIFT(134),
  [315] = {.entry = {.count = 1, .reusable = true}}, SHIFT(114),
  [317] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [319] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [321] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [323] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_param_list_repeat1, 2, 0, 0),
  [325] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_param_list_repeat1, 2, 0, 0), SHIFT_REPEAT(134),
  [328] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_method, 1, 0, 0),
  [330] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_param_list, 2, 0, 0),
  [332] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_path_segment, 1, 0, 0),
  [334] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_path_segment, 1, 0, 0),
  [336] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_list_repeat1, 2, 0, 0), SHIFT_REPEAT(18),
  [339] = {.entry = {.count = 1, .reusable = true}}, SHIFT(106),
  [341] = {.entry = {.count = 1, .reusable = true}}, SHIFT(125),
  [343] = {.entry = {.count = 1, .reusable = true}}, SHIFT(40),
  [345] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_path_repeat1, 2, 0, 0),
  [347] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_path_segment, 3, 0, 0),
  [349] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_path_segment, 3, 0, 0),
  [351] = {.entry = {.count = 1, .reusable = true}}, SHIFT(73),
  [353] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
  [355] = {.entry = {.count = 1, .reusable = true}}, SHIFT(76),
  [357] = {.entry = {.count = 1, .reusable = true}}, SHIFT(72),
  [359] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_variable_def, 5, 0, 0),
  [361] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fun_return, 3, 0, 0),
  [363] = {.entry = {.count = 1, .reusable = true}}, SHIFT(87),
  [365] = {.entry = {.count = 1, .reusable = true}}, SHIFT(126),
  [367] = {.entry = {.count = 1, .reusable = true}}, SHIFT(138),
  [369] = {.entry = {.count = 1, .reusable = true}}, SHIFT(150),
  [371] = {.entry = {.count = 1, .reusable = true}}, SHIFT(158),
  [373] = {.entry = {.count = 1, .reusable = true}}, SHIFT(129),
  [375] = {.entry = {.count = 1, .reusable = true}}, SHIFT(107),
  [377] = {.entry = {.count = 1, .reusable = true}}, SHIFT(83),
  [379] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_service_body, 3, 0, 0),
  [381] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [383] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_rules_version_def, 4, 0, 0),
  [385] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_namespace_reserved_function, 1, 0, 0),
  [387] = {.entry = {.count = 1, .reusable = true}}, SHIFT(145),
  [389] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 3, 0, 0),
  [391] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_service_body, 2, 0, 0),
  [393] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_range, 3, 0, 0),
  [395] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [397] = {.entry = {.count = 1, .reusable = true}}, SHIFT(116),
  [399] = {.entry = {.count = 1, .reusable = true}}, SHIFT(152),
  [401] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [403] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [405] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_service_name, 2, 0, 0),
  [407] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 2, 0, 0),
  [409] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef TREE_SITTER_HIDE_SYMBOLS
#define TS_PUBLIC
#elif defined(_WIN32)
#define TS_PUBLIC __declspec(dllexport)
#else
#define TS_PUBLIC __attribute__((visibility("default")))
#endif

TS_PUBLIC const TSLanguage *tree_sitter_firestore_rules(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
